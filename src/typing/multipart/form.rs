use super::part::Part;
use crate::error::Error;
use pyo3::{prelude::*, types::PyTuple};
use wreq::multipart::Form;

/// A multipart form for a request.
#[pyclass(subclass)]
pub struct Multipart(pub Option<Form>);

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
                .ok_or_else(|| Error::Memory)?;
        }
        Ok(Multipart(Some(new_form)))
    }
}

pub struct MultipartExtractor(pub Form);

impl FromPyObject<'_> for MultipartExtractor {
    fn extract_bound(ob: &Bound<'_, PyAny>) -> PyResult<Self> {
        let form = ob.downcast::<Multipart>()?;
        form.borrow_mut()
            .0
            .take()
            .map(Self)
            .ok_or_else(|| Error::Memory)
            .map_err(Into::into)
    }
}
