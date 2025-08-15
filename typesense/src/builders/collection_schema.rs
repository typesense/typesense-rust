//! # Collection
//!
//! In Typesense, a group of related documents is called a collection. A collection
//! is roughly equivalent to a table in a relational database.
//!
use bon::builder;
use typesense_codegen::models::{CollectionSchema, Field, VoiceQueryModelCollectionConfig};

/// Creates a new [`CollectionSchema`] builder.
///
/// In Typesense, a collection is a group of related documents, similar to a table
/// in a relational database. This builder enforces that `name` must be provided
/// before [`build`](CollectionSchemaBuilder::build) can be called.
///
/// # Example
///
/// ```
/// use typesense::builders::new_collection_schema;
/// let fields = vec![];
/// let schema = new_collection_schema("companies", fields)
///     .default_sorting_field("num_employees")
///     .build();
/// ```
#[builder(
    builder_type(name = CollectionSchemaBuilder, vis = "pub"),
    finish_fn(name = build, vis = "pub"),
    state_mod(name = collection_schema_builder, vis = "pub"),
    on(String, into)
)]
pub fn new_collection_schema(
    /// The name of the collection. Must be unique within the Typesense instance.
    #[builder(start_fn)]
    name: String,

    /// The list of fields that describe the schema of documents in this collection.
    #[builder(start_fn)]
    fields: Vec<Field>,

    /// The name of the default sorting field for the collection.
    default_sorting_field: Option<String>,

    /// A list of token separators to use when indexing text fields.
    token_separators: Option<Vec<String>>,

    /// Whether nested fields are enabled.
    enable_nested_fields: Option<bool>,

    /// Symbols that should be indexed for this collection.
    symbols_to_index: Option<Vec<String>>,

    /// Configuration for Typesense’s Voice Query Model.
    voice_query_model: Option<Box<VoiceQueryModelCollectionConfig>>,
) -> CollectionSchema {
    CollectionSchema {
        name,
        fields,
        default_sorting_field,
        token_separators,
        enable_nested_fields,
        symbols_to_index,
        voice_query_model,
    }
}

// custom convenience methods; the typestate module name matches `state_mod`
impl<S: collection_schema_builder::State> CollectionSchemaBuilder<S> {
    /// Adds a single [`Field`] to the collection schema.
    ///
    /// This is a convenience method for pushing one field at a time.
    pub fn field(mut self, field: Field) -> Self {
        self.fields.push(field);
        self
    }

    /// Adds multiple [`Field`] values to the collection schema.
    ///
    /// This is a convenience method for appending a slice of fields.
    pub fn fields(mut self, fields: &[Field]) -> Self
    where
        Field: Clone,
    {
        self.fields.extend_from_slice(fields);
        self
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::builders::new_collection_field;
    use serde_json::json;

    #[test]
    fn collection_schema_serializes_as_expected() {
        let fields = [
            ("company_name", "string".to_owned(), None),
            ("num_employees", "int32".to_owned(), None),
            ("country", "string".to_owned(), Some(true)),
        ]
        .map(|(name, typesense_type, facet)| {
            if facet.is_some() {
                new_collection_field(name, typesense_type.into())
                    .facet(facet.unwrap())
                    .build()
            } else {
                new_collection_field(name, typesense_type.into()).build()
            }
        });

        let collection: CollectionSchema =
            new_collection_schema("companies", fields.clone().to_vec())
                .fields(&fields)
                .field(new_collection_field("size", "string".into()).build())
                .default_sorting_field("num_employees")
                .build();

        let expected = json!(
            {
                "name": "companies",
                "fields": [
                  { "name": "company_name", "type": "string" },
                  { "name": "num_employees", "type": "int32" },
                  { "name": "country", "type": "string", "facet": true },

                  { "name": "company_name", "type": "string" },
                  { "name": "num_employees", "type": "int32" },
                  { "name": "country", "type": "string", "facet": true },

                  { "name": "size", "type": "string" },
                ],
                "default_sorting_field": "num_employees"
            }
        );

        assert_eq!(serde_json::to_value(&collection).unwrap(), expected)
    }
}
