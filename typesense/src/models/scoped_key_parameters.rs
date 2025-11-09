use crate::models::SearchParameters;
use serde::Serialize;

/// Defines the parameters for generating a scoped API key.
///
/// A scoped key is a temporary, client-side key that has a specific set of
/// search restrictions and an optional expiration time embedded within it. It allows
/// you to delegate search permissions securely without exposing your main API key.
#[derive(Debug, Clone, Default, Serialize)]
pub struct ScopedKeyParameters<'a> {
    /// The search parameters to embed in the key. These parameters will be
    /// enforced for all searches made with the generated key.
    /// For example, you can use `filter_by` to restrict searches to a subset of documents.
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub search_params: Option<SearchParameters<'a>>,

    /// The number of `multi_search` requests that can be performed using this key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_multi_searches: Option<i64>,

    /// The Unix timestamp (in seconds) after which the generated key will expire.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<i64>,
}
