//! Provides access to the API endpoints for managing a specific synonym set.
//!
//! Synonym sets allows you to define search terms that should be considered equivalent.
//!
//! A `SynonymSet` instance is created via the main `client.synonym_set("synonym_set_name")` method.

mod item;
mod items;

use crate::{Client, Error, execute_wrapper};
use item::SynonymSetItem;
use items::SynonymSetItems;
use typesense_codegen::{apis::synonyms_api, models};

/// Provides methods for  managing a specific synonym set.
///
/// This struct is created by calling `client.synonym_set("synonym_set_name")`.
pub struct SynonymSet<'a> {
    pub(super) client: &'a Client,
    pub(super) synonym_set_name: &'a str,
}

impl<'a> SynonymSet<'a> {
    /// Creates a new `SynonymSet` instance.
    #[inline]
    pub(super) fn new(client: &'a Client, synonym_set_name: &'a str) -> Self {
        Self {
            client,
            synonym_set_name,
        }
    }

    /// Provides access to the items of this synonym set.
    #[inline]
    pub fn items(&self) -> SynonymSetItems<'_> {
        SynonymSetItems::new(self.client, self.synonym_set_name)
    }

    /// Provides access to this specific item of this synonym set.
    #[inline]
    pub fn item(&self, item_id: &'a str) -> SynonymSetItem<'a> {
        SynonymSetItem::new(self.client, self.synonym_set_name, item_id)
    }

    /// Retrieves the details of this synonym set.
    pub async fn retrieve(
        &self,
    ) -> Result<models::SynonymSetSchema, Error<synonyms_api::RetrieveSynonymSetError>> {
        let params = synonyms_api::RetrieveSynonymSetParams {
            synonym_set_name: self.synonym_set_name.into(),
        };
        execute_wrapper!(self, synonyms_api::retrieve_synonym_set, params)
    }

    /// Delete this synonym set.
    pub async fn delete(
        &self,
    ) -> Result<models::SynonymSetDeleteSchema, Error<synonyms_api::DeleteSynonymSetError>> {
        let params = synonyms_api::DeleteSynonymSetParams {
            synonym_set_name: self.synonym_set_name.into(),
        };
        execute_wrapper!(self, synonyms_api::delete_synonym_set, params)
    }
}
