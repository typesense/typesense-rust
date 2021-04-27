//! # Collection
//!
//! In Typesense, a group of related documents is called a collection. A collection
//! is roughly equivalent to a table in a relational database.
//!
use crate::field::Field;
use serde::{Deserialize, Serialize};

/// Schema used to create collections in the [Typesense API](https://typesense.org/docs/0.19.0/api/collections.html#create-a-collection).
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CollectionSchema {
    /// Name of the collection you wish to create.
    name: String,
    /// A list of fields that you wish to index for querying, filtering and faceting.
    fields: Vec<Field>,
    /// Optional: The name of an int32 / float field that determines the order in which the search
    /// results are ranked when a sort_by clause is not provided during searching.
    /// When not present, text match score and insertion order are used
    #[serde(skip_serializing_if = "Option::is_none")]
    default_sorting_field: Option<String>,
}

/// Builder for the [CollectionSchema] struct.
#[derive(Debug, Default)]
pub struct CollectionSchemaBuilder {
    name: Option<String>,
    fields: Option<Vec<Field>>,
    default_sorting_field: Option<String>,
}

impl CollectionSchemaBuilder {
    /// Create a builder for [CollectionSchema]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set name
    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }
    /// Insert field
    pub fn field(mut self, field: Field) -> Self {
        self.fields = if let Some(mut f) = self.fields {
            f.push(field);
            Some(f)
        } else {
            Some(vec![field])
        };
        self
    }

    /// Set fields
    pub fn fields(mut self, fields: Vec<Field>) -> Self {
        self.fields = Some(fields);
        self
    }

    /// Set default_sorting_field
    pub fn default_sorting_field(mut self, default_sorting_field: String) -> Self {
        self.default_sorting_field = Some(default_sorting_field);
        self
    }

    /// Create a `CollectionSchema` with the current values of the builder,
    /// It can fail if any of the required fields is not not defined.
    pub fn build(self) -> Result<CollectionSchema, Box<dyn std::error::Error>> {
        Ok(CollectionSchema {
            name: self.name.ok_or("name is not set")?,
            fields: self.fields.ok_or("typesense_type is not set")?,
            default_sorting_field: self.default_sorting_field,
        })
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
            ("company_name", FieldType::String, None),
            ("num_employees", FieldType::Int32, None),
            ("country", FieldType::String, Some(true)),
        ]
        .iter()
        .map(|(name, typesense_type, facet)| {
            FieldBuilder::new()
                .name(name.to_string())
                .typesense_type(*typesense_type)
                .facet(*facet)
                .build()
                .unwrap()
        })
        .collect::<Vec<Field>>();
        let collection = CollectionSchemaBuilder::new()
            .name("companies".to_string())
            .default_sorting_field("num_employees".to_string())
            .fields(fields)
            .build()
            .unwrap();

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
