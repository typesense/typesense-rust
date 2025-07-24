//! Provides access to top-level, non-namespaced API endpoints.
//!
//! An `Operations` instance is created via the main `Client::operations()` method.

use super::{Client, Error};
use std::sync::Arc;
use typesense_codegen::{
    apis::{
        configuration,
        debug_api,
        health_api, // Add this line
    },
    models,
};

/// Provides methods for top-level, non-namespaced Typesense operations.
///
/// This struct is created by calling `client.operations()`.
pub struct Operations<'a> {
    pub(super) client: &'a Client,
}

impl<'a> Operations<'a> {
    /// Creates a new `Operations` instance
    pub(super) fn new(client: &'a Client) -> Self {
        Self { client }
    }
    /// Retrieves debugging information from a Typesense node.
    ///
    /// This method will try nodes in sequence according to the health policy
    /// until it gets a successful response. The returned information pertains
    /// to the specific node that responded successfully.
    pub async fn debug(&self) -> Result<models::Debug200Response, Error<debug_api::DebugError>> {
        self.client
            .execute(|config: Arc<configuration::Configuration>| async move {
                debug_api::debug(&config).await
            })
            .await
    }

    /// Checks if a Typesense node is healthy and ready to accept requests.
    ///
    /// This method will try nodes in sequence according to the health policy
    /// until it gets a successful response (`{"ok": true}`).
    pub async fn health(&self) -> Result<models::HealthStatus, Error<health_api::HealthError>> {
        self.client
            .execute(|config: Arc<configuration::Configuration>| async move {
                health_api::health(&config).await
            })
            .await
    }
}
