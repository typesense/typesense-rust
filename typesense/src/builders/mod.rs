//! Contain convenient builders for Typesense schemas.

mod collection_field;
mod collection_schema;
mod multi_search_collection_parameters;
mod multi_search_parameters;
mod multi_search_searches_parameters;
mod search_parameters;

pub use collection_field::{FieldBuilder, new_collection_field};
pub use collection_schema::{CollectionSchemaBuilder, new_collection_schema};

pub use search_parameters::{SearchParametersBuilder, new_search_parameters};

pub use multi_search_collection_parameters::{
    MultiSearchCollectionParametersBuilder, new_multi_search_collection_parameters,
};
pub use multi_search_parameters::{MultiSearchParametersBuilder, new_multi_search_parameters};
pub use multi_search_searches_parameters::{
    MultiSearchSearchesParameterBuilder, new_multi_search_searches_parameter,
};
