//! Provides access to the collection aliases-related API endpoints.
//!
//! An `Aliases` instance is created via the main `client.aliases()` method.

use crate::{Client, Error};
use typesense_codegen::{
    apis::{collections_api, configuration},
    models,
};

/// Provides methods for interacting with Typesense collection aliases.
///
/// This struct is created by calling `client.aliases()`.
pub struct Aliases<'a> {
    pub(super) client: &'a Client,
}

impl<'a> Aliases<'a> {
    /// Creates a new `Aliases` instance.
    pub(super) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Creates or updates a collection alias.
    ///
    /// An alias is a virtual collection name that points to a real collection.
    /// Aliases are useful when you want to re-index your data in the background
    /// on a new collection and then switch your application to it without any
    /// changes to your code.
    ///
    /// # Arguments
    /// * `schema` - A `CollectionAliasSchema` pointing to the target collection.
    pub async fn upsert(
        &self,
        alias_name: &str,
        schema: models::CollectionAliasSchema,
    ) -> Result<models::CollectionAlias, Error<collections_api::UpsertAliasError>> {
        let params = collections_api::UpsertAliasParams {
            alias_name: alias_name.to_string(),
            collection_alias_schema: Some(schema),
        };
        self.client
            .execute(|config: configuration::Configuration| {
                let params_for_move = params.clone();
                async move { collections_api::upsert_alias(&config, params_for_move).await }
            })
            .await
    }

    /// Lists all aliases and the corresponding collections that they map to.
    pub async fn retrieve(
        &self,
    ) -> Result<models::CollectionAliasesResponse, Error<collections_api::GetAliasesError>> {
        self.client
            .execute(|config: configuration::Configuration| async move {
                collections_api::get_aliases(&config).await
            })
            .await
    }
}
