use bytes::Bytes;
use futures_util::Stream;
use pyo3::{
    PyObject, PyResult, Python,
    buffer::PyBuffer,
    exceptions::{PyTypeError, PyValueError},
};
use std::{pin::Pin, task::Context};

pub struct SyncStream {
    iter: PyObject,
}

pub struct AsyncStream {
    stream: Pin<Box<dyn Stream<Item = PyObject> + Send + Sync + 'static>>,
}

impl SyncStream {
    #[inline]
    pub fn new(iter: PyObject) -> Self {
        SyncStream { iter }
    }
}

impl AsyncStream {
    #[inline]
    pub fn new(stream: impl Stream<Item = PyObject> + Send + Sync + 'static) -> Self {
        AsyncStream {
            stream: Box::pin(stream),
        }
    }
}

impl Stream for SyncStream {
    type Item = PyResult<Bytes>;

    fn poll_next(
        self: Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        Python::with_gil(|py| {
            let next = self
                .iter
                .call_method0(py, "__next__")
                .ok()
                .map(|item| downcast_bound_bytes(py, item));
            py.allow_threads(|| std::task::Poll::Ready(next))
        })
    }
}

impl Stream for AsyncStream {
    type Item = PyResult<Bytes>;

    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        let waker = cx.waker();
        Python::with_gil(|py| {
            py.allow_threads(|| {
                self.stream
                    .as_mut()
                    .poll_next(&mut Context::from_waker(waker))
            })
            .map(|item| item.map(|item| downcast_bound_bytes(py, item)))
        })
    }
}

#[inline]
fn downcast_bound_bytes<'p>(py: Python<'p>, ob: PyObject) -> PyResult<Bytes> {
    let buffer: PyBuffer<u8> = PyBuffer::get(ob.bind(py))?;
    if !buffer.readonly() {
        return Err(PyValueError::new_err("Must be read-only byte buffer"));
    }

    let slice = buffer
        .as_slice(py)
        .ok_or_else(|| PyTypeError::new_err("Must be a contiguous sequence of bytes"))?;

    // issue: https://github.com/PyO3/pyo3/issues/2824
    // Safety: The slice is &[ReadOnlyCell<u8>]. A ReadOnlyCell has the same
    // memory representation as the underlying data; it's
    // #[repr(transparent)] newtype around UnsafeCell. And per Rust docs
    // "UnsafeCell<T> has the same in-memory representation as its inner
    // type T". So the main issue is whether the data is _really_ read-only.
    // We do the read-only check above, and yes a caller can probably somehow
    // lie, but if they do that, that's really their fault.
    let cbor: &[u8] = unsafe { std::mem::transmute(slice) };

    Ok(Bytes::from(cbor))
}

impl From<SyncStream> for rquest::Body {
    #[inline]
    fn from(iterator: SyncStream) -> Self {
        rquest::Body::wrap_stream(iterator)
    }
}

impl From<SyncStream> for rquest::multipart::Part {
    #[inline]
    fn from(iterator: SyncStream) -> Self {
        rquest::multipart::Part::stream(iterator)
    }
}

impl From<AsyncStream> for rquest::Body {
    #[inline]
    fn from(stream: AsyncStream) -> Self {
        rquest::Body::wrap_stream(stream)
    }
}

impl From<AsyncStream> for rquest::multipart::Part {
    #[inline]
    fn from(stream: AsyncStream) -> Self {
        rquest::multipart::Part::stream(stream)
    }
}
