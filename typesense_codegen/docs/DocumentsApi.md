# \DocumentsApi

All URIs are relative to *http://localhost:8108*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_document**](DocumentsApi.md#delete_document) | **DELETE** /collections/{collectionName}/documents/{documentId} | Delete a document
[**delete_documents**](DocumentsApi.md#delete_documents) | **DELETE** /collections/{collectionName}/documents | Delete a bunch of documents
[**delete_search_override**](DocumentsApi.md#delete_search_override) | **DELETE** /collections/{collectionName}/overrides/{overrideId} | Delete an override associated with a collection
[**export_documents**](DocumentsApi.md#export_documents) | **GET** /collections/{collectionName}/documents/export | Export all documents in a collection
[**get_document**](DocumentsApi.md#get_document) | **GET** /collections/{collectionName}/documents/{documentId} | Retrieve a document
[**get_search_override**](DocumentsApi.md#get_search_override) | **GET** /collections/{collectionName}/overrides/{overrideId} | Retrieve a single search override
[**get_search_overrides**](DocumentsApi.md#get_search_overrides) | **GET** /collections/{collectionName}/overrides | List all collection overrides
[**import_documents**](DocumentsApi.md#import_documents) | **POST** /collections/{collectionName}/documents/import | Import documents into a collection
[**index_document**](DocumentsApi.md#index_document) | **POST** /collections/{collectionName}/documents | Index a document
[**multi_search**](DocumentsApi.md#multi_search) | **POST** /multi_search | send multiple search requests in a single HTTP request
[**search_collection**](DocumentsApi.md#search_collection) | **GET** /collections/{collectionName}/documents/search | Search for documents in a collection
[**update_document**](DocumentsApi.md#update_document) | **PATCH** /collections/{collectionName}/documents/{documentId} | Update a document
[**update_documents**](DocumentsApi.md#update_documents) | **PATCH** /collections/{collectionName}/documents | Update documents with conditional query
[**upsert_search_override**](DocumentsApi.md#upsert_search_override) | **PUT** /collections/{collectionName}/overrides/{overrideId} | Create or update an override to promote certain documents over others



## delete_document

> serde_json::Value delete_document(collection_name, document_id)
Delete a document

Delete an individual document from a collection by using its ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_name** | **String** | The name of the collection to search for the document under | [required] |
**document_id** | **String** | The Document ID | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_documents

> models::DeleteDocuments200Response delete_documents(collection_name, filter_by, batch_size, ignore_not_found, truncate)
Delete a bunch of documents

Delete a bunch of documents that match a specific filter condition. Use the `batch_size` parameter to control the number of documents that should deleted at a time. A larger value will speed up deletions, but will impact performance of other operations running on the server.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_name** | **String** | The name of the collection to delete documents from | [required] |
**filter_by** | Option<**String**> |  |  |
**batch_size** | Option<**i32**> |  |  |
**ignore_not_found** | Option<**bool**> |  |  |
**truncate** | Option<**bool**> |  |  |

### Return type

[**models::DeleteDocuments200Response**](deleteDocuments_200_response.md)

### Authorization

[api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_search_override

> models::SearchOverrideDeleteResponse delete_search_override(collection_name, override_id)
Delete an override associated with a collection

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_name** | **String** | The name of the collection | [required] |
**override_id** | **String** | The ID of the search override to delete | [required] |

### Return type

[**models::SearchOverrideDeleteResponse**](SearchOverrideDeleteResponse.md)

### Authorization

[api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## export_documents

> String export_documents(collection_name, filter_by, include_fields, exclude_fields)
Export all documents in a collection

Export all documents in a collection in JSON lines format.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_name** | **String** | The name of the collection | [required] |
**filter_by** | Option<**String**> |  |  |
**include_fields** | Option<**String**> |  |  |
**exclude_fields** | Option<**String**> |  |  |

### Return type

**String**

### Authorization

[api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octet-stream, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_document

> serde_json::Value get_document(collection_name, document_id)
Retrieve a document

Fetch an individual document from a collection by using its ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_name** | **String** | The name of the collection to search for the document under | [required] |
**document_id** | **String** | The Document ID | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_search_override

> models::SearchOverride get_search_override(collection_name, override_id)
Retrieve a single search override

Retrieve the details of a search override, given its id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_name** | **String** | The name of the collection | [required] |
**override_id** | **String** | The id of the search override | [required] |

### Return type

[**models::SearchOverride**](SearchOverride.md)

### Authorization

[api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_search_overrides

> models::SearchOverridesResponse get_search_overrides(collection_name)
List all collection overrides

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_name** | **String** | The name of the collection | [required] |

### Return type

[**models::SearchOverridesResponse**](SearchOverridesResponse.md)

### Authorization

[api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## import_documents

> String import_documents(collection_name, body, batch_size, return_id, remote_embedding_batch_size, return_doc, action, dirty_values)
Import documents into a collection

The documents to be imported must be formatted in a newline delimited JSON structure. You can feed the output file from a Typesense export operation directly as import.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_name** | **String** | The name of the collection | [required] |
**body** | **String** | The json array of documents or the JSONL file to import | [required] |
**batch_size** | Option<**i32**> |  |  |
**return_id** | Option<**bool**> |  |  |
**remote_embedding_batch_size** | Option<**i32**> |  |  |
**return_doc** | Option<**bool**> |  |  |
**action** | Option<[**IndexAction**](.md)> |  |  |
**dirty_values** | Option<[**DirtyValues**](.md)> |  |  |

### Return type

**String**

### Authorization

[api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: application/octet-stream
- **Accept**: application/octet-stream, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## index_document

> serde_json::Value index_document(collection_name, body, action, dirty_values)
Index a document

A document to be indexed in a given collection must conform to the schema of the collection.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_name** | **String** | The name of the collection to add the document to | [required] |
**body** | **serde_json::Value** | The document object to be indexed | [required] |
**action** | Option<**IndexAction**> | Additional action to perform |  |
**dirty_values** | Option<[**DirtyValues**](.md)> | Dealing with Dirty Data |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## multi_search

> models::MultiSearchResult multi_search(q, query_by, query_by_weights, text_match_type, prefix, infix, max_extra_prefix, max_extra_suffix, filter_by, sort_by, facet_by, max_facet_values, facet_query, num_typos, page, per_page, limit, offset, group_by, group_limit, group_missing_values, include_fields, exclude_fields, highlight_full_fields, highlight_affix_num_tokens, highlight_start_tag, highlight_end_tag, snippet_threshold, drop_tokens_threshold, drop_tokens_mode, typo_tokens_threshold, enable_typos_for_alpha_numerical_tokens, filter_curated_hits, enable_synonyms, enable_analytics, synonym_prefix, synonym_num_typos, pinned_hits, hidden_hits, override_tags, highlight_fields, pre_segmented_query, preset, enable_overrides, prioritize_exact_match, prioritize_token_position, prioritize_num_matching_fields, enable_typos_for_numerical_tokens, exhaustive_search, search_cutoff_ms, use_cache, cache_ttl, min_len_1typo, min_len_2typo, vector_query, remote_embedding_timeout_ms, remote_embedding_num_tries, facet_strategy, stopwords, facet_return_parent, voice_query, conversation, conversation_model_id, conversation_id, multi_search_searches_parameter)
send multiple search requests in a single HTTP request

This is especially useful to avoid round-trip network latencies incurred otherwise if each of these requests are sent in separate HTTP requests. You can also use this feature to do a federated search across multiple collections in a single HTTP request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**q** | Option<**String**> |  |  |
**query_by** | Option<**String**> |  |  |
**query_by_weights** | Option<**String**> |  |  |
**text_match_type** | Option<**String**> |  |  |
**prefix** | Option<**String**> |  |  |
**infix** | Option<**String**> |  |  |
**max_extra_prefix** | Option<**i32**> |  |  |
**max_extra_suffix** | Option<**i32**> |  |  |
**filter_by** | Option<**String**> |  |  |
**sort_by** | Option<**String**> |  |  |
**facet_by** | Option<**String**> |  |  |
**max_facet_values** | Option<**i32**> |  |  |
**facet_query** | Option<**String**> |  |  |
**num_typos** | Option<**String**> |  |  |
**page** | Option<**i32**> |  |  |
**per_page** | Option<**i32**> |  |  |
**limit** | Option<**i32**> |  |  |
**offset** | Option<**i32**> |  |  |
**group_by** | Option<**String**> |  |  |
**group_limit** | Option<**i32**> |  |  |
**group_missing_values** | Option<**bool**> |  |  |
**include_fields** | Option<**String**> |  |  |
**exclude_fields** | Option<**String**> |  |  |
**highlight_full_fields** | Option<**String**> |  |  |
**highlight_affix_num_tokens** | Option<**i32**> |  |  |
**highlight_start_tag** | Option<**String**> |  |  |
**highlight_end_tag** | Option<**String**> |  |  |
**snippet_threshold** | Option<**i32**> |  |  |
**drop_tokens_threshold** | Option<**i32**> |  |  |
**drop_tokens_mode** | Option<[**DropTokensMode**](.md)> |  |  |
**typo_tokens_threshold** | Option<**i32**> |  |  |
**enable_typos_for_alpha_numerical_tokens** | Option<**bool**> |  |  |
**filter_curated_hits** | Option<**bool**> |  |  |
**enable_synonyms** | Option<**bool**> |  |  |
**enable_analytics** | Option<**bool**> |  |  |[default to true]
**synonym_prefix** | Option<**bool**> |  |  |
**synonym_num_typos** | Option<**i32**> |  |  |
**pinned_hits** | Option<**String**> |  |  |
**hidden_hits** | Option<**String**> |  |  |
**override_tags** | Option<**String**> |  |  |
**highlight_fields** | Option<**String**> |  |  |
**pre_segmented_query** | Option<**bool**> |  |  |[default to false]
**preset** | Option<**String**> |  |  |
**enable_overrides** | Option<**bool**> |  |  |[default to false]
**prioritize_exact_match** | Option<**bool**> |  |  |[default to true]
**prioritize_token_position** | Option<**bool**> |  |  |[default to false]
**prioritize_num_matching_fields** | Option<**bool**> |  |  |[default to true]
**enable_typos_for_numerical_tokens** | Option<**bool**> |  |  |[default to true]
**exhaustive_search** | Option<**bool**> |  |  |
**search_cutoff_ms** | Option<**i32**> |  |  |
**use_cache** | Option<**bool**> |  |  |
**cache_ttl** | Option<**i32**> |  |  |
**min_len_1typo** | Option<**i32**> |  |  |
**min_len_2typo** | Option<**i32**> |  |  |
**vector_query** | Option<**String**> |  |  |
**remote_embedding_timeout_ms** | Option<**i32**> |  |  |
**remote_embedding_num_tries** | Option<**i32**> |  |  |
**facet_strategy** | Option<**String**> |  |  |
**stopwords** | Option<**String**> |  |  |
**facet_return_parent** | Option<**String**> |  |  |
**voice_query** | Option<**String**> |  |  |
**conversation** | Option<**bool**> |  |  |
**conversation_model_id** | Option<**String**> |  |  |
**conversation_id** | Option<**String**> |  |  |
**multi_search_searches_parameter** | Option<[**MultiSearchSearchesParameter**](MultiSearchSearchesParameter.md)> |  |  |

### Return type

[**models::MultiSearchResult**](MultiSearchResult.md)

### Authorization

[api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_collection

> models::SearchResult search_collection(collection_name, q, query_by, nl_query, nl_model_id, query_by_weights, text_match_type, prefix, infix, max_extra_prefix, max_extra_suffix, filter_by, max_filter_by_candidates, sort_by, facet_by, max_facet_values, facet_query, num_typos, page, per_page, limit, offset, group_by, group_limit, group_missing_values, include_fields, exclude_fields, highlight_full_fields, highlight_affix_num_tokens, highlight_start_tag, highlight_end_tag, enable_highlight_v1, enable_analytics, snippet_threshold, synonym_sets, drop_tokens_threshold, drop_tokens_mode, typo_tokens_threshold, enable_typos_for_alpha_numerical_tokens, filter_curated_hits, enable_synonyms, synonym_prefix, synonym_num_typos, pinned_hits, hidden_hits, override_tags, highlight_fields, split_join_tokens, pre_segmented_query, preset, enable_overrides, prioritize_exact_match, max_candidates, prioritize_token_position, prioritize_num_matching_fields, enable_typos_for_numerical_tokens, exhaustive_search, search_cutoff_ms, use_cache, cache_ttl, min_len_1typo, min_len_2typo, vector_query, remote_embedding_timeout_ms, remote_embedding_num_tries, facet_strategy, stopwords, facet_return_parent, voice_query, conversation, conversation_model_id, conversation_id)
Search for documents in a collection

Search for documents in a collection that match the search criteria.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_name** | **String** | The name of the collection to search for the document under | [required] |
**q** | Option<**String**> |  |  |
**query_by** | Option<**String**> |  |  |
**nl_query** | Option<**bool**> |  |  |
**nl_model_id** | Option<**String**> |  |  |
**query_by_weights** | Option<**String**> |  |  |
**text_match_type** | Option<**String**> |  |  |
**prefix** | Option<**String**> |  |  |
**infix** | Option<**String**> |  |  |
**max_extra_prefix** | Option<**i32**> |  |  |
**max_extra_suffix** | Option<**i32**> |  |  |
**filter_by** | Option<**String**> |  |  |
**max_filter_by_candidates** | Option<**i32**> |  |  |
**sort_by** | Option<**String**> |  |  |
**facet_by** | Option<**String**> |  |  |
**max_facet_values** | Option<**i32**> |  |  |
**facet_query** | Option<**String**> |  |  |
**num_typos** | Option<**String**> |  |  |
**page** | Option<**i32**> |  |  |
**per_page** | Option<**i32**> |  |  |
**limit** | Option<**i32**> |  |  |
**offset** | Option<**i32**> |  |  |
**group_by** | Option<**String**> |  |  |
**group_limit** | Option<**i32**> |  |  |
**group_missing_values** | Option<**bool**> |  |  |
**include_fields** | Option<**String**> |  |  |
**exclude_fields** | Option<**String**> |  |  |
**highlight_full_fields** | Option<**String**> |  |  |
**highlight_affix_num_tokens** | Option<**i32**> |  |  |
**highlight_start_tag** | Option<**String**> |  |  |
**highlight_end_tag** | Option<**String**> |  |  |
**enable_highlight_v1** | Option<**bool**> |  |  |[default to true]
**enable_analytics** | Option<**bool**> |  |  |[default to true]
**snippet_threshold** | Option<**i32**> |  |  |
**synonym_sets** | Option<**String**> |  |  |
**drop_tokens_threshold** | Option<**i32**> |  |  |
**drop_tokens_mode** | Option<[**DropTokensMode**](.md)> |  |  |
**typo_tokens_threshold** | Option<**i32**> |  |  |
**enable_typos_for_alpha_numerical_tokens** | Option<**bool**> |  |  |
**filter_curated_hits** | Option<**bool**> |  |  |
**enable_synonyms** | Option<**bool**> |  |  |
**synonym_prefix** | Option<**bool**> |  |  |
**synonym_num_typos** | Option<**i32**> |  |  |
**pinned_hits** | Option<**String**> |  |  |
**hidden_hits** | Option<**String**> |  |  |
**override_tags** | Option<**String**> |  |  |
**highlight_fields** | Option<**String**> |  |  |
**split_join_tokens** | Option<**String**> |  |  |
**pre_segmented_query** | Option<**bool**> |  |  |
**preset** | Option<**String**> |  |  |
**enable_overrides** | Option<**bool**> |  |  |[default to false]
**prioritize_exact_match** | Option<**bool**> |  |  |[default to true]
**max_candidates** | Option<**i32**> |  |  |
**prioritize_token_position** | Option<**bool**> |  |  |[default to false]
**prioritize_num_matching_fields** | Option<**bool**> |  |  |[default to true]
**enable_typos_for_numerical_tokens** | Option<**bool**> |  |  |[default to true]
**exhaustive_search** | Option<**bool**> |  |  |
**search_cutoff_ms** | Option<**i32**> |  |  |
**use_cache** | Option<**bool**> |  |  |
**cache_ttl** | Option<**i32**> |  |  |
**min_len_1typo** | Option<**i32**> |  |  |
**min_len_2typo** | Option<**i32**> |  |  |
**vector_query** | Option<**String**> |  |  |
**remote_embedding_timeout_ms** | Option<**i32**> |  |  |
**remote_embedding_num_tries** | Option<**i32**> |  |  |
**facet_strategy** | Option<**String**> |  |  |
**stopwords** | Option<**String**> |  |  |
**facet_return_parent** | Option<**String**> |  |  |
**voice_query** | Option<**String**> |  |  |
**conversation** | Option<**bool**> |  |  |
**conversation_model_id** | Option<**String**> |  |  |
**conversation_id** | Option<**String**> |  |  |

### Return type

[**models::SearchResult**](SearchResult.md)

### Authorization

[api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_document

> serde_json::Value update_document(collection_name, document_id, body, dirty_values)
Update a document

Update an individual document from a collection by using its ID. The update can be partial.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_name** | **String** | The name of the collection to search for the document under | [required] |
**document_id** | **String** | The Document ID | [required] |
**body** | **serde_json::Value** | The document object with fields to be updated | [required] |
**dirty_values** | Option<[**DirtyValues**](.md)> | Dealing with Dirty Data |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_documents

> models::UpdateDocuments200Response update_documents(collection_name, body, filter_by)
Update documents with conditional query

The filter_by query parameter is used to filter to specify a condition against which the documents are matched. The request body contains the fields that should be updated for any documents that match the filter condition. This endpoint is only available if the Typesense server is version `0.25.0.rc12` or later.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_name** | **String** | The name of the collection to update documents in | [required] |
**body** | **serde_json::Value** | The document fields to be updated | [required] |
**filter_by** | Option<**String**> |  |  |

### Return type

[**models::UpdateDocuments200Response**](updateDocuments_200_response.md)

### Authorization

[api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upsert_search_override

> models::SearchOverride upsert_search_override(collection_name, override_id, search_override_schema)
Create or update an override to promote certain documents over others

Create or update an override to promote certain documents over others. Using overrides, you can include or exclude specific documents for a given query.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_name** | **String** | The name of the collection | [required] |
**override_id** | **String** | The ID of the search override to create/update | [required] |
**search_override_schema** | [**SearchOverrideSchema**](SearchOverrideSchema.md) | The search override object to be created/updated | [required] |

### Return type

[**models::SearchOverride**](SearchOverride.md)

### Authorization

[api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

