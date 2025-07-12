# SearchResult

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**conversation** | Option<[**models::SearchResultConversation**](SearchResultConversation.md)> |  | [optional]
**facet_counts** | Option<[**Vec<models::FacetCounts>**](FacetCounts.md)> |  | [optional]
**found** | Option<**i32**> | The number of documents found | [optional]
**found_docs** | Option<**i32**> |  | [optional]
**grouped_hits** | Option<[**Vec<models::SearchGroupedHit>**](SearchGroupedHit.md)> |  | [optional]
**hits** | Option<[**Vec<models::SearchResultHit>**](SearchResultHit.md)> | The documents that matched the search query | [optional]
**out_of** | Option<**i32**> | The total number of documents in the collection | [optional]
**page** | Option<**i32**> | The search result page number | [optional]
**request_params** | Option<[**models::SearchResultRequestParams**](SearchResult_request_params.md)> |  | [optional]
**search_cutoff** | Option<**bool**> | Whether the search was cut off | [optional]
**search_time_ms** | Option<**i32**> | The number of milliseconds the search took | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


