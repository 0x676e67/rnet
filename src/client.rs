use pyo3::prelude::*;
use std::ops::Deref;

#[pyclass]
#[derive(Clone, Debug)]
pub struct Client(rquest::Client);

impl Deref for Client {
    type Target = rquest::Client;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
