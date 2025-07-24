# DeleteDocumentsDeleteDocumentsParametersParameter

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**filter_by** | **String** |  | 
**batch_size** | Option<**i32**> | Batch size parameter controls the number of documents that should be deleted at a time. A larger value will speed up deletions, but will impact performance of other operations running on the server. | [optional]
**ignore_not_found** | Option<**bool**> |  | [optional]
**truncate** | Option<**bool**> | When true, removes all documents from the collection while preserving the collection and its schema. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


