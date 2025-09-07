use crate::models::{MultiSearchResult, MultiSearchResultItem, SearchGroupedHit, SearchResultHit};
use serde::de::DeserializeOwned;
use serde_json::Value;

use crate::{MultiSearchParseError, models::SearchResult};

/// An extension trait for `MultiSearchResult` to provide typed parsing.
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

/// Small helpers to convert documents stored as `serde_json::Value` into a concrete `D`.
fn deserialize_opt_document<D: DeserializeOwned>(
    doc: Option<Value>,
) -> Result<Option<D>, serde_json::Error> {
    match doc {
        Some(v) => Ok(Some(serde_json::from_value(v)?)),
        None => Ok(None),
    }
}

fn convert_hit_ref<D: DeserializeOwned>(
    raw_hit: &SearchResultHit<Value>,
) -> Result<SearchResultHit<D>, serde_json::Error> {
    Ok(SearchResultHit {
        document: deserialize_opt_document(raw_hit.document.clone())?,
        highlights: raw_hit.highlights.clone(),
        highlight: raw_hit.highlight.clone(),
        text_match: raw_hit.text_match,
        text_match_info: raw_hit.text_match_info.clone(),
        geo_distance_meters: raw_hit.geo_distance_meters.clone(),
        vector_distance: raw_hit.vector_distance,
        hybrid_search_info: raw_hit.hybrid_search_info.clone(),
        search_index: raw_hit.search_index,
    })
}

fn convert_group_ref<D: DeserializeOwned>(
    raw_group: &SearchGroupedHit<Value>,
) -> Result<SearchGroupedHit<D>, serde_json::Error> {
    let hits = raw_group
        .hits
        .iter()
        .map(convert_hit_ref::<D>)
        .collect::<Result<Vec<_>, _>>()?;

    Ok(SearchGroupedHit {
        found: raw_group.found,
        group_key: raw_group.group_key.clone(),
        hits,
    })
}

/// Convert a single `MultiSearchResultItem<Value>` into a strongly-typed `SearchResult<D>`.
fn multi_search_item_to_search_result<D: DeserializeOwned>(
    item: &MultiSearchResultItem<Value>,
) -> Result<SearchResult<D>, serde_json::Error> {
    let typed_hits = match &item.hits {
        Some(raw_hits) => Some(
            raw_hits
                .iter()
                .map(convert_hit_ref::<D>)
                .collect::<Result<Vec<_>, _>>()?,
        ),
        None => None,
    };

    let typed_grouped_hits = match &item.grouped_hits {
        Some(raw_groups) => Some(
            raw_groups
                .iter()
                .map(convert_group_ref::<D>)
                .collect::<Result<Vec<_>, _>>()?,
        ),
        None => None,
    };

    Ok(SearchResult {
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

/// Extension to parse an item out of a `MultiSearchResult<Value>` into a typed `SearchResult<T>`.
impl MultiSearchResultExt for MultiSearchResult<Value> {
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
