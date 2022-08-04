# CollectionSchema

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | Name of the collection | 
**fields** | [**Vec<crate::models::Field>**](Field.md) | A list of fields for querying, filtering and faceting | 
**default_sorting_field** | Option<**String**> | The name of an int32 / float field that determines the order in which the search results are ranked when a sort_by clause is not provided during searching. This field must indicate some kind of popularity. | [optional][default to ]
**token_separators** | Option<**Vec<String>**> | List of symbols or special characters to be used for  splitting the text into individual words in addition to space and new-line characters.  | [optional][default to []]
**symbols_to_index** | Option<**Vec<String>**> | List of symbols or special characters to be indexed.  | [optional][default to []]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


