#[macro_export]
macro_rules! apply_option {
    (apply_if_some, $builder:expr, $option:expr, $method:ident) => {
        if let Some(value) = $option.take() {
            $builder = $builder.$method(value);
        }
    };
    (apply_if_ok, $builder:expr, $result:expr, $method:ident) => {
        if let Ok(value) = $result() {
            $builder = $builder.$method(value);
        }
    };
    (apply_if_some_ref, $builder:expr, $option:expr, $method:ident) => {
        if let Some(value) = $option.take() {
            $builder = $builder.$method(&value);
        }
    };
    (apply_transformed_option, $builder:expr, $option:expr, $method:ident, $transform:expr) => {
        if let Some(value) = $option.take() {
            $builder = $builder.$method($transform(value));
        }
    };
    (apply_option_or_default, $builder:expr, $option:expr, $method:ident, $default:expr) => {
        if $option.unwrap_or($default) {
            $builder = $builder.$method();
        }
    };
    (apply_option_or_default_with_value, $builder:expr, $option:expr, $method:ident, $default:expr, $value:expr) => {
        if $option.unwrap_or($default) {
            $builder = $builder.$method($value);
        }
    };
}

#[macro_export]
macro_rules! define_enum_with_conversion {
    ($(#[$meta:meta])* $enum_type:ident, $ffi_type:ty, $($variant:ident),* $(,)?) => {
        $(#[$meta])*
        #[gen_stub_pyclass_enum]
        #[pyclass(eq, eq_int)]
        #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
        #[allow(non_camel_case_types)]
        pub enum $enum_type {
            $($variant),*
        }

        impl $enum_type {
            pub const fn into_ffi(self) -> $ffi_type {
                match self {
                    $(<$enum_type>::$variant => <$ffi_type>::$variant,)*
                }
            }

            pub fn from_ffi(ffi: $ffi_type) -> Self {
                #[allow(unreachable_patterns)]
                match ffi {
                    $(<$ffi_type>::$variant => <$enum_type>::$variant,)*
                    _ => unreachable!(),
                }
            }
        }

    };

    (const, $(#[$meta:meta])* $enum_type:ident, $ffi_type:ty, $($variant:ident),* $(,)?) => {
        $(#[$meta])*
        #[gen_stub_pyclass_enum]
        #[pyclass(eq, eq_int)]
        #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
        #[allow(non_camel_case_types)]
        pub enum $enum_type {
            $($variant),*
        }

        impl $enum_type {
            pub const fn into_ffi(self) -> $ffi_type {
                match self {
                    $(<$enum_type>::$variant => <$ffi_type>::$variant,)*
                }
            }

            pub const fn from_ffi(ffi: $ffi_type) -> Self {
                #[allow(unreachable_patterns)]
                match ffi {
                    $(<$ffi_type>::$variant => <$enum_type>::$variant,)*
                    _ => unreachable!(),
                }
            }
        }
    };
}
