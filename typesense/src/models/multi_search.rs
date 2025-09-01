use crate::models;
use serde::{Deserialize, Serialize};
use typesense_codegen::models::MultiSearchCollectionParameters;

/// Represents the body of a multi-search request.
///
/// This struct acts as a container for a list of individual search queries that will be
/// sent to the Typesense multi-search endpoint. Each search query is defined in a
/// `MultiSearchCollectionParameters` struct within the `searches` vector.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MultiSearchBody {
    /// A vector of individual search queries to be executed. The order of the search results returned by Typesense will match the order of these queries.
    #[serde(rename = "searches")]
    pub searches: Vec<models::MultiSearchCollectionParameters>,
}

impl MultiSearchBody {
    /// Creates a new `MultiSearchBody` instance.
    pub fn new(searches: Vec<models::MultiSearchCollectionParameters>) -> MultiSearchBody {
        MultiSearchBody { searches }
    }
}

impl MultiSearchBody {
    /// Creates a new, empty builder.
    pub fn builder() -> Self {
        Self::default()
    }

    /// Adds a single search query to the multi-search request.
    ///
    /// # Arguments
    ///
    /// * `search` - A `MultiSearchCollectionParameters` object representing an
    ///   individual search query.
    pub fn add_search(mut self, search: MultiSearchCollectionParameters) -> Self {
        self.searches.push(search);
        self
    }

    /// Adds multiple search queries to the multi-search request from an iterator.
    ///
    /// # Arguments
    ///
    /// * `searches` - An iterator that yields `MultiSearchCollectionParameters` objects.
    pub fn add_searches(
        mut self,
        searches: impl IntoIterator<Item = MultiSearchCollectionParameters>,
    ) -> Self {
        self.searches.extend(searches);
        self
    }

    /// Consumes the builder and returns a `MultiSearchBody` object.
    pub fn build(self) -> MultiSearchBody {
        MultiSearchBody {
            searches: self.searches,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::MultiSearchCollectionParameters;

    #[test]
    fn test_multi_search_builder_new_is_empty() {
        let multi_search_request = MultiSearchBody::builder().build();
        assert!(multi_search_request.searches.is_empty());
    }

    #[test]
    fn test_multi_search_builder_add_one_search() {
        let search1 = MultiSearchCollectionParameters::builder()
            .collection("products")
            .q("shoe")
            .build();

        let multi_search_request = MultiSearchBody::builder()
            .add_search(search1.clone())
            .build();

        assert_eq!(multi_search_request.searches.len(), 1);
        assert_eq!(multi_search_request.searches[0], search1);
    }

    #[test]
    fn test_multi_search_builder_add_multiple_searches_chained() {
        let search1 = MultiSearchCollectionParameters::builder()
            .collection("products")
            .q("shoe")
            .build();
        let search2 = MultiSearchCollectionParameters::builder()
            .collection("brands")
            .q("Nike")
            .build();

        let multi_search_request = MultiSearchBody::builder()
            .add_search(search1.clone())
            .add_search(search2.clone())
            .build();

        assert_eq!(multi_search_request.searches.len(), 2);
        assert_eq!(multi_search_request.searches[0], search1);
        assert_eq!(multi_search_request.searches[1], search2);
    }

    #[test]
    fn test_multi_search_builder_add_searches_from_iterator() {
        let searches_vec = vec![
            MultiSearchCollectionParameters::builder()
                .collection("c1")
                .build(),
            MultiSearchCollectionParameters::builder()
                .collection("c2")
                .build(),
            MultiSearchCollectionParameters::builder()
                .collection("c3")
                .build(),
        ];

        let multi_search_request = MultiSearchBody::builder()
            .add_searches(searches_vec.clone())
            .build();

        assert_eq!(multi_search_request.searches.len(), 3);
        assert_eq!(multi_search_request.searches, searches_vec);
    }
}
