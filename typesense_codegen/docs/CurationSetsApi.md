# \CurationSetsApi

All URIs are relative to *http://localhost:8108*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_curation_set**](CurationSetsApi.md#delete_curation_set) | **DELETE** /curation_sets/{curationSetName} | Delete a curation set
[**delete_curation_set_item**](CurationSetsApi.md#delete_curation_set_item) | **DELETE** /curation_sets/{curationSetName}/items/{itemId} | Delete a curation set item
[**retrieve_curation_set**](CurationSetsApi.md#retrieve_curation_set) | **GET** /curation_sets/{curationSetName} | Retrieve a curation set
[**retrieve_curation_set_item**](CurationSetsApi.md#retrieve_curation_set_item) | **GET** /curation_sets/{curationSetName}/items/{itemId} | Retrieve a curation set item
[**retrieve_curation_set_items**](CurationSetsApi.md#retrieve_curation_set_items) | **GET** /curation_sets/{curationSetName}/items | List items in a curation set
[**retrieve_curation_sets**](CurationSetsApi.md#retrieve_curation_sets) | **GET** /curation_sets | List all curation sets
[**upsert_curation_set**](CurationSetsApi.md#upsert_curation_set) | **PUT** /curation_sets/{curationSetName} | Create or update a curation set
[**upsert_curation_set_item**](CurationSetsApi.md#upsert_curation_set_item) | **PUT** /curation_sets/{curationSetName}/items/{itemId} | Create or update a curation set item



## delete_curation_set

> models::CurationSetDeleteSchema delete_curation_set(curation_set_name)
Delete a curation set

Delete a specific curation set by its name

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**curation_set_name** | **String** | The name of the curation set to delete | [required] |

### Return type

[**models::CurationSetDeleteSchema**](CurationSetDeleteSchema.md)

### Authorization

[api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_curation_set_item

> models::CurationItemDeleteSchema delete_curation_set_item(curation_set_name, item_id)
Delete a curation set item

Delete a specific curation item by its id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**curation_set_name** | **String** | The name of the curation set | [required] |
**item_id** | **String** | The id of the curation item to delete | [required] |

### Return type

[**models::CurationItemDeleteSchema**](CurationItemDeleteSchema.md)

### Authorization

[api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_curation_set

> models::CurationSetCreateSchema retrieve_curation_set(curation_set_name)
Retrieve a curation set

Retrieve a specific curation set by its name

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**curation_set_name** | **String** | The name of the curation set to retrieve | [required] |

### Return type

[**models::CurationSetCreateSchema**](CurationSetCreateSchema.md)

### Authorization

[api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_curation_set_item

> models::CurationItemSchema retrieve_curation_set_item(curation_set_name, item_id)
Retrieve a curation set item

Retrieve a specific curation item by its id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**curation_set_name** | **String** | The name of the curation set | [required] |
**item_id** | **String** | The id of the curation item to retrieve | [required] |

### Return type

[**models::CurationItemSchema**](CurationItemSchema.md)

### Authorization

[api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_curation_set_items

> Vec<models::CurationItemSchema> retrieve_curation_set_items(curation_set_name)
List items in a curation set

Retrieve all curation items in a set

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**curation_set_name** | **String** | The name of the curation set to retrieve items for | [required] |

### Return type

[**Vec<models::CurationItemSchema>**](CurationItemSchema.md)

### Authorization

[api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_curation_sets

> Vec<models::CurationSetSchema> retrieve_curation_sets()
List all curation sets

Retrieve all curation sets

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::CurationSetSchema>**](CurationSetSchema.md)

### Authorization

[api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upsert_curation_set

> models::CurationSetSchema upsert_curation_set(curation_set_name, curation_set_create_schema)
Create or update a curation set

Create or update a curation set with the given name

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**curation_set_name** | **String** | The name of the curation set to create/update | [required] |
**curation_set_create_schema** | [**CurationSetCreateSchema**](CurationSetCreateSchema.md) | The curation set to be created/updated | [required] |

### Return type

[**models::CurationSetSchema**](CurationSetSchema.md)

### Authorization

[api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upsert_curation_set_item

> models::CurationItemSchema upsert_curation_set_item(curation_set_name, item_id, curation_item_create_schema)
Create or update a curation set item

Create or update a curation set item with the given id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**curation_set_name** | **String** | The name of the curation set | [required] |
**item_id** | **String** | The id of the curation item to upsert | [required] |
**curation_item_create_schema** | [**CurationItemCreateSchema**](CurationItemCreateSchema.md) | The curation item to be created/updated | [required] |

### Return type

[**models::CurationItemSchema**](CurationItemSchema.md)

### Authorization

[api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

