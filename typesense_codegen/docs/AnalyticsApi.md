# \AnalyticsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_analytics_event**](AnalyticsApi.md#create_analytics_event) | **POST** /analytics/events | Create an analytics event
[**create_analytics_rule**](AnalyticsApi.md#create_analytics_rule) | **POST** /analytics/rules | Creates an analytics rule
[**delete_analytics_rule**](AnalyticsApi.md#delete_analytics_rule) | **DELETE** /analytics/rules/{ruleName} | Delete an analytics rule
[**retrieve_analytics_rule**](AnalyticsApi.md#retrieve_analytics_rule) | **GET** /analytics/rules/{ruleName} | Retrieves an analytics rule
[**retrieve_analytics_rules**](AnalyticsApi.md#retrieve_analytics_rules) | **GET** /analytics/rules | Retrieves all analytics rules
[**upsert_analytics_rule**](AnalyticsApi.md#upsert_analytics_rule) | **PUT** /analytics/rules/{ruleName} | Upserts an analytics rule



## create_analytics_event

> models::AnalyticsEventCreateResponse create_analytics_event(analytics_event_create_schema)
Create an analytics event

Sending events for analytics e.g rank search results based on popularity.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**analytics_event_create_schema** | [**AnalyticsEventCreateSchema**](AnalyticsEventCreateSchema.md) | The Analytics event to be created | [required] |

### Return type

[**models::AnalyticsEventCreateResponse**](AnalyticsEventCreateResponse.md)

### Authorization

[api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_analytics_rule

> models::AnalyticsRuleSchema create_analytics_rule(analytics_rule_schema)
Creates an analytics rule

When an analytics rule is created, we give it a name and describe the type, the source collections and the destination collection.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**analytics_rule_schema** | [**AnalyticsRuleSchema**](AnalyticsRuleSchema.md) | The Analytics rule to be created | [required] |

### Return type

[**models::AnalyticsRuleSchema**](AnalyticsRuleSchema.md)

### Authorization

[api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_analytics_rule

> models::AnalyticsRuleDeleteResponse delete_analytics_rule(rule_name)
Delete an analytics rule

Permanently deletes an analytics rule, given it's name

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rule_name** | **String** | The name of the analytics rule to delete | [required] |

### Return type

[**models::AnalyticsRuleDeleteResponse**](AnalyticsRuleDeleteResponse.md)

### Authorization

[api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_analytics_rule

> models::AnalyticsRuleSchema retrieve_analytics_rule(rule_name)
Retrieves an analytics rule

Retrieve the details of an analytics rule, given it's name

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rule_name** | **String** | The name of the analytics rule to retrieve | [required] |

### Return type

[**models::AnalyticsRuleSchema**](AnalyticsRuleSchema.md)

### Authorization

[api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_analytics_rules

> models::AnalyticsRulesRetrieveSchema retrieve_analytics_rules()
Retrieves all analytics rules

Retrieve the details of all analytics rules

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::AnalyticsRulesRetrieveSchema**](AnalyticsRulesRetrieveSchema.md)

### Authorization

[api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upsert_analytics_rule

> models::AnalyticsRuleSchema upsert_analytics_rule(rule_name, analytics_rule_upsert_schema)
Upserts an analytics rule

Upserts an analytics rule with the given name.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rule_name** | **String** | The name of the analytics rule to upsert | [required] |
**analytics_rule_upsert_schema** | [**AnalyticsRuleUpsertSchema**](AnalyticsRuleUpsertSchema.md) | The Analytics rule to be upserted | [required] |

### Return type

[**models::AnalyticsRuleSchema**](AnalyticsRuleSchema.md)

### Authorization

[api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

