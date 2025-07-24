# \OperationsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_schema_changes**](OperationsApi.md#get_schema_changes) | **GET** /operations/schema_changes | Get the status of in-progress schema change operations
[**retrieve_api_stats**](OperationsApi.md#retrieve_api_stats) | **GET** /stats.json | Get stats about API endpoints.
[**retrieve_metrics**](OperationsApi.md#retrieve_metrics) | **GET** /metrics.json | Get current RAM, CPU, Disk & Network usage metrics.
[**take_snapshot**](OperationsApi.md#take_snapshot) | **POST** /operations/snapshot | Creates a point-in-time snapshot of a Typesense node's state and data in the specified directory.
[**vote**](OperationsApi.md#vote) | **POST** /operations/vote | Triggers a follower node to initiate the raft voting process, which triggers leader re-election.



## get_schema_changes

> Vec<models::SchemaChangeStatus> get_schema_changes()
Get the status of in-progress schema change operations

Returns the status of any ongoing schema change operations. If no schema changes are in progress, returns an empty response.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::SchemaChangeStatus>**](SchemaChangeStatus.md)

### Authorization

[api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_api_stats

> models::ApiStatsResponse retrieve_api_stats()
Get stats about API endpoints.

Retrieve the stats about API endpoints.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ApiStatsResponse**](APIStatsResponse.md)

### Authorization

[api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_metrics

> serde_json::Value retrieve_metrics()
Get current RAM, CPU, Disk & Network usage metrics.

Retrieve the metrics.

### Parameters

This endpoint does not need any parameter.

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## take_snapshot

> models::SuccessStatus take_snapshot(snapshot_path)
Creates a point-in-time snapshot of a Typesense node's state and data in the specified directory.

Creates a point-in-time snapshot of a Typesense node's state and data in the specified directory. You can then backup the snapshot directory that gets created and later restore it as a data directory, as needed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**snapshot_path** | **String** | The directory on the server where the snapshot should be saved. | [required] |

### Return type

[**models::SuccessStatus**](SuccessStatus.md)

### Authorization

[api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## vote

> models::SuccessStatus vote()
Triggers a follower node to initiate the raft voting process, which triggers leader re-election.

Triggers a follower node to initiate the raft voting process, which triggers leader re-election. The follower node that you run this operation against will become the new leader, once this command succeeds.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::SuccessStatus**](SuccessStatus.md)

### Authorization

[api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

