//! Provides access to the API endpoints for managing conversation models.
//!
//! An instance of `Models` is created via the `client.conversations().models()` method.

use crate::{Client, Error, execute_wrapper, models};
use typesense_codegen::apis::conversations_api;

/// Provides methods for creating and listing conversation models.
///
/// This struct is created by calling `client.conversations().models()`.
pub struct Models<'a> {
    pub(super) client: &'a Client,
}

impl<'a> Models<'a> {
    /// Creates a new `Models` instance.
    #[inline]
    pub(super) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Creates a new conversation model.
    ///
    /// # Arguments
    /// * `schema` - A `ConversationModelCreateSchema` object describing the model.
    pub async fn create(
        &self,
        schema: models::ConversationModelCreateSchema<'_>,
    ) -> Result<
        models::ConversationModelSchema<'static>,
        Error<conversations_api::CreateConversationModelError<'static>>,
    > {
        let params = conversations_api::CreateConversationModelParams {
            conversation_model_create_schema: schema,
            _phantom: core::marker::PhantomData,
        };
        execute_wrapper!(self, conversations_api::create_conversation_model, params)
    }

    /// Retrieves a summary of all conversation models.
    pub async fn retrieve(
        &self,
    ) -> Result<
        Vec<models::ConversationModelSchema<'static>>,
        Error<conversations_api::RetrieveAllConversationModelsError<'static>>,
    > {
        execute_wrapper!(self, conversations_api::retrieve_all_conversation_models)
    }
}
