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
}

impl CollectionSchemaBuilder {
    /// Create a builder for [CollectionSchema]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ..Default::default()
        }
    }
    /// Insert field
    pub fn field(mut self, field: Field) -> Self {
        self.fields.push(field);
        self
    }

    /// Set fields
    pub fn fields(mut self, fields: &[Field]) -> Self {
        self.fields.extend_from_slice(fields);
        self
    }

    /// Set default_sorting_field
    pub fn default_sorting_field(mut self, default_sorting_field: String) -> Self {
        self.default_sorting_field = Some(default_sorting_field);
        self
    }

    /// Create a `CollectionSchema` with the current values of the builder,
    /// It can fail if any of the required fields is not not defined.
    pub fn build(self) -> CollectionSchema {
        CollectionSchema {
            name: self.name,
            fields: self.fields,
            default_sorting_field: self.default_sorting_field,
            token_separators: None,
            enable_nested_fields: None,
            symbols_to_index: None,
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
            ("company_name".to_owned(), "string".to_owned(), None),
            ("num_employees".to_owned(), "int32".to_owned(), None),
            ("country".to_owned(), "string".to_owned(), Some(true)),
        ]
        .map(|(name, typesense_type, facet)| {
            FieldBuilder::new(name, typesense_type).facet(facet).build()
        });

        let collection = CollectionSchemaBuilder::new("companies".to_owned())
            .default_sorting_field("num_employees".to_owned())
            .fields(&fields)
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
