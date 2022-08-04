# \CollectionsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_collection**](CollectionsApi.md#create_collection) | **POST** /collections | Create a new collection
[**delete_alias**](CollectionsApi.md#delete_alias) | **DELETE** /aliases/{aliasName} | Delete an alias
[**delete_collection**](CollectionsApi.md#delete_collection) | **DELETE** /collections/{collectionName} | Delete a collection
[**get_alias**](CollectionsApi.md#get_alias) | **GET** /aliases/{aliasName} | Retrieve an alias
[**get_aliases**](CollectionsApi.md#get_aliases) | **GET** /aliases | List all aliases
[**get_collection**](CollectionsApi.md#get_collection) | **GET** /collections/{collectionName} | Retrieve a single collection
[**get_collections**](CollectionsApi.md#get_collections) | **GET** /collections | List all collections
[**update_collection**](CollectionsApi.md#update_collection) | **PATCH** /collections/{collectionName} | Update a collection
[**upsert_alias**](CollectionsApi.md#upsert_alias) | **PUT** /aliases/{aliasName} | Create or update a collection alias



## create_collection

> crate::models::CollectionResponse create_collection(collection_schema)
Create a new collection

When a collection is created, we give it a name and describe the fields that will be indexed from the documents added to the collection.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_schema** | [**CollectionSchema**](CollectionSchema.md) | The collection object to be created | [required] |

### Return type

[**crate::models::CollectionResponse**](CollectionResponse.md)

### Authorization

[api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_alias

> crate::models::CollectionAlias delete_alias(alias_name)
Delete an alias

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**alias_name** | **String** | The name of the alias to delete | [required] |

### Return type

[**crate::models::CollectionAlias**](CollectionAlias.md)

### Authorization

[api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_collection

> crate::models::CollectionResponse delete_collection(collection_name)
Delete a collection

Permanently drops a collection. This action cannot be undone. For large collections, this might have an impact on read latencies.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_name** | **String** | The name of the collection to delete | [required] |

### Return type

[**crate::models::CollectionResponse**](CollectionResponse.md)

### Authorization

[api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_alias

> crate::models::CollectionAlias get_alias(alias_name)
Retrieve an alias

Find out which collection an alias points to by fetching it

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**alias_name** | **String** | The name of the alias to retrieve | [required] |

### Return type

[**crate::models::CollectionAlias**](CollectionAlias.md)

### Authorization

[api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_aliases

> crate::models::CollectionAliasesResponse get_aliases()
List all aliases

List all aliases and the corresponding collections that they map to.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CollectionAliasesResponse**](CollectionAliasesResponse.md)

### Authorization

[api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_collection

> crate::models::CollectionResponse get_collection(collection_name)
Retrieve a single collection

Retrieve the details of a collection, given its name.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_name** | **String** | The name of the collection to retrieve | [required] |

### Return type

[**crate::models::CollectionResponse**](CollectionResponse.md)

### Authorization

[api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_collections

> Vec<crate::models::CollectionResponse> get_collections()
List all collections

Returns a summary of all your collections. The collections are returned sorted by creation date, with the most recent collections appearing first.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::CollectionResponse>**](CollectionResponse.md)

### Authorization

[api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_collection

> crate::models::CollectionUpdateSchema update_collection(collection_name, collection_update_schema)
Update a collection

Update a collection's schema to modify the fields and their types.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_name** | **String** | The name of the collection to update | [required] |
**collection_update_schema** | [**CollectionUpdateSchema**](CollectionUpdateSchema.md) | The document object with fields to be updated | [required] |

### Return type

[**crate::models::CollectionUpdateSchema**](CollectionUpdateSchema.md)

### Authorization

[api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upsert_alias

> crate::models::CollectionAlias upsert_alias(alias_name, collection_alias_schema)
Create or update a collection alias

Create or update a collection alias. An alias is a virtual collection name that points to a real collection. If you're familiar with symbolic links on Linux, it's very similar to that. Aliases are useful when you want to reindex your data in the background on a new collection and switch your application to it without any changes to your code.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**alias_name** | **String** | The name of the alias to create/update | [required] |
**collection_alias_schema** | Option<[**CollectionAliasSchema**](CollectionAliasSchema.md)> | Collection alias to be created/updated |  |

### Return type

[**crate::models::CollectionAlias**](CollectionAlias.md)

### Authorization

[api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

