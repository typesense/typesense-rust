//! Provides access to the API endpoints for managing conversation models.
//!
//! An instance of `Models` is created via the `Conversations::models()` method.

use crate::client::{Client, Error};
use std::sync::Arc;
use typesense_codegen::{
    apis::{configuration, conversations_api},
    models,
};

/// Provides methods for creating and listing conversation models.
///
/// This struct is created by calling `conversations.models()`.
pub struct Models<'a> {
    pub(super) client: &'a Client,
}

impl<'a> Models<'a> {
    /// Creates a new `Models` instance.
    pub(super) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Creates a new conversation model.
    ///
    /// # Arguments
    /// * `schema` - A `ConversationModelCreateSchema` object describing the model.
    pub async fn create(
        &self,
        schema: models::ConversationModelCreateSchema,
    ) -> Result<
        models::ConversationModelSchema,
        Error<conversations_api::CreateConversationModelError>,
    > {
        let params = conversations_api::CreateConversationModelParams {
            conversation_model_create_schema: schema,
        };
        self.client
            .execute(|config: Arc<configuration::Configuration>| {
                let params_for_move = params.clone();
                async move {
                    conversations_api::create_conversation_model(&config, params_for_move).await
                }
            })
            .await
    }

    /// Retrieves a summary of all conversation models.
    pub async fn retrieve(
        &self,
    ) -> Result<
        Vec<models::ConversationModelSchema>,
        Error<conversations_api::RetrieveAllConversationModelsError>,
    > {
        self.client
            .execute(|config: Arc<configuration::Configuration>| async move {
                conversations_api::retrieve_all_conversation_models(&config).await
            })
            .await
    }
}
