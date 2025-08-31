# \StopwordsApi

All URIs are relative to *http://localhost:8108*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_stopwords_set**](StopwordsApi.md#delete_stopwords_set) | **DELETE** /stopwords/{setId} | Delete a stopwords set.
[**retrieve_stopwords_set**](StopwordsApi.md#retrieve_stopwords_set) | **GET** /stopwords/{setId} | Retrieves a stopwords set.
[**retrieve_stopwords_sets**](StopwordsApi.md#retrieve_stopwords_sets) | **GET** /stopwords | Retrieves all stopwords sets.
[**upsert_stopwords_set**](StopwordsApi.md#upsert_stopwords_set) | **PUT** /stopwords/{setId} | Upserts a stopwords set.



## delete_stopwords_set

> models::DeleteStopwordsSet200Response delete_stopwords_set(set_id)
Delete a stopwords set.

Permanently deletes a stopwords set, given it's name.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**set_id** | **String** | The ID of the stopwords set to delete. | [required] |

### Return type

[**models::DeleteStopwordsSet200Response**](deleteStopwordsSet_200_response.md)

### Authorization

[api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_stopwords_set

> models::StopwordsSetRetrieveSchema retrieve_stopwords_set(set_id)
Retrieves a stopwords set.

Retrieve the details of a stopwords set, given it's name.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**set_id** | **String** | The ID of the stopwords set to retrieve. | [required] |

### Return type

[**models::StopwordsSetRetrieveSchema**](StopwordsSetRetrieveSchema.md)

### Authorization

[api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_stopwords_sets

> models::StopwordsSetsRetrieveAllSchema retrieve_stopwords_sets()
Retrieves all stopwords sets.

Retrieve the details of all stopwords sets

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::StopwordsSetsRetrieveAllSchema**](StopwordsSetsRetrieveAllSchema.md)

### Authorization

[api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upsert_stopwords_set

> models::StopwordsSetSchema upsert_stopwords_set(set_id, stopwords_set_upsert_schema)
Upserts a stopwords set.

When an analytics rule is created, we give it a name and describe the type, the source collections and the destination collection.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**set_id** | **String** | The ID of the stopwords set to upsert. | [required] |
**stopwords_set_upsert_schema** | [**StopwordsSetUpsertSchema**](StopwordsSetUpsertSchema.md) | The stopwords set to upsert. | [required] |

### Return type

[**models::StopwordsSetSchema**](StopwordsSetSchema.md)

### Authorization

[api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

