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
    pub async fn list_all(
        &self,
    ) -> Result<Vec<models::CollectionResponse>, Error<collections_api::GetCollectionsError>> {
        self.client
            .execute(|config: Arc<configuration::Configuration>| async move {
                collections_api::get_collections(&config).await
            })
            .await
    }

    // --- Alias-Specific Methods ---

    /// Creates or updates a collection alias.
    ///
    /// An alias is a virtual collection name that points to a real collection.
    /// Aliases are useful when you want to re-index your data in the background
    /// on a new collection and then switch your application to it without any
    /// changes to your code.
    ///
    /// # Arguments
    /// * `name` - The name of the alias to create or update.
    /// * `schema` - A `CollectionAliasSchema` pointing to the target collection.
    pub async fn upsert_alias(
        &self,
        name: &str,
        schema: models::CollectionAliasSchema,
    ) -> Result<models::CollectionAlias, Error<collections_api::UpsertAliasError>> {
        let params = collections_api::UpsertAliasParams {
            alias_name: name.to_string(),
            collection_alias_schema: Some(schema),
        };
        self.client
            .execute(|config: Arc<configuration::Configuration>| {
                let params_for_move = params.clone();
                async move { collections_api::upsert_alias(&config, params_for_move).await }
            })
            .await
    }

    /// Retrieves the details of a collection alias, including the collection it points to.
    ///
    /// # Arguments
    /// * `name` - The name of the alias to retrieve.
    pub async fn get_alias(
        &self,
        name: &str,
    ) -> Result<models::CollectionAlias, Error<collections_api::GetAliasError>> {
        let params = collections_api::GetAliasParams {
            alias_name: name.to_string(),
        };

        self.client
            .execute(|config: Arc<configuration::Configuration>| {
                let params_for_move = params.clone();
                async move { collections_api::get_alias(&config, params_for_move).await }
            })
            .await
    }

    /// Lists all aliases and the corresponding collections that they map to.
    pub async fn list_aliases(
        &self,
    ) -> Result<models::CollectionAliasesResponse, Error<collections_api::GetAliasesError>> {
        self.client
            .execute(|config: Arc<configuration::Configuration>| async move {
                collections_api::get_aliases(&config).await
            })
            .await
    }

    /// Deletes a collection alias.
    ///
    /// # Arguments
    /// * `name` - The name of the alias to delete.
    pub async fn delete_alias(
        &self,
        name: &str,
    ) -> Result<models::CollectionAlias, Error<collections_api::DeleteAliasError>> {
        let params = collections_api::DeleteAliasParams {
            alias_name: name.to_string(),
        };
        self.client
            .execute(|config: Arc<configuration::Configuration>| {
                let params_for_move = params.clone();
                async move { collections_api::delete_alias(&config, params_for_move).await }
            })
            .await
    }
}
