//! Provides access to the collection alias-related API endpoints.
//!
//! An `Alias` instance is created via the main `client.alias("alias_name")` method.

use crate::{Client, Error, execute_wrapper};
use typesense_codegen::{apis::collections_api, models};

/// Provides methods for interacting with a specific Typesense collection alias.
///
/// This struct is created by calling `client.alias("alias_name")`.
pub struct Alias<'a> {
    pub(super) client: &'a Client,
    pub(super) alias_name: &'a str,
}

impl<'a> Alias<'a> {
    /// Creates a new `Alias` instance.
    #[inline]
    pub(super) fn new(client: &'a Client, alias_name: &'a str) -> Self {
        Self { client, alias_name }
    }

    /// Retrieves the details of a collection alias, including the collection it points to.
    pub async fn retrieve(
        &self,
    ) -> Result<models::CollectionAlias, Error<collections_api::GetAliasError>> {
        let params = collections_api::GetAliasParams {
            alias_name: self.alias_name.to_owned(),
        };

        execute_wrapper!(self, collections_api::get_alias, params)
    }

    /// Deletes a collection alias.
    pub async fn delete(
        &self,
    ) -> Result<models::CollectionAlias, Error<collections_api::DeleteAliasError>> {
        let params = collections_api::DeleteAliasParams {
            alias_name: self.alias_name.to_owned(),
        };
        execute_wrapper!(self, collections_api::delete_alias, params)
    }
}
