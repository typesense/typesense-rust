//! Provides access to the API endpoints for managing the collection of API keys.
//!
//! An `Keys` instance is created via the `client.keys()` method.

use crate::{
    Client, Error, execute_wrapper,
    models::{self, ScopedKeyParameters},
};
use base64::{Engine, engine::general_purpose::STANDARD as Base64Engine};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use typesense_codegen::apis::keys_api;

/// Provides methods for managing a collection of Typesense API keys.
///
/// This struct is created by calling `client.keys()`.
pub struct Keys<'c> {
    pub(super) client: &'c Client,
}

impl<'c> Keys<'c> {
    /// Creates a new `Keys` instance.
    #[inline]
    pub(super) fn new(client: &'c Client) -> Self {
        Self { client }
    }

    /// Creates a new API key with fine-grained access control.
    ///
    /// You can restrict access on a per-collection and per-action level.
    /// The full, unhashed key is only returned on creation.
    ///
    /// # Arguments
    /// * `schema` - An `ApiKeySchema` object describing the key's permissions.
    #[inline]
    pub async fn create(
        &self,
        schema: models::ApiKeySchema<'_>,
    ) -> Result<models::ApiKey, Error<keys_api::CreateKeyError>> {
        let params = keys_api::CreateKeyParams {
            api_key_schema: Some(schema),
            _phantom: core::marker::PhantomData,
        };
        execute_wrapper!(self, keys_api::create_key, params)
    }

    /// Lists all API keys and their metadata.
    #[inline]
    pub async fn retrieve(&self) -> Result<models::ApiKeysResponse, Error<keys_api::GetKeysError>> {
        execute_wrapper!(self, keys_api::get_keys)
    }

    /// Generate a scoped search API key that can have embedded search parameters in them.
    ///
    /// More info [here](https://typesense.org/docs/latest/api/api-keys.html#generate-scoped-search-key).
    pub fn generate_scoped_search_key(
        &self,
        key: impl AsRef<str>,
        params: &ScopedKeyParameters<'_>,
    ) -> anyhow::Result<String> {
        let params = serde_json::to_string(params)?;

        let mut mac = Hmac::<Sha256>::new_from_slice(key.as_ref().as_bytes())?;
        mac.update(params.as_bytes());
        let result = mac.finalize();
        let digest = Base64Engine.encode(result.into_bytes());

        let key_prefix = &key.as_ref()[0..4];
        let raw_scoped_key = format!("{}{}{}", digest, key_prefix, params);

        Ok(Base64Engine.encode(raw_scoped_key.as_bytes()))
    }
}
