//! Contains the core traits and extensions for Typesense client operations

mod document;
mod field_type;
mod multi_search_ext;

pub use document::Document;
pub use field_type::*;
pub use multi_search_ext::{MultiSearchResultExt, UnionSearchResultExt};
