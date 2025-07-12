# \SynonymsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_search_synonym**](SynonymsApi.md#delete_search_synonym) | **DELETE** /collections/{collectionName}/synonyms/{synonymId} | Delete a synonym associated with a collection
[**get_search_synonym**](SynonymsApi.md#get_search_synonym) | **GET** /collections/{collectionName}/synonyms/{synonymId} | Retrieve a single search synonym
[**get_search_synonyms**](SynonymsApi.md#get_search_synonyms) | **GET** /collections/{collectionName}/synonyms | List all collection synonyms
[**upsert_search_synonym**](SynonymsApi.md#upsert_search_synonym) | **PUT** /collections/{collectionName}/synonyms/{synonymId} | Create or update a synonym



## delete_search_synonym

> models::SearchSynonymDeleteResponse delete_search_synonym(collection_name, synonym_id)
Delete a synonym associated with a collection

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_name** | **String** | The name of the collection | [required] |
**synonym_id** | **String** | The ID of the search synonym to delete | [required] |

### Return type

[**models::SearchSynonymDeleteResponse**](SearchSynonymDeleteResponse.md)

### Authorization

[api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_search_synonym

> models::SearchSynonym get_search_synonym(collection_name, synonym_id)
Retrieve a single search synonym

Retrieve the details of a search synonym, given its id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_name** | **String** | The name of the collection | [required] |
**synonym_id** | **String** | The id of the search synonym | [required] |

### Return type

[**models::SearchSynonym**](SearchSynonym.md)

### Authorization

[api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_search_synonyms

> models::SearchSynonymsResponse get_search_synonyms(collection_name)
List all collection synonyms

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_name** | **String** | The name of the collection | [required] |

### Return type

[**models::SearchSynonymsResponse**](SearchSynonymsResponse.md)

### Authorization

[api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upsert_search_synonym

> models::SearchSynonym upsert_search_synonym(collection_name, synonym_id, search_synonym_schema)
Create or update a synonym

Create or update a synonym  to define search terms that should be considered equivalent.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_name** | **String** | The name of the collection | [required] |
**synonym_id** | **String** | The ID of the search synonym to create/update | [required] |
**search_synonym_schema** | [**SearchSynonymSchema**](SearchSynonymSchema.md) | The search synonym object to be created/updated | [required] |

### Return type

[**models::SearchSynonym**](SearchSynonym.md)

### Authorization

[api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

