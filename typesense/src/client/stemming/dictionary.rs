//! Provides access to the API endpoints for managing a single stemming dictionary.
//!
//! An instance of `Dictionary` is created via the `client.stemming().dictionary()` method.

use crate::{
    client::{Client, Error},
    execute_wrapper,
};
use typesense_codegen::{apis::stemming_api, models};

/// Provides methods for interacting with a specific stemming dictionary.
///
/// This struct is created by calling `client.stemming().dictionary("dictionary_id")`.
pub struct Dictionary<'a> {
    pub(super) client: &'a Client,
    pub(super) dictionary_id: &'a str,
}

impl<'a> Dictionary<'a> {
    /// Creates a new `Dictionary` instance for a specific dictionary ID.
    #[inline]
    pub(super) fn new(client: &'a Client, dictionary_id: &'a str) -> Self {
        Self {
            client,
            dictionary_id,
        }
    }

    /// Retrieves the details of this specific stemming dictionary.
    pub async fn retrieve(
        &self,
    ) -> Result<models::StemmingDictionary<'static>, Error<stemming_api::GetStemmingDictionaryError>>
    {
        let params = stemming_api::GetStemmingDictionaryParams {
            dictionary_id: self.dictionary_id.into(),
            _phantom: core::marker::PhantomData,
        };
        execute_wrapper!(self, stemming_api::get_stemming_dictionary, params)
    }
}
