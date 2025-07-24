//! Provides access to the API endpoints for managing a single stemming dictionary.
//!
//! An instance of `Dictionary` is created via the `Client::stemming().dictionary()` method.

use crate::client::{Client, Error};
use std::sync::Arc;
use typesense_codegen::{
    apis::{configuration, stemming_api},
    models,
};

/// Provides methods for interacting with a specific stemming dictionary.
///
/// This struct is created by calling `client.stemming().dictionary("dictionary_id")`.
pub struct Dictionary<'a> {
    pub(super) client: &'a Client,
    pub(super) dictionary_id: &'a str,
}

impl<'a> Dictionary<'a> {
    /// Creates a new `Dictionary` instance for a specific dictionary ID.
    pub(super) fn new(client: &'a Client, dictionary_id: &'a str) -> Self {
        Self {
            client,
            dictionary_id,
        }
    }

    /// Retrieves the details of this specific stemming dictionary.
    pub async fn retrieve(
        &self,
    ) -> Result<models::StemmingDictionary, Error<stemming_api::GetStemmingDictionaryError>> {
        let params = stemming_api::GetStemmingDictionaryParams {
            dictionary_id: self.dictionary_id.to_string(),
        };
        self.client
            .execute(|config: Arc<configuration::Configuration>| {
                let params_for_move = params.clone();
                async move { stemming_api::get_stemming_dictionary(&config, params_for_move).await }
            })
            .await
    }
}
