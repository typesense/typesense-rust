# \SynonymsApi

All URIs are relative to *http://localhost:8108*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_synonym_set**](SynonymsApi.md#delete_synonym_set) | **DELETE** /synonym_sets/{synonymSetName} | Delete a synonym set
[**retrieve_synonym_set**](SynonymsApi.md#retrieve_synonym_set) | **GET** /synonym_sets/{synonymSetName} | Retrieve a synonym set
[**retrieve_synonym_sets**](SynonymsApi.md#retrieve_synonym_sets) | **GET** /synonym_sets | List all synonym sets
[**upsert_synonym_set**](SynonymsApi.md#upsert_synonym_set) | **PUT** /synonym_sets/{synonymSetName} | Create or update a synonym set



## delete_synonym_set

> models::SynonymSetDeleteSchema delete_synonym_set(synonym_set_name)
Delete a synonym set

Delete a specific synonym set by its name

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**synonym_set_name** | **String** | The name of the synonym set to delete | [required] |

### Return type

[**models::SynonymSetDeleteSchema**](SynonymSetDeleteSchema.md)

### Authorization

[api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_synonym_set

> models::SynonymSetCreateSchema retrieve_synonym_set(synonym_set_name)
Retrieve a synonym set

Retrieve a specific synonym set by its name

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**synonym_set_name** | **String** | The name of the synonym set to retrieve | [required] |

### Return type

[**models::SynonymSetCreateSchema**](SynonymSetCreateSchema.md)

### Authorization

[api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_synonym_sets

> Vec<models::SynonymSetSchema> retrieve_synonym_sets()
List all synonym sets

Retrieve all synonym sets

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::SynonymSetSchema>**](SynonymSetSchema.md)

### Authorization

[api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upsert_synonym_set

> models::SynonymSetSchema upsert_synonym_set(synonym_set_name, synonym_set_create_schema)
Create or update a synonym set

Create or update a synonym set with the given name

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**synonym_set_name** | **String** | The name of the synonym set to create/update | [required] |
**synonym_set_create_schema** | [**SynonymSetCreateSchema**](SynonymSetCreateSchema.md) | The synonym set to be created/updated | [required] |

### Return type

[**models::SynonymSetSchema**](SynonymSetSchema.md)

### Authorization

[api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

