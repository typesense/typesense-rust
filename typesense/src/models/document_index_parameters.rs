use serde::{Deserialize, Serialize};

/// Parameters for indexing documents into a Typesense collection.
///
/// These parameters control how Typesense handles special cases such as
/// malformed, invalid, or "dirty" values during indexing.
///
/// See the official Typesense documentation:
/// <https://typesense.org/docs/latest/api/documents.html#dealing-with-dirty-data>
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, bon::Builder)]
pub struct DocumentIndexParameters {
    /// Controls how Typesense handles "dirty values" (e.g. invalid dates,
    /// numbers where a string is expected, values that don't match the schema).
    ///
    /// If `None`, Typesense will use the default behavior (rejecting invalid data).
    #[serde(rename = "dirty_values", skip_serializing_if = "Option::is_none")]
    pub dirty_values: Option<typesense_codegen::models::DirtyValues>,
}

impl DocumentIndexParameters {
    /// Creates a new `DocumentIndexParameters`.
    pub fn new() -> DocumentIndexParameters {
        DocumentIndexParameters { dirty_values: None }
    }
}
