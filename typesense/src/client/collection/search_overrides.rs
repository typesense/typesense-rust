//! Provides access to the API endpoints for managing a collection's search overrides.
//!
//! An instance of `SearchOverrides` is created via the `Client::collection("collection_name").search_overrides()` method.

use crate::{Client, Error};
use std::sync::Arc;
use typesense_codegen::{
    apis::{configuration, documents_api},
    models,
};

/// Provides methods for interacting with a collection of search overrides.
///
/// This struct is created by calling `client.collection("collection_name").search_overrides()`.
pub struct SearchOverrides<'a> {
    pub(super) client: &'a Client,
    pub(super) collection_name: &'a str,
}

impl<'a> SearchOverrides<'a> {
    /// Creates a new `Overrides` instance.
    pub(super) fn new(client: &'a Client, collection_name: &'a str) -> Self {
        Self {
            client,
            collection_name,
        }
    }

    /// Creates or updates a search override.
    ///
    /// Overrides allow you to rank certain documents higher than others for specific queries.
    ///
    /// # Arguments
    /// * `override_id` - The ID of the search override to create or update.
    /// * `schema` - The `SearchOverrideSchema` defining the override rules.
    pub async fn upsert(
        &self,
        override_id: &str,
        schema: models::SearchOverrideSchema,
    ) -> Result<models::SearchOverride, Error<documents_api::UpsertSearchOverrideError>> {
        let params = documents_api::UpsertSearchOverrideParams {
            collection_name: self.collection_name.to_string(),
            override_id: override_id.to_string(),
            search_override_schema: schema,
        };
        self.client
            .execute(|config: Arc<configuration::Configuration>| {
                let params_for_move = params.clone();
                async move { documents_api::upsert_search_override(&config, params_for_move).await }
            })
            .await
    }

    /// Lists all search overrides associated with the collection.
    pub async fn list(
        &self,
    ) -> Result<models::SearchOverridesResponse, Error<documents_api::GetSearchOverridesError>>
    {
        let params = documents_api::GetSearchOverridesParams {
            collection_name: self.collection_name.to_string(),
        };

        self.client
            .execute(|config: Arc<configuration::Configuration>| {
                let params_for_move = params.clone();
                async move { documents_api::get_search_overrides(&config, params_for_move).await }
            })
            .await
    }
}
