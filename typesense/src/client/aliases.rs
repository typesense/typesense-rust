//! Provides access to the collection aliases-related API endpoints.
//!
//! An `Aliases` instance is created via the main `client.aliases()` method.

use crate::{Client, Error, execute_wrapper};
use ::std::borrow::Cow;
use typesense_codegen::{apis::collections_api, models};

/// Provides methods for interacting with Typesense collection aliases.
///
/// This struct is created by calling `client.aliases()`.
pub struct Aliases<'a> {
    pub(super) client: &'a Client,
}

impl<'a> Aliases<'a> {
    /// Creates a new `Aliases` instance.
    #[inline]
    pub(super) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Creates or updates a collection alias.
    ///
    /// An alias is a virtual collection name that points to a real collection.
    /// Aliases are useful when you want to re-index your data in the background
    /// on a new collection and then switch your application to it without any
    /// changes to your code.
    ///
    /// # Arguments
    /// * `schema` - A `CollectionAliasSchema` pointing to the target collection.
    pub async fn upsert(
        &self,
        alias_name: impl Into<Cow<'_, str>>,
        schema: models::CollectionAliasSchema<'_>,
    ) -> Result<models::CollectionAlias<'static>, Error<collections_api::UpsertAliasError<'static>>>
    {
        let params = collections_api::UpsertAliasParams {
            alias_name: alias_name.into(),
            collection_alias_schema: Some(schema),
            _phantom: core::marker::PhantomData,
        };
        execute_wrapper!(self, collections_api::upsert_alias, params)
    }

    /// Lists all aliases and the corresponding collections that they map to.
    pub async fn retrieve(
        &self,
    ) -> Result<
        models::CollectionAliasesResponse<'static>,
        Error<collections_api::GetAliasesError<'static>>,
    > {
        execute_wrapper!(self, collections_api::get_aliases)
    }
}
