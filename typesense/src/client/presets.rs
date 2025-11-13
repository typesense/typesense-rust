//! Provides access to the API endpoints for managing presets.
//!
//! Presets are a set of search parameters that can be applied to a search query by using the `preset` search parameter.
//!
//! A `Presets` instance is created via the main `client.presets()` method.

use crate::{Client, Error, execute_wrapper};
use ::std::borrow::Cow;
use typesense_codegen::{apis::presets_api, models};

/// Provides methods for managing all of your Typesense presets.
///
/// This struct is created by calling `client.presets()`.
pub struct Presets<'a> {
    pub(super) client: &'a Client,
}

impl<'a> Presets<'a> {
    /// Creates a new `Presets` instance.
    #[inline]
    pub(super) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Retrieves the details of all presets.
    pub async fn retrieve(
        &self,
    ) -> Result<models::PresetsRetrieveSchema, Error<presets_api::RetrieveAllPresetsError>> {
        execute_wrapper!(self, presets_api::retrieve_all_presets)
    }

    /// Creates or updates an existing preset.
    ///
    /// # Arguments
    /// * `preset_id` - The ID of the preset to create or update.
    /// * `schema` - A `PresetUpsertSchema` object with the preset's value.
    pub async fn upsert(
        &self,
        preset_id: impl Into<Cow<'_, str>>,
        schema: models::PresetUpsertSchema<'_>,
    ) -> Result<models::PresetSchema, Error<presets_api::UpsertPresetError>> {
        let params = presets_api::UpsertPresetParams {
            preset_id: preset_id.into(),
            preset_upsert_schema: schema,
            _phantom: core::marker::PhantomData,
        };
        execute_wrapper!(self, presets_api::upsert_preset, params)
    }
}
