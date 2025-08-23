//! Provides access to the API endpoints for managing a collection's search synonyms.
//!
//! An instance of `Synonyms` is created via the `client.collection("collection_name").synonyms()` method.

use crate::{Client, Error};
use typesense_codegen::{
    apis::{configuration, synonyms_api},
    models,
};

/// Provides methods for interacting with a collection of search synonyms.
///
/// This struct is created by calling `client.collection("collection_name").synonyms()`.
pub struct Synonyms<'a> {
    pub(super) client: &'a Client,
    pub(super) collection_name: &'a str,
}

impl<'a> Synonyms<'a> {
    /// Creates a new `Synonyms` instance.
    pub(super) fn new(client: &'a Client, collection_name: &'a str) -> Self {
        Self {
            client,
            collection_name,
        }
    }

    /// Creates or updates a search synonym.
    ///
    /// # Arguments
    /// * `synonym_id` - The ID of the search synonym to create or update.
    /// * `schema` - A `SearchSynonymSchema` object defining the equivalent terms.
    pub async fn upsert(
        &self,
        synonym_id: &str,
        schema: models::SearchSynonymSchema,
    ) -> Result<models::SearchSynonym, Error<synonyms_api::UpsertSearchSynonymError>> {
        let params = synonyms_api::UpsertSearchSynonymParams {
            collection_name: self.collection_name.to_string(),
            synonym_id: synonym_id.to_string(),
            search_synonym_schema: schema,
        };
        self.client
            .execute(|config: configuration::Configuration| {
                let params_for_move = params.clone();
                async move { synonyms_api::upsert_search_synonym(&config, params_for_move).await }
            })
            .await
    }

    /// Retrieve all search synonyms associated with the collection.
    pub async fn retrieve(
        &self,
    ) -> Result<models::SearchSynonymsResponse, Error<synonyms_api::GetSearchSynonymsError>> {
        let params = synonyms_api::GetSearchSynonymsParams {
            collection_name: self.collection_name.to_string(),
        };

        self.client
            .execute(|config: configuration::Configuration| {
                let params_for_move = params.clone();
                async move { synonyms_api::get_search_synonyms(&config, params_for_move).await }
            })
            .await
    }
}
