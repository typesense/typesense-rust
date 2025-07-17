//! Provides access to the API endpoints for managing a single search synonym.
//!
//! An instance of `Synonym` is created via the `client.collection("collection_name").synonym("synonym_id")` method.

use super::{Client, Error};
use std::sync::Arc;
use typesense_codegen::{
    apis::{configuration, synonyms_api},
    models,
};

/// Provides methods for interacting with a specific search synonym.
///
/// This struct is created by calling `client.collection("collection_name").synonym("synonym_id")`.
pub struct Synonym<'a> {
    pub(super) client: &'a Client,
    pub(super) collection_name: &'a str,
    pub(super) synonym_id: &'a str,
}

impl<'a> Synonym<'a> {
    /// Creates a new `Synonym` instance for a specific synonym ID.
    pub(super) fn new(client: &'a Client, collection_name: &'a str, synonym_id: &'a str) -> Self {
        Self {
            client,
            collection_name,
            synonym_id,
        }
    }

    /// Retrieves this specific search synonym.
    pub async fn get(
        &self,
    ) -> Result<models::SearchSynonym, Error<synonyms_api::GetSearchSynonymError>> {
        let params = synonyms_api::GetSearchSynonymParams {
            collection_name: self.collection_name.to_string(),
            synonym_id: self.synonym_id.to_string(),
        };

        self.client
            .execute(|config: Arc<configuration::Configuration>| {
                let params_for_move = params.clone();
                async move { synonyms_api::get_search_synonym(&config, params_for_move).await }
            })
            .await
    }

    /// Deletes this specific search synonym.
    pub async fn delete(
        &self,
    ) -> Result<models::SearchSynonymDeleteResponse, Error<synonyms_api::DeleteSearchSynonymError>>
    {
        let params = synonyms_api::DeleteSearchSynonymParams {
            collection_name: self.collection_name.to_string(),
            synonym_id: self.synonym_id.to_string(),
        };

        self.client
            .execute(|config: Arc<configuration::Configuration>| {
                let params_for_move = params.clone();
                async move { synonyms_api::delete_search_synonym(&config, params_for_move).await }
            })
            .await
    }
}
