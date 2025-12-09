//! Provides access to the API endpoints for managing a specific curation set.
//!
//! Curation sets allow you to include or exclude specific documents for a given query.
//!
//! A `CurationSet` instance is created via the main `client.curation_set("curation_set_name")` method.

mod item;
mod items;

use crate::{Client, Error, execute_wrapper};
use item::CurationSetItem;
use items::CurationSetItems;
use typesense_codegen::{
    apis::curation_sets_api::{self, RetrieveCurationSetParams},
    models,
};

/// Provides methods for  managing a specific curation set.
///
/// This struct is created by calling `client.curation_set("curation_set_name")`.
pub struct CurationSet<'a> {
    pub(super) client: &'a Client,
    pub(super) curation_set_name: &'a str,
}

impl<'a> CurationSet<'a> {
    /// Creates a new `CurationSet` instance.
    #[inline]
    pub(super) fn new(client: &'a Client, curation_set_name: &'a str) -> Self {
        Self {
            client,
            curation_set_name,
        }
    }

    /// Provides access to the items of this curation set.
    #[inline]
    pub fn items(&self) -> CurationSetItems<'_> {
        CurationSetItems::new(self.client, self.curation_set_name)
    }

    /// Provides access to this specific item of this curation set.
    #[inline]
    pub fn item(&self, item_id: &'a str) -> CurationSetItem<'a> {
        CurationSetItem::new(self.client, self.curation_set_name, item_id)
    }

    /// Retrieves the details of this curation set.
    pub async fn retrieve(
        &self,
    ) -> Result<models::CurationSetSchema, Error<curation_sets_api::RetrieveCurationSetError>> {
        let params = RetrieveCurationSetParams {
            curation_set_name: self.curation_set_name.into(),
        };
        execute_wrapper!(self, curation_sets_api::retrieve_curation_set, params)
    }

    /// Delete this curation set.
    pub async fn delete(
        &self,
    ) -> Result<models::CurationSetDeleteSchema, Error<curation_sets_api::DeleteCurationSetError>>
    {
        let params = curation_sets_api::DeleteCurationSetParams {
            curation_set_name: self.curation_set_name.into(),
        };
        execute_wrapper!(self, curation_sets_api::delete_curation_set, params)
    }
}
