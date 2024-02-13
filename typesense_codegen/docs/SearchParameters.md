# SearchParameters

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**q** | **String** | The query text to search for in the collection. Use * as the search string to return all documents. This is typically useful when used in conjunction with filter_by. | 
**query_by** | **String** | A list of `string` fields that should be queried against. Multiple fields are separated with a comma. | 
**query_by_weights** | Option<**String**> | The relative weight to give each `query_by` field when ranking results. This can be used to boost fields in priority, when looking for matches. Multiple fields are separated with a comma. | [optional]
**text_match_type** | Option<**String**> | In a multi-field matching context, this parameter determines how the representative text match score of a record is calculated. Possible values are max_score (default) or max_weight. | [optional]
**prefix** | Option<**String**> | Boolean field to indicate that the last word in the query should be treated as a prefix, and not as a whole word. This is used for building autocomplete and instant search interfaces. Defaults to true. | [optional]
**infix** | Option<**String**> | If infix index is enabled for this field, infix searching can be done on a per-field basis by sending a comma separated string parameter called infix to the search query. This parameter can have 3 values; `off` infix search is disabled, which is default `always` infix search is performed along with regular search `fallback` infix search is performed if regular search does not produce results | [optional]
**max_extra_prefix** | Option<**i32**> | There are also 2 parameters that allow you to control the extent of infix searching max_extra_prefix and max_extra_suffix which specify the maximum number of symbols before or after the query that can be present in the token. For example query \"K2100\" has 2 extra symbols in \"6PK2100\". By default, any number of prefixes/suffixes can be present for a match. | [optional]
**max_extra_suffix** | Option<**i32**> | There are also 2 parameters that allow you to control the extent of infix searching max_extra_prefix and max_extra_suffix which specify the maximum number of symbols before or after the query that can be present in the token. For example query \"K2100\" has 2 extra symbols in \"6PK2100\". By default, any number of prefixes/suffixes can be present for a match. | [optional]
**filter_by** | Option<**String**> | Filter conditions for refining youropen api validator search results. Separate multiple conditions with &&. | [optional]
**sort_by** | Option<**String**> | A list of numerical fields and their corresponding sort orders that will be used for ordering your results. Up to 3 sort fields can be specified. The text similarity score is exposed as a special `_text_match` field that you can use in the list of sorting fields. If no `sort_by` parameter is specified, results are sorted by `_text_match:desc,default_sorting_field:desc` | [optional]
**facet_by** | Option<**String**> | A list of fields that will be used for faceting your results on. Separate multiple fields with a comma. | [optional]
**max_facet_values** | Option<**i32**> | Maximum number of facet values to be returned. | [optional]
**facet_query** | Option<**String**> | Facet values that are returned can now be filtered via this parameter. The matching facet text is also highlighted. For example, when faceting by `category`, you can set `facet_query=category:shoe` to return only facet values that contain the prefix \"shoe\". | [optional]
**num_typos** | Option<**String**> | The number of typographical errors (1 or 2) that would be tolerated. Default: 2  | [optional]
**page** | Option<**i32**> | Results from this specific page number would be fetched. | [optional]
**per_page** | Option<**i32**> | Number of results to fetch per page. Default: 10 | [optional]
**limit** | Option<**i32**> | Number of hits to fetch. Can be used as an alternative to the per_page parameter.  Default: 10.  | [optional]
**offset** | Option<**i32**> | Identifies the starting point to return hits from a result set. Can be used as an alternative to the page parameter. | [optional]
**group_by** | Option<**String**> | You can aggregate search results into groups or buckets by specify one or more `group_by` fields. Separate multiple fields with a comma. To group on a particular field, it must be a faceted field. | [optional]
**group_limit** | Option<**i32**> | Maximum number of hits to be returned for every group. If the `group_limit` is set as `K` then only the top K hits in each group are returned in the response. Default: 3  | [optional]
**include_fields** | Option<**String**> | List of fields from the document to include in the search result | [optional]
**exclude_fields** | Option<**String**> | List of fields from the document to exclude in the search result | [optional]
**highlight_full_fields** | Option<**String**> | List of fields which should be highlighted fully without snippeting | [optional]
**highlight_affix_num_tokens** | Option<**i32**> | The number of tokens that should surround the highlighted text on each side. Default: 4  | [optional]
**highlight_start_tag** | Option<**String**> | The start tag used for the highlighted snippets. Default: `<mark>`  | [optional]
**highlight_end_tag** | Option<**String**> | The end tag used for the highlighted snippets. Default: `</mark>`  | [optional]
**enable_highlight_v1** | Option<**bool**> | Flag for enabling/disabling the deprecated, old highlight structure in the response. Default: true  | [optional][default to true]
**snippet_threshold** | Option<**i32**> | Field values under this length will be fully highlighted, instead of showing a snippet of relevant portion. Default: 30  | [optional]
**drop_tokens_threshold** | Option<**i32**> | If the number of results found for a specific query is less than this number, Typesense will attempt to drop the tokens in the query until enough results are found. Tokens that have the least individual hits are dropped first. Set to 0 to disable. Default: 10  | [optional]
**typo_tokens_threshold** | Option<**i32**> | If the number of results found for a specific query is less than this number, Typesense will attempt to look for tokens with more typos until enough results are found. Default: 100  | [optional]
**pinned_hits** | Option<**String**> | A list of records to unconditionally include in the search results at specific positions. An example use case would be to feature or promote certain items on the top of search results. A list of `record_id:hit_position`. Eg: to include a record with ID 123 at Position 1 and another record with ID 456 at Position 5, you'd specify `123:1,456:5`. You could also use the Overrides feature to override search results based on rules. Overrides are applied first, followed by `pinned_hits` and  finally `hidden_hits`.  | [optional]
**hidden_hits** | Option<**String**> | A list of records to unconditionally hide from search results. A list of `record_id`s to hide. Eg: to hide records with IDs 123 and 456, you'd specify `123,456`. You could also use the Overrides feature to override search results based on rules. Overrides are applied first, followed by `pinned_hits` and finally `hidden_hits`.  | [optional]
**highlight_fields** | Option<**String**> | A list of custom fields that must be highlighted even if you don't query  for them  | [optional]
**split_join_tokens** | Option<**String**> | Treat space as typo: search for q=basket ball if q=basketball is not found or vice-versa. Splitting/joining of tokens will only be attempted if the original query produces no results. To always trigger this behavior, set value to `always``. To disable, set value to `off`. Default is `fallback`.  | [optional]
**pre_segmented_query** | Option<**bool**> | You can index content from any logographic language into Typesense if you are able to segment / split the text into space-separated words yourself  before indexing and querying. Set this parameter to true to do the same  | [optional]
**preset** | Option<**String**> | Search using a bunch of search parameters by setting this parameter to the name of the existing Preset.  | [optional]
**enable_overrides** | Option<**bool**> | If you have some overrides defined but want to disable all of them during query time, you can do that by setting this parameter to false  | [optional]
**prioritize_exact_match** | Option<**bool**> | Set this parameter to true to ensure that an exact match is ranked above the others  | [optional]
**max_candidates** | Option<**i32**> | Control the number of words that Typesense considers for typo and prefix searching.  | [optional]
**prioritize_token_position** | Option<**bool**> | Make Typesense prioritize documents where the query words appear earlier in the text.  | [optional]
**exhaustive_search** | Option<**bool**> | Setting this to true will make Typesense consider all prefixes and typo  corrections of the words in the query without stopping early when enough results are found  (drop_tokens_threshold and typo_tokens_threshold configurations are ignored).  | [optional]
**search_cutoff_ms** | Option<**i32**> | Typesense will attempt to return results early if the cutoff time has elapsed.  This is not a strict guarantee and facet computation is not bound by this parameter.  | [optional]
**use_cache** | Option<**bool**> | Enable server side caching of search query results. By default, caching is disabled.  | [optional]
**cache_ttl** | Option<**i32**> | The duration (in seconds) that determines how long the search query is cached.  This value can be set on a per-query basis. Default: 60.  | [optional]
**min_len_1typo** | Option<**i32**> | Minimum word length for 1-typo correction to be applied.  The value of num_typos is still treated as the maximum allowed typos.  | [optional]
**min_len_2typo** | Option<**i32**> | Minimum word length for 2-typo correction to be applied.  The value of num_typos is still treated as the maximum allowed typos.  | [optional]
**vector_query** | Option<**String**> | Vector query expression for fetching documents \"closest\" to a given query/document vector.  | [optional]
**remote_embedding_timeout_ms** | Option<**i32**> | Timeout (in milliseconds) for fetching remote embeddings.  | [optional]
**remote_embedding_num_tries** | Option<**i32**> | Number of times to retry fetching remote embeddings.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


