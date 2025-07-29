//! Provides access to the collection and alias-related API endpoints.
//!
//! A `Collections` instance is created via the main `Client::collections()` method.

mod document;
mod documents;
mod search_override;
mod search_overrides;
mod synonym;
mod synonyms;
use crate::{Client, Error};

use search_override::SearchOverride;
use search_overrides::SearchOverrides;
use serde::{de::DeserializeOwned, Serialize};
use std::sync::Arc;
use synonym::Synonym;
use synonyms::Synonyms;
use typesense_codegen::{
    apis::{collections_api, configuration},
    models,
};

/// Provides methods for interacting with a Typesense collection.
///
/// This struct is created by calling `client.collection("collection_name")`.
pub struct Collection<'a, T = serde_json::Value>
where
    T: DeserializeOwned + Serialize + Send + Sync,
{
    pub(super) client: &'a Client,
    pub(super) collection_name: &'a str,
    pub(super) _phantom: std::marker::PhantomData<T>,
}

impl<'a, T> Collection<'a, T>
where
    T: DeserializeOwned + Serialize + Send + Sync,
{
    /// Creates a new `Collection` instance.
    pub(super) fn new(client: &'a Client, collection_name: &'a str) -> Self {
        Self {
            client,
            collection_name,
            _phantom: std::marker::PhantomData,
        }
    }
    // --- Documents Accessors ---

    /// Provides access to the document-related API endpoints for a specific collection.
    pub fn documents(&'a self) -> documents::Documents<'a, T> {
        documents::Documents::new(self.client, self.collection_name)
    }

    /// Provides access to the API endpoints for a single document within a Typesense collection.
    pub fn document(&'a self, document_id: &'a str) -> document::Document<'a, T> {
        document::Document::new(self.client, self.collection_name, document_id)
    }

    // --- Overrides Accessors ---

    /// Provides access to endpoints for managing the collection of search overrides.
    ///
    /// Example: `client.collection("collection_name").search_overrides().retrieve().await`
    pub fn search_overrides(&self) -> SearchOverrides<'a> {
        SearchOverrides::new(self.client, self.collection_name)
    }

    /// Provides access to endpoints for managing a single search override.
    ///
    /// # Arguments
    /// * `override_id` - The ID of the search override to manage.
    ///
    /// Example: `client.collection("collection_name").search_override("...").retrieve().await`
    pub fn search_override(&self, override_id: &'a str) -> SearchOverride<'a> {
        SearchOverride::new(self.client, self.collection_name, override_id)
    }

    // --- Synonym Accessors ---

    /// Provides access to endpoints for managing the collection of search synonyms.
    ///
    /// Example: `client.collection("collection_name").synonyms().retrieve().await`
    pub fn synonyms(&self) -> Synonyms<'a> {
        Synonyms::new(self.client, self.collection_name)
    }

    /// Provides access to endpoints for managing a single search synonym.
    ///
    /// # Arguments
    /// * `synonym_id` - The ID of the search synonym to manage.
    ///
    /// Example: `client.collection("collection_name").synonym("synonym_id").delete().await`
    pub fn synonym(&self, synonym_id: &'a str) -> Synonym<'a> {
        Synonym::new(self.client, self.collection_name, synonym_id)
    }

    // ---  Methods for this collection ---

    /// Retrieves the details of a collection, given its name.
    pub async fn retrieve(
        &self,
    ) -> Result<models::CollectionResponse, Error<collections_api::GetCollectionError>> {
        let params = collections_api::GetCollectionParams {
            collection_name: self.collection_name.to_string(),
        };

        self.client
            .execute(|config: Arc<configuration::Configuration>| {
                let params_for_move = params.clone();
                async move { collections_api::get_collection(&config, params_for_move).await }
            })
            .await
    }

    /// Permanently drops a collection.
    ///
    /// This action cannot be undone. For large collections, this might have an
    /// impact on read latencies during the delete operation.
    pub async fn delete(
        &self,
    ) -> Result<models::CollectionResponse, Error<collections_api::DeleteCollectionError>> {
        let params = collections_api::DeleteCollectionParams {
            collection_name: self.collection_name.to_string(),
        };
        self.client
            .execute(|config: Arc<configuration::Configuration>| {
                let params_for_move = params.clone();
                async move { collections_api::delete_collection(&config, params_for_move).await }
            })
            .await
    }

    /// Updates a collection's schema to modify the fields and their types.
    ///
    /// # Arguments
    /// * `update_schema` - A `CollectionUpdateSchema` object describing the fields to update.
    pub async fn update(
        &self,
        update_schema: models::CollectionUpdateSchema,
    ) -> Result<models::CollectionUpdateSchema, Error<collections_api::UpdateCollectionError>> {
        let params = collections_api::UpdateCollectionParams {
            collection_name: self.collection_name.to_string(),
            collection_update_schema: update_schema,
        };
        self.client
            .execute(|config: Arc<configuration::Configuration>| {
                let params_for_move = params.clone();
                async move { collections_api::update_collection(&config, params_for_move).await }
            })
            .await
    }
}
