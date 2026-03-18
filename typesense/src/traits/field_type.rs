use crate::traits::Document;
use std::collections::{BTreeMap, HashMap};
/// Type for a field. Currently it is a wrapping to a `String` but it could be extended to a enum
pub type FieldType = String;

/// Trait that should implement each type of a document, in order to properly serialize the
/// Collection Schema according to the Typesense reference.
pub trait ToTypesenseField {
    /// Mapping of a Typesense type.
    fn to_typesense_type() -> &'static str;
}
/// Generic implementation for any type that is also a Typesense document.
impl<T: Document> ToTypesenseField for T {
    #[inline(always)]
    fn to_typesense_type() -> &'static str {
        "object"
    }
}

/// Generic implementation for a Vec of any type that is also a Typesense document.
impl<T: Document> ToTypesenseField for Vec<T> {
    #[inline(always)]
    fn to_typesense_type() -> &'static str {
        "object[]"
    }
}

impl<T: ToTypesenseField> ToTypesenseField for Option<T> {
    #[inline(always)]
    fn to_typesense_type() -> &'static str {
        T::to_typesense_type()
    }
}

/// macro used internally to add implementations of ToTypesenseField for several rust types.
#[macro_export]
macro_rules! impl_to_typesense_field (
    ($for:ty, $typesense_type:expr) => {
        impl $crate::prelude::ToTypesenseField for $for {
            #[inline(always)]
            fn to_typesense_type() -> &'static str {
                $typesense_type
            }
        }
        impl $crate::prelude::ToTypesenseField for Vec<$for> {
            #[inline(always)]
            fn to_typesense_type() -> &'static str {
                concat!($typesense_type, "[]")
            }
        }
        impl $crate::prelude::ToTypesenseField for Vec<Option<$for>> {
            #[inline(always)]
            fn to_typesense_type() -> &'static str {
                concat!($typesense_type, "[]")
            }
        }
    };

    ($for:ty, $typesense_type:expr, $any:ident $(: $any_bound:path)?) => {
        impl<$any $(: $any_bound)?> $crate::prelude::ToTypesenseField for $for {
            #[inline(always)]
            fn to_typesense_type() -> &'static str {
                $typesense_type
            }
        }
        impl<$any $(: $any_bound)?> $crate::prelude::ToTypesenseField for Vec<$for> {
            #[inline(always)]
            fn to_typesense_type() -> &'static str {
                concat!($typesense_type, "[]")
            }
        }
        impl<$any $(: $any_bound)?> $crate::prelude::ToTypesenseField for Vec<Option<$for>> {
            #[inline(always)]
            fn to_typesense_type() -> &'static str {
                concat!($typesense_type, "[]")
            }
        }
    };
);

impl_to_typesense_field!(String, "string");
impl_to_typesense_field!(i8, "int32");
impl_to_typesense_field!(u8, "int32");
impl_to_typesense_field!(i16, "int32");
impl_to_typesense_field!(u16, "int32");
impl_to_typesense_field!(i32, "int32");
impl_to_typesense_field!(u32, "int64");
impl_to_typesense_field!(i64, "int64");
impl_to_typesense_field!(u64, "int64");
impl_to_typesense_field!(isize, "int64");
impl_to_typesense_field!(usize, "int64");
impl_to_typesense_field!(f32, "float");
impl_to_typesense_field!(f64, "float");
impl_to_typesense_field!(bool, "bool");
impl_to_typesense_field!(HashMap<String, T>, "object", T);
impl_to_typesense_field!(BTreeMap<String, T>, "object", T);

#[cfg(feature = "chrono")]
mod chrono_support {
    impl_to_typesense_field!(chrono::DateTime<T>, "string", T: chrono::TimeZone);
    impl_to_typesense_field!(chrono::NaiveDate, "string");
    impl_to_typesense_field!(chrono::NaiveDateTime, "string");
    impl_to_typesense_field!(chrono::NaiveTime, "string");
}
