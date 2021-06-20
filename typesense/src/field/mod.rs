//! Module with the common definitions for  the
//! [fields](https://github.com/typesense/typesense/blob/v0.19.0/include/field.)
//! available in Typesense.

use serde::{Deserialize, Serialize};

mod field_type;
pub use field_type::*;

/// Struct used to represent a [field](https://github.com/typesense/typesense/blob/v0.19.0/include/field.) in Typesense.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Field {
    /// Required. The name of the field.
    name: String,
    /// Required. The `FieldType` of the field.
    #[serde(rename = "type")]
    typesense_type: FieldType,
    /// Optional parameter. Indicates if the field is optional or not.
    #[serde(skip_serializing_if = "Option::is_none")]
    optional: Option<bool>,
    /// Optional parameter. Faceted fields are indexed verbatim without
    /// any tokenization or preprocessing. For example, if you are building
    /// a product search, color and brand could be defined as facet fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    facet: Option<bool>,
}

/// Builder for the `Field` struct.
#[derive(Debug, Default)]
pub struct FieldBuilder {
    name: Option<String>,
    typesense_type: Option<FieldType>,
    optional: Option<bool>,
    facet: Option<bool>,
}

impl FieldBuilder {
    /// Create a Builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Set name of the field.
    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    /// Set type of the field.
    pub fn typesense_type(mut self, typesense_type: FieldType) -> Self {
        self.typesense_type = Some(typesense_type);
        self
    }

    /// Set if field is facet.
    pub fn facet(mut self, facet: Option<bool>) -> Self {
        self.facet = facet;
        self
    }

    /// Set if field is optional.
    pub fn optional(mut self, optional: Option<bool>) -> Self {
        self.optional = optional;
        self
    }

    /// Create a `Field` with the current values of the builder,
    /// It can fail if the name or the typesense_type are not defined.
    pub fn build(self) -> Result<Field, Box<dyn std::error::Error>> {
        Ok(Field {
            name: self.name.ok_or("name is not set")?,
            typesense_type: self.typesense_type.ok_or("typesense_type is not set")?,
            optional: self.optional,
            facet: self.facet,
        })
    }
}
