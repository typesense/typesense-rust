//! Provides access to the API endpoints for managing synonym sets.
//!
//! Synonym sets allows you to define search terms that should be considered equivalent.
//!
//! A `SynonymSets` instance is created via the main `client.synonym_sets()` method.

use crate::{Client, Error, execute_wrapper};
use ::std::borrow::Cow;
use typesense_codegen::{apis::synonyms_api, models};

/// Provides methods for managing all of your Typesense synonym sets.
///
/// This struct is created by calling `client.synonym_sets()`.
pub struct SynonymSets<'a> {
    pub(super) client: &'a Client,
}

impl<'a> SynonymSets<'a> {
    /// Creates a new `SynonymSets` instance.
    #[inline]
    pub(super) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Retrieves the details of all synonym sets.
    pub async fn retrieve(
        &self,
    ) -> Result<Vec<models::SynonymSetSchema>, Error<synonyms_api::RetrieveSynonymSetsError>> {
        execute_wrapper!(self, synonyms_api::retrieve_synonym_sets)
    }

    /// Creates or updates an existing synonym set.
    ///
    /// # Arguments
    /// * `name` - The name of the synonym set to create or update.
    /// * `schema` - A `SynonymSetCreateSchema` object.
    pub async fn upsert(
        &self,
        name: impl Into<Cow<'_, str>>,
        schema: models::SynonymSetCreateSchema,
    ) -> Result<models::SynonymSetSchema, Error<synonyms_api::UpsertSynonymSetError>> {
        let params = synonyms_api::UpsertSynonymSetParams {
            synonym_set_name: name.into(),
            synonym_set_create_schema: schema,
        };
        execute_wrapper!(self, synonyms_api::upsert_synonym_set, params)
    }
}
