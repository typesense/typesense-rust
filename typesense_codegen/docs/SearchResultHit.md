# SearchResultHit

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**highlights** | Option<[**Vec<models::SearchHighlight>**](SearchHighlight.md)> | (Deprecated) Contains highlighted portions of the search fields | [optional]
**highlight** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | Highlighted version of the matching document | [optional]
**document** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | Can be any key-value pair | [optional]
**text_match** | Option<**i64**> |  | [optional]
**text_match_info** | Option<[**models::SearchResultHitTextMatchInfo**](SearchResultHit_text_match_info.md)> |  | [optional]
**geo_distance_meters** | Option<**std::collections::HashMap<String, i32>**> | Can be any key-value pair | [optional]
**vector_distance** | Option<**f32**> | Distance between the query vector and matching document's vector value | [optional]
**hybrid_search_info** | Option<[**models::SearchResultHitHybridSearchInfo**](SearchResultHit_hybrid_search_info.md)> |  | [optional]
**search_index** | Option<**i32**> | Returned only for union query response. Indicates the index of the query which this document matched to. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


