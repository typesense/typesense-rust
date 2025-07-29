//! Provides access to the API endpoints for managing a single search override.
//!
//! An instance of `SearchOverride` is created via the `Client::collection("collection_name").search_override("search_override_id")` method.

use crate::{Client, Error};
use std::sync::Arc;
use typesense_codegen::{
    apis::{configuration, documents_api},
    models,
};

/// Provides methods for interacting with a specific search override.
///
/// This struct is created by calling `client.collection("colelction_name").search_override("override_id")`.
pub struct SearchOverride<'a> {
    pub(super) client: &'a Client,
    pub(super) collection_name: &'a str,
    pub(super) override_id: &'a str,
}

impl<'a> SearchOverride<'a> {
    /// Creates a new `Override` instance for a specific override ID.
    pub(super) fn new(client: &'a Client, collection_name: &'a str, override_id: &'a str) -> Self {
        Self {
            client,
            collection_name,
            override_id,
        }
    }

    /// Retrieves this specific search override.
    pub async fn retrieve(
        &self,
    ) -> Result<models::SearchOverride, Error<documents_api::GetSearchOverrideError>> {
        let params = documents_api::GetSearchOverrideParams {
            collection_name: self.collection_name.to_string(),
            override_id: self.override_id.to_string(),
        };

        self.client
            .execute(|config: Arc<configuration::Configuration>| {
                let params_for_move = params.clone();
                async move { documents_api::get_search_override(&config, params_for_move).await }
            })
            .await
    }

    /// Deletes this specific search override.
    pub async fn delete(
        &self,
    ) -> Result<models::SearchOverrideDeleteResponse, Error<documents_api::DeleteSearchOverrideError>>
    {
        let params = documents_api::DeleteSearchOverrideParams {
            collection_name: self.collection_name.to_string(),
            override_id: self.override_id.to_string(),
        };

        self.client
            .execute(|config: Arc<configuration::Configuration>| {
                let params_for_move = params.clone();
                async move { documents_api::delete_search_override(&config, params_for_move).await }
            })
            .await
    }
}
