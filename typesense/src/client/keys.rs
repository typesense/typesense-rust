//! Provides access to the API endpoints for managing the collection of API keys.
//!
//! An `Keys` instance is created via the `Client::keys()` method.

use crate::{Client, Error};
use std::sync::Arc;
use typesense_codegen::{
    apis::{configuration, keys_api},
    models,
};

/// Provides methods for managing a collection of Typesense API keys.
///
/// This struct is created by calling `client.keys()`.
pub struct Keys<'a> {
    pub(super) client: &'a Client,
}

impl<'a> Keys<'a> {
    /// Creates a new `Keys` instance.
    pub(super) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Creates a new API key with fine-grained access control.
    ///
    /// You can restrict access on a per-collection and per-action level.
    /// The full, unhashed key is only returned on creation.
    ///
    /// # Arguments
    /// * `schema` - An `ApiKeySchema` object describing the key's permissions.
    pub async fn create(
        &self,
        schema: models::ApiKeySchema,
    ) -> Result<models::ApiKey, Error<keys_api::CreateKeyError>> {
        let params = keys_api::CreateKeyParams {
            api_key_schema: Some(schema),
        };
        self.client
            .execute(|config: Arc<configuration::Configuration>| {
                let params_for_move = params.clone();
                async move { keys_api::create_key(&config, params_for_move).await }
            })
            .await
    }

    /// Lists all API keys and their metadata.
    pub async fn retrieve(&self) -> Result<models::ApiKeysResponse, Error<keys_api::GetKeysError>> {
        self.client
            .execute(|config: Arc<configuration::Configuration>| async move {
                keys_api::get_keys(&config).await
            })
            .await
    }
}
