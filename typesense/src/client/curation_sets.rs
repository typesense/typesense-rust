//! Provides access to the API endpoints for managing curation sets.
//!
//! Curation sets allow you to include or exclude specific documents for a given query.
//!
//! A `CurationSets` instance is created via the main `client.curation_sets()` method.

use crate::{Client, Error, execute_wrapper};
use ::std::borrow::Cow;
use typesense_codegen::{apis::curation_sets_api, models};

/// Provides methods for managing all of your Typesense curation sets.
///
/// This struct is created by calling `client.curation_sets()`.
pub struct CurationSets<'a> {
    pub(super) client: &'a Client,
}

impl<'a> CurationSets<'a> {
    /// Creates a new `CurationSets` instance.
    #[inline]
    pub(super) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Retrieves the details of all curation sets.
    pub async fn retrieve(
        &self,
    ) -> Result<Vec<models::CurationSetSchema>, Error<curation_sets_api::RetrieveCurationSetsError>>
    {
        execute_wrapper!(self, curation_sets_api::retrieve_curation_sets)
    }

    /// Creates or updates an existing curation set.
    ///
    /// # Arguments
    /// * `name` - The name of the curation set to create or update.
    /// * `schema` - A `CurationSetCreateSchema` object.
    pub async fn upsert(
        &self,
        name: impl Into<Cow<'_, str>>,
        schema: models::CurationSetCreateSchema<'_>,
    ) -> Result<models::CurationSetSchema, Error<curation_sets_api::UpsertCurationSetError>> {
        let params = curation_sets_api::UpsertCurationSetParams {
            curation_set_name: name.into(),
            curation_set_create_schema: schema,
        };
        execute_wrapper!(self, curation_sets_api::upsert_curation_set, params)
    }
}
