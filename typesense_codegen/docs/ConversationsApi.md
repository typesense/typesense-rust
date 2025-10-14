# \ConversationsApi

All URIs are relative to *http://localhost:8108*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_conversation_model**](ConversationsApi.md#create_conversation_model) | **POST** /conversations/models | Create a conversation model
[**delete_conversation_model**](ConversationsApi.md#delete_conversation_model) | **DELETE** /conversations/models/{modelId} | Delete a conversation model
[**retrieve_all_conversation_models**](ConversationsApi.md#retrieve_all_conversation_models) | **GET** /conversations/models | List all conversation models
[**retrieve_conversation_model**](ConversationsApi.md#retrieve_conversation_model) | **GET** /conversations/models/{modelId} | Retrieve a conversation model
[**update_conversation_model**](ConversationsApi.md#update_conversation_model) | **PUT** /conversations/models/{modelId} | Update a conversation model



## create_conversation_model

> models::ConversationModelSchema create_conversation_model(conversation_model_create_schema)
Create a conversation model

Create a Conversation Model

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_model_create_schema** | [**ConversationModelCreateSchema**](ConversationModelCreateSchema.md) |  | [required] |

### Return type

[**models::ConversationModelSchema**](ConversationModelSchema.md)

### Authorization

[api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_conversation_model

> models::ConversationModelSchema delete_conversation_model(model_id)
Delete a conversation model

Delete a conversation model

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**model_id** | **String** | The id of the conversation model to delete | [required] |

### Return type

[**models::ConversationModelSchema**](ConversationModelSchema.md)

### Authorization

[api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_all_conversation_models

> Vec<models::ConversationModelSchema> retrieve_all_conversation_models()
List all conversation models

Retrieve all conversation models

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::ConversationModelSchema>**](ConversationModelSchema.md)

### Authorization

[api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_conversation_model

> models::ConversationModelSchema retrieve_conversation_model(model_id)
Retrieve a conversation model

Retrieve a conversation model

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**model_id** | **String** | The id of the conversation model to retrieve | [required] |

### Return type

[**models::ConversationModelSchema**](ConversationModelSchema.md)

### Authorization

[api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_conversation_model

> models::ConversationModelSchema update_conversation_model(model_id, conversation_model_update_schema)
Update a conversation model

Update a conversation model

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**model_id** | **String** | The id of the conversation model to update | [required] |
**conversation_model_update_schema** | [**ConversationModelUpdateSchema**](ConversationModelUpdateSchema.md) |  | [required] |

### Return type

[**models::ConversationModelSchema**](ConversationModelSchema.md)

### Authorization

[api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

