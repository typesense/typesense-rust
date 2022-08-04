# \DocumentsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_document**](DocumentsApi.md#delete_document) | **DELETE** /collections/{collectionName}/documents/{documentId} | Delete a document
[**delete_documents**](DocumentsApi.md#delete_documents) | **DELETE** /collections/{collectionName}/documents | Delete a bunch of documents
[**delete_search_override**](DocumentsApi.md#delete_search_override) | **DELETE** /collections/{collectionName}/overrides/{overrideId} | Delete an override associated with a collection
[**delete_search_synonym**](DocumentsApi.md#delete_search_synonym) | **DELETE** /collections/{collectionName}/synonyms/{synonymId} | Delete a synonym associated with a collection
[**export_documents**](DocumentsApi.md#export_documents) | **GET** /collections/{collectionName}/documents/export | Export all documents in a collection
[**get_document**](DocumentsApi.md#get_document) | **GET** /collections/{collectionName}/documents/{documentId} | Retreive a document
[**get_search_override**](DocumentsApi.md#get_search_override) | **GET** /collections/{collectionName}/overrides/{overrideId} | Retrieve a single search override
[**get_search_overrides**](DocumentsApi.md#get_search_overrides) | **GET** /collections/{collectionName}/overrides | List all collection overrides
[**get_search_synonym**](DocumentsApi.md#get_search_synonym) | **GET** /collections/{collectionName}/synonyms/{synonymId} | Retrieve a single search synonym
[**get_search_synonyms**](DocumentsApi.md#get_search_synonyms) | **GET** /collections/{collectionName}/synonyms | List all collection synonyms
[**import_documents**](DocumentsApi.md#import_documents) | **POST** /collections/{collectionName}/documents/import | Import documents into a collection
[**index_document**](DocumentsApi.md#index_document) | **POST** /collections/{collectionName}/documents | Index a document
[**multi_search**](DocumentsApi.md#multi_search) | **POST** /multi_search | send multiple search requests in a single HTTP request
[**search_collection**](DocumentsApi.md#search_collection) | **GET** /collections/{collectionName}/documents/search | Search for documents in a collection
[**update_document**](DocumentsApi.md#update_document) | **PATCH** /collections/{collectionName}/documents/{documentId} | Update a document
[**upsert_search_override**](DocumentsApi.md#upsert_search_override) | **PUT** /collections/{collectionName}/overrides/{overrideId} | Create or update an override to promote certain documents over others
[**upsert_search_synonym**](DocumentsApi.md#upsert_search_synonym) | **PUT** /collections/{collectionName}/synonyms/{synonymId} | Create or update a synonym



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

> crate::models::DeleteDocuments200Response delete_documents(collection_name, delete_documents_parameters)
Delete a bunch of documents

Delete a bunch of documents that match a specific filter condition. Use the `batch_size` parameter to control the number of documents that should deleted at a time. A larger value will speed up deletions, but will impact performance of other operations running on the server.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_name** | **String** | The name of the collection to delete documents from | [required] |
**delete_documents_parameters** | Option<[**DeleteDocumentsDeleteDocumentsParametersParameter**](.md)> |  |  |

### Return type

[**crate::models::DeleteDocuments200Response**](deleteDocuments_200_response.md)

### Authorization

[api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_search_override

> crate::models::SearchOverride delete_search_override(collection_name, override_id)
Delete an override associated with a collection

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_name** | **String** | The name of the collection | [required] |
**override_id** | **String** | The ID of the search override to delete | [required] |

### Return type

[**crate::models::SearchOverride**](SearchOverride.md)

### Authorization

[api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_search_synonym

> crate::models::SearchSynonym delete_search_synonym(collection_name, synonym_id)
Delete a synonym associated with a collection

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_name** | **String** | The name of the collection | [required] |
**synonym_id** | **String** | The ID of the search synonym to delete | [required] |

### Return type

[**crate::models::SearchSynonym**](SearchSynonym.md)

### Authorization

[api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## export_documents

> String export_documents(collection_name, export_documents_parameters)
Export all documents in a collection

Export all documents in a collection in JSON lines format.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_name** | **String** | The name of the collection | [required] |
**export_documents_parameters** | Option<[**ExportDocumentsExportDocumentsParametersParameter**](.md)> |  |  |

### Return type

**String**

### Authorization

[api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octet-stream

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_document

> serde_json::Value get_document(collection_name, document_id)
Retreive a document

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

> crate::models::SearchOverride get_search_override(collection_name, override_id)
Retrieve a single search override

Retrieve the details of a search override, given its id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_name** | **String** | The name of the collection | [required] |
**override_id** | **String** | The id of the search override | [required] |

### Return type

[**crate::models::SearchOverride**](SearchOverride.md)

### Authorization

[api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_search_overrides

> crate::models::SearchOverridesResponse get_search_overrides(collection_name)
List all collection overrides

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_name** | **String** | The name of the collection | [required] |

### Return type

[**crate::models::SearchOverridesResponse**](SearchOverridesResponse.md)

### Authorization

[api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_search_synonym

> crate::models::SearchSynonym get_search_synonym(collection_name, synonym_id)
Retrieve a single search synonym

Retrieve the details of a search synonym, given its id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_name** | **String** | The name of the collection | [required] |
**synonym_id** | **String** | The id of the search synonym | [required] |

### Return type

[**crate::models::SearchSynonym**](SearchSynonym.md)

### Authorization

[api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_search_synonyms

> crate::models::SearchSynonymsResponse get_search_synonyms(collection_name)
List all collection synonyms

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_name** | **String** | The name of the collection | [required] |

### Return type

[**crate::models::SearchSynonymsResponse**](SearchSynonymsResponse.md)

### Authorization

[api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## import_documents

> String import_documents(collection_name, body, import_documents_parameters)
Import documents into a collection

The documents to be imported must be formatted in a newline delimited JSON structure. You can feed the output file from a Typesense export operation directly as import.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_name** | **String** | The name of the collection | [required] |
**body** | **String** | The json array of documents or the JSONL file to import | [required] |
**import_documents_parameters** | Option<[**ImportDocumentsImportDocumentsParametersParameter**](.md)> |  |  |

### Return type

**String**

### Authorization

[api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: application/octet-stream
- **Accept**: application/octet-stream, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## index_document

> serde_json::Value index_document(collection_name, body, action)
Index a document

A document to be indexed in a given collection must conform to the schema of the collection.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_name** | **String** | The name of the collection to add the document to | [required] |
**body** | **serde_json::Value** | The document object to be indexed | [required] |
**action** | Option<**String**> | Additional action to perform |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## multi_search

> crate::models::MultiSearchResult multi_search(multi_search_parameters, multi_search_searches_parameter)
send multiple search requests in a single HTTP request

This is especially useful to avoid round-trip network latencies incurred otherwise if each of these requests are sent in separate HTTP requests. You can also use this feature to do a federated search across multiple collections in a single HTTP request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**multi_search_parameters** | [**MultiSearchParameters**](.md) |  | [required] |
**multi_search_searches_parameter** | Option<[**MultiSearchSearchesParameter**](MultiSearchSearchesParameter.md)> |  |  |

### Return type

[**crate::models::MultiSearchResult**](MultiSearchResult.md)

### Authorization

[api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_collection

> crate::models::SearchResult search_collection(collection_name, search_parameters)
Search for documents in a collection

Search for documents in a collection that match the search criteria.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_name** | **String** | The name of the collection to search for the document under | [required] |
**search_parameters** | [**SearchParameters**](.md) |  | [required] |

### Return type

[**crate::models::SearchResult**](SearchResult.md)

### Authorization

[api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_document

> serde_json::Value update_document(collection_name, document_id, body)
Update a document

Update an individual document from a collection by using its ID. The update can be partial.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_name** | **String** | The name of the collection to search for the document under | [required] |
**document_id** | **String** | The Document ID | [required] |
**body** | **serde_json::Value** | The document object with fields to be updated | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upsert_search_override

> crate::models::SearchOverride upsert_search_override(collection_name, override_id, search_override_schema)
Create or update an override to promote certain documents over others

Create or update an override to promote certain documents over others. Using overrides, you can include or exclude specific documents for a given query.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_name** | **String** | The name of the collection | [required] |
**override_id** | **String** | The ID of the search override to create/update | [required] |
**search_override_schema** | [**SearchOverrideSchema**](SearchOverrideSchema.md) | The search override object to be created/updated | [required] |

### Return type

[**crate::models::SearchOverride**](SearchOverride.md)

### Authorization

[api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upsert_search_synonym

> crate::models::SearchSynonym upsert_search_synonym(collection_name, synonym_id, search_synonym_schema)
Create or update a synonym

Create or update a synonym  to define search terms that should be considered equivalent.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_name** | **String** | The name of the collection | [required] |
**synonym_id** | **String** | The ID of the search synonym to create/update | [required] |
**search_synonym_schema** | [**SearchSynonymSchema**](SearchSynonymSchema.md) | The search synonym object to be created/updated | [required] |

### Return type

[**crate::models::SearchSynonym**](SearchSynonym.md)

### Authorization

[api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

