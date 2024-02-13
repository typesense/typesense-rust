//! Module with the common definitions for  the
//! [fields](https://github.com/typesense/typesense/blob/v0.19.0/include/field.)
//! available in Typesense.

mod field_type;
pub use field_type::*;
pub use typesense_codegen::models::{Field, FieldEmbed};

/// Builder for the `Field` struct.
#[derive(Debug, Default)]
pub struct FieldBuilder {
    name: String,
    typesense_type: FieldType,
    optional: Option<bool>,
    facet: Option<bool>,
    index: Option<bool>,
    locale: Option<String>,
    sort: Option<bool>,
    infix: Option<bool>,
    num_dim: Option<i32>,
    drop: Option<bool>,
    embed: Option<Box<FieldEmbed>>,
}

impl FieldBuilder {
    /// Create a Builder
    pub fn new(name: String, typesense_type: FieldType) -> Self {
        Self {
            name,
            typesense_type,
            ..Default::default()
        }
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

    /// Set if field is index.
    pub fn index(mut self, index: Option<bool>) -> Self {
        self.index = index;
        self
    }

    /// Set field locale.
    pub fn locale(mut self, locale: Option<String>) -> Self {
        self.locale = locale;
        self
    }

    /// Set sort attribute from field
    pub fn sort(mut self, sort: Option<bool>) -> Self {
        self.sort = sort;
        self
    }

    /// Set drop attribute from field
    pub fn drop(mut self, drop: Option<bool>) -> Self {
        self.drop = drop;
        self
    }

    /// Set infix attribute from field
    pub fn infix(mut self, infix: Option<bool>) -> Self {
        self.infix = infix;
        self
    }

    /// Create a `Field` with the current values of the builder,
    /// It can fail if the name or the typesense_type are not defined.
    pub fn build(self) -> Field {
        Field {
            name: self.name,
            r#type: self.typesense_type,
            optional: self.optional,
            facet: self.facet,
            index: self.index,
            locale: self.locale,
            sort: self.sort,
            infix: self.infix,
            num_dim: self.num_dim,
            drop: self.drop,
            embed: self.embed,
        }
    }
}
