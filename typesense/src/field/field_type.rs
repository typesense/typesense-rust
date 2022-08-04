/// Type for a field. Currently it is a wrapping to a `String` but it could be extended to a enum
pub type FieldType = String;

/// Trait that should implement each type of a document, in order to properly serialize the
/// Collection Schema according to the Typesense reference.
pub trait ToTypesenseField {
    /// Static function that should implement the types of the typesense documents.
    fn to_typesense_type() -> &'static str;
}

/// macro used internally to add implementations of ToTypesenseField for several rust types.
#[macro_export]
macro_rules! impl_to_typesense_field (
    ($from:ty, $typesense_variant:expr) => {
        impl ToTypesenseField for $from {
            fn to_typesense_type() -> &'static str {
                $typesense_variant
            }
        }
    };
);

impl_to_typesense_field!(String, "string");
impl_to_typesense_field!(u8, "int32");
impl_to_typesense_field!(i32, "int32");
impl_to_typesense_field!(i64, "int64");
impl_to_typesense_field!(u32, "int64");
impl_to_typesense_field!(usize, "int64");
impl_to_typesense_field!(f32, "float");
impl_to_typesense_field!(f64, "float");
impl_to_typesense_field!(bool, "bool");
impl_to_typesense_field!(Vec<String>, "string[]");
impl_to_typesense_field!(Vec<i32>, "int32[]");
impl_to_typesense_field!(Vec<i64>, "int64[]");
impl_to_typesense_field!(Vec<f32>, "float[]");
impl_to_typesense_field!(Vec<f64>, "float[]");
impl_to_typesense_field!(Vec<bool>, "bool[]");
