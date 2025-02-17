use crate::define_constants;
use pyo3::prelude::*;
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods};

/// A impersonate.
#[gen_stub_pyclass]
#[pyclass]
#[derive(Clone, Copy, Default, Debug)]
pub struct Impersonate(rquest::Impersonate);

#[gen_stub_pyclass]
#[pyclass]
#[derive(Clone, Copy, Default, Debug)]
pub struct ImpersonateOS(rquest::ImpersonateOS);

impl From<Impersonate> for rquest::Impersonate {
    fn from(impersonate: Impersonate) -> rquest::Impersonate {
        impersonate.0
    }
}

impl From<ImpersonateOS> for rquest::ImpersonateOS {
    fn from(impersonate: ImpersonateOS) -> rquest::ImpersonateOS {
        impersonate.0
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl Impersonate {
    /// Returns a string representation of the impersonate.
    fn __str__(&self) -> String {
        format!("{:?}", self.0)
    }

    /// Returns a string representation of the impersonate.
    fn __repr__(&self) -> String {
        self.__str__()
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl ImpersonateOS {
    /// Returns a string representation of the impersonate.
    fn __str__(&self) -> String {
        format!("{:?}", self.0)
    }

    /// Returns a string representation of the impersonate.
    fn __repr__(&self) -> String {
        self.__str__()
    }
}

define_constants!(
    Impersonate,
    rquest::Impersonate,
    Chrome100,
    Chrome101,
    Chrome104,
    Chrome105,
    Chrome106,
    Chrome107,
    Chrome108,
    Chrome109,
    Chrome114,
    Chrome116,
    Chrome117,
    Chrome118,
    Chrome119,
    Chrome120,
    Chrome123,
    Chrome124,
    Chrome126,
    Chrome127,
    Chrome128,
    Chrome129,
    Chrome130,
    Chrome131,
    Chrome133,
    Edge101,
    Edge122,
    Edge127,
    Edge131,
    Firefox109,
    Firefox117,
    Firefox128,
    Firefox133,
    Firefox135,
    FirefoxAndroid135,
    SafariIos17_2,
    SafariIos17_4_1,
    SafariIos16_5,
    Safari15_3,
    Safari15_5,
    Safari15_6_1,
    Safari16,
    Safari16_5,
    Safari17_0,
    Safari17_2_1,
    Safari17_4_1,
    Safari17_5,
    Safari18,
    SafariIPad18,
    Safari18_2,
    SafariIos18_1_1,
    OkHttp3_9,
    OkHttp3_11,
    OkHttp3_13,
    OkHttp3_14,
    OkHttp4_9,
    OkHttp4_10,
    OkHttp5
);

define_constants!(
    ImpersonateOS,
    rquest::ImpersonateOS,
    Windows,
    MacOS,
    Linux,
    Android,
    IOS
);
