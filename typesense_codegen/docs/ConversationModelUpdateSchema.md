# ConversationModelUpdateSchema

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**account_id** | Option<**String**> | LLM service's account ID (only applicable for Cloudflare) | [optional]
**api_key** | Option<**String**> | The LLM service's API Key | [optional]
**history_collection** | Option<**String**> | Typesense collection that stores the historical conversations | [optional]
**id** | Option<**String**> | An explicit id for the model, otherwise the API will return a response with an auto-generated conversation model id. | [optional]
**max_bytes** | Option<**i32**> | The maximum number of bytes to send to the LLM in every API call. Consult the LLM's documentation on the number of bytes supported in the context window.  | [optional]
**model_name** | Option<**String**> | Name of the LLM model offered by OpenAI, Cloudflare or vLLM | [optional]
**system_prompt** | Option<**String**> | The system prompt that contains special instructions to the LLM | [optional]
**ttl** | Option<**i32**> | Time interval in seconds after which the messages would be deleted. Default: 86400 (24 hours)  | [optional]
**vllm_url** | Option<**String**> | URL of vLLM service | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


