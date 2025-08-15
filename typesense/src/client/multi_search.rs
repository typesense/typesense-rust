//! Provides access to the API endpoints for Multi Search.
//!
//! A `MultiSearch` instance is created via the main `Client::multi_search()` method.

use crate::{
    models::SearchResult, traits::MultiSearchResultExt, Client, Error, MultiSearchParseError,
    MultiSearchSearchesParameter,
};
use serde::de::DeserializeOwned;
use std::sync::Arc;
use typesense_codegen::{
    apis::{
        configuration::Configuration,
        documents_api::{self, MultiSearchParams},
    },
    models as raw_models,
};

fn multi_search_item_to_search_result(
    item: &raw_models::MultiSearchResultItem,
) -> raw_models::SearchResult {
    raw_models::SearchResult {
        hits: item.hits.clone(),
        facet_counts: item.facet_counts.clone(),
        grouped_hits: item.grouped_hits.clone(),
        found: item.found,
        found_docs: item.found_docs,
        out_of: item.out_of,
        page: item.page,
        search_time_ms: item.search_time_ms,
        search_cutoff: item.search_cutoff,
        request_params: item.request_params.clone(),
        conversation: item.conversation.clone(),
    }
}

impl MultiSearchResultExt for raw_models::MultiSearchResult {
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

        let raw_search_result = multi_search_item_to_search_result(raw_item);
        //    Map the serde error into our new, more specific error type.
        SearchResult::<T>::from_raw(raw_search_result)
            .map_err(|source| MultiSearchParseError::Deserialization { index, source })
    }
}

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

    /// Performs a **federated** multi-search operation, returning a list of search results.
    ///
    /// This function allows you to send multiple search queries in a single HTTP request, which is
    /// efficient for reducing network latency. It is specifically designed for federated searches,
    /// where each query in the request runs independently and returns its own corresponding result.
    ///
    /// The returned `MultiSearchResult` contains a `results` vector where each item maps to a
    /// query in the request, in the exact same order. To process these results in a type-safe
    /// way, you can use the `MultiSearchResultExt::parse_at` helper method.
    ///
    /// This is the default multi-search behavior in Typesense. For more details, see the
    /// [official Typesense API documentation on federated search](https://typesense.org/docs/latest/api/federated-multi-search.html#federated-search).
    ///
    /// For **union** searches that merge all hits into a single ranked list, use the
    /// `perform_union` method instead.
    ///
    /// # Example
    ///
    /// This example demonstrates a federated search across two different collections.
    ///
    /// ```no_run
    /// # use typesense::{Client, MultiNodeConfiguration, SearchResult, models, prelude::*};
    /// # use reqwest::Url;
    /// # use serde::Deserialize;
    /// #
    /// # // Define the structs for your documents for typed parsing.
    /// # #[derive(Deserialize, Debug)]
    /// # struct Product { id: String, name: String }
    /// # #[derive(Deserialize, Debug)]
    /// # struct Brand { id: String, company_name: String }
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let config = MultiNodeConfiguration {
    /// #     nodes: vec![Url::parse("http://localhost:8108")?],
    /// #     api_key: "xyz".to_string(),
    /// #     ..Default::default()
    /// # };
    /// # let client = Client::new(config)?;
    /// // Define the individual search queries for different collections.
    /// let search_requests = models::MultiSearchSearchesParameter {
    ///     searches: vec![
    ///         // Search #0 targets the 'products' collection
    ///         models::MultiSearchCollectionParameters {
    ///             collection: Some("products".to_string()),
    ///             q: Some("shoe".to_string()),
    ///             query_by: Some("name".to_string()),
    ///             ..Default::default()
    ///         },
    ///         // Search #1 targets the 'brands' collection
    ///         models::MultiSearchCollectionParameters {
    ///             collection: Some("brands".to_string()),
    ///             q: Some("nike".to_string()),
    ///             query_by: Some("company_name".to_string()),
    ///             ..Default::default()
    ///         },
    ///     ],
    ///     ..Default::default()
    /// };
    ///
    /// // Define parameters that will apply to all searches.
    /// let common_params = models::MultiSearchParameters::default();
    ///
    /// // Perform the federated multi-search.
    /// let raw_response = client
    ///     .multi_search()
    ///     .perform(&search_requests, &common_params)
    ///     .await?;
    ///
    /// // The raw response contains a vector of results.
    /// assert_eq!(raw_response.results.len(), 2);
    ///
    /// // Use the `parse_at` helper to get strongly-typed results for each search.
    /// let typed_products: SearchResult<Product> = raw_response.parse_at(0)?;
    /// let typed_brands: SearchResult<Brand> = raw_response.parse_at(1)?;
    ///
    /// println!("Found {} products.", typed_products.found.unwrap_or(0));
    /// println!("Found {} brands.", typed_brands.found.unwrap_or(0));
    /// # Ok(())
    /// # }
    /// ```
    /// # Arguments
    /// * `search_requests` - A reference to a `MultiSearchSearchesParameter` containing the list of individual search queries. The `union` field is ignored.
    /// * `common_search_params` - A reference to a `MultiSearchParameters` struct describing search parameters that are common to all searches.
    pub async fn perform(
        &self,
        search_requests: &MultiSearchSearchesParameter,
        common_search_params: &raw_models::MultiSearchParameters,
    ) -> Result<raw_models::MultiSearchResult, Error<documents_api::MultiSearchError>> {
        let request_body = raw_models::MultiSearchSearchesParameter {
            searches: search_requests.searches.clone(),
            ..Default::default()
        };
        let multi_search_params = build_multi_search_params(request_body, common_search_params);

        let raw_result = self
            .client
            .execute(|config: Arc<Configuration>| {
                let params_for_move: MultiSearchParams = multi_search_params.clone();
                async move { documents_api::multi_search(&config, params_for_move).await }
            })
            .await;

        // Now, handle the raw result and parse it into the strong type.
        match raw_result {
            Ok(json_value) => {
                // The API call was successful and returned a JSON value.
                // Now, we try to deserialize this value into our target struct.
                serde_json::from_value(json_value)
                    // If from_value fails, it returns a `serde_json::Error`.
                    // We need to map this into the expected `Error::Serde` variant
                    // that the calling function expects.
                    .map_err(Error::from)
            }
            Err(e) => {
                // The API call itself failed (e.g., network error, server 500).
                // In this case, we just propagate the original error.
                Err(e)
            }
        }
    }

    /// Performs a multi-search request in **union** mode, returning a single, merged `SearchResult`.
    ///
    /// This function is ideal for building a federated search experience where results from
    /// different collections are displayed together in a single, ranked list. It forces
    /// `union: true` in the search request.
    ///
    /// For more details, see the
    /// [official Typesense API documentation on union search](https://typesense.org/docs/latest/api/federated-multi-search.html#union-search).
    ///
    /// ### Handling Search Results
    ///
    /// The return type of this function is always `SearchResult<serde_json::Value>` because
    /// the search queries can target collections with different document schemas.
    ///
    /// #### 1. Heterogeneous Documents (Different Schemas)
    ///
    /// When searching across different collections (e.g., `products` and `brands`), you must
    /// inspect the `serde_json::Value` of each document to determine its type before
    /// deserializing it into a concrete struct.
    ///
    /// ```no_run
    /// # use typesense::models::SearchResult;
    /// # use serde_json::Value;
    /// # #[derive(serde::Deserialize)]
    /// # struct Product { name: String }
    /// # #[derive(serde::Deserialize)]
    /// # struct Brand { company_name: String }
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// # let search_result: SearchResult<Value> = todo!();
    /// for hit in search_result.hits.unwrap_or_default() {
    ///     if let Some(doc) = hit.document {
    ///         if doc.get("price").is_some() {
    ///             let product: Product = serde_json::from_value(doc)?;
    ///             println!("Found Product: {}", product.name);
    ///         } else if doc.get("country").is_some() {
    ///             let brand: Brand = serde_json::from_value(doc)?;
    ///             println!("Found Brand: {}", brand.company_name);
    ///         }
    ///     }
    /// }
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// #### 2. Homogeneous Documents (Same Schema)
    ///
    /// If all search queries target collections that share the **same schema**, you can
    /// convert the entire result into a strongly-typed `SearchResult<T>` using the
    /// [`SearchResult::try_into_typed`] helper method. This is much more convenient
    /// than parsing each hit individually.
    ///
    /// ```no_run
    /// # use typesense::models::SearchResult;
    /// # use serde_json::Value;
    /// # #[derive(serde::Deserialize)]
    /// # struct Product { name: String }
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client: typesense::Client = todo!();
    /// let value_result: SearchResult<Value> = client.multi_search().perform_union(todo!(), todo!()).await?;
    ///
    /// // Convert the entire result into a strongly-typed one.
    /// let typed_result: SearchResult<Product> = value_result.try_into_typed()?;
    ///
    /// if let Some(product) = typed_result.hits.unwrap_or_default().get(0) {
    ///     println!("Found product: {}", product.document.as_ref().unwrap().name);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Arguments
    ///
    /// * `search_requests` - A reference to a `MultiSearchSearchesParameter` containing the list of search queries to perform.
    /// * `common_search_params` - A reference to search parameters that will be applied to all individual searches.
    ///
    /// # Returns
    ///
    /// A `Result` containing a `SearchResult<serde_json::Value>` on success, or an `Error` on failure.
    pub async fn perform_union(
        &self,
        search_requests: &MultiSearchSearchesParameter,
        common_search_params: &raw_models::MultiSearchParameters,
    ) -> Result<SearchResult<serde_json::Value>, Error<documents_api::MultiSearchError>> {
        // Explicitly set `union: true` for the request body, overriding any user value.
        let request_body = raw_models::MultiSearchSearchesParameter {
            union: Some(true),
            searches: search_requests.searches.clone(),
        };

        let multi_search_params = build_multi_search_params(request_body, common_search_params);

        // Execute the request to get the raw JSON value
        let raw_result = self
            .client
            .execute(|config: Arc<Configuration>| {
                let params_for_move = multi_search_params.clone();
                async move { documents_api::multi_search(&config, params_for_move).await }
            })
            .await;

        // Handle the result: parse to raw SearchResult, then convert to generic SearchResult<Value>
        match raw_result {
            Ok(json_value) => {
                // A union search returns a single SearchResult object, not a MultiSearchResult.
                // First, parse into the non-generic, raw model.
                let raw_search_result: raw_models::SearchResult =
                    serde_json::from_value(json_value).map_err(Error::from)?;

                // Then, use your existing constructor to convert the raw result to the typed one,
                // specifying `serde_json::Value` as the document type.
                SearchResult::<serde_json::Value>::from_raw(raw_search_result).map_err(Error::from)
            }
            Err(e) => Err(e),
        }
    }
}

// Private helper function to construct the final search parameters object.
// This encapsulates the repetitive mapping logic.
fn build_multi_search_params(
    request_body: raw_models::MultiSearchSearchesParameter,
    params: &raw_models::MultiSearchParameters,
) -> MultiSearchParams {
    MultiSearchParams {
        multi_search_searches_parameter: Some(request_body),
        // Common URL search params
        cache_ttl: params.cache_ttl,
        conversation: params.conversation,
        conversation_id: params.conversation_id.clone(),
        conversation_model_id: params.conversation_model_id.clone(),
        drop_tokens_mode: params.drop_tokens_mode,
        drop_tokens_threshold: params.drop_tokens_threshold,
        enable_overrides: params.enable_overrides,
        enable_synonyms: params.enable_synonyms,
        enable_typos_for_alpha_numerical_tokens: params.enable_typos_for_alpha_numerical_tokens,
        enable_typos_for_numerical_tokens: params.enable_typos_for_numerical_tokens,
        exclude_fields: params.exclude_fields.clone(),
        exhaustive_search: params.exhaustive_search,
        facet_by: params.facet_by.clone(),
        facet_query: params.facet_query.clone(),
        facet_return_parent: params.facet_return_parent.clone(),
        facet_strategy: params.facet_strategy.clone(),
        filter_by: params.filter_by.clone(),
        filter_curated_hits: params.filter_curated_hits,
        group_by: params.group_by.clone(),
        group_limit: params.group_limit,
        group_missing_values: params.group_missing_values,
        hidden_hits: params.hidden_hits.clone(),
        highlight_affix_num_tokens: params.highlight_affix_num_tokens,
        highlight_end_tag: params.highlight_end_tag.clone(),
        highlight_fields: params.highlight_fields.clone(),
        highlight_full_fields: params.highlight_full_fields.clone(),
        highlight_start_tag: params.highlight_start_tag.clone(),
        include_fields: params.include_fields.clone(),
        infix: params.infix.clone(),
        limit: params.limit,
        max_extra_prefix: params.max_extra_prefix,
        max_extra_suffix: params.max_extra_suffix,
        max_facet_values: params.max_facet_values,
        min_len_1typo: params.min_len_1typo,
        min_len_2typo: params.min_len_2typo,
        num_typos: params.num_typos.clone(),
        offset: params.offset,
        override_tags: params.override_tags.clone(),
        page: params.page,
        per_page: params.per_page,
        pinned_hits: params.pinned_hits.clone(),
        pre_segmented_query: params.pre_segmented_query,
        prefix: params.prefix.clone(),
        preset: params.preset.clone(),
        prioritize_exact_match: params.prioritize_exact_match,
        prioritize_num_matching_fields: params.prioritize_num_matching_fields,
        prioritize_token_position: params.prioritize_token_position,
        q: params.q.clone(),
        query_by: params.query_by.clone(),
        query_by_weights: params.query_by_weights.clone(),
        remote_embedding_num_tries: params.remote_embedding_num_tries,
        remote_embedding_timeout_ms: params.remote_embedding_timeout_ms,
        search_cutoff_ms: params.search_cutoff_ms,
        snippet_threshold: params.snippet_threshold,
        sort_by: params.sort_by.clone(),
        stopwords: params.stopwords.clone(),
        synonym_num_typos: params.synonym_num_typos,
        synonym_prefix: params.synonym_prefix,
        text_match_type: params.text_match_type.clone(),
        typo_tokens_threshold: params.typo_tokens_threshold,
        use_cache: params.use_cache,
        vector_query: params.vector_query.clone(),
        voice_query: params.voice_query.clone(),
        // enable_highlight_v1: None,
        // max_candidates: None,
        // max_filter_by_candidates: None,
        // split_join_tokens: None,
    }
}
