//! Provides access to the API endpoints for managing a single API key.
//!
//! A `Key` instance is created via the `Client::key(key_id)` method.

use crate::{Client, Error};
use typesense_codegen::{
    apis::{configuration, keys_api},
    models,
};

/// Provides methods for managing a specific Typesense API key.
///
/// This struct is created by calling `client.key(key_id)`.
pub struct Key<'a> {
    pub(super) client: &'a Client,
    pub(super) key_id: i64,
}

impl<'a> Key<'a> {
    /// Creates a new `Key` instance for a specific key ID.
    pub(super) fn new(client: &'a Client, key_id: i64) -> Self {
        Self { client, key_id }
    }

    /// Retrieves metadata about this specific API key.
    ///
    /// For security reasons, this endpoint only returns the key prefix and metadata,
    /// not the full key value.
    pub async fn retrieve(&self) -> Result<models::ApiKey, Error<keys_api::GetKeyError>> {
        let params = keys_api::GetKeyParams {
            key_id: self.key_id,
        };
        self.client
            .execute(|config: configuration::Configuration| {
                let params_for_move = params.clone();
                async move { keys_api::get_key(&config, params_for_move).await }
            })
            .await
    }

    /// Deletes this specific API key.
    pub async fn delete(
        &self,
    ) -> Result<models::ApiKeyDeleteResponse, Error<keys_api::DeleteKeyError>> {
        let params = keys_api::DeleteKeyParams {
            key_id: self.key_id,
        };
        self.client
            .execute(|config: configuration::Configuration| {
                let params_for_move = params.clone();
                async move { keys_api::delete_key(&config, params_for_move).await }
            })
            .await
    }
}
