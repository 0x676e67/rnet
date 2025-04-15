#[macro_export]
macro_rules! define_enum_with_conversion {
    ($(#[$meta:meta])* $enum_type:ident, $ffi_type:ty, $($variant:ident),* $(,)?) => {
        define_enum_with_conversion!($(#[$meta])* $enum_type, $ffi_type, $( ($variant, $variant) ),*);
    };

    ($(#[$meta:meta])* const, $enum_type:ident, $ffi_type:ty, $($variant:ident),* $(,)?) => {
        define_enum_with_conversion!($(#[$meta])* const, $enum_type, $ffi_type, $( ($variant, $variant) ),*);
    };

    ($(#[$meta:meta])* $enum_type:ident, $ffi_type:ty, $(($rust_variant:ident, $ffi_variant:ident)),* $(,)?) => {
        $(#[$meta])*
        #[cfg_attr(feature = "docs", pyo3_stub_gen::derive::gen_stub_pyclass_enum)]
        #[pyclass(eq, eq_int)]
        #[derive(Clone, Copy, PartialEq, Eq, Hash)]
        #[allow(non_camel_case_types)]
        #[allow(clippy::upper_case_acronyms)]
        pub enum $enum_type {
            $($rust_variant),*
        }

        impl $enum_type {
            pub fn into_ffi(self) -> $ffi_type {
                match self {
                    $(<$enum_type>::$rust_variant => <$ffi_type>::$ffi_variant,)*
                }
            }

            pub fn from_ffi(ffi: $ffi_type) -> Self {
                #[allow(unreachable_patterns)]
                match ffi {
                    $(<$ffi_type>::$ffi_variant => <$enum_type>::$rust_variant,)*
                    _ => unreachable!(),
                }
            }
        }
    };

    ($(#[$meta:meta])* const, $enum_type:ident, $ffi_type:ty, $(($rust_variant:ident, $ffi_variant:ident)),* $(,)?) => {
        $(#[$meta])*
        #[cfg_attr(feature = "docs", pyo3_stub_gen::derive::gen_stub_pyclass_enum)]
        #[pyclass(eq, eq_int)]
        #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
        #[allow(non_camel_case_types)]
        #[allow(clippy::upper_case_acronyms)]
        pub enum $enum_type {
            $($rust_variant),*
        }

        impl $enum_type {
            pub const fn into_ffi(self) -> $ffi_type {
                match self {
                    $(<$enum_type>::$rust_variant => <$ffi_type>::$ffi_variant,)*
                }
            }

            pub const fn from_ffi(ffi: $ffi_type) -> Self {
                #[allow(unreachable_patterns)]
                match ffi {
                    $(<$ffi_type>::$ffi_variant => <$enum_type>::$rust_variant,)*
                    _ => unreachable!(),
                }
            }
        }
    };
}

#[macro_export]
macro_rules! define_py_stub_gen {
    ($type_name:ty, $type_str:expr, $module:expr) => {
        #[cfg(feature = "docs")]
        impl pyo3_stub_gen::PyStubType for $type_name {
            fn type_output() -> pyo3_stub_gen::TypeInfo {
                pyo3_stub_gen::TypeInfo::with_module($type_str, $module.into())
            }
        }
    };
}

#[macro_export]
macro_rules! define_into_pyobject_todo {
    ($type_name:ty, $msg:expr) => {
        impl<'py> pyo3::conversion::IntoPyObject<'py> for $type_name {
            type Target = $type_name;
            type Output = pyo3::prelude::Bound<'py, Self::Target>;
            type Error = pyo3::PyErr;

            fn into_pyobject(self, _: pyo3::Python<'py>) -> Result<Self::Output, Self::Error> {
                todo!($msg);
            }
        }
    };

    ($type_name:ty) => {
        define_into_pyobject_todo!(
            $type_name,
            concat!(
                stringify!($type_name),
                "::into_pyobject is not implemented yet"
            )
        );
    };
}
