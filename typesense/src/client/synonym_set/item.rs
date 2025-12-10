//! Provides access to the API endpoints for managing a synonym set item.
//!
//! Synonym sets allows you to define search terms that should be considered equivalent.
//!
//! A `SynonymSetItem` instance is created via the main `client.synonym_set("synonym_set_name").item("item_id")` method.

use crate::{Client, Error, execute_wrapper};
use typesense_codegen::{apis::synonyms_api, models};

/// Provides methods for managing a synonym set item.
///
/// This struct is created by calling `client.synonym_set("synonym_set_name").item("item_id")`.
pub struct SynonymSetItem<'a> {
    pub(super) client: &'a Client,
    pub(super) synonym_set_name: &'a str,
    pub(super) item_id: &'a str,
}

impl<'a> SynonymSetItem<'a> {
    /// Creates a new `SynonymSetItem` instance.
    #[inline]
    pub(super) fn new(client: &'a Client, synonym_set_name: &'a str, item_id: &'a str) -> Self {
        Self {
            client,
            synonym_set_name,
            item_id,
        }
    }

    /// Retrieve this synonym set item.
    pub async fn retrieve(
        &self,
    ) -> Result<models::SynonymItemSchema, Error<synonyms_api::RetrieveSynonymSetItemError>> {
        let params = synonyms_api::RetrieveSynonymSetItemParams {
            synonym_set_name: self.synonym_set_name.into(),
            item_id: self.item_id.into(),
        };
        execute_wrapper!(self, synonyms_api::retrieve_synonym_set_item, params)
    }

    /// Delete this synonym set item.
    pub async fn delete(
        &self,
    ) -> Result<models::SynonymItemDeleteSchema, Error<synonyms_api::DeleteSynonymSetItemError>>
    {
        let params = synonyms_api::DeleteSynonymSetItemParams {
            synonym_set_name: self.synonym_set_name.into(),
            item_id: self.item_id.into(),
        };
        execute_wrapper!(self, synonyms_api::delete_synonym_set_item, params)
    }
}
