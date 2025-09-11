//! Provides access to the API endpoints for Multi Search.
//!
//! A `MultiSearch` instance is created via the main `client.multi_search()` method.

use crate::{
    Client, Error, execute_wrapper,
    models::{MultiSearchBody, SearchResult},
};
use typesense_codegen::{
    apis::documents_api::{self, MultiSearchParams},
    models as raw_models,
};

/// Provides methods for performing multi-search operations across multiple collections.
///
/// This struct is created by calling `client.multi_search()`.
pub struct MultiSearch<'a> {
    pub(super) client: &'a Client,
}

impl<'a> MultiSearch<'a> {
    /// Creates a new `MultiSearch` instance.
    #[inline]
    pub(super) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Performs a **federated** multi-search operation, returning a list of search results.
    ///
    /// This function allows you to send multiple search queries in a single HTTP request, which is
    /// efficient for reducing network latency.
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
    /// # #[cfg(not(target_family = "wasm"))]
    /// # {
    /// # use typesense::{Client, models::{self, SearchResult}, prelude::*};
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
    /// # let client = Client::builder()
    /// #    .nodes(vec![Url::parse("http://localhost:8108").unwrap()])
    /// #    .api_key("xyz")
    /// #    .build()
    /// #    .unwrap();
    /// // Define the individual search queries for different collections.
    /// let search_requests = models::MultiSearchBody {
    ///     searches: vec![
    ///         // Search #0 targets the 'products' collection
    ///         models::MultiSearchCollectionParameters {
    ///             collection: Some("products".to_owned()),
    ///             q: Some("shoe".to_owned()),
    ///             query_by: Some("name".to_owned()),
    ///             ..Default::default()
    ///         },
    ///         // Search #1 targets the 'brands' collection
    ///         models::MultiSearchCollectionParameters {
    ///             collection: Some("brands".to_owned()),
    ///             q: Some("nike".to_owned()),
    ///             query_by: Some("company_name".to_owned()),
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
    ///     .perform(search_requests, common_params)
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
    /// # }
    /// ```
    /// # Arguments
    /// * `search_requests` - A `MultiSearchBody` containing the list of individual search queries. The `union` field is ignored.
    /// * `common_search_params` - A `MultiSearchParameters` struct describing search parameters that are common to all searches.
    pub async fn perform(
        &self,
        search_requests: MultiSearchBody,
        common_search_params: raw_models::MultiSearchParameters,
    ) -> Result<
        raw_models::MultiSearchResult<serde_json::Value>,
        Error<documents_api::MultiSearchError>,
    > {
        let request_body = raw_models::MultiSearchSearchesParameter {
            searches: search_requests.searches,
            ..Default::default()
        };
        let multi_search_params = build_multi_search_params(request_body, common_search_params);

        let raw_result = execute_wrapper!(self, documents_api::multi_search, multi_search_params);

        // Now, handle the raw result and parse it into the strong type.
        match raw_result {
            Ok(json_value) => serde_json::from_value(json_value).map_err(Error::from),
            Err(e) => Err(e),
        }
    }

    /// Performs a multi-search request in **union** mode, returning a single, merged `SearchResult`.
    ///
    /// For more details, see the
    /// [official Typesense API documentation on union search](https://typesense.org/docs/latest/api/federated-multi-search.html#union-search).
    ///
    /// ### Handling Search Results
    ///
    /// #### 1. Heterogeneous Documents (Different Schemas)
    ///
    /// When searching across different collections (e.g., `products` and `brands`), generic parameter `D` must be `serde_json::Value`.
    /// You must inspect the `serde_json::Value` of each document to determine its type before
    /// deserializing it into a concrete struct.
    ///
    /// ```no_run
    /// # use typesense::{models, Client};
    /// # use serde_json::Value;
    /// # use reqwest::Url;
    /// # #[derive(serde::Deserialize)]
    /// # struct Product { name: String }
    /// # #[derive(serde::Deserialize)]
    /// # struct Brand { company_name: String }
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = Client::builder()
    /// #    .nodes(vec![Url::parse("http://localhost:8108").unwrap()])
    /// #    .api_key("xyz")
    /// #    .build()
    /// #    .unwrap();
    /// let search_requests = models::MultiSearchBody {
    ///     searches: vec![
    ///         // Search #0 targets the 'products' collection
    ///         models::MultiSearchCollectionParameters {
    ///             collection: Some("products".to_owned()),
    ///             q: Some("shoe".to_owned()),
    ///             query_by: Some("name".to_owned()),
    ///             ..Default::default()
    ///         },
    ///         // Search #1 targets the 'brands' collection
    ///         models::MultiSearchCollectionParameters {
    ///             collection: Some("brands".to_owned()),
    ///             q: Some("nike".to_owned()),
    ///             query_by: Some("company_name".to_owned()),
    ///             ..Default::default()
    ///         },
    ///     ],
    ///     ..Default::default()
    /// };
    /// let common_params = models::MultiSearchParameters::default();
    ///
    /// let search_result: models::SearchResult<Value> = client.multi_search().perform_union(search_requests, common_params).await?;
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
    /// If all search queries target collections that share the **same schema**, you can directly use the concrete type for `D`.
    ///
    /// ```no_run
    /// # use typesense::{models, Client};
    /// # use reqwest::Url;
    /// # use serde_json::Value;
    /// # #[derive(serde::Deserialize)]
    /// # struct Product { name: String }
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = Client::builder()
    /// #    .nodes(vec![Url::parse("http://localhost:8108").unwrap()])
    /// #    .api_key("xyz")
    /// #    .build()
    /// #    .unwrap();
    /// let search_requests = models::MultiSearchBody {
    ///     searches: vec![
    ///         models::MultiSearchCollectionParameters {
    ///             collection: Some("products".to_owned()),
    ///             q: Some("shoe".to_owned()),
    ///             query_by: Some("name".to_owned()),
    ///             ..Default::default()
    ///         },
    ///         models::MultiSearchCollectionParameters {
    ///             collection: Some("products".to_owned()),
    ///             q: Some("sock".to_owned()),
    ///             query_by: Some("name".to_owned()),
    ///             ..Default::default()
    ///         },
    ///     ],
    ///     ..Default::default()
    /// };
    /// let common_params = models::MultiSearchParameters::default();
    ///
    /// let typed_result: models::SearchResult<Product> = client.multi_search().perform_union(search_requests, common_params).await?;
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
    /// * `search_requests` - A `MultiSearchBody` containing the list of search queries to perform.
    /// * `common_search_params` - A `MultiSearchParameters` which will be applied to all individual searches.
    ///
    /// # Returns
    ///
    /// A `Result` containing a `SearchResult<D>` on success, or an `Error` on failure.
    pub async fn perform_union<D: for<'de> serde::Deserialize<'de>>(
        &self,
        search_requests: MultiSearchBody,
        common_search_params: raw_models::MultiSearchParameters,
    ) -> Result<SearchResult<D>, Error<documents_api::MultiSearchError>> {
        // Explicitly set `union: true` for the request body
        let request_body = raw_models::MultiSearchSearchesParameter {
            union: Some(true),
            searches: search_requests.searches,
        };

        let multi_search_params = build_multi_search_params(request_body, common_search_params);

        // Execute the request to get the raw JSON value
        let raw_result = execute_wrapper!(self, documents_api::multi_search, multi_search_params);

        match raw_result {
            Ok(json_value) => serde_json::from_value(json_value).map_err(Error::from),
            Err(e) => Err(e),
        }
    }
}
// Private helper function to construct the final search parameters object.
// This encapsulates the repetitive mapping logic.
fn build_multi_search_params(
    request_body: raw_models::MultiSearchSearchesParameter,
    params: raw_models::MultiSearchParameters,
) -> MultiSearchParams {
    MultiSearchParams {
        multi_search_searches_parameter: Some(request_body),
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
        enable_analytics: params.enable_analytics,
        // enable_highlight_v1: None,
        // max_candidates: None,
        // max_filter_by_candidates: None,
        // split_join_tokens: None,
    }
}
