//! Provides access to the API endpoints for managing presets.
//!
//! Presets are a set of search parameters that can be applied to a search query by using the `preset` search parameter.
//!
//! A `Presets` instance is created via the main `Client::presets()` method.

use crate::{Client, Error};
use std::sync::Arc;
use typesense_codegen::{
    apis::{configuration, presets_api},
    models,
};

/// Provides methods for managing all of your Typesense presets.
///
/// This struct is created by calling `client.presets()`.
pub struct Presets<'a> {
    pub(super) client: &'a Client,
}

impl<'a> Presets<'a> {
    /// Creates a new `Presets` instance.
    pub(super) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Retrieves the details of all presets.
    pub async fn retrieve(
        &self,
    ) -> Result<models::PresetsRetrieveSchema, Error<presets_api::RetrieveAllPresetsError>> {
        self.client
            .execute(|config: Arc<configuration::Configuration>| async move {
                presets_api::retrieve_all_presets(&config).await
            })
            .await
    }

    /// Creates or updates an existing preset.
    ///
    /// # Arguments
    /// * `preset_id` - The ID of the preset to create or update.
    /// * `schema` - A `PresetUpsertSchema` object with the preset's value.
    pub async fn upsert(
        &self,
        preset_id: &str,
        schema: models::PresetUpsertSchema,
    ) -> Result<models::PresetSchema, Error<presets_api::UpsertPresetError>> {
        let params = presets_api::UpsertPresetParams {
            preset_id: preset_id.to_string(),
            preset_upsert_schema: schema,
        };
        self.client
            .execute(|config: Arc<configuration::Configuration>| {
                let params_for_move = params.clone();
                async move { presets_api::upsert_preset(&config, params_for_move).await }
            })
            .await
    }
}
