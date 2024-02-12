//! # Document
//!
//! In Typesense, documents are each one of the JSON elements that are stored in the collections.
//! A document to be indexed in a given collection must conform to the schema of the collection.
//!
use crate::{collection::CollectionSchema, synonym::SynonymSchema};
use serde::{de::DeserializeOwned, Serialize};

/// Trait that should implement every struct that wants to be represented as a Typesense
/// Document
pub trait Document: DeserializeOwned + Serialize {
    /// Collection schema associated with the document.
    fn collection_schema() -> CollectionSchema;

    /// Synonym schema associated with the document.
    fn synonym_schema() -> SynonymSchema;
}
