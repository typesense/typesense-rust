# \AnalyticsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_analytics_rule**](AnalyticsApi.md#create_analytics_rule) | **POST** /analytics/rules | Creates an analytics rule
[**delete_analytics_rule**](AnalyticsApi.md#delete_analytics_rule) | **DELETE** /analytics/rules/{ruleName} | Delete an analytics rule
[**retrieve_analytics_rule**](AnalyticsApi.md#retrieve_analytics_rule) | **GET** /analytics/rules/{ruleName} | Retrieves an analytics rule
[**retrieve_analytics_rules**](AnalyticsApi.md#retrieve_analytics_rules) | **GET** /analytics/rules | Retrieves all analytics rules



## create_analytics_rule

> crate::models::AnalyticsRuleSchema create_analytics_rule(analytics_rule_schema)
Creates an analytics rule

When an analytics rule is created, we give it a name and describe the type, the source collections and the destination collection.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**analytics_rule_schema** | [**AnalyticsRuleSchema**](AnalyticsRuleSchema.md) | The Analytics rule to be created | [required] |

### Return type

[**crate::models::AnalyticsRuleSchema**](AnalyticsRuleSchema.md)

### Authorization

[api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_analytics_rule

> crate::models::AnalyticsRuleSchema delete_analytics_rule(rule_name)
Delete an analytics rule

Permanently deletes an analytics rule, given it's name

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rule_name** | **String** | The name of the analytics rule to delete | [required] |

### Return type

[**crate::models::AnalyticsRuleSchema**](AnalyticsRuleSchema.md)

### Authorization

[api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_analytics_rule

> crate::models::AnalyticsRuleSchema retrieve_analytics_rule(rule_name)
Retrieves an analytics rule

Retrieve the details of an analytics rule, given it's name

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rule_name** | **String** | The name of the analytics rule to retrieve | [required] |

### Return type

[**crate::models::AnalyticsRuleSchema**](AnalyticsRuleSchema.md)

### Authorization

[api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_analytics_rules

> crate::models::AnalyticsRulesRetrieveSchema retrieve_analytics_rules()
Retrieves all analytics rules

Retrieve the details of all analytics rules

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::AnalyticsRulesRetrieveSchema**](AnalyticsRulesRetrieveSchema.md)

### Authorization

[api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

