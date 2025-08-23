# \StemmingApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_stemming_dictionary**](StemmingApi.md#get_stemming_dictionary) | **GET** /stemming/dictionaries/{dictionaryId} | Retrieve a stemming dictionary
[**import_stemming_dictionary**](StemmingApi.md#import_stemming_dictionary) | **POST** /stemming/dictionaries/import | Import a stemming dictionary
[**list_stemming_dictionaries**](StemmingApi.md#list_stemming_dictionaries) | **GET** /stemming/dictionaries | List all stemming dictionaries



## get_stemming_dictionary

> models::StemmingDictionary get_stemming_dictionary(dictionary_id)
Retrieve a stemming dictionary

Fetch details of a specific stemming dictionary.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dictionary_id** | **String** | The ID of the dictionary to retrieve | [required] |

### Return type

[**models::StemmingDictionary**](StemmingDictionary.md)

### Authorization

[api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## import_stemming_dictionary

> String import_stemming_dictionary(id, body)
Import a stemming dictionary

Upload a JSONL file containing word mappings to create or update a stemming dictionary.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID to assign to the dictionary | [required] |
**body** | **String** | The JSONL file containing word mappings | [required] |

### Return type

**String**

### Authorization

[api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/octet-stream, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_stemming_dictionaries

> models::ListStemmingDictionaries200Response list_stemming_dictionaries()
List all stemming dictionaries

Retrieve a list of all available stemming dictionaries.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ListStemmingDictionaries200Response**](listStemmingDictionaries_200_response.md)

### Authorization

[api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

