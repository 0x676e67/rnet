use pyo3::{IntoPyObjectExt, call::PyCallArgs, prelude::*, types::PyDict};

use super::CheckedCompletor;

pub fn set_result(
    py: Python,
    event_loop: Bound<PyAny>,
    future: &Bound<PyAny>,
    result: PyResult<Py<PyAny>>,
) -> PyResult<()> {
    let none = py.None().into_bound(py);
    let (complete, val) = match result {
        Ok(val) => (future.getattr("set_result")?, val.into_pyobject(py)?),
        Err(err) => (future.getattr("set_exception")?, err.into_bound_py_any(py)?),
    };
    call_soon_threadsafe(
        &event_loop,
        &none,
        (CheckedCompletor, future, complete, val),
    )?;

    Ok(())
}

pub fn call_soon_threadsafe<'py>(
    event_loop: &Bound<'py, PyAny>,
    context: &Bound<PyAny>,
    args: impl PyCallArgs<'py>,
) -> PyResult<()> {
    let kwargs = PyDict::new(event_loop.py());
    kwargs.set_item("context", context)?;
    event_loop.call_method("call_soon_threadsafe", args, Some(&kwargs))?;
    Ok(())
}

pub fn dump_err(py: Python<'_>) -> impl FnOnce(PyErr) + '_ {
    move |e| {
        // We can't display Python exceptions via std::fmt::Display,
        // so print the error here manually.
        e.print_and_set_sys_last_vars(py);
    }
}
