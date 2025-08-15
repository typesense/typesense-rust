//! The Typesense prelude.
//!
//! This module re-exports the most commonly used traits and types from the library,
//! making them easy to import with a single `use` statement.

pub use crate::traits::{Document, FieldType, MultiSearchResultExt, ToTypesenseField};

// pub use crate::error::Error as TypesenseError;
// pub use crate::error::MultiSearchParseError;
