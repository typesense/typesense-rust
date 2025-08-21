//! Module for the `MultiSearchSearchesParameter` builder.

use crate::models::{MultiSearchCollectionParameters, MultiSearchSearchesParameter};

/// A builder for creating a `MultiSearchSearchesParameter` object.
///
/// This builder is used to construct the body of a multi-search request by
/// adding individual search queries one by one.
#[derive(Debug, Default)]
pub struct MultiSearchSearchesParameterBuilder {
    searches: Vec<MultiSearchCollectionParameters>,
}

impl MultiSearchSearchesParameterBuilder {
    /// Creates a new, empty builder.
    pub fn new() -> Self {
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

    /// Consumes the builder and returns a `MultiSearchSearchesParameter` object.
    pub fn build(self) -> MultiSearchSearchesParameter {
        MultiSearchSearchesParameter {
            searches: self.searches,
        }
    }
}

/// Creates a new [`MultiSearchSearchesParameterBuilder`].
///
/// This is the entry point for building a multi-search request body.
pub fn new_multi_search_searches_parameter() -> MultiSearchSearchesParameterBuilder {
    MultiSearchSearchesParameterBuilder::new()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::builders::new_multi_search_collection_parameters;

    #[test]
    fn test_multi_search_builder_new_is_empty() {
        let multi_search_request = new_multi_search_searches_parameter().build();
        assert!(multi_search_request.searches.is_empty());
    }

    #[test]
    fn test_multi_search_builder_add_one_search() {
        let search1 = new_multi_search_collection_parameters()
            .collection("products")
            .q("shoe")
            .build();

        let multi_search_request = new_multi_search_searches_parameter()
            .add_search(search1.clone())
            .build();

        assert_eq!(multi_search_request.searches.len(), 1);
        assert_eq!(multi_search_request.searches[0], search1);
    }

    #[test]
    fn test_multi_search_builder_add_multiple_searches_chained() {
        let search1 = new_multi_search_collection_parameters()
            .collection("products")
            .q("shoe")
            .build();
        let search2 = new_multi_search_collection_parameters()
            .collection("brands")
            .q("Nike")
            .build();

        let multi_search_request = new_multi_search_searches_parameter()
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
            new_multi_search_collection_parameters()
                .collection("c1")
                .build(),
            new_multi_search_collection_parameters()
                .collection("c2")
                .build(),
            new_multi_search_collection_parameters()
                .collection("c3")
                .build(),
        ];

        let multi_search_request = new_multi_search_searches_parameter()
            .add_searches(searches_vec.clone())
            .build();

        assert_eq!(multi_search_request.searches.len(), 3);
        assert_eq!(multi_search_request.searches, searches_vec);
    }
}
