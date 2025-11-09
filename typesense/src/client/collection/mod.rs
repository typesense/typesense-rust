//! Provides access to the collection and alias-related API endpoints.
//!
//! A `Collections` instance is created via the main `client.collections()` method.

mod document;
mod documents;
use crate::{Client, Error, execute_wrapper};

use serde::{Serialize, de::DeserializeOwned};
use typesense_codegen::{apis::collections_api, models};

/// Provides methods for interacting with a Typesense collection.
///
/// This struct is created by calling `client.collection()`.
pub struct Collection<'c, 'n, D = serde_json::Value>
where
    D: DeserializeOwned + Serialize,
{
    client: &'c Client,
    collection_name: &'n str,
    _phantom: core::marker::PhantomData<D>,
}

impl<'c, 'n, D> Collection<'c, 'n, D>
where
    D: DeserializeOwned + Serialize,
{
    /// Creates a new `Collection` instance.
    #[inline]
    pub(super) fn new(client: &'c Client, collection_name: &'n str) -> Self {
        Self {
            client,
            collection_name,
            _phantom: core::marker::PhantomData,
        }
    }

    /// Provides access to the document-related API endpoints for a specific collection.
    #[inline]
    pub fn documents(&self) -> documents::Documents<'c, 'n, D> {
        documents::Documents::new(self.client, self.collection_name)
    }

    /// Provides access to the API endpoints for a single document within a Typesense collection.
    #[inline]
    pub fn document(&self, document_id: impl Into<String>) -> document::Document<'c, 'n, D> {
        document::Document::new(self.client, self.collection_name, document_id.into())
    }

    /// Retrieves the details of a collection, given its name.
    #[inline]
    pub async fn retrieve(
        &self,
    ) -> Result<
        models::CollectionResponse<'static>,
        Error<collections_api::GetCollectionError<'static>>,
    > {
        let params = collections_api::GetCollectionParams {
            collection_name: self.collection_name.into(),
            _phantom: core::marker::PhantomData,
        };
        execute_wrapper!(self, collections_api::get_collection, params)
    }

    /// Permanently drops a collection.
    #[inline]
    pub async fn delete(
        &self,
    ) -> Result<
        models::CollectionResponse<'static>,
        Error<collections_api::DeleteCollectionError<'static>>,
    > {
        let params = collections_api::DeleteCollectionParams {
            collection_name: self.collection_name.into(),
            _phantom: core::marker::PhantomData,
        };
        execute_wrapper!(self, collections_api::delete_collection, params)
    }

    /// Updates a collection's schema to modify the fields and their types.
    ///
    /// # Arguments
    /// * `update_schema` - A `CollectionUpdateSchema` object describing the fields to update.
    #[inline]
    pub async fn update(
        &self,
        update_schema: models::CollectionUpdateSchema<'_>,
    ) -> Result<
        models::CollectionUpdateSchema<'static>,
        Error<collections_api::UpdateCollectionError<'static>>,
    > {
        let params = collections_api::UpdateCollectionParams {
            collection_name: self.collection_name.into(),
            collection_update_schema: update_schema,
            _phantom: core::marker::PhantomData,
        };
        execute_wrapper!(self, collections_api::update_collection, params)
    }
}
