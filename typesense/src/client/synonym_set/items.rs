//! Provides access to the API endpoints for managing items of a synonym set.
//!
//! Synonym sets allows you to define search terms that should be considered equivalent.
//!
//! A `SynonymSetItems` instance is created via the main `client.synonym_set("synonym_set_name").items()` method.

use ::std::borrow::Cow;

use crate::{Client, Error, execute_wrapper};
use typesense_codegen::{apis::synonyms_api, models};

/// Provides methods for managing items of a synonym set.
///
/// This struct is created by calling `client.synonym_set("synonym_set_name").items()`.
pub struct SynonymSetItems<'a> {
    pub(super) client: &'a Client,
    pub(super) synonym_set_name: &'a str,
}

impl<'a> SynonymSetItems<'a> {
    /// Creates a new `SynonymSetItems` instance.
    #[inline]
    pub(super) fn new(client: &'a Client, synonym_set_name: &'a str) -> Self {
        Self {
            client,
            synonym_set_name,
        }
    }

    /// Retrieves all the items of this synonym set.
    pub async fn retrieve(
        &self,
    ) -> Result<Vec<models::SynonymItemSchema>, Error<synonyms_api::RetrieveSynonymSetItemsError>>
    {
        let params = synonyms_api::RetrieveSynonymSetItemsParams {
            synonym_set_name: self.synonym_set_name.into(),
        };
        execute_wrapper!(self, synonyms_api::retrieve_synonym_set_items, params)
    }

    /// Creates or updates an existing item of a synonym set.
    ///
    /// # Arguments
    /// * `item_id` - The id of the synonym set item to create or update.
    /// * `schema` - A `SynonymItemUpsertSchema` object.
    pub async fn upsert(
        &self,
        item_id: impl Into<Cow<'_, str>>,
        schema: models::SynonymItemUpsertSchema<'_>,
    ) -> Result<models::SynonymItemSchema, Error<synonyms_api::UpsertSynonymSetItemError>> {
        let params = synonyms_api::UpsertSynonymSetItemParams {
            item_id: item_id.into(),
            synonym_set_name: self.synonym_set_name.into(),
            synonym_item_upsert_schema: schema,
        };
        execute_wrapper!(self, synonyms_api::upsert_synonym_set_item, params)
    }
}
