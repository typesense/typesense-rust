# SearchResult

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**facet_counts** | Option<[**Vec<crate::models::FacetCounts>**](FacetCounts.md)> |  | [optional]
**found** | Option<**i32**> | The number of documents found | [optional]
**search_time_ms** | Option<**i32**> | The number of milliseconds the search took | [optional]
**out_of** | Option<**i32**> | The total number of pages | [optional]
**search_cutoff** | Option<**bool**> | Whether the search was cut off | [optional]
**page** | Option<**i32**> | The search result page number | [optional]
**grouped_hits** | Option<[**Vec<crate::models::SearchGroupedHit>**](SearchGroupedHit.md)> |  | [optional]
**hits** | Option<[**Vec<crate::models::SearchResultHit>**](SearchResultHit.md)> | The documents that matched the search query | [optional]
**request_params** | Option<[**crate::models::SearchResultRequestParams**](SearchResult_request_params.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


