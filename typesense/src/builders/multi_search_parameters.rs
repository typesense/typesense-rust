//! Module for the `MultiSearchParameters` builder.

use crate::models::{DropTokensMode, MultiSearchParameters};
use bon::builder;

/// Creates a new [`MultiSearchParameters`] builder.
///
/// This builder helps construct a set of common search parameters that can be applied
/// to all search queries within a multi-search request.
#[builder(
    // expose a public builder type and a public finish_fn
    builder_type(name = MultiSearchParametersBuilder, vis = "pub"),
    finish_fn(name = build, vis = "pub"),
    // allow passing &str into String params
    on(String, into)
)]
pub fn new_multi_search_parameters(
    /// The query text to search for in the collection.
    q: Option<String>,
    /// A list of `string` fields that should be queried against.
    query_by: Option<String>,
    /// The relative weight to give each `query_by` field when ranking results.
    query_by_weights: Option<String>,
    /// How the representative text match score is calculated.
    text_match_type: Option<String>,
    /// Indicates if the last word in the query should be treated as a prefix.
    prefix: Option<String>,
    /// Infix search configuration. Can be `off`, `always`, or `fallback`.
    infix: Option<String>,
    /// Maximum number of extra symbols before a query token for infix searching.
    max_extra_prefix: Option<i32>,
    /// Maximum number of extra symbols after a query token for infix searching.
    max_extra_suffix: Option<i32>,
    /// Filter conditions for refining search results.
    filter_by: Option<String>,
    /// A list of fields and their sort orders.
    sort_by: Option<String>,
    /// A list of fields to facet by.
    facet_by: Option<String>,
    /// Maximum number of facet values to be returned.
    max_facet_values: Option<i32>,
    /// A query to filter facet values.
    facet_query: Option<String>,
    /// The number of typographical errors (1 or 2) that would be tolerated.
    num_typos: Option<String>,
    /// The page number to fetch.
    page: Option<i32>,
    /// Number of results to fetch per page.
    per_page: Option<i32>,
    /// Number of hits to fetch.
    limit: Option<i32>,
    /// The starting point of the result set.
    offset: Option<i32>,
    /// Fields to group results by.
    group_by: Option<String>,
    /// Maximum number of hits to return for every group.
    group_limit: Option<i32>,
    /// Whether to group documents with null values in the `group_by` field.
    group_missing_values: Option<bool>,
    /// List of fields to include in the search result.
    include_fields: Option<String>,
    /// List of fields to exclude from the search result.
    exclude_fields: Option<String>,
    /// List of fields which should be highlighted fully.
    highlight_full_fields: Option<String>,
    /// The number of tokens surrounding the highlighted text.
    highlight_affix_num_tokens: Option<i32>,
    /// The start tag for highlighted snippets.
    highlight_start_tag: Option<String>,
    /// The end tag for highlighted snippets.
    highlight_end_tag: Option<String>,
    /// Field values under this length will be fully highlighted.
    snippet_threshold: Option<i32>,
    /// Threshold for dropping query tokens to find more results.
    drop_tokens_threshold: Option<i32>,
    drop_tokens_mode: Option<DropTokensMode>,
    /// Threshold for trying more typos to find more results.
    typo_tokens_threshold: Option<i32>,
    /// Whether to enable typos on alphanumerical tokens.
    enable_typos_for_alpha_numerical_tokens: Option<bool>,
    /// Whether the `filter_by` condition applies to curated results.
    filter_curated_hits: Option<bool>,
    /// Whether to enable synonyms for the query.
    enable_synonyms: Option<bool>,
    /// Allow synonym resolution on word prefixes.
    synonym_prefix: Option<bool>,
    /// Number of typos allowed for synonym resolution.
    synonym_num_typos: Option<i32>,
    /// A list of records to unconditionally include at specific positions.
    pinned_hits: Option<String>,
    /// A list of records to unconditionally hide from search results.
    hidden_hits: Option<String>,
    /// Comma-separated list of tags to trigger curation rules.
    override_tags: Option<String>,
    /// A list of custom fields that must be highlighted.
    highlight_fields: Option<String>,
    /// You can index content from any logographic language into Typesense if you are able to segment / split the text into space-separated words yourself before indexing and querying. Set this parameter to true to do the same
    pre_segmented_query: Option<bool>,
    /// Search using a preset of search parameters.
    preset: Option<String>,
    /// Whether to enable overrides for the query.
    enable_overrides: Option<bool>,
    /// Whether to prioritize an exact match.
    prioritize_exact_match: Option<bool>,
    /// Prioritize documents where query words appear earlier in the text.
    prioritize_token_position: Option<bool>,
    /// Prioritize documents where query words appear in more fields.
    prioritize_num_matching_fields: Option<bool>,
    /// Disable typos for numerical tokens.
    enable_typos_for_numerical_tokens: Option<bool>,
    /// Whether to perform an exhaustive search.
    exhaustive_search: Option<bool>,
    /// Search cutoff time in milliseconds.
    search_cutoff_ms: Option<i32>,
    /// Enable server-side caching of search results.
    use_cache: Option<bool>,
    /// The TTL for the search query cache.
    cache_ttl: Option<i32>,
    /// Minimum word length for 1-typo correction.
    min_len_1typo: Option<i32>,
    /// Minimum word length for 2-typo correction.
    min_len_2typo: Option<i32>,
    /// Vector query expression.
    vector_query: Option<String>,
    /// Timeout for fetching remote embeddings.
    remote_embedding_timeout_ms: Option<i32>,
    /// Number of retries for fetching remote embeddings.
    remote_embedding_num_tries: Option<i32>,
    /// The underlying faceting strategy to use.
    facet_strategy: Option<String>,
    /// Name of the stopwords set to apply for this search.
    stopwords: Option<String>,
    /// Nested facet fields whose parent object should be returned.
    facet_return_parent: Option<String>,
    /// The base64 encoded audio file.
    voice_query: Option<String>,
    /// Enable conversational search.
    conversation: Option<bool>,
    /// The ID of the Conversation Model to use.
    conversation_model_id: Option<String>,
    /// The ID of a previous conversation to continue.
    conversation_id: Option<String>,
) -> MultiSearchParameters {
    MultiSearchParameters {
        q,
        query_by,
        query_by_weights,
        text_match_type,
        prefix,
        infix,
        max_extra_prefix,
        max_extra_suffix,
        filter_by,
        sort_by,
        facet_by,
        max_facet_values,
        facet_query,
        num_typos,
        page,
        per_page,
        limit,
        offset,
        group_by,
        group_limit,
        group_missing_values,
        include_fields,
        exclude_fields,
        highlight_full_fields,
        highlight_affix_num_tokens,
        highlight_start_tag,
        highlight_end_tag,
        snippet_threshold,
        drop_tokens_threshold,
        drop_tokens_mode,
        typo_tokens_threshold,
        enable_typos_for_alpha_numerical_tokens,
        filter_curated_hits,
        enable_synonyms,
        synonym_prefix,
        synonym_num_typos,
        pinned_hits,
        hidden_hits,
        override_tags,
        highlight_fields,
        pre_segmented_query,
        preset,
        enable_overrides,
        prioritize_exact_match,
        prioritize_token_position,
        prioritize_num_matching_fields,
        enable_typos_for_numerical_tokens,
        exhaustive_search,
        search_cutoff_ms,
        use_cache,
        cache_ttl,
        min_len_1typo,
        min_len_2typo,
        vector_query,
        remote_embedding_timeout_ms,
        remote_embedding_num_tries,
        facet_strategy,
        stopwords,
        facet_return_parent,
        voice_query,
        conversation,
        conversation_model_id,
        conversation_id,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{DropTokensMode, MultiSearchParameters};

    #[test]
    fn test_multi_search_parameters_builder_basic() {
        let built = new_multi_search_parameters()
            .query_by("title")
            .per_page(5)
            .build();

        let expected = MultiSearchParameters {
            query_by: Some("title".to_string()),
            per_page: Some(5),
            ..Default::default()
        };

        assert_eq!(built, expected);
    }

    #[test]
    fn test_multi_search_parameters_builder_full() {
        let built = new_multi_search_parameters()
            .q("*")
            .filter_by("category:shoes")
            .use_cache(true)
            .drop_tokens_mode(DropTokensMode::LeftToRight)
            .search_cutoff_ms(100)
            .build();

        let expected = MultiSearchParameters {
            q: Some("*".to_string()),
            filter_by: Some("category:shoes".to_string()),
            use_cache: Some(true),
            drop_tokens_mode: Some(DropTokensMode::LeftToRight),
            search_cutoff_ms: Some(100),
            ..Default::default()
        };

        assert_eq!(built, expected);
    }

    #[test]
    fn test_multi_search_parameters_builder_defaults() {
        let built = new_multi_search_parameters().build();
        let expected = MultiSearchParameters::default();
        assert_eq!(built, expected);
    }
}
