/*
 * Typesense API
 *
 * An open source search engine for building delightful search experiences.
 *
 * The version of the OpenAPI document: 0.25.0
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchParameters {
    /// The query text to search for in the collection. Use * as the search string to return all documents. This is typically useful when used in conjunction with filter_by.
    #[serde(rename = "q")]
    pub q: String,
    /// A list of `string` fields that should be queried against. Multiple fields are separated with a comma.
    #[serde(rename = "query_by")]
    pub query_by: String,
    /// The relative weight to give each `query_by` field when ranking results. This can be used to boost fields in priority, when looking for matches. Multiple fields are separated with a comma.
    #[serde(rename = "query_by_weights", skip_serializing_if = "Option::is_none")]
    pub query_by_weights: Option<String>,
    /// In a multi-field matching context, this parameter determines how the representative text match score of a record is calculated. Possible values are max_score (default) or max_weight.
    #[serde(rename = "text_match_type", skip_serializing_if = "Option::is_none")]
    pub text_match_type: Option<String>,
    /// Boolean field to indicate that the last word in the query should be treated as a prefix, and not as a whole word. This is used for building autocomplete and instant search interfaces. Defaults to true.
    #[serde(rename = "prefix", skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    /// If infix index is enabled for this field, infix searching can be done on a per-field basis by sending a comma separated string parameter called infix to the search query. This parameter can have 3 values; `off` infix search is disabled, which is default `always` infix search is performed along with regular search `fallback` infix search is performed if regular search does not produce results
    #[serde(rename = "infix", skip_serializing_if = "Option::is_none")]
    pub infix: Option<String>,
    /// There are also 2 parameters that allow you to control the extent of infix searching max_extra_prefix and max_extra_suffix which specify the maximum number of symbols before or after the query that can be present in the token. For example query \"K2100\" has 2 extra symbols in \"6PK2100\". By default, any number of prefixes/suffixes can be present for a match.
    #[serde(rename = "max_extra_prefix", skip_serializing_if = "Option::is_none")]
    pub max_extra_prefix: Option<i32>,
    /// There are also 2 parameters that allow you to control the extent of infix searching max_extra_prefix and max_extra_suffix which specify the maximum number of symbols before or after the query that can be present in the token. For example query \"K2100\" has 2 extra symbols in \"6PK2100\". By default, any number of prefixes/suffixes can be present for a match.
    #[serde(rename = "max_extra_suffix", skip_serializing_if = "Option::is_none")]
    pub max_extra_suffix: Option<i32>,
    /// Filter conditions for refining youropen api validator search results. Separate multiple conditions with &&.
    #[serde(rename = "filter_by", skip_serializing_if = "Option::is_none")]
    pub filter_by: Option<String>,
    /// A list of numerical fields and their corresponding sort orders that will be used for ordering your results. Up to 3 sort fields can be specified. The text similarity score is exposed as a special `_text_match` field that you can use in the list of sorting fields. If no `sort_by` parameter is specified, results are sorted by `_text_match:desc,default_sorting_field:desc`
    #[serde(rename = "sort_by", skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// A list of fields that will be used for faceting your results on. Separate multiple fields with a comma.
    #[serde(rename = "facet_by", skip_serializing_if = "Option::is_none")]
    pub facet_by: Option<String>,
    /// Maximum number of facet values to be returned.
    #[serde(rename = "max_facet_values", skip_serializing_if = "Option::is_none")]
    pub max_facet_values: Option<i32>,
    /// Facet values that are returned can now be filtered via this parameter. The matching facet text is also highlighted. For example, when faceting by `category`, you can set `facet_query=category:shoe` to return only facet values that contain the prefix \"shoe\".
    #[serde(rename = "facet_query", skip_serializing_if = "Option::is_none")]
    pub facet_query: Option<String>,
    /// The number of typographical errors (1 or 2) that would be tolerated. Default: 2
    #[serde(rename = "num_typos", skip_serializing_if = "Option::is_none")]
    pub num_typos: Option<String>,
    /// Results from this specific page number would be fetched.
    #[serde(rename = "page", skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    /// Number of results to fetch per page. Default: 10
    #[serde(rename = "per_page", skip_serializing_if = "Option::is_none")]
    pub per_page: Option<i32>,
    /// Number of hits to fetch. Can be used as an alternative to the per_page parameter.  Default: 10.
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    /// Identifies the starting point to return hits from a result set. Can be used as an alternative to the page parameter.
    #[serde(rename = "offset", skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    /// You can aggregate search results into groups or buckets by specify one or more `group_by` fields. Separate multiple fields with a comma. To group on a particular field, it must be a faceted field.
    #[serde(rename = "group_by", skip_serializing_if = "Option::is_none")]
    pub group_by: Option<String>,
    /// Maximum number of hits to be returned for every group. If the `group_limit` is set as `K` then only the top K hits in each group are returned in the response. Default: 3
    #[serde(rename = "group_limit", skip_serializing_if = "Option::is_none")]
    pub group_limit: Option<i32>,
    /// List of fields from the document to include in the search result
    #[serde(rename = "include_fields", skip_serializing_if = "Option::is_none")]
    pub include_fields: Option<String>,
    /// List of fields from the document to exclude in the search result
    #[serde(rename = "exclude_fields", skip_serializing_if = "Option::is_none")]
    pub exclude_fields: Option<String>,
    /// List of fields which should be highlighted fully without snippeting
    #[serde(
        rename = "highlight_full_fields",
        skip_serializing_if = "Option::is_none"
    )]
    pub highlight_full_fields: Option<String>,
    /// The number of tokens that should surround the highlighted text on each side. Default: 4
    #[serde(
        rename = "highlight_affix_num_tokens",
        skip_serializing_if = "Option::is_none"
    )]
    pub highlight_affix_num_tokens: Option<i32>,
    /// The start tag used for the highlighted snippets. Default: `<mark>`
    #[serde(
        rename = "highlight_start_tag",
        skip_serializing_if = "Option::is_none"
    )]
    pub highlight_start_tag: Option<String>,
    /// The end tag used for the highlighted snippets. Default: `</mark>`
    #[serde(rename = "highlight_end_tag", skip_serializing_if = "Option::is_none")]
    pub highlight_end_tag: Option<String>,
    /// Flag for enabling/disabling the deprecated, old highlight structure in the response. Default: true
    #[serde(
        rename = "enable_highlight_v1",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_highlight_v1: Option<bool>,
    /// Field values under this length will be fully highlighted, instead of showing a snippet of relevant portion. Default: 30
    #[serde(rename = "snippet_threshold", skip_serializing_if = "Option::is_none")]
    pub snippet_threshold: Option<i32>,
    /// If the number of results found for a specific query is less than this number, Typesense will attempt to drop the tokens in the query until enough results are found. Tokens that have the least individual hits are dropped first. Set to 0 to disable. Default: 10
    #[serde(
        rename = "drop_tokens_threshold",
        skip_serializing_if = "Option::is_none"
    )]
    pub drop_tokens_threshold: Option<i32>,
    /// If the number of results found for a specific query is less than this number, Typesense will attempt to look for tokens with more typos until enough results are found. Default: 100
    #[serde(
        rename = "typo_tokens_threshold",
        skip_serializing_if = "Option::is_none"
    )]
    pub typo_tokens_threshold: Option<i32>,
    /// A list of records to unconditionally include in the search results at specific positions. An example use case would be to feature or promote certain items on the top of search results. A list of `record_id:hit_position`. Eg: to include a record with ID 123 at Position 1 and another record with ID 456 at Position 5, you'd specify `123:1,456:5`. You could also use the Overrides feature to override search results based on rules. Overrides are applied first, followed by `pinned_hits` and  finally `hidden_hits`.
    #[serde(rename = "pinned_hits", skip_serializing_if = "Option::is_none")]
    pub pinned_hits: Option<String>,
    /// A list of records to unconditionally hide from search results. A list of `record_id`s to hide. Eg: to hide records with IDs 123 and 456, you'd specify `123,456`. You could also use the Overrides feature to override search results based on rules. Overrides are applied first, followed by `pinned_hits` and finally `hidden_hits`.
    #[serde(rename = "hidden_hits", skip_serializing_if = "Option::is_none")]
    pub hidden_hits: Option<String>,
    /// A list of custom fields that must be highlighted even if you don't query  for them
    #[serde(rename = "highlight_fields", skip_serializing_if = "Option::is_none")]
    pub highlight_fields: Option<String>,
    /// Treat space as typo: search for q=basket ball if q=basketball is not found or vice-versa. Splitting/joining of tokens will only be attempted if the original query produces no results. To always trigger this behavior, set value to `always``. To disable, set value to `off`. Default is `fallback`.
    #[serde(rename = "split_join_tokens", skip_serializing_if = "Option::is_none")]
    pub split_join_tokens: Option<String>,
    /// You can index content from any logographic language into Typesense if you are able to segment / split the text into space-separated words yourself  before indexing and querying. Set this parameter to true to do the same
    #[serde(
        rename = "pre_segmented_query",
        skip_serializing_if = "Option::is_none"
    )]
    pub pre_segmented_query: Option<bool>,
    /// Search using a bunch of search parameters by setting this parameter to the name of the existing Preset.
    #[serde(rename = "preset", skip_serializing_if = "Option::is_none")]
    pub preset: Option<String>,
    /// If you have some overrides defined but want to disable all of them during query time, you can do that by setting this parameter to false
    #[serde(rename = "enable_overrides", skip_serializing_if = "Option::is_none")]
    pub enable_overrides: Option<bool>,
    /// Set this parameter to true to ensure that an exact match is ranked above the others
    #[serde(
        rename = "prioritize_exact_match",
        skip_serializing_if = "Option::is_none"
    )]
    pub prioritize_exact_match: Option<bool>,
    /// Control the number of words that Typesense considers for typo and prefix searching.
    #[serde(rename = "max_candidates", skip_serializing_if = "Option::is_none")]
    pub max_candidates: Option<i32>,
    /// Make Typesense prioritize documents where the query words appear earlier in the text.
    #[serde(
        rename = "prioritize_token_position",
        skip_serializing_if = "Option::is_none"
    )]
    pub prioritize_token_position: Option<bool>,
    /// Setting this to true will make Typesense consider all prefixes and typo  corrections of the words in the query without stopping early when enough results are found  (drop_tokens_threshold and typo_tokens_threshold configurations are ignored).
    #[serde(rename = "exhaustive_search", skip_serializing_if = "Option::is_none")]
    pub exhaustive_search: Option<bool>,
    /// Typesense will attempt to return results early if the cutoff time has elapsed.  This is not a strict guarantee and facet computation is not bound by this parameter.
    #[serde(rename = "search_cutoff_ms", skip_serializing_if = "Option::is_none")]
    pub search_cutoff_ms: Option<i32>,
    /// Enable server side caching of search query results. By default, caching is disabled.
    #[serde(rename = "use_cache", skip_serializing_if = "Option::is_none")]
    pub use_cache: Option<bool>,
    /// The duration (in seconds) that determines how long the search query is cached.  This value can be set on a per-query basis. Default: 60.
    #[serde(rename = "cache_ttl", skip_serializing_if = "Option::is_none")]
    pub cache_ttl: Option<i32>,
    /// Minimum word length for 1-typo correction to be applied.  The value of num_typos is still treated as the maximum allowed typos.
    #[serde(rename = "min_len_1typo", skip_serializing_if = "Option::is_none")]
    pub min_len_1typo: Option<i32>,
    /// Minimum word length for 2-typo correction to be applied.  The value of num_typos is still treated as the maximum allowed typos.
    #[serde(rename = "min_len_2typo", skip_serializing_if = "Option::is_none")]
    pub min_len_2typo: Option<i32>,
    /// Vector query expression for fetching documents \"closest\" to a given query/document vector.
    #[serde(rename = "vector_query", skip_serializing_if = "Option::is_none")]
    pub vector_query: Option<String>,
    /// Timeout (in milliseconds) for fetching remote embeddings.
    #[serde(
        rename = "remote_embedding_timeout_ms",
        skip_serializing_if = "Option::is_none"
    )]
    pub remote_embedding_timeout_ms: Option<i32>,
    /// Number of times to retry fetching remote embeddings.
    #[serde(
        rename = "remote_embedding_num_tries",
        skip_serializing_if = "Option::is_none"
    )]
    pub remote_embedding_num_tries: Option<i32>,
}

impl SearchParameters {
    pub fn new(q: String, query_by: String) -> SearchParameters {
        SearchParameters {
            q,
            query_by,
            query_by_weights: None,
            text_match_type: None,
            prefix: None,
            infix: None,
            max_extra_prefix: None,
            max_extra_suffix: None,
            filter_by: None,
            sort_by: None,
            facet_by: None,
            max_facet_values: None,
            facet_query: None,
            num_typos: None,
            page: None,
            per_page: None,
            limit: None,
            offset: None,
            group_by: None,
            group_limit: None,
            include_fields: None,
            exclude_fields: None,
            highlight_full_fields: None,
            highlight_affix_num_tokens: None,
            highlight_start_tag: None,
            highlight_end_tag: None,
            enable_highlight_v1: None,
            snippet_threshold: None,
            drop_tokens_threshold: None,
            typo_tokens_threshold: None,
            pinned_hits: None,
            hidden_hits: None,
            highlight_fields: None,
            split_join_tokens: None,
            pre_segmented_query: None,
            preset: None,
            enable_overrides: None,
            prioritize_exact_match: None,
            max_candidates: None,
            prioritize_token_position: None,
            exhaustive_search: None,
            search_cutoff_ms: None,
            use_cache: None,
            cache_ttl: None,
            min_len_1typo: None,
            min_len_2typo: None,
            vector_query: None,
            remote_embedding_timeout_ms: None,
            remote_embedding_num_tries: None,
        }
    }
}
