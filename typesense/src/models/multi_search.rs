use crate::models;
use serde::{Deserialize, Serialize};

/// Represents the body of a multi-search request.
///
/// This struct acts as a container for a list of individual search queries that will be
/// sent to the Typesense multi-search endpoint. Each search query is defined in a
/// `MultiSearchCollectionParameters` struct within the `searches` vector.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MultiSearchSearchesParameter {
    /// A vector of individual search queries to be executed. The order of the search results returned by Typesense will match the order of these queries.
    #[serde(rename = "searches")]
    pub searches: Vec<models::MultiSearchCollectionParameters>,
}

impl MultiSearchSearchesParameter {
    /// Creates a new `MultiSearchSearchesParameter` instance.
    pub fn new(
        searches: Vec<models::MultiSearchCollectionParameters>,
    ) -> MultiSearchSearchesParameter {
        MultiSearchSearchesParameter { searches }
    }
}
