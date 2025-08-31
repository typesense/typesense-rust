# \OperationsApi

All URIs are relative to *http://localhost:8108*

Method | HTTP request | Description
------------- | ------------- | -------------
[**clear_cache**](OperationsApi.md#clear_cache) | **POST** /operations/cache/clear | Clear the cached responses of search requests in the LRU cache.
[**compact_db**](OperationsApi.md#compact_db) | **POST** /operations/db/compact | Compacting the on-disk database
[**get_schema_changes**](OperationsApi.md#get_schema_changes) | **GET** /operations/schema_changes | Get the status of in-progress schema change operations
[**retrieve_api_stats**](OperationsApi.md#retrieve_api_stats) | **GET** /stats.json | Get stats about API endpoints.
[**retrieve_metrics**](OperationsApi.md#retrieve_metrics) | **GET** /metrics.json | Get current RAM, CPU, Disk & Network usage metrics.
[**take_snapshot**](OperationsApi.md#take_snapshot) | **POST** /operations/snapshot | Creates a point-in-time snapshot of a Typesense node's state and data in the specified directory.
[**toggle_slow_request_log**](OperationsApi.md#toggle_slow_request_log) | **POST** /config | Toggle Slow Request Log
[**vote**](OperationsApi.md#vote) | **POST** /operations/vote | Triggers a follower node to initiate the raft voting process, which triggers leader re-election.



## clear_cache

> models::SuccessStatus clear_cache()
Clear the cached responses of search requests in the LRU cache.

Clear the cached responses of search requests that are sent with `use_cache` parameter in the LRU cache.

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


## compact_db

> models::SuccessStatus compact_db()
Compacting the on-disk database

Typesense uses RocksDB to store your documents on the disk. If you do frequent writes or updates, you could benefit from running a compaction of the underlying RocksDB database. This could reduce the size of the database and decrease read latency. While the database will not block during this operation, we recommend running it during off-peak hours.

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


## toggle_slow_request_log

> models::SuccessStatus toggle_slow_request_log(toggle_slow_request_log_request)
Toggle Slow Request Log

Enable logging of requests that take over a defined threshold of time. Default is `-1` which disables slow request logging. Slow requests are logged to the primary log file, with the prefix SLOW REQUEST.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**toggle_slow_request_log_request** | Option<[**ToggleSlowRequestLogRequest**](ToggleSlowRequestLogRequest.md)> |  |  |

### Return type

[**models::SuccessStatus**](SuccessStatus.md)

### Authorization

[api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: application/json
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

