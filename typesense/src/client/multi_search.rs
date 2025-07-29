//! Provides access to the API endpoints for Multi Search.
//!
//! A `MultiSearch` instance is created via the main `Client::multi_search()` method.

use crate::{Client, Error};
use std::sync::Arc;
use typesense_codegen::{
    apis::{
        configuration::Configuration,
        documents_api::{self, MultiSearchParams},
    },
    models, // The generated model structs
};

/// Provides methods for managing Typesense API keys.
///
/// This struct is created by calling `client.keys()`.
pub struct MultiSearch<'a> {
    pub(super) client: &'a Client,
}

impl<'a> MultiSearch<'a> {
    /// Creates a new `MultiSearch` instance.
    pub(super) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Make multiple search requests in a single HTTP request to avoid round-trip network latencies.
    ///
    /// You can use it in two different modes:

    /// - Federated search: each search request in the multi-search payload returns results as independently.
    /// The results vector in the `multi_search` response is guaranteed to be in the same order as the queries you send in the `searches` vector in your request.
    /// - Union search: the response of each search request is merged into a single unified order.
    ///
    /// # Arguments
    /// * `search_requests` - A `MultiSearchSearchesParameter` contain multiple search requests, this will be sent in the request body.
    /// * `common_search_params` - A `MultiSearchParameters` describing search parameters that are common to all searches, these will be sent as URL query parameters.
    pub async fn perform(
        &self,
        search_requests: models::MultiSearchSearchesParameter,
        common_search_params: models::MultiSearchParameters,
    ) -> Result<models::MultiSearchResult, Error<documents_api::MultiSearchError>> {
        let params = common_search_params;
        let multi_search_params = MultiSearchParams {
            // enable_highlight_v1: None,
            // max_candidates: None,
            // max_filter_by_candidates: None,
            // split_join_tokens: None,
            multi_search_searches_parameter: Some(search_requests),

            // Common URL search params
            cache_ttl: params.cache_ttl,
            conversation: params.conversation,
            conversation_id: params.conversation_id,
            conversation_model_id: params.conversation_model_id,
            drop_tokens_mode: params.drop_tokens_mode,
            drop_tokens_threshold: params.drop_tokens_threshold,
            enable_overrides: params.enable_overrides,
            enable_synonyms: params.enable_synonyms,
            enable_typos_for_alpha_numerical_tokens: params.enable_typos_for_alpha_numerical_tokens,
            enable_typos_for_numerical_tokens: params.enable_typos_for_numerical_tokens,
            exclude_fields: params.exclude_fields,
            exhaustive_search: params.exhaustive_search,
            facet_by: params.facet_by,
            facet_query: params.facet_query,
            facet_return_parent: params.facet_return_parent,
            facet_strategy: params.facet_strategy,
            filter_by: params.filter_by,
            filter_curated_hits: params.filter_curated_hits,
            group_by: params.group_by,
            group_limit: params.group_limit,
            group_missing_values: params.group_missing_values,
            hidden_hits: params.hidden_hits,
            highlight_affix_num_tokens: params.highlight_affix_num_tokens,
            highlight_end_tag: params.highlight_end_tag,
            highlight_fields: params.highlight_fields,
            highlight_full_fields: params.highlight_full_fields,
            highlight_start_tag: params.highlight_start_tag,
            include_fields: params.include_fields,
            infix: params.infix,
            limit: params.limit,
            max_extra_prefix: params.max_extra_prefix,
            max_extra_suffix: params.max_extra_suffix,
            max_facet_values: params.max_facet_values,
            min_len_1typo: params.min_len_1typo,
            min_len_2typo: params.min_len_2typo,
            num_typos: params.num_typos,
            offset: params.offset,
            override_tags: params.override_tags,
            page: params.page,
            per_page: params.per_page,
            pinned_hits: params.pinned_hits,
            pre_segmented_query: params.pre_segmented_query,
            prefix: params.prefix,
            preset: params.preset,
            prioritize_exact_match: params.prioritize_exact_match,
            prioritize_num_matching_fields: params.prioritize_num_matching_fields,
            prioritize_token_position: params.prioritize_token_position,
            q: params.q,
            query_by: params.query_by,
            query_by_weights: params.query_by_weights,
            remote_embedding_num_tries: params.remote_embedding_num_tries,
            remote_embedding_timeout_ms: params.remote_embedding_timeout_ms,
            search_cutoff_ms: params.search_cutoff_ms,
            snippet_threshold: params.snippet_threshold,
            sort_by: params.sort_by,
            stopwords: params.stopwords,
            synonym_num_typos: params.synonym_num_typos,
            synonym_prefix: params.synonym_prefix,
            text_match_type: params.text_match_type,
            typo_tokens_threshold: params.typo_tokens_threshold,
            use_cache: params.use_cache,
            vector_query: params.vector_query,
            voice_query: params.voice_query,
        };
        self.client
            .execute(|config: Arc<Configuration>| {
                let params_for_move = multi_search_params.clone();
                async move { documents_api::multi_search(&config, params_for_move).await }
            })
            .await
    }
}
