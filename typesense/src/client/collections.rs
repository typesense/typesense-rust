//! Provides access to the collection and alias-related API endpoints.
//!
//! A `Collections` instance is created via the main `client.collections()` method.

use super::{Client, Error};
use std::sync::Arc;
use typesense_codegen::{
    apis::{collections_api, configuration},
    models,
};

/// Provides methods for interacting with Typesense collections and aliases.
///
/// This struct is created by calling `client.collections()`.
pub struct Collections<'a> {
    pub(super) client: &'a Client,
}

impl<'a> Collections<'a> {
    /// Creates a new `Collection` instance
    pub(super) fn new(client: &'a Client) -> Self {
        Self { client }
    }
    // --- Collection-Specific Methods ---

    /// Creates a new collection with the given schema.
    ///
    /// When a collection is created, you give it a name and describe the fields
    /// that will be indexed from the documents added to the collection.
    ///
    /// # Arguments
    /// * `schema` - A `CollectionSchema` object describing the collection to be created.
    pub async fn create(
        &self,
        schema: models::CollectionSchema,
    ) -> Result<models::CollectionResponse, Error<collections_api::CreateCollectionError>> {
        let params = collections_api::CreateCollectionParams {
            collection_schema: schema,
        };

        self.client
            .execute(|config: Arc<configuration::Configuration>| {
                let params_for_move = params.clone();
                async move { collections_api::create_collection(&config, params_for_move).await }
            })
            .await
    }

    /// Returns a summary of all collections in the Typesense cluster.
    ///
    /// The collections are returned sorted by creation date, with the most
    /// recent collections appearing first.
    pub async fn retrieve(
        &self,
    ) -> Result<Vec<models::CollectionResponse>, Error<collections_api::GetCollectionsError>> {
        self.client
            .execute(|config: Arc<configuration::Configuration>| async move {
                collections_api::get_collections(&config).await
            })
            .await
    }
}
