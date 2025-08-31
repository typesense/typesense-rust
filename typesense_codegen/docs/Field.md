# Field

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** |  | 
**r#type** | **String** |  | 
**optional** | Option<**bool**> |  | [optional]
**facet** | Option<**bool**> |  | [optional]
**index** | Option<**bool**> |  | [optional][default to true]
**locale** | Option<**String**> |  | [optional]
**sort** | Option<**bool**> |  | [optional]
**infix** | Option<**bool**> |  | [optional][default to false]
**reference** | Option<**String**> | Name of a field in another collection that should be linked to this collection so that it can be joined during query.  | [optional]
**num_dim** | Option<**i32**> |  | [optional]
**drop** | Option<**bool**> |  | [optional]
**store** | Option<**bool**> | When set to false, the field value will not be stored on disk. Default: true.  | [optional]
**vec_dist** | Option<**String**> | The distance metric to be used for vector search. Default: `cosine`. You can also use `ip` for inner product.  | [optional]
**range_index** | Option<**bool**> | Enables an index optimized for range filtering on numerical fields (e.g. rating:>3.5). Default: false.  | [optional]
**stem** | Option<**bool**> | Values are stemmed before indexing in-memory. Default: false.  | [optional]
**stem_dictionary** | Option<**String**> | Name of the stemming dictionary to use for this field | [optional]
**token_separators** | Option<**Vec<String>**> | List of symbols or special characters to be used for splitting the text into individual words in addition to space and new-line characters.  | [optional][default to []]
**symbols_to_index** | Option<**Vec<String>**> | List of symbols or special characters to be indexed.  | [optional][default to []]
**embed** | Option<[**models::FieldEmbed**](Field_embed.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


