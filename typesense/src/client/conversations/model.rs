//! Provides access to the API endpoints for managing a single conversation model.
//!
//! An instance of `Model` is created via the `client.conversations().model("model_id")` method.

use crate::{Client, Error, execute_wrapper, models};
use typesense_codegen::apis::conversations_api;

/// Provides methods for interacting with a specific conversation model.
///
/// This struct is created by calling `client.conversations().model("model_id")`.
pub struct Model<'a> {
    pub(super) client: &'a Client,
    pub(super) model_id: &'a str,
}

impl<'a> Model<'a> {
    /// Creates a new `Model` instance for a specific model ID.
    #[inline]
    pub(super) fn new(client: &'a Client, model_id: &'a str) -> Self {
        Self { client, model_id }
    }

    /// Retrieves the details of this specific conversation model.
    pub async fn retrieve(
        &self,
    ) -> Result<
        models::ConversationModelSchema,
        Error<conversations_api::RetrieveConversationModelError>,
    > {
        let params = conversations_api::RetrieveConversationModelParams {
            model_id: self.model_id.to_owned(),
        };
        execute_wrapper!(self, conversations_api::retrieve_conversation_model, params)
    }

    /// Updates this specific conversation model.
    ///
    /// # Arguments
    /// * `schema` - A `ConversationModelUpdateSchema` object with the fields to update.
    pub async fn update(
        &self,
        schema: models::ConversationModelUpdateSchema,
    ) -> Result<
        models::ConversationModelSchema,
        Error<conversations_api::UpdateConversationModelError>,
    > {
        let params = conversations_api::UpdateConversationModelParams {
            model_id: self.model_id.to_owned(),
            conversation_model_update_schema: schema,
        };
        execute_wrapper!(self, conversations_api::update_conversation_model, params)
    }

    /// Deletes this specific conversation model.
    pub async fn delete(
        &self,
    ) -> Result<
        models::ConversationModelSchema,
        Error<conversations_api::DeleteConversationModelError>,
    > {
        let params = conversations_api::DeleteConversationModelParams {
            model_id: self.model_id.to_owned(),
        };
        execute_wrapper!(self, conversations_api::delete_conversation_model, params)
    }
}
