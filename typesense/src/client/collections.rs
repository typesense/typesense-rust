//! Provides access to the collection and alias-related API endpoints.
//!
//! A `Collections` instance is created via the main `client.collections()` method.

use crate::{Client, Error, execute_wrapper};
use typesense_codegen::{
    apis::collections_api::{self, GetCollectionsParams},
    models::{self, GetCollectionsParameters},
};

/// Provides methods for interacting with Typesense collections and aliases.
///
/// This struct is created by calling `client.collections()`.
pub struct Collections<'c> {
    pub(super) client: &'c Client,
}

impl<'c> Collections<'c> {
    /// Creates a new `Collection` instance
    #[inline]
    pub(super) fn new(client: &'c Client) -> Self {
        Self { client }
    }

    /// Creates a new collection with the given schema.
    ///
    /// # Arguments
    /// * `schema` - A `CollectionSchema` object describing the collection to be created.
    pub async fn create(
        &self,
        schema: models::CollectionSchema<'_>,
    ) -> Result<
        models::CollectionResponse<'static>,
        Error<collections_api::CreateCollectionError<'static>>,
    > {
        let params = collections_api::CreateCollectionParams {
            collection_schema: schema,
            _phantom: core::marker::PhantomData,
        };
        execute_wrapper!(self, collections_api::create_collection, params)
    }

    /// List the existing Typesense collections.
    pub async fn retrieve(
        &self,
        params: GetCollectionsParameters<'_>,
    ) -> Result<
        Vec<models::CollectionResponse<'static>>,
        Error<collections_api::GetCollectionsError<'static>>,
    > {
        let params = GetCollectionsParams {
            exclude_fields: params.exclude_fields,
            limit: params.limit,
            offset: params.offset,
            _phantom: core::marker::PhantomData,
        };
        execute_wrapper!(self, collections_api::get_collections, params)
    }
}
