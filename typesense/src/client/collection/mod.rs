//! Provides access to the collection and alias-related API endpoints.
//!
//! A `Collections` instance is created via the main `client.collections()` method.

mod document;
mod documents;
use crate::{Client, Error};

use serde::{Serialize, de::DeserializeOwned};
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
    pub(super) collection_name: String,
    pub(super) _phantom: std::marker::PhantomData<T>,
}

impl<'a, T> Collection<'a, T>
where
    T: DeserializeOwned + Serialize + Send + Sync,
{
    /// Creates a new `Collection` instance.
    pub(super) fn new(client: &'a Client, collection_name: String) -> Self {
        Self {
            client,
            collection_name,
            _phantom: std::marker::PhantomData,
        }
    }

    /// Provides access to the document-related API endpoints for a specific collection.
    pub fn documents(&'a self) -> documents::Documents<'a, T> {
        documents::Documents::new(self.client, self.collection_name.to_owned())
    }

    /// Provides access to the API endpoints for a single document within a Typesense collection.
    pub fn document(&'a self, document_id: impl Into<String>) -> document::Document<'a, T> {
        document::Document::new(
            self.client,
            self.collection_name.to_owned(),
            document_id.into(),
        )
    }

    /// Retrieves the details of a collection, given its name.
    pub async fn retrieve(
        &self,
    ) -> Result<models::CollectionResponse, Error<collections_api::GetCollectionError>> {
        let params = collections_api::GetCollectionParams {
            collection_name: self.collection_name.to_owned(),
        };

        self.client
            .execute(|config: configuration::Configuration| {
                let params_for_move = params.clone();
                async move { collections_api::get_collection(&config, params_for_move).await }
            })
            .await
    }

    /// Permanently drops a collection.
    pub async fn delete(
        &self,
    ) -> Result<models::CollectionResponse, Error<collections_api::DeleteCollectionError>> {
        let params = collections_api::DeleteCollectionParams {
            collection_name: self.collection_name.to_owned(),
        };
        self.client
            .execute(|config: configuration::Configuration| {
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
            collection_name: self.collection_name.to_owned(),
            collection_update_schema: update_schema,
        };
        self.client
            .execute(|config: configuration::Configuration| {
                let params_for_move = params.clone();
                async move { collections_api::update_collection(&config, params_for_move).await }
            })
            .await
    }
}
