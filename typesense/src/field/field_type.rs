use serde::{Deserialize, Serialize};

/// Types that are supported by [Typesense](https://github.com/typesense/typesense/blob/v0.19.0/include/field.h#L8).
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum FieldType {
    /// string
    String,
    /// int32
    Int32,
    /// int64
    Int64,
    /// float
    Float,
    /// bool
    Bool,
    /// string[]
    #[serde(rename = "string[]")]
    StringArray,
    /// int32[]
    #[serde(rename = "int32[]")]
    Int32Array,
    /// int64[]
    #[serde(rename = "int64[]")]
    Int64Array,
    /// float[]
    #[serde(rename = "float[]")]
    FloatArray,
    /// bool[]
    #[serde(rename = "bool[]")]
    BoolArray,
}

/// Trait that should implement each type of a document, in order to properly serialize the
/// Collection Schema according to the Typesense reference.
pub trait ToTypesenseField {
    /// Static function that should implement the types of the typesense documents.
    fn to_typesense_type() -> FieldType;
}

/// macro used internally to add implementations of ToTypesenseField for several rust types.
#[macro_export]
macro_rules! impl_to_typesense_field (
    ($from:ty, $typesense_variant:expr) => {
        impl ToTypesenseField for $from {
            fn to_typesense_type() -> FieldType {
                $typesense_variant
            }
        }
    };
);

impl_to_typesense_field!(String, FieldType::String);
impl_to_typesense_field!(u8, FieldType::Int32);
impl_to_typesense_field!(i32, FieldType::Int32);
impl_to_typesense_field!(i64, FieldType::Int64);
impl_to_typesense_field!(u32, FieldType::Int64);
impl_to_typesense_field!(usize, FieldType::Int64);
impl_to_typesense_field!(f32, FieldType::Float);
impl_to_typesense_field!(f64, FieldType::Float);
impl_to_typesense_field!(bool, FieldType::Bool);
impl_to_typesense_field!(Vec<String>, FieldType::StringArray);
impl_to_typesense_field!(Vec<i32>, FieldType::Int32Array);
impl_to_typesense_field!(Vec<i64>, FieldType::Int64Array);
impl_to_typesense_field!(Vec<f32>, FieldType::FloatArray);
impl_to_typesense_field!(Vec<f64>, FieldType::FloatArray);
impl_to_typesense_field!(Vec<bool>, FieldType::BoolArray);
