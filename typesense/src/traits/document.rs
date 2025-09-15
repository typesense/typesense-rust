//! # Document
//!
//! In Typesense, documents are each one of the JSON elements that are stored in the collections.
//! A document to be indexed in a given collection must conform to the schema of the collection.
//!
use crate::models::CollectionSchema;
use serde::{Serialize, de::DeserializeOwned};

/// Trait for partial structs
pub trait DocumentPartial: Serialize {}

/// Trait that every struct should implement that wants to be represented as a Typesense
/// Document
pub trait Document: DeserializeOwned + Serialize {
    /// Collection name
    const COLLECTION_NAME: &'static str;
    /// A struct for partial updates
    type Partial: DocumentPartial;

    /// Collection schema associated with the document.
    fn collection_schema() -> CollectionSchema;
}
