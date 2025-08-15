//! Contain convenient builders for Typesense schemas.

mod collection_field;
mod collection_schema;
pub use collection_field::new_collection_field;
pub use collection_schema::new_collection_schema;
