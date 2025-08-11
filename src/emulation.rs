use pyo3::prelude::*;

define_enum!(
    /// An emulation.
    const,
    Emulation,
    wreq_util::Emulation,
    Chrome100,
    Chrome101,
    Chrome104,
    Chrome105,
    Chrome106,
    Chrome107,
    Chrome108,
    Chrome109,
    Chrome110,
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
    Chrome132,
    Chrome133,
    Chrome134,
    Chrome135,
    Chrome136,
    Chrome137,
    Edge101,
    Edge122,
    Edge127,
    Edge131,
    Edge134,
    Firefox109,
    Firefox117,
    Firefox128,
    Firefox133,
    Firefox135,
    FirefoxPrivate135,
    FirefoxAndroid135,
    Firefox136,
    FirefoxPrivate136,
    Firefox139,
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
    Safari18_3,
    Safari18_3_1,
    SafariIos18_1_1,
    Safari18_5,
    OkHttp3_9,
    OkHttp3_11,
    OkHttp3_13,
    OkHttp3_14,
    OkHttp4_9,
    OkHttp4_10,
    OkHttp4_12,
    OkHttp5,
    Opera116,
    Opera117,
    Opera118,
    Opera119
);

define_enum!(
    /// An emulation operating system.
    const,
    EmulationOS,
    wreq_util::EmulationOS,
    Windows,
    MacOS,
    Linux,
    Android,
    IOS,
);

/// A struct to represent the `EmulationOption` class.
#[derive(Clone)]
#[pyclass(subclass)]
pub struct EmulationOption(pub wreq_util::EmulationOption);

#[pymethods]
impl EmulationOption {
    /// Create a new Emulation option instance.
    #[new]
    #[pyo3(signature = (
        emulation,
        emulation_os = None,
        skip_http2 = None,
        skip_headers = None
    ))]
    fn new(
        emulation: Emulation,
        emulation_os: Option<EmulationOS>,
        skip_http2: Option<bool>,
        skip_headers: Option<bool>,
    ) -> Self {
        let emulation = wreq_util::EmulationOption::builder()
            .emulation(emulation.into_ffi())
            .emulation_os(emulation_os.map(|os| os.into_ffi()).unwrap_or_default())
            .skip_http2(skip_http2.unwrap_or(false))
            .skip_headers(skip_headers.unwrap_or(false))
            .build();

        Self(emulation)
    }

    /// Creates a new random Emulation option instance.
    #[staticmethod]
    fn random() -> Self {
        Self(wreq_util::Emulation::random())
    }
}
