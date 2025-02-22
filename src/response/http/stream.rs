use crate::error::wrap_rquest_error;
use bytes::Bytes;
use futures_util::{Stream, StreamExt};
use pyo3::{exceptions::PyStopAsyncIteration, prelude::*, IntoPyObjectExt};
use pyo3_async_runtimes::tokio::future_into_py;
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods};
use std::{pin::Pin, sync::Arc};
use tokio::sync::Mutex;

/// A bytes streaming response.
/// This is an asynchronous iterator that yields chunks of data from the response stream.
/// This is used to stream the response content.
/// This is used in the `stream` method of the `Response` class.
/// This is used in an asynchronous for loop in Python.
///
/// # Examples
///
/// ```python
/// import asyncio
/// import rnet
/// from rnet import Method, Impersonate
///
/// async def main():
///     resp = await rnet.get("https://httpbin.org/stream/20")
///     print("Status Code: ", resp.status_code)
///     print("Version: ", resp.version)
///     print("Response URL: ", resp.url)
///     print("Headers: ", resp.headers.to_dict())
///     print("Content-Length: ", resp.content_length)
///     print("Encoding: ", resp.encoding)
///     print("Remote Address: ", resp.remote_addr)
///
///     streamer = resp.stream()
///     async for chunk in streamer:
///         print("Chunk: ", chunk)
///         await asyncio.sleep(0.1)
///
/// if __name__ == "__main__":
///     asyncio.run(main())
/// ```
#[gen_stub_pyclass]
#[pyclass]
pub struct Streamer(
    pub  Arc<
        Mutex<
            Option<
                Pin<Box<dyn Stream<Item = Result<bytes::Bytes, rquest::Error>> + Send + 'static>>,
            >,
        >,
    >,
);

impl Streamer {
    /// Create a new `Streamer` instance.
    ///
    /// # Arguments
    ///
    /// * `stream` - A stream of bytes.
    ///
    /// # Returns
    ///
    /// A new `Streamer` instance.
    pub fn new(
        stream: impl Stream<Item = Result<Bytes, rquest::Error>> + Send + 'static,
    ) -> Streamer {
        Streamer(Arc::new(Mutex::new(Some(Box::pin(stream)))))
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl Streamer {
    /// Returns the `Streamer` instance itself to be used as an asynchronous iterator.
    ///
    /// This method allows the `Streamer` to be used in an asynchronous for loop in Python.
    ///
    /// # Returns
    ///
    /// The `Streamer` instance itself.
    #[inline(always)]
    fn __aiter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    /// Returns the next chunk of the response as an asynchronous iterator.
    ///
    /// This method implements the `__anext__` method for the `Streamer` class, allowing it to be
    /// used as an asynchronous iterator in Python. It returns the next chunk of the response or
    /// raises `PyStopAsyncIteration` if the iterator is exhausted.
    ///
    /// # Returns
    ///
    /// A `PyResult` containing an `Option<PyObject>`. If there is a next chunk, it returns `Some(PyObject)`.
    /// If the iterator is exhausted, it raises `PyStopAsyncIteration`.
    fn __anext__<'rt>(&self, py: Python<'rt>) -> PyResult<Option<Bound<'rt, PyAny>>> {
        // Here we clone the inner field, so we can use it
        // in our future.
        let streamer = self.0.clone();
        future_into_py(py, async move {
            // Here we lock the mutex to access the data inside
            // and call chunk() method to get the next value.
            let val = streamer
                .lock()
                .await
                .as_mut()
                .ok_or_else(|| PyStopAsyncIteration::new_err("The iterator is exhausted"))?
                .next()
                .await;

            match val {
                Some(Ok(val)) => {
                    // If we have a value, we return it as a PyObject.
                    Python::with_gil(|py| Ok(Some(val.into_bound_py_any(py)?.unbind())))
                }
                Some(Err(err)) => Err(wrap_rquest_error(err)),
                // Here we return PyStopAsyncIteration error,
                // because python needs exceptions to tell that iterator
                // has ended.
                None => Err(PyStopAsyncIteration::new_err("The iterator is exhausted")),
            }
        })
        .map(Some)
    }

    fn __aenter__<'a>(slf: PyRef<'a, Self>, py: Python<'a>) -> PyResult<Bound<'a, PyAny>> {
        let slf = slf.into_py_any(py)?;
        future_into_py(py, async move { Ok(slf) })
    }

    fn __aexit__<'a>(
        &'a mut self,
        py: Python<'a>,
        _exc_type: &Bound<'a, PyAny>,
        _exc_value: &Bound<'a, PyAny>,
        _traceback: &Bound<'a, PyAny>,
    ) -> PyResult<Bound<'a, PyAny>> {
        let streamer = self.0.clone();
        future_into_py(py, async move { Ok(drop(streamer.lock().await.take())) })
    }
}
