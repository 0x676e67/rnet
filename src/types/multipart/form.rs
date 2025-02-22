use super::part::Part;
use crate::error::memory_error;
use pyo3::prelude::*;
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods};
use rquest::multipart::Form;

/// A multipart form for a request.
#[gen_stub_pyclass]
#[pyclass]
#[derive(Debug)]
pub struct Multipart(pub Option<Form>);

#[gen_stub_pymethods]
#[pymethods]
impl Multipart {
    /// Creates a new multipart form.
    #[new]
    #[pyo3(signature = (*parts))]
    pub fn new(py: Python, parts: Vec<Py<Part>>) -> PyResult<Multipart> {
        let mut new_form = rquest::multipart::Form::new();
        for part in parts {
            // Take the part's name and inner part
            let mut part = part.borrow_mut(py);
            new_form = new_form.part(
                part.name.take().ok_or_else(memory_error)?,
                part.inner.take().ok_or_else(memory_error)?,
            );
        }

        Ok(Multipart(Some(new_form)))
    }
}
