# \OperationsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**take_snapshot**](OperationsApi.md#take_snapshot) | **POST** /operations/snapshot | Creates a point-in-time snapshot of a Typesense node's state and data in the specified directory.
[**vote**](OperationsApi.md#vote) | **POST** /operations/vote | Triggers a follower node to initiate the raft voting process, which triggers leader re-election.



## take_snapshot

> crate::models::SuccessStatus take_snapshot(snapshot_path)
Creates a point-in-time snapshot of a Typesense node's state and data in the specified directory.

Creates a point-in-time snapshot of a Typesense node's state and data in the specified directory. You can then backup the snapshot directory that gets created and later restore it as a data directory, as needed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**snapshot_path** | **String** | The directory on the server where the snapshot should be saved. | [required] |

### Return type

[**crate::models::SuccessStatus**](SuccessStatus.md)

### Authorization

[api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## vote

> crate::models::SuccessStatus vote()
Triggers a follower node to initiate the raft voting process, which triggers leader re-election.

Triggers a follower node to initiate the raft voting process, which triggers leader re-election. The follower node that you run this operation against will become the new leader, once this command succeeds.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::SuccessStatus**](SuccessStatus.md)

### Authorization

[api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

