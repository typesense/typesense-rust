# \PresetsApi

All URIs are relative to *http://localhost:8108*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_preset**](PresetsApi.md#delete_preset) | **DELETE** /presets/{presetId} | Delete a preset.
[**retrieve_all_presets**](PresetsApi.md#retrieve_all_presets) | **GET** /presets | Retrieves all presets.
[**retrieve_preset**](PresetsApi.md#retrieve_preset) | **GET** /presets/{presetId} | Retrieves a preset.
[**upsert_preset**](PresetsApi.md#upsert_preset) | **PUT** /presets/{presetId} | Upserts a preset.



## delete_preset

> models::PresetDeleteSchema delete_preset(preset_id)
Delete a preset.

Permanently deletes a preset, given it's name.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**preset_id** | **String** | The ID of the preset to delete. | [required] |

### Return type

[**models::PresetDeleteSchema**](PresetDeleteSchema.md)

### Authorization

[api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_all_presets

> models::PresetsRetrieveSchema retrieve_all_presets()
Retrieves all presets.

Retrieve the details of all presets

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::PresetsRetrieveSchema**](PresetsRetrieveSchema.md)

### Authorization

[api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_preset

> models::PresetSchema retrieve_preset(preset_id)
Retrieves a preset.

Retrieve the details of a preset, given it's name.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**preset_id** | **String** | The ID of the preset to retrieve. | [required] |

### Return type

[**models::PresetSchema**](PresetSchema.md)

### Authorization

[api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upsert_preset

> models::PresetSchema upsert_preset(preset_id, preset_upsert_schema)
Upserts a preset.

Create or update an existing preset.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**preset_id** | **String** | The name of the preset set to upsert. | [required] |
**preset_upsert_schema** | [**PresetUpsertSchema**](PresetUpsertSchema.md) | The stopwords set to upsert. | [required] |

### Return type

[**models::PresetSchema**](PresetSchema.md)

### Authorization

[api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

