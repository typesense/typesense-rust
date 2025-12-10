# \SynonymsApi

All URIs are relative to *http://localhost:8108*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_synonym_set**](SynonymsApi.md#delete_synonym_set) | **DELETE** /synonym_sets/{synonymSetName} | Delete a synonym set
[**delete_synonym_set_item**](SynonymsApi.md#delete_synonym_set_item) | **DELETE** /synonym_sets/{synonymSetName}/items/{itemId} | Delete a synonym set item
[**retrieve_synonym_set**](SynonymsApi.md#retrieve_synonym_set) | **GET** /synonym_sets/{synonymSetName} | Retrieve a synonym set
[**retrieve_synonym_set_item**](SynonymsApi.md#retrieve_synonym_set_item) | **GET** /synonym_sets/{synonymSetName}/items/{itemId} | Retrieve a synonym set item
[**retrieve_synonym_set_items**](SynonymsApi.md#retrieve_synonym_set_items) | **GET** /synonym_sets/{synonymSetName}/items | List items in a synonym set
[**retrieve_synonym_sets**](SynonymsApi.md#retrieve_synonym_sets) | **GET** /synonym_sets | List all synonym sets
[**upsert_synonym_set**](SynonymsApi.md#upsert_synonym_set) | **PUT** /synonym_sets/{synonymSetName} | Create or update a synonym set
[**upsert_synonym_set_item**](SynonymsApi.md#upsert_synonym_set_item) | **PUT** /synonym_sets/{synonymSetName}/items/{itemId} | Create or update a synonym set item



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


## delete_synonym_set_item

> models::SynonymItemDeleteSchema delete_synonym_set_item(synonym_set_name, item_id)
Delete a synonym set item

Delete a specific synonym item by its id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**synonym_set_name** | **String** | The name of the synonym set | [required] |
**item_id** | **String** | The id of the synonym item to delete | [required] |

### Return type

[**models::SynonymItemDeleteSchema**](SynonymItemDeleteSchema.md)

### Authorization

[api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_synonym_set

> models::SynonymSetSchema retrieve_synonym_set(synonym_set_name)
Retrieve a synonym set

Retrieve a specific synonym set by its name

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**synonym_set_name** | **String** | The name of the synonym set to retrieve | [required] |

### Return type

[**models::SynonymSetSchema**](SynonymSetSchema.md)

### Authorization

[api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_synonym_set_item

> models::SynonymItemSchema retrieve_synonym_set_item(synonym_set_name, item_id)
Retrieve a synonym set item

Retrieve a specific synonym item by its id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**synonym_set_name** | **String** | The name of the synonym set | [required] |
**item_id** | **String** | The id of the synonym item to retrieve | [required] |

### Return type

[**models::SynonymItemSchema**](SynonymItemSchema.md)

### Authorization

[api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_synonym_set_items

> Vec<models::SynonymItemSchema> retrieve_synonym_set_items(synonym_set_name)
List items in a synonym set

Retrieve all synonym items in a set

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**synonym_set_name** | **String** | The name of the synonym set to retrieve items for | [required] |

### Return type

[**Vec<models::SynonymItemSchema>**](SynonymItemSchema.md)

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


## upsert_synonym_set_item

> models::SynonymItemSchema upsert_synonym_set_item(synonym_set_name, item_id, synonym_item_upsert_schema)
Create or update a synonym set item

Create or update a synonym set item with the given id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**synonym_set_name** | **String** | The name of the synonym set | [required] |
**item_id** | **String** | The id of the synonym item to upsert | [required] |
**synonym_item_upsert_schema** | [**SynonymItemUpsertSchema**](SynonymItemUpsertSchema.md) | The synonym item to be created/updated | [required] |

### Return type

[**models::SynonymItemSchema**](SynonymItemSchema.md)

### Authorization

[api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

