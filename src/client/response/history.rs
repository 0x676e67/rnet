use pyo3::prelude::*;

use crate::http::header::HeaderMap;

/// An entry in the redirect history.
#[pyclass(subclass, frozen)]
pub struct History(wreq::redirect::History);

#[pymethods]
impl History {
    /// Get the status code of the redirect response.
    #[getter]
    fn status(&self) -> u16 {
        self.0.status().as_u16()
    }

    /// Get the URL of the redirect response.
    #[getter]
    fn url(&self) -> String {
        self.0.uri().to_string()
    }

    /// Get the previous URL before the redirect response.
    #[getter]
    fn previous(&self) -> String {
        self.0.previous().to_string()
    }

    #[getter]
    fn headers(&self) -> HeaderMap {
        HeaderMap(self.0.headers().clone())
    }
}

impl From<wreq::redirect::History> for History {
    fn from(history: wreq::redirect::History) -> Self {
        History(history)
    }
}
