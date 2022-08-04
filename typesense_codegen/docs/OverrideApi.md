# \OverrideApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_search_override**](OverrideApi.md#get_search_override) | **GET** /collections/{collectionName}/overrides/{overrideId} | Retrieve a single search override



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

