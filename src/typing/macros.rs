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
