# SearchHighlight

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**field** | Option<**String**> |  | [optional]
**snippet** | Option<**String**> | Present only for (non-array) string fields | [optional]
**snippets** | Option<**Vec<String>**> | Present only for (array) string[] fields | [optional]
**indices** | Option<**Vec<i32>**> | The indices property will be present only for string[] fields and will contain the corresponding indices of the snippets in the search field | [optional]
**matched_tokens** | Option<[**Vec<serde_json::Value>**](serde_json::Value.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


