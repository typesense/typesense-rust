//! Provides access to the API endpoints for managing a curation set item.
//!
//! Curation sets allow you to include or exclude specific documents for a given query.
//!
//! A `CurationSetItem` instance is created via the main `client.curation_set("curation_set_name").item("item_id")` method.
use crate::{Client, Error, execute_wrapper};
use typesense_codegen::{
    apis::curation_sets_api::{self, RetrieveCurationSetItemParams},
    models,
};

/// Provides methods for managing a curation set item.
///
/// This struct is created by calling `client.curation_set("curation_set_name").item("item_id")`.
pub struct CurationSetItem<'a> {
    pub(super) client: &'a Client,
    pub(super) curation_set_name: &'a str,
    pub(super) item_id: &'a str,
}

impl<'a> CurationSetItem<'a> {
    /// Creates a new `CurationSetItem` instance.
    #[inline]
    pub(super) fn new(client: &'a Client, curation_set_name: &'a str, item_id: &'a str) -> Self {
        Self {
            client,
            curation_set_name,
            item_id,
        }
    }

    /// Retrieve this curation set item.
    pub async fn retrieve(
        &self,
    ) -> Result<models::CurationItemSchema, Error<curation_sets_api::RetrieveCurationSetItemError>>
    {
        let params = RetrieveCurationSetItemParams {
            curation_set_name: self.curation_set_name.into(),
            item_id: self.item_id.into(),
        };
        execute_wrapper!(self, curation_sets_api::retrieve_curation_set_item, params)
    }

    /// Delete this curation set item.
    pub async fn delete(
        &self,
    ) -> Result<
        models::CurationItemDeleteSchema,
        Error<curation_sets_api::DeleteCurationSetItemError>,
    > {
        let params = curation_sets_api::DeleteCurationSetItemParams {
            curation_set_name: self.curation_set_name.into(),
            item_id: self.item_id.into(),
        };
        execute_wrapper!(self, curation_sets_api::delete_curation_set_item, params)
    }
}
