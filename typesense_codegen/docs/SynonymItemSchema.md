# SynonymItemSchema

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | Unique identifier for the synonym item | 
**synonyms** | **Vec<String>** | Array of words that should be considered as synonyms | 
**root** | Option<**String**> | For 1-way synonyms, indicates the root word that words in the synonyms parameter map to | [optional]
**locale** | Option<**String**> | Locale for the synonym, leave blank to use the standard tokenizer | [optional]
**symbols_to_index** | Option<**Vec<String>**> | By default, special characters are dropped from synonyms. Use this attribute to specify which special characters should be indexed as is | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


