use super::part::Part;
use crate::error::memory_error;
use pyo3::{prelude::*, types::PyTuple};
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods};
use rquest::multipart::Form;

/// A multipart form for a request.
#[gen_stub_pyclass]
#[pyclass]
pub struct Multipart(pub Option<Form>);

#[gen_stub_pymethods]
#[pymethods]
impl Multipart {
    /// Creates a new multipart form.
    #[new]
    #[pyo3(signature = (*parts))]
    pub fn new(parts: &Bound<PyTuple>) -> PyResult<Multipart> {
        let mut new_form = Form::new();
        for part in parts {
            let part = part.downcast::<Part>()?;
            let mut part = part.borrow_mut();
            new_form = part
                .name
                .take()
                .zip(part.inner.take())
                .map(|(name, inner)| new_form.part(name, inner))
                .ok_or_else(memory_error)?;
        }
        Ok(Multipart(Some(new_form)))
    }
}
