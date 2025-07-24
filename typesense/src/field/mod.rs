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
    #[inline]
    pub fn new(name: impl Into<String>, typesense_type: FieldType) -> Self {
        Self {
            name: name.into(),
            typesense_type,
            ..Default::default()
        }
    }

    /// Set if field is optional.
    #[inline]
    pub fn optional(mut self, optional: Option<bool>) -> Self {
        self.optional = optional;
        self
    }

    /// Set if field is facet.
    #[inline]
    pub fn facet(mut self, facet: Option<bool>) -> Self {
        self.facet = facet;
        self
    }

    /// Set if field is index.
    #[inline]
    pub fn index(mut self, index: Option<bool>) -> Self {
        self.index = index;
        self
    }

    /// Set field locale.
    #[inline]
    pub fn locale(mut self, locale: Option<String>) -> Self {
        self.locale = locale;
        self
    }

    /// Set sort attribute for field
    #[inline]
    pub fn sort(mut self, sort: Option<bool>) -> Self {
        self.sort = sort;
        self
    }

    /// Set infix attribute for field
    #[inline]
    pub fn infix(mut self, infix: Option<bool>) -> Self {
        self.infix = infix;
        self
    }

    /// Set num_dim attribute for field
    #[inline]
    pub fn num_dim(mut self, num_dim: Option<i32>) -> Self {
        self.num_dim = num_dim;
        self
    }

    /// Set drop attribute for field
    #[inline]
    pub fn drop(mut self, drop: Option<bool>) -> Self {
        self.drop = drop;
        self
    }

    /// Create a `Field` with the current values of the builder,
    /// It can fail if the name or the typesense_type are not defined.
    #[inline]
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
            ..Default::default()
        }
    }
}
