//! Contains the generic `SearchResult` and `SearchResultHit` structs

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use typesense_codegen::models as raw_models;

/// Represents a single search result hit, with the document deserialized into a strongly-typed struct `T`.
///
/// This struct is generic over the document type `T`, which must be deserializable from JSON.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
// Add this line to help the derive macro with the generic bound.
#[serde(bound(serialize = "T: Serialize", deserialize = "T: DeserializeOwned"))]
pub struct SearchResultHit<T: DeserializeOwned> {
    /// (Deprecated) Contains highlighted portions of the search fields
    #[serde(rename = "highlights", skip_serializing_if = "Option::is_none")]
    pub highlights: Option<Vec<raw_models::SearchHighlight>>,

    /// Highlighted version of the matching document
    #[serde(rename = "highlight", skip_serializing_if = "Option::is_none")]
    pub highlight: Option<std::collections::HashMap<String, serde_json::Value>>,

    /// The full document that was matched, deserialized into type `T`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub document: Option<T>,

    /// The score of the text match.
    #[serde(rename = "text_match", skip_serializing_if = "Option::is_none")]
    pub text_match: Option<i64>,

    /// Detailed information about the text match.
    #[serde(rename = "text_match_info", skip_serializing_if = "Option::is_none")]
    pub text_match_info: Option<Box<raw_models::SearchResultHitTextMatchInfo>>,

    /// Can be any key-value pair
    #[serde(
        rename = "geo_distance_meters",
        skip_serializing_if = "Option::is_none"
    )]
    pub geo_distance_meters: Option<std::collections::HashMap<String, i32>>,

    /// Distance between the query vector and matching document's vector value
    #[serde(rename = "vector_distance", skip_serializing_if = "Option::is_none")]
    pub vector_distance: Option<f32>,
}

/// Represents the full response from a Typesense search query, containing strongly-typed hits.
///
/// This struct is generic over the document type `T`. It is the return type of the
/// `documents().search()` method.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
// Add this line to help the derive macro with the generic bound.
#[serde(bound(serialize = "T: Serialize", deserialize = "T: DeserializeOwned"))]
pub struct SearchResult<T: DeserializeOwned> {
    /// The search result hits, with documents deserialized into type `T`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hits: Option<Vec<SearchResultHit<T>>>,

    /// The number of documents found.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub found: Option<i32>,

    /// The number of documents that matched the search query.
    #[serde(rename = "found_docs", skip_serializing_if = "Option::is_none")]
    pub found_docs: Option<i32>,

    /// The total number of documents in the collection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub out_of: Option<i32>,

    /// The search result page number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,

    /// The number of milliseconds the search took.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_time_ms: Option<i32>,

    /// Counts of values for each facet field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub facet_counts: Option<Vec<raw_models::FacetCounts>>,

    /// Results grouped by a field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grouped_hits: Option<Vec<raw_models::SearchGroupedHit>>,

    /// Whether the search was cut off.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_cutoff: Option<bool>,

    /// The request parameters that were used for this search.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_params: Option<Box<raw_models::SearchResultRequestParams>>,

    /// Conversation object for conversational search.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation: Option<Box<raw_models::SearchResultConversation>>,
}

impl<T> SearchResult<T>
where
    T: DeserializeOwned,
{
    /// Transforms a raw, non-generic `SearchResult` from the API into a strongly-typed `SearchResult<T>`.
    pub(crate) fn from_raw(
        raw_result: raw_models::SearchResult,
    ) -> Result<Self, serde_json::Error> {
        let typed_hits = match raw_result.hits {
            Some(raw_hits) => {
                let mut hits = Vec::with_capacity(raw_hits.len());
                for raw_hit in raw_hits {
                    let document: Option<T> = match raw_hit.document {
                        Some(doc_value) => Some(serde_json::from_value(doc_value)?),
                        None => None,
                    };

                    hits.push(SearchResultHit {
                        document,
                        highlights: raw_hit.highlights,
                        highlight: raw_hit.highlight,
                        text_match: raw_hit.text_match,
                        text_match_info: raw_hit.text_match_info,
                        geo_distance_meters: raw_hit.geo_distance_meters,
                        vector_distance: raw_hit.vector_distance,
                    });
                }
                Some(hits)
            }
            None => None,
        };

        Ok(SearchResult {
            found_docs: raw_result.found_docs,
            hits: typed_hits,
            facet_counts: raw_result.facet_counts,
            found: raw_result.found,
            out_of: raw_result.out_of,
            page: raw_result.page,
            search_time_ms: raw_result.search_time_ms,
            grouped_hits: raw_result.grouped_hits,
            search_cutoff: raw_result.search_cutoff,
            request_params: raw_result.request_params,
            conversation: raw_result.conversation,
        })
    }
}
