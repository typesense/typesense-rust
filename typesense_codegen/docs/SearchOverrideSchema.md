# SearchOverrideSchema

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**rule** | [**models::SearchOverrideRule**](SearchOverrideRule.md) |  | 
**includes** | Option<[**Vec<models::SearchOverrideInclude>**](SearchOverrideInclude.md)> | List of document `id`s that should be included in the search results with their corresponding `position`s. | [optional]
**excludes** | Option<[**Vec<models::SearchOverrideExclude>**](SearchOverrideExclude.md)> | List of document `id`s that should be excluded from the search results. | [optional]
**filter_by** | Option<**String**> | A filter by clause that is applied to any search query that matches the override rule.  | [optional]
**remove_matched_tokens** | Option<**bool**> | Indicates whether search query tokens that exist in the override's rule should be removed from the search query.  | [optional]
**metadata** | Option<[**serde_json::Value**](.md)> | Return a custom JSON object in the Search API response, when this rule is triggered. This can can be used to display a pre-defined message (eg: a promotion banner) on the front-end when a particular rule is triggered.  | [optional]
**sort_by** | Option<**String**> | A sort by clause that is applied to any search query that matches the override rule.  | [optional]
**replace_query** | Option<**String**> | Replaces the current search query with this value, when the search query matches the override rule.  | [optional]
**filter_curated_hits** | Option<**bool**> | When set to true, the filter conditions of the query is applied to the curated records as well. Default: false.  | [optional]
**effective_from_ts** | Option<**i32**> | A Unix timestamp that indicates the date/time from which the override will be active. You can use this to create override rules that start applying from a future point in time.  | [optional]
**effective_to_ts** | Option<**i32**> | A Unix timestamp that indicates the date/time until which the override will be active. You can use this to create override rules that stop applying after a period of time.  | [optional]
**stop_processing** | Option<**bool**> | When set to true, override processing will stop at the first matching rule. When set to false override processing will continue and multiple override actions will be triggered in sequence. Overrides are processed in the lexical sort order of their id field. Default: true.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


