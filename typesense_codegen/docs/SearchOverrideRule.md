# SearchOverrideRule

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**filter_by** | Option<**String**> | Indicates that the override should apply when the filter_by parameter in a search query exactly matches the string specified here (including backticks, spaces, brackets, etc).  | [optional]
**r#match** | Option<**String**> | Indicates whether the match on the query term should be `exact` or `contains`. If we want to match all queries that contained the word `apple`, we will use the `contains` match instead.  | [optional]
**query** | Option<**String**> | Indicates what search queries should be overridden | [optional]
**tags** | Option<**Vec<String>**> | List of tag values to associate with this override rule. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


