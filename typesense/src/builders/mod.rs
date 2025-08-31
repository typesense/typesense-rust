//! Contain convenient builders for Typesense schemas.

mod multi_search_collection_parameters;
mod multi_search_parameters;
mod multi_search_searches_parameters;

pub use multi_search_collection_parameters::{
    MultiSearchCollectionParametersBuilder, new_multi_search_collection_parameters,
};
pub use multi_search_parameters::{MultiSearchParametersBuilder, new_multi_search_parameters};
pub use multi_search_searches_parameters::{
    MultiSearchSearchesParameterBuilder, new_multi_search_searches_parameter,
};
