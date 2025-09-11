//! Provides access to the API endpoints for managing a single API key.
//!
//! A `Key` instance is created via the `client.key(key_id)` method.

use crate::{Client, Error, execute_wrapper};
use typesense_codegen::{apis::keys_api, models};

/// Provides methods for managing a specific Typesense API key.
///
/// This struct is created by calling `client.key(key_id)`.
pub struct Key<'a> {
    pub(super) client: &'a Client,
    pub(super) key_id: i64,
}

impl<'a> Key<'a> {
    /// Creates a new `Key` instance for a specific key ID.
    #[inline]
    pub(super) fn new(client: &'a Client, key_id: i64) -> Self {
        Self { client, key_id }
    }

    /// Retrieves metadata about this specific API key.
    ///
    /// For security reasons, this endpoint only returns the key prefix and metadata,
    /// not the full key value.
    #[inline]
    pub async fn retrieve(&self) -> Result<models::ApiKey, Error<keys_api::GetKeyError>> {
        let params = keys_api::GetKeyParams {
            key_id: self.key_id,
        };
        execute_wrapper!(self, keys_api::get_key, params)
    }

    /// Deletes this specific API key.
    #[inline]
    pub async fn delete(
        &self,
    ) -> Result<models::ApiKeyDeleteResponse, Error<keys_api::DeleteKeyError>> {
        let params = keys_api::DeleteKeyParams {
            key_id: self.key_id,
        };
        execute_wrapper!(self, keys_api::delete_key, params)
    }
}
