//! Module for the `SearchParameters` builder.

use crate::models::{DropTokensMode, SearchParameters};
use bon::builder;

/// Creates a new [`SearchParameters`] builder.
///
/// This builder provides a convenient way to construct a `SearchParameters` object
/// for a Typesense search query. All parameters are optional.
#[builder(
    // expose a public builder type named `SearchParametersBuilder` and a public finish_fn `build()`
    builder_type(name = SearchParametersBuilder, vis = "pub"),
    finish_fn(name = build, vis = "pub"),
    // allow passing &str into String params
    on(String, into)
)]
pub fn new_search_parameters(
    /// The query text to search for in the collection. Use * as the search string to return all documents. This is typically useful when used in conjunction with filter_by.
    q: Option<String>,
    /// A list of `string` fields that should be queried against. Multiple fields are separated with a comma.
    query_by: Option<String>,
    /// Whether to use natural language processing to parse the query.
    nl_query: Option<bool>,
    /// The ID of the natural language model to use.
    nl_model_id: Option<String>,
    /// The relative weight to give each `query_by` field when ranking results. This can be used to boost fields in priority, when looking for matches. Multiple fields are separated with a comma.
    query_by_weights: Option<String>,
    /// In a multi-field matching context, this parameter determines how the representative text match score of a record is calculated. Possible values are max_score (default) or max_weight.
    text_match_type: Option<String>,
    /// Boolean field to indicate that the last word in the query should be treated as a prefix, and not as a whole word. This is used for building autocomplete and instant search interfaces. Defaults to true.
    prefix: Option<String>,
    /// If infix index is enabled for this field, infix searching can be done on a per-field basis by sending a comma separated string parameter called infix to the search query. This parameter can have 3 values; `off` infix search is disabled, which is default `always` infix search is performed along with regular search `fallback` infix search is performed if regular search does not produce results
    infix: Option<String>,
    /// There are also 2 parameters that allow you to control the extent of infix searching max_extra_prefix and max_extra_suffix which specify the maximum number of symbols before or after the query that can be present in the token. For example query \"K2100\" has 2 extra symbols in \"6PK2100\". By default, any number of prefixes/suffixes can be present for a match.
    max_extra_prefix: Option<i32>,
    /// There are also 2 parameters that allow you to control the extent of infix searching max_extra_prefix and max_extra_suffix which specify the maximum number of symbols before or after the query that can be present in the token. For example query \"K2100\" has 2 extra symbols in \"6PK2100\". By default, any number of prefixes/suffixes can be present for a match.
    max_extra_suffix: Option<i32>,
    /// Filter conditions for refining youropen api validator search results. Separate multiple conditions with &&.
    filter_by: Option<String>,
    /// Controls the number of similar words that Typesense considers during fuzzy search on filter_by values. Useful for controlling prefix matches like company_name:Acm*.
    max_filter_by_candidates: Option<i32>,
    /// A list of numerical fields and their corresponding sort orders that will be used for ordering your results. Up to 3 sort fields can be specified. The text similarity score is exposed as a special `_text_match` field that you can use in the list of sorting fields. If no `sort_by` parameter is specified, results are sorted by `_text_match:desc,default_sorting_field:desc`
    sort_by: Option<String>,
    /// A list of fields that will be used for faceting your results on. Separate multiple fields with a comma.
    facet_by: Option<String>,
    /// Maximum number of facet values to be returned.
    max_facet_values: Option<i32>,
    /// Facet values that are returned can now be filtered via this parameter. The matching facet text is also highlighted. For example, when faceting by `category`, you can set `facet_query=category:shoe` to return only facet values that contain the prefix \"shoe\".
    facet_query: Option<String>,
    /// The number of typographical errors (1 or 2) that would be tolerated. Default: 2
    num_typos: Option<String>,
    /// Results from this specific page number would be fetched.
    page: Option<i32>,
    /// Number of results to fetch per page. Default: 10
    per_page: Option<i32>,
    /// Number of hits to fetch. Can be used as an alternative to the per_page parameter. Default: 10.
    limit: Option<i32>,
    /// Identifies the starting point to return hits from a result set. Can be used as an alternative to the page parameter.
    offset: Option<i32>,
    /// You can aggregate search results into groups or buckets by specify one or more `group_by` fields. Separate multiple fields with a comma. To group on a particular field, it must be a faceted field.
    group_by: Option<String>,
    /// Maximum number of hits to be returned for every group. If the `group_limit` is set as `K` then only the top K hits in each group are returned in the response. Default: 3
    group_limit: Option<i32>,
    /// Setting this parameter to true will place all documents that have a null value in the group_by field, into a single group. Setting this parameter to false, will cause each document with a null value in the group_by field to not be grouped with other documents. Default: true
    group_missing_values: Option<bool>,
    /// List of fields from the document to include in the search result
    include_fields: Option<String>,
    /// List of fields from the document to exclude in the search result
    exclude_fields: Option<String>,
    /// List of fields which should be highlighted fully without snippeting
    highlight_full_fields: Option<String>,
    /// The number of tokens that should surround the highlighted text on each side. Default: 4
    highlight_affix_num_tokens: Option<i32>,
    /// The start tag used for the highlighted snippets. Default: `<mark>`
    highlight_start_tag: Option<String>,
    /// The end tag used for the highlighted snippets. Default: `</mark>`
    highlight_end_tag: Option<String>,
    /// Flag for enabling/disabling the deprecated, old highlight structure in the response. Default: true
    enable_highlight_v1: Option<bool>,
    /// Field values under this length will be fully highlighted, instead of showing a snippet of relevant portion. Default: 30
    snippet_threshold: Option<i32>,
    /// If the number of results found for a specific query is less than this number, Typesense will attempt to drop the tokens in the query until enough results are found. Tokens that have the least individual hits are dropped first. Set to 0 to disable. Default: 10
    drop_tokens_threshold: Option<i32>,
    drop_tokens_mode: Option<DropTokensMode>,
    /// If the number of results found for a specific query is less than this number, Typesense will attempt to look for tokens with more typos until enough results are found. Default: 100
    typo_tokens_threshold: Option<i32>,
    /// Set this parameter to false to disable typos on alphanumerical query tokens. Default: true.
    enable_typos_for_alpha_numerical_tokens: Option<bool>,
    /// Whether the filter_by condition of the search query should be applicable to curated results (override definitions, pinned hits, hidden hits, etc.). Default: false
    filter_curated_hits: Option<bool>,
    /// If you have some synonyms defined but want to disable all of them for a particular search query, set enable_synonyms to false. Default: true
    enable_synonyms: Option<bool>,
    /// Allow synonym resolution on word prefixes in the query. Default: false
    synonym_prefix: Option<bool>,
    /// Allow synonym resolution on typo-corrected words in the query. Default: 0
    synonym_num_typos: Option<i32>,
    /// A list of records to unconditionally include in the search results at specific positions. An example use case would be to feature or promote certain items on the top of search results. A list of `record_id:hit_position`. Eg: to include a record with ID 123 at Position 1 and another record with ID 456 at Position 5, you'd specify `123:1,456:5`. You could also use the Overrides feature to override search results based on rules. Overrides are applied first, followed by `pinned_hits` and finally `hidden_hits`.
    pinned_hits: Option<String>,
    /// A list of records to unconditionally hide from search results. A list of `record_id`s to hide. Eg: to hide records with IDs 123 and 456, you'd specify `123,456`. You could also use the Overrides feature to override search results based on rules. Overrides are applied first, followed by `pinned_hits` and finally `hidden_hits`.
    hidden_hits: Option<String>,
    /// Comma separated list of tags to trigger the curations rules that match the tags.
    override_tags: Option<String>,
    /// A list of custom fields that must be highlighted even if you don't query for them
    highlight_fields: Option<String>,
    /// Treat space as typo: search for q=basket ball if q=basketball is not found or vice-versa. Splitting/joining of tokens will only be attempted if the original query produces no results. To always trigger this behavior, set value to `always``. To disable, set value to `off`. Default is `fallback`.
    split_join_tokens: Option<String>,
    /// You can index content from any logographic language into Typesense if you are able to segment / split the text into space-separated words yourself before indexing and querying. Set this parameter to true to do the same
    pre_segmented_query: Option<bool>,
    /// Search using a bunch of search parameters by setting this parameter to the name of the existing Preset.
    preset: Option<String>,
    /// If you have some overrides defined but want to disable all of them during query time, you can do that by setting this parameter to false
    enable_overrides: Option<bool>,
    /// Set this parameter to true to ensure that an exact match is ranked above the others
    prioritize_exact_match: Option<bool>,
    /// Control the number of words that Typesense considers for typo and prefix searching.
    max_candidates: Option<i32>,
    /// Make Typesense prioritize documents where the query words appear earlier in the text.
    prioritize_token_position: Option<bool>,
    /// Make Typesense prioritize documents where the query words appear in more number of fields.
    prioritize_num_matching_fields: Option<bool>,
    /// Make Typesense disable typos for numerical tokens.
    enable_typos_for_numerical_tokens: Option<bool>,
    /// Setting this to true will make Typesense consider all prefixes and typo corrections of the words in the query without stopping early when enough results are found (drop_tokens_threshold and typo_tokens_threshold configurations are ignored).
    exhaustive_search: Option<bool>,
    /// Typesense will attempt to return results early if the cutoff time has elapsed. This is not a strict guarantee and facet computation is not bound by this parameter.
    search_cutoff_ms: Option<i32>,
    /// Enable server side caching of search query results. By default, caching is disabled.
    use_cache: Option<bool>,
    /// The duration (in seconds) that determines how long the search query is cached. This value can be set on a per-query basis. Default: 60.
    cache_ttl: Option<i32>,
    /// Minimum word length for 1-typo correction to be applied. The value of num_typos is still treated as the maximum allowed typos.
    min_len_1typo: Option<i32>,
    /// Minimum word length for 2-typo correction to be applied. The value of num_typos is still treated as the maximum allowed typos.
    min_len_2typo: Option<i32>,
    /// Vector query expression for fetching documents \"closest\" to a given query/document vector.
    vector_query: Option<String>,
    /// Timeout (in milliseconds) for fetching remote embeddings.
    remote_embedding_timeout_ms: Option<i32>,
    /// Number of times to retry fetching remote embeddings.
    remote_embedding_num_tries: Option<i32>,
    /// Choose the underlying faceting strategy used. Comma separated string of allows values: exhaustive, top_values or automatic (default).
    facet_strategy: Option<String>,
    /// Name of the stopwords set to apply for this search, the keywords present in the set will be removed from the search query.
    stopwords: Option<String>,
    /// Comma separated string of nested facet fields whose parent object should be returned in facet response.
    facet_return_parent: Option<String>,
    /// The base64 encoded audio file in 16 khz 16-bit WAV format.
    voice_query: Option<String>,
    /// Enable conversational search.
    conversation: Option<bool>,
    /// The Id of Conversation Model to be used.
    conversation_model_id: Option<String>,
    /// The Id of a previous conversation to continue, this tells Typesense to include prior context when communicating with the LLM.
    conversation_id: Option<String>,
) -> SearchParameters {
    SearchParameters {
        q,
        query_by,
        nl_query,
        nl_model_id,
        query_by_weights,
        text_match_type,
        prefix,
        infix,
        max_extra_prefix,
        max_extra_suffix,
        filter_by,
        max_filter_by_candidates,
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
        enable_highlight_v1,
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
        split_join_tokens,
        pre_segmented_query,
        preset,
        enable_overrides,
        prioritize_exact_match,
        max_candidates,
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
    use crate::models::{DropTokensMode, SearchParameters};

    #[test]
    fn test_search_parameters_builder_basic() {
        let built = new_search_parameters()
            .q("a test query")
            .query_by("title,description")
            .per_page(15)
            .build();

        let expected = SearchParameters {
            q: Some("a test query".to_string()),
            query_by: Some("title,description".to_string()),
            per_page: Some(15),
            ..Default::default()
        };

        assert_eq!(built, expected);
    }

    #[test]
    fn test_search_parameters_builder_full() {
        let built = new_search_parameters()
            .q("*")
            .filter_by("stock > 0")
            .use_cache(true)
            .drop_tokens_mode(DropTokensMode::LeftToRight)
            .search_cutoff_ms(50)
            .build();

        let expected = SearchParameters {
            q: Some("*".to_string()),
            filter_by: Some("stock > 0".to_string()),
            use_cache: Some(true),
            drop_tokens_mode: Some(DropTokensMode::LeftToRight),
            search_cutoff_ms: Some(50),
            ..Default::default()
        };

        assert_eq!(built, expected);
    }

    #[test]
    fn test_search_parameters_builder_defaults() {
        let built = new_search_parameters().build();
        let expected = SearchParameters::default();
        assert_eq!(built, expected);
    }
}
