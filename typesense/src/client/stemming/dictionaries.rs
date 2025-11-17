//! Provides access to the API endpoints for managing the collection of stemming dictionaries.
//!
//! A `Dictionaries` instance is created via the `client.stemming().dictionaries()` method.

use crate::{
    client::{Client, Error},
    execute_wrapper,
};
use ::std::borrow::Cow;
use typesense_codegen::{apis::stemming_api, models};

/// Provides methods for interacting with the collection of stemming dictionaries.
///
/// This struct is created by calling `client.stemming().dictionaries()`.
pub struct Dictionaries<'a> {
    pub(super) client: &'a Client,
}

impl<'a> Dictionaries<'a> {
    /// Creates a new `Dictionaries` instance.
    #[inline]
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
        dictionary_id: impl Into<Cow<'_, str>>,
        dictionary_jsonl: impl Into<Cow<'_, str>>,
    ) -> Result<String, Error<stemming_api::ImportStemmingDictionaryError>> {
        let params = stemming_api::ImportStemmingDictionaryParams {
            id: dictionary_id.into(),
            body: dictionary_jsonl.into(),
        };
        execute_wrapper!(self, stemming_api::import_stemming_dictionary, params)
    }

    /// Retrieves a list of all stemming dictionaries.
    pub async fn retrieve(
        &self,
    ) -> Result<
        models::ListStemmingDictionaries200Response,
        Error<stemming_api::ListStemmingDictionariesError>,
    > {
        execute_wrapper!(self, stemming_api::list_stemming_dictionaries)
    }
}
