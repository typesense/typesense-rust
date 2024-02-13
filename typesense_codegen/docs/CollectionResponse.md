# CollectionResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | Name of the collection | 
**fields** | [**Vec<crate::models::Field>**](Field.md) | A list of fields for querying, filtering and faceting | 
**default_sorting_field** | Option<**String**> | The name of an int32 / float field that determines the order in which the search results are ranked when a sort_by clause is not provided during searching. This field must indicate some kind of popularity. | [optional][default to ]
**token_separators** | Option<**Vec<String>**> | List of symbols or special characters to be used for  splitting the text into individual words in addition to space and new-line characters.  | [optional][default to []]
**enable_nested_fields** | Option<**bool**> | Enables experimental support at a collection level for nested object or object array fields. This field is only available if the Typesense server is version `0.24.0.rcn34` or later. | [optional][default to false]
**symbols_to_index** | Option<**Vec<String>**> | List of symbols or special characters to be indexed.  | [optional][default to []]
**num_documents** | **i64** | Number of documents in the collection | [readonly]
**created_at** | **i64** | Timestamp of when the collection was created (Unix epoch in seconds) | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


