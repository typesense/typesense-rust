# \AnalyticsApi

All URIs are relative to *http://localhost:8108*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_analytics_event**](AnalyticsApi.md#create_analytics_event) | **POST** /analytics/events | Create an analytics event
[**create_analytics_rule**](AnalyticsApi.md#create_analytics_rule) | **POST** /analytics/rules | Create analytics rule(s)
[**delete_analytics_rule**](AnalyticsApi.md#delete_analytics_rule) | **DELETE** /analytics/rules/{ruleName} | Delete an analytics rule
[**flush_analytics**](AnalyticsApi.md#flush_analytics) | **POST** /analytics/flush | Flush in-memory analytics to disk
[**get_analytics_events**](AnalyticsApi.md#get_analytics_events) | **GET** /analytics/events | Retrieve analytics events
[**get_analytics_status**](AnalyticsApi.md#get_analytics_status) | **GET** /analytics/status | Get analytics subsystem status
[**retrieve_analytics_rule**](AnalyticsApi.md#retrieve_analytics_rule) | **GET** /analytics/rules/{ruleName} | Retrieves an analytics rule
[**retrieve_analytics_rules**](AnalyticsApi.md#retrieve_analytics_rules) | **GET** /analytics/rules | Retrieve analytics rules
[**upsert_analytics_rule**](AnalyticsApi.md#upsert_analytics_rule) | **PUT** /analytics/rules/{ruleName} | Upserts an analytics rule



## create_analytics_event

> models::AnalyticsEventCreateResponse create_analytics_event(analytics_event)
Create an analytics event

Submit a single analytics event. The event must correspond to an existing analytics rule by name.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**analytics_event** | [**AnalyticsEvent**](AnalyticsEvent.md) | The analytics event to be created | [required] |

### Return type

[**models::AnalyticsEventCreateResponse**](AnalyticsEventCreateResponse.md)

### Authorization

[api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_analytics_rule

> models::CreateAnalyticsRule200Response create_analytics_rule(create_analytics_rule_request)
Create analytics rule(s)

Create one or more analytics rules. You can send a single rule object or an array of rule objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_analytics_rule_request** | [**CreateAnalyticsRuleRequest**](CreateAnalyticsRuleRequest.md) | The analytics rule(s) to be created | [required] |

### Return type

[**models::CreateAnalyticsRule200Response**](createAnalyticsRule_200_response.md)

### Authorization

[api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_analytics_rule

> models::AnalyticsRule delete_analytics_rule(rule_name)
Delete an analytics rule

Permanently deletes an analytics rule, given it's name

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rule_name** | **String** | The name of the analytics rule to delete | [required] |

### Return type

[**models::AnalyticsRule**](AnalyticsRule.md)

### Authorization

[api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## flush_analytics

> models::AnalyticsEventCreateResponse flush_analytics()
Flush in-memory analytics to disk

Triggers a flush of analytics data to persistent storage.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::AnalyticsEventCreateResponse**](AnalyticsEventCreateResponse.md)

### Authorization

[api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_analytics_events

> models::AnalyticsEventsResponse get_analytics_events(user_id, name, n)
Retrieve analytics events

Retrieve the most recent events for a user and rule.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** |  | [required] |
**name** | **String** | Analytics rule name | [required] |
**n** | **i32** | Number of events to return (max 1000) | [required] |

### Return type

[**models::AnalyticsEventsResponse**](AnalyticsEventsResponse.md)

### Authorization

[api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_analytics_status

> models::AnalyticsStatus get_analytics_status()
Get analytics subsystem status

Returns sizes of internal analytics buffers and queues.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::AnalyticsStatus**](AnalyticsStatus.md)

### Authorization

[api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_analytics_rule

> models::AnalyticsRule retrieve_analytics_rule(rule_name)
Retrieves an analytics rule

Retrieve the details of an analytics rule, given it's name

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rule_name** | **String** | The name of the analytics rule to retrieve | [required] |

### Return type

[**models::AnalyticsRule**](AnalyticsRule.md)

### Authorization

[api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_analytics_rules

> Vec<models::AnalyticsRule> retrieve_analytics_rules(rule_tag)
Retrieve analytics rules

Retrieve all analytics rules. Use the optional rule_tag filter to narrow down results.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rule_tag** | Option<**String**> | Filter rules by rule_tag |  |

### Return type

[**Vec<models::AnalyticsRule>**](AnalyticsRule.md)

### Authorization

[api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upsert_analytics_rule

> models::AnalyticsRule upsert_analytics_rule(rule_name, analytics_rule_update)
Upserts an analytics rule

Upserts an analytics rule with the given name.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rule_name** | **String** | The name of the analytics rule to upsert | [required] |
**analytics_rule_update** | [**AnalyticsRuleUpdate**](AnalyticsRuleUpdate.md) | The Analytics rule to be upserted | [required] |

### Return type

[**models::AnalyticsRule**](AnalyticsRule.md)

### Authorization

[api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

