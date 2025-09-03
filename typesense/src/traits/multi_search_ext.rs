use serde::de::DeserializeOwned;
use serde_json::Value;
use typesense_codegen::models::{self as raw_models, SearchGroupedHit, SearchResultHit};

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

/// An extension trait for `typesense_codegen::models::MultiSearchResult` to provide typed parsing.
pub trait UnionSearchResultExt {
    /// Parses the result at a specific index from a multi-search response into a strongly-typed `SearchResult<T>`.
    ///
    /// # Arguments
    /// * `index` - The zero-based index of the search result to parse.
    ///
    /// # Type Parameters
    /// * `D` - The concrete document type to deserialize the hits into.
    fn try_into_typed<D: DeserializeOwned>(self) -> Result<SearchResult<D>, serde_json::Error>;
}

fn multi_search_item_to_search_result<D: DeserializeOwned>(
    item: &raw_models::MultiSearchResultItem<Value>,
) -> Result<raw_models::SearchResult<D>, serde_json::Error> {
    let typed_hits = match &item.hits {
        Some(raw_hits) => {
            let hits_result: Result<Vec<SearchResultHit<D>>, _> = raw_hits
                .into_iter()
                .map(|raw_hit| {
                    // Map each raw hit to a Result<SearchResultHit<D>, _>
                    let document: Result<Option<D>, _> = raw_hit
                        .document
                        .clone()
                        .map(|doc_value| serde_json::from_value(doc_value))
                        .transpose();
                    Ok(SearchResultHit {
                        document: document?,
                        highlights: raw_hit.highlights.clone(),
                        highlight: raw_hit.highlight.clone(),
                        text_match: raw_hit.text_match,
                        text_match_info: raw_hit.text_match_info.clone(),
                        geo_distance_meters: raw_hit.geo_distance_meters.clone(),
                        vector_distance: raw_hit.vector_distance,
                        hybrid_search_info: raw_hit.hybrid_search_info.clone(),
                        search_index: raw_hit.search_index,
                    })
                })
                .collect();

            Some(hits_result?)
        }
        None => None,
    };
    // Parse grouped_hits into SearchGroupedHit<D>
    let typed_grouped_hits = match &item.grouped_hits {
        Some(raw_groups) => {
            let groups_result: Result<Vec<SearchGroupedHit<D>>, _> = raw_groups
                .iter()
                .map(|raw_group| {
                    let hits_result: Result<Vec<SearchResultHit<D>>, _> = raw_group
                        .hits
                        .iter()
                        .map(|raw_hit| {
                            let document: Result<Option<D>, _> = raw_hit
                                .document
                                .clone()
                                .map(|doc_value| serde_json::from_value(doc_value))
                                .transpose();
                            Ok(SearchResultHit {
                                document: document?,
                                highlights: raw_hit.highlights.clone(),
                                highlight: raw_hit.highlight.clone(),
                                text_match: raw_hit.text_match,
                                text_match_info: raw_hit.text_match_info.clone(),
                                geo_distance_meters: raw_hit.geo_distance_meters.clone(),
                                vector_distance: raw_hit.vector_distance,
                                hybrid_search_info: raw_hit.hybrid_search_info.clone(),
                                search_index: raw_hit.search_index.clone(),
                            })
                        })
                        .collect();

                    Ok(SearchGroupedHit {
                        found: raw_group.found,
                        group_key: raw_group.group_key.clone(),
                        hits: hits_result?,
                    })
                })
                .collect();
            Some(groups_result?)
        }
        None => None,
    };

    Ok(raw_models::SearchResult {
        hits: typed_hits,
        grouped_hits: typed_grouped_hits,
        facet_counts: item.facet_counts.clone(),
        found: item.found,
        found_docs: item.found_docs,
        out_of: item.out_of,
        page: item.page,
        search_time_ms: item.search_time_ms,
        search_cutoff: item.search_cutoff,
        request_params: item.request_params.clone(),
        conversation: item.conversation.clone(),
        union_request_params: item.union_request_params.clone(),
    })
}

impl MultiSearchResultExt for raw_models::MultiSearchResult<Value> {
    fn parse_at<T: DeserializeOwned>(
        &self,
        index: usize,
    ) -> Result<SearchResult<T>, MultiSearchParseError> {
        let raw_item = self
            .results
            .get(index)
            .ok_or(MultiSearchParseError::IndexOutOfBounds(index))?;

        if let Some(error_msg) = &raw_item.error {
            return Err(MultiSearchParseError::ApiError {
                index,
                message: error_msg.clone(),
            });
        }

        multi_search_item_to_search_result(raw_item)
            .map_err(|source| MultiSearchParseError::Deserialization { index, source })
    }
}

// This impl block specifically targets `SearchResult<serde_json::Value>`.
// The methods inside will only be available on a search result of that exact type.
impl UnionSearchResultExt for SearchResult<Value> {
    /// Attempts to convert a `SearchResult<serde_json::Value>` into a `SearchResult<D>`.
    ///
    /// This method is useful after a `perform_union` call where you know all resulting
    /// documents share the same schema and can be deserialized into a single concrete type `D`.
    ///
    /// It iterates through each hit and tries to deserialize its `document` field. If any
    /// document fails to deserialize into type `D`, the entire conversion fails.
    ///
    /// # Type Parameters
    ///
    /// * `D` - The concrete, `DeserializeOwned` type you want to convert the documents into.
    ///
    /// # Errors
    ///
    /// Returns a `serde_json::Error` if any document in the hit list cannot be successfully
    /// deserialized into `D`.
    fn try_into_typed<D: DeserializeOwned>(self) -> Result<SearchResult<D>, serde_json::Error> {
        // This logic is very similar to `from_raw`, but it converts between generic types
        // instead of from a raw model.
        let typed_hits = match self.hits {
            Some(value_hits) => {
                let hits_result: Result<Vec<SearchResultHit<D>>, _> = value_hits
                    .into_iter()
                    .map(|value_hit| {
                        // `value_hit` here is `SearchResultHit<serde_json::Value>`
                        let document: Option<D> = match value_hit.document {
                            Some(doc_value) => Some(serde_json::from_value(doc_value)?),
                            None => None,
                        };

                        Ok(SearchResultHit {
                            document,
                            highlights: value_hit.highlights,
                            highlight: value_hit.highlight,
                            text_match: value_hit.text_match,
                            text_match_info: value_hit.text_match_info,
                            geo_distance_meters: value_hit.geo_distance_meters,
                            vector_distance: value_hit.vector_distance,
                            hybrid_search_info: value_hit.hybrid_search_info,
                            search_index: value_hit.search_index,
                        })
                    })
                    .collect();

                Some(hits_result?)
            }
            None => None,
        };

        // Parse grouped_hits into SearchGroupedHit<D>
        let typed_grouped_hits = match &self.grouped_hits {
            Some(raw_groups) => {
                let groups_result: Result<Vec<SearchGroupedHit<D>>, _> = raw_groups
                    .iter()
                    .map(|raw_group| {
                        let hits_result: Result<Vec<SearchResultHit<D>>, _> = raw_group
                            .hits
                            .iter()
                            .map(|raw_hit| {
                                let document: Result<Option<D>, _> = raw_hit
                                    .document
                                    .clone()
                                    .map(|doc_value| serde_json::from_value(doc_value))
                                    .transpose();
                                Ok(SearchResultHit {
                                    document: document?,
                                    highlights: raw_hit.highlights.clone(),
                                    highlight: raw_hit.highlight.clone(),
                                    text_match: raw_hit.text_match,
                                    text_match_info: raw_hit.text_match_info.clone(),
                                    geo_distance_meters: raw_hit.geo_distance_meters.clone(),
                                    vector_distance: raw_hit.vector_distance,
                                    hybrid_search_info: raw_hit.hybrid_search_info.clone(),
                                    search_index: raw_hit.search_index.clone(),
                                })
                            })
                            .collect();

                        Ok(SearchGroupedHit {
                            found: raw_group.found,
                            group_key: raw_group.group_key.clone(),
                            hits: hits_result?,
                        })
                    })
                    .collect();
                Some(groups_result?)
            }
            None => None,
        };

        Ok(SearchResult {
            hits: typed_hits,
            grouped_hits: typed_grouped_hits,

            found: self.found,
            found_docs: self.found_docs,
            out_of: self.out_of,
            page: self.page,
            search_time_ms: self.search_time_ms,
            facet_counts: self.facet_counts,
            search_cutoff: self.search_cutoff,
            request_params: self.request_params,
            conversation: self.conversation,
            union_request_params: self.union_request_params,
        })
    }
}
