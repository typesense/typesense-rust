//! Provides access to the API endpoints for managing a single preset.
//!
//! A `Preset` instance is created via the main `client.preset("preset_id")` method.

use crate::{Client, Error, execute_wrapper};
use typesense_codegen::{apis::presets_api, models};

/// Provides methods for managing a single Typesense preset.
///
/// This struct is created by calling `client.preset("preset_id")`.
pub struct Preset<'a> {
    pub(super) client: &'a Client,
    pub(super) preset_id: &'a str,
}

impl<'a> Preset<'a> {
    /// Creates a new `Preset` instance.
    #[inline]
    pub(super) fn new(client: &'a Client, preset_id: &'a str) -> Self {
        Self { client, preset_id }
    }

    /// Retrieves the details of a preset, given its Id.
    pub async fn retrieve(
        &self,
    ) -> Result<models::PresetSchema<'static>, Error<presets_api::RetrievePresetError>> {
        let params = presets_api::RetrievePresetParams {
            preset_id: self.preset_id.into(),
            _phantom: core::marker::PhantomData,
        };
        execute_wrapper!(self, presets_api::retrieve_preset, params)
    }

    /// Permanently deletes a preset, given its Id.
    pub async fn delete(
        &self,
    ) -> Result<models::PresetDeleteSchema<'static>, Error<presets_api::DeletePresetError>> {
        let params = presets_api::DeletePresetParams {
            preset_id: self.preset_id.into(),
            _phantom: core::marker::PhantomData,
        };
        execute_wrapper!(self, presets_api::delete_preset, params)
    }
}
