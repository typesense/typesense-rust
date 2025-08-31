# \NlSearchModelsApi

All URIs are relative to *http://localhost:8108*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_nl_search_model**](NlSearchModelsApi.md#create_nl_search_model) | **POST** /nl_search_models | Create a NL search model
[**delete_nl_search_model**](NlSearchModelsApi.md#delete_nl_search_model) | **DELETE** /nl_search_models/{modelId} | Delete a NL search model
[**retrieve_all_nl_search_models**](NlSearchModelsApi.md#retrieve_all_nl_search_models) | **GET** /nl_search_models | List all NL search models
[**retrieve_nl_search_model**](NlSearchModelsApi.md#retrieve_nl_search_model) | **GET** /nl_search_models/{modelId} | Retrieve a NL search model
[**update_nl_search_model**](NlSearchModelsApi.md#update_nl_search_model) | **PUT** /nl_search_models/{modelId} | Update a NL search model



## create_nl_search_model

> models::NlSearchModelSchema create_nl_search_model(nl_search_model_create_schema)
Create a NL search model

Create a new NL search model.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**nl_search_model_create_schema** | [**NlSearchModelCreateSchema**](NlSearchModelCreateSchema.md) | The NL search model to be created | [required] |

### Return type

[**models::NlSearchModelSchema**](NLSearchModelSchema.md)

### Authorization

[api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_nl_search_model

> models::NlSearchModelDeleteSchema delete_nl_search_model(model_id)
Delete a NL search model

Delete a specific NL search model by its ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**model_id** | **String** | The ID of the NL search model to delete | [required] |

### Return type

[**models::NlSearchModelDeleteSchema**](NLSearchModelDeleteSchema.md)

### Authorization

[api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_all_nl_search_models

> Vec<models::NlSearchModelSchema> retrieve_all_nl_search_models()
List all NL search models

Retrieve all NL search models.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::NlSearchModelSchema>**](NLSearchModelSchema.md)

### Authorization

[api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_nl_search_model

> models::NlSearchModelSchema retrieve_nl_search_model(model_id)
Retrieve a NL search model

Retrieve a specific NL search model by its ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**model_id** | **String** | The ID of the NL search model to retrieve | [required] |

### Return type

[**models::NlSearchModelSchema**](NLSearchModelSchema.md)

### Authorization

[api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_nl_search_model

> models::NlSearchModelSchema update_nl_search_model(model_id, body)
Update a NL search model

Update an existing NL search model.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**model_id** | **String** | The ID of the NL search model to update | [required] |
**body** | **models::NlSearchModelCreateSchema** | The NL search model fields to update | [required] |

### Return type

[**models::NlSearchModelSchema**](NLSearchModelSchema.md)

### Authorization

[api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

