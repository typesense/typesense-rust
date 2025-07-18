//! Provides access to the collection alias-related API endpoints.
//!
//! An `Alias` instance is created via the main `client.alias()` method.

use super::{Client, Error};
use std::sync::Arc;
use typesense_codegen::{
    apis::{collections_api, configuration},
    models,
};

/// Provides methods for interacting with a specific Typesense collection alias.
///
/// This struct is created by calling `client.alias()`.
pub struct Alias<'a> {
    pub(super) client: &'a Client,
    pub(super) name: &'a str,
}

impl<'a> Alias<'a> {
    /// Creates a new `Alias` instance.
    pub(super) fn new(client: &'a Client, name: &'a str) -> Self {
        Self { client, name }
    }

    /// Retrieves the details of a collection alias, including the collection it points to.
    pub async fn retrieve(
        &self,
    ) -> Result<models::CollectionAlias, Error<collections_api::GetAliasError>> {
        let params = collections_api::GetAliasParams {
            alias_name: self.name.to_string(),
        };

        self.client
            .execute(|config: Arc<configuration::Configuration>| {
                let params_for_move = params.clone();
                async move { collections_api::get_alias(&config, params_for_move).await }
            })
            .await
    }

    /// Deletes a collection alias.
    pub async fn delete(
        &self,
    ) -> Result<models::CollectionAlias, Error<collections_api::DeleteAliasError>> {
        let params = collections_api::DeleteAliasParams {
            alias_name: self.name.to_string(),
        };
        self.client
            .execute(|config: Arc<configuration::Configuration>| {
                let params_for_move = params.clone();
                async move { collections_api::delete_alias(&config, params_for_move).await }
            })
            .await
    }
}
