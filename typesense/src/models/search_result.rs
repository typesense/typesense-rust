//! Contains the generic `SearchResult` and `SearchResultHit` structs

use serde::{Deserialize, Serialize, de::DeserializeOwned};
use serde_json::Value;
use typesense_codegen::models as raw_models;

/// Represents a single search result hit, with the document deserialized into a strongly-typed struct `D`.
///
/// This struct is generic over the document type `D`, which must be deserializable from JSON.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SearchResultHit<D> {
    /// (Deprecated) Contains highlighted portions of the search fields
    #[serde(rename = "highlights", skip_serializing_if = "Option::is_none")]
    pub highlights: Option<Vec<raw_models::SearchHighlight>>,

    /// Highlighted version of the matching document
    #[serde(rename = "highlight", skip_serializing_if = "Option::is_none")]
    pub highlight: Option<std::collections::HashMap<String, serde_json::Value>>,

    /// The full document that was matched, deserialized into type `D`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub document: Option<D>,

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
/// This struct is generic over the document type `D`. It is the return type of the
/// `documents().search()` method.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SearchResult<D> {
    /// The search result hits, with documents deserialized into type `D`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hits: Option<Vec<SearchResultHit<D>>>,

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

impl<D> SearchResult<D>
where
    D: DeserializeOwned,
{
    /// Transforms a raw, non-generic `SearchResult` from the API into a strongly-typed `SearchResult<D>`.
    pub(crate) fn from_raw(
        raw_result: raw_models::SearchResult,
    ) -> Result<Self, serde_json::Error> {
        let typed_hits = match raw_result.hits {
            Some(raw_hits) => {
                let hits_result: Result<Vec<SearchResultHit<D>>, _> = raw_hits
                    .into_iter()
                    .map(|raw_hit| {
                        // Map each raw hit to a Result<SearchResultHit<D>, _>
                        let document: Result<Option<D>, _> = raw_hit
                            .document
                            .map(|doc_value| serde_json::from_value(doc_value))
                            .transpose();

                        Ok(SearchResultHit {
                            document: document?,
                            highlights: raw_hit.highlights,
                            highlight: raw_hit.highlight,
                            text_match: raw_hit.text_match,
                            text_match_info: raw_hit.text_match_info,
                            geo_distance_meters: raw_hit.geo_distance_meters,
                            vector_distance: raw_hit.vector_distance,
                        })
                    })
                    .collect();

                Some(hits_result?)
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

// This impl block specifically targets `SearchResult<serde_json::Value>`.
// The methods inside will only be available on a search result of that exact type.
impl SearchResult<Value> {
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
    pub fn try_into_typed<D: DeserializeOwned>(self) -> Result<SearchResult<D>, serde_json::Error> {
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
                        })
                    })
                    .collect();

                Some(hits_result?)
            }
            None => None,
        };

        Ok(SearchResult {
            hits: typed_hits,
            found: self.found,
            found_docs: self.found_docs,
            out_of: self.out_of,
            page: self.page,
            search_time_ms: self.search_time_ms,
            facet_counts: self.facet_counts,
            grouped_hits: self.grouped_hits,
            search_cutoff: self.search_cutoff,
            request_params: self.request_params,
            conversation: self.conversation,
        })
    }
}
