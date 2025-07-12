# Field

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**drop** | Option<**bool**> |  | [optional]
**embed** | Option<[**models::FieldEmbed**](Field_embed.md)> |  | [optional]
**facet** | Option<**bool**> |  | [optional]
**index** | Option<**bool**> |  | [optional][default to true]
**infix** | Option<**bool**> |  | [optional][default to false]
**locale** | Option<**String**> |  | [optional]
**name** | **String** |  | 
**num_dim** | Option<**i32**> |  | [optional]
**optional** | Option<**bool**> |  | [optional]
**range_index** | Option<**bool**> | Enables an index optimized for range filtering on numerical fields (e.g. rating:>3.5). Default: false.  | [optional]
**reference** | Option<**String**> | Name of a field in another collection that should be linked to this collection so that it can be joined during query.  | [optional]
**sort** | Option<**bool**> |  | [optional]
**stem** | Option<**bool**> | Values are stemmed before indexing in-memory. Default: false.  | [optional]
**stem_dictionary** | Option<**String**> | Name of the stemming dictionary to use for this field | [optional]
**store** | Option<**bool**> | When set to false, the field value will not be stored on disk. Default: true.  | [optional]
**symbols_to_index** | Option<**Vec<String>**> | List of symbols or special characters to be indexed.  | [optional][default to []]
**token_separators** | Option<**Vec<String>**> | List of symbols or special characters to be used for splitting the text into individual words in addition to space and new-line characters.  | [optional][default to []]
**r#type** | **String** |  | 
**vec_dist** | Option<**String**> | The distance metric to be used for vector search. Default: `cosine`. You can also use `ip` for inner product.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


