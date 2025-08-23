//! Provides access to the API endpoint for posting analytics events.
//!
//! An `Events` instance is created via the `Client::analytics().events()` method.

use crate::{Client, Error};
use typesense_codegen::{
    apis::{analytics_api, configuration},
    models,
};

/// Provides methods for interacting with analytics events.
///
/// This struct is created by calling `client.analytics().events()`.
pub struct Events<'a> {
    pub(super) client: &'a Client,
}

impl<'a> Events<'a> {
    /// Creates a new `Events` instance.
    pub(super) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Posts an analytics event for tracking user behavior.
    ///
    /// This is useful for features like "search result ranking based on popularity."
    ///
    /// # Arguments
    /// * `schema` - An `AnalyticsEventCreateSchema` object representing the event.
    pub async fn create(
        &self,
        schema: models::AnalyticsEventCreateSchema,
    ) -> Result<models::AnalyticsEventCreateResponse, Error<analytics_api::CreateAnalyticsEventError>>
    {
        let params = analytics_api::CreateAnalyticsEventParams {
            analytics_event_create_schema: schema,
        };
        self.client
            .execute(|config: configuration::Configuration| {
                let params_for_move = params.clone();
                async move { analytics_api::create_analytics_event(&config, params_for_move).await }
            })
            .await
    }
}
