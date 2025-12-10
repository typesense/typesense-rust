//! Provides access to the API endpoints for managing items of a curation set.
//!
//! Curation sets allow you to include or exclude specific documents for a given query.
//!
//! A `CurationSetItems` instance is created via the main `client.curation_set("curation_set_name").items()` method.

use ::std::borrow::Cow;

use crate::{Client, Error, execute_wrapper};
use typesense_codegen::{
    apis::curation_sets_api::{self, RetrieveCurationSetItemsParams},
    models,
};

/// Provides methods for managing items of a curation set.
///
/// This struct is created by calling `client.curation_set("curation_set_name").items()`.
pub struct CurationSetItems<'a> {
    pub(super) client: &'a Client,
    pub(super) curation_set_name: &'a str,
}

impl<'a> CurationSetItems<'a> {
    /// Creates a new `CurationSetItems` instance.
    #[inline]
    pub(super) fn new(client: &'a Client, curation_set_name: &'a str) -> Self {
        Self {
            client,
            curation_set_name,
        }
    }

    /// Retrieves all the items of this curation set.
    pub async fn retrieve(
        &self,
    ) -> Result<
        Vec<models::CurationItemSchema>,
        Error<curation_sets_api::RetrieveCurationSetItemsError>,
    > {
        let params = RetrieveCurationSetItemsParams {
            curation_set_name: self.curation_set_name.into(),
        };
        execute_wrapper!(self, curation_sets_api::retrieve_curation_set_items, params)
    }

    /// Creates or updates an existing item of a curation set.
    ///
    /// # Arguments
    /// * `item_id` - The id of the curation set item to create or update.
    /// * `schema` - A `CurationItemCreateSchema` object.
    pub async fn upsert(
        &self,
        item_id: impl Into<Cow<'_, str>>,
        schema: models::CurationItemCreateSchema<'_>,
    ) -> Result<models::CurationItemSchema, Error<curation_sets_api::UpsertCurationSetItemError>>
    {
        let params = curation_sets_api::UpsertCurationSetItemParams {
            item_id: item_id.into(),
            curation_set_name: self.curation_set_name.into(),
            curation_item_create_schema: schema,
        };
        execute_wrapper!(self, curation_sets_api::upsert_curation_set_item, params)
    }
}
