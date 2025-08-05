//! The Typesense prelude.
//!
//! This module re-exports the most commonly used traits and types from the library,
//! making them easy to import with a single `use` statement.

use serde::de::DeserializeOwned;

use crate::{MultiSearchParseError, SearchResult};

/// An extension trait for `typesense_codegen::models::MultiSearchResult` to provide typed parsing.
pub trait MultiSearchResultExt {
    /// Parses the result at a specific index from a multi-search response into a strongly-typed `SearchResult<T>`.
    ///
    /// # Arguments
    /// * `index` - The zero-based index of the search result to parse.
    ///
    /// # Type Parameters
    /// * `T` - The concrete document type to deserialize the hits into.
    fn parse_at<T: DeserializeOwned>(
        &self,
        index: usize,
    ) -> Result<SearchResult<T>, MultiSearchParseError>;
}
