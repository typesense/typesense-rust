//! # Collection
//!
//! In Typesense, a group of related documents is called a collection. A collection
//! is roughly equivalent to a table in a relational database.
//!
use crate::field::Field;
pub use typesense_codegen::models::CollectionSchema;

/// Builder for the [CollectionSchema] struct.
#[derive(Debug, Default)]
pub struct CollectionSchemaBuilder {
    name: String,
    fields: Vec<Field>,
    default_sorting_field: Option<String>,
    token_separators: Option<Vec<String>>,
    enable_nested_fields: Option<bool>,
    symbols_to_index: Option<Vec<String>>,
}

impl CollectionSchemaBuilder {
    /// Create a builder for [CollectionSchema]
    #[inline]
    pub fn new(name: impl Into<String>, fields: Vec<Field>) -> Self {
        Self {
            name: name.into(),
            fields,
            ..Default::default()
        }
    }

    /// Insert field
    #[inline]
    pub fn field(mut self, field: Field) -> Self {
        self.fields.push(field);
        self
    }

    /// Set fields
    #[inline]
    pub fn fields(mut self, fields: &[Field]) -> Self {
        self.fields.extend_from_slice(fields);
        self
    }

    /// Set default sorting field
    #[inline]
    pub fn default_sorting_field(mut self, default_sorting_field: impl Into<String>) -> Self {
        self.default_sorting_field = Some(default_sorting_field.into());
        self
    }

    /// Set token separators
    #[inline]
    pub fn token_separators(mut self, token_separators: Vec<String>) -> Self {
        self.token_separators = Some(token_separators);
        self
    }

    /// Enable nested fields
    #[inline]
    pub fn enable_nested_fields(mut self, enable_nested_fields: Option<bool>) -> Self {
        self.enable_nested_fields = enable_nested_fields;
        self
    }

    /// Set symbols to index
    #[inline]
    pub fn symbols_to_index(mut self, symbols_to_index: Vec<String>) -> Self {
        self.symbols_to_index = Some(symbols_to_index);
        self
    }

    /// Create a `CollectionSchema` with the current values of the builder,
    /// It can fail if any of the required fields is not not defined.
    #[inline]
    pub fn build(self) -> CollectionSchema {
        CollectionSchema {
            name: self.name,
            fields: self.fields,
            default_sorting_field: self.default_sorting_field,
            token_separators: self.token_separators,
            enable_nested_fields: self.enable_nested_fields,
            symbols_to_index: self.symbols_to_index,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::field::*;
    use serde_json::json;

    #[test]
    fn collection_schema_serializes_as_expected() {
        let fields = [
            ("company_name", "string".to_owned(), None),
            ("num_employees", "int32".to_owned(), None),
            ("country", "string".to_owned(), Some(true)),
        ]
        .map(|(name, typesense_type, facet)| {
            FieldBuilder::new(name, typesense_type).facet(facet).build()
        })
        .to_vec();

        let collection = CollectionSchemaBuilder::new("companies", fields)
            .default_sorting_field("num_employees".to_owned())
            .build();

        let expected = json!(
            {
                "name": "companies",
                "fields": [
                  {
                    "name"  :  "company_name",
                    "type"  :  "string"
                  },
                  {
                    "name"  :  "num_employees",
                    "type"  :  "int32"
                  },
                  {
                    "name"  :  "country",
                    "type"  :  "string",
                    "facet" :  true
                  }
                ],
                "default_sorting_field": "num_employees"
              }
        );

        assert_eq!(serde_json::to_value(&collection).unwrap(), expected)
    }
}
