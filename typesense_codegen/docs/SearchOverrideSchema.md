# SearchOverrideSchema

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**rule** | [**crate::models::SearchOverrideRule**](SearchOverrideRule.md) |  | 
**includes** | Option<[**Vec<crate::models::SearchOverrideInclude>**](SearchOverrideInclude.md)> | List of document `id`s that should be included in the search results with their corresponding `position`s. | [optional]
**excludes** | Option<[**Vec<crate::models::SearchOverrideExclude>**](SearchOverrideExclude.md)> | List of document `id`s that should be excluded from the search results. | [optional]
**filter_by** | Option<**String**> | A filter by clause that is applied to any search query that matches the override rule.  | [optional]
**remove_matched_tokens** | Option<**bool**> | Indicates whether search query tokens that exist in the override's rule should be removed from the search query.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


