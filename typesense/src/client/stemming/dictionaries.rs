//! Provides access to the API endpoints for managing the collection of stemming dictionaries.
//!
//! A `Dictionaries` instance is created via the `Client::stemming().dictionaries()` method.

use crate::client::{Client, Error};
use typesense_codegen::{
    apis::{configuration, stemming_api},
    models,
};

/// Provides methods for interacting with the collection of stemming dictionaries.
///
/// This struct is created by calling `client.stemming().dictionaries()`.
pub struct Dictionaries<'a> {
    pub(super) client: &'a Client,
}

impl<'a> Dictionaries<'a> {
    /// Creates a new `Dictionaries` instance.
    pub(super) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Imports a stemming dictionary from a JSONL file content.
    ///
    /// This creates or updates a dictionary with the given ID.
    ///
    /// # Arguments
    /// * `dictionary_id` - The ID to assign to the dictionary.
    /// * `dictionary_jsonl` - A string containing the word mappings in JSONL format.
    pub async fn import(
        &self,
        dictionary_id: &str,
        dictionary_jsonl: String,
    ) -> Result<String, Error<stemming_api::ImportStemmingDictionaryError>> {
        let params = stemming_api::ImportStemmingDictionaryParams {
            id: dictionary_id.to_string(),
            body: dictionary_jsonl,
        };
        self.client
            .execute(|config: configuration::Configuration| {
                let params_for_move = params.clone();
                async move { stemming_api::import_stemming_dictionary(&config, params_for_move).await }
            })
            .await
    }

    /// Retrieves a list of all available stemming dictionaries.
    pub async fn retrieve(
        &self,
    ) -> Result<
        models::ListStemmingDictionaries200Response,
        Error<stemming_api::ListStemmingDictionariesError>,
    > {
        self.client
            .execute(|config: configuration::Configuration| async move {
                stemming_api::list_stemming_dictionaries(&config).await
            })
            .await
    }
}
