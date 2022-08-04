# \KeysApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_key**](KeysApi.md#create_key) | **POST** /keys | Create an API Key
[**delete_key**](KeysApi.md#delete_key) | **DELETE** /keys/{keyId} | Delete an API key given its ID.
[**get_key**](KeysApi.md#get_key) | **GET** /keys/{keyId} | Retrieve (metadata about) a key
[**get_keys**](KeysApi.md#get_keys) | **GET** /keys | Retrieve (metadata about) all keys.



## create_key

> crate::models::ApiKey create_key(api_key_schema)
Create an API Key

Create an API Key with fine-grain access control. You can restrict access on both a per-collection and per-action level. The generated key is returned only during creation. You want to store this key carefully in a secure place.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_key_schema** | Option<[**ApiKeySchema**](ApiKeySchema.md)> | The object that describes API key scope |  |

### Return type

[**crate::models::ApiKey**](ApiKey.md)

### Authorization

[api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_key

> crate::models::ApiKey delete_key(key_id)
Delete an API key given its ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key_id** | **i64** | The ID of the key to delete | [required] |

### Return type

[**crate::models::ApiKey**](ApiKey.md)

### Authorization

[api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_key

> crate::models::ApiKey get_key(key_id)
Retrieve (metadata about) a key

Retrieve (metadata about) a key. Only the key prefix is returned when you retrieve a key. Due to security reasons, only the create endpoint returns the full API key.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key_id** | **i64** | The ID of the key to retrieve | [required] |

### Return type

[**crate::models::ApiKey**](ApiKey.md)

### Authorization

[api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_keys

> crate::models::ApiKeysResponse get_keys()
Retrieve (metadata about) all keys.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ApiKeysResponse**](ApiKeysResponse.md)

### Authorization

[api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

