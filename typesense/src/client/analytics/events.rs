//! Provides access to the API endpoint for posting analytics events.
//!
//! An `Events` instance is created via the `client.analytics().events()` method.

use crate::{Client, Error, execute_wrapper, models};
use typesense_codegen::apis::analytics_api;

/// Provides methods for interacting with analytics events.
///
/// This struct is created by calling `client.analytics().events()`.
pub struct Events<'a> {
    pub(super) client: &'a Client,
}

impl<'a> Events<'a> {
    /// Creates a new `Events` instance.
    #[inline]
    pub(super) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Posts an analytics event for tracking user behavior.
    ///
    /// # Arguments
    /// * `schema` - An `AnalyticsEvent` object representing the event.
    pub async fn create(
        &self,
        schema: models::AnalyticsEvent<'_>,
    ) -> Result<models::AnalyticsEventCreateResponse, Error<analytics_api::CreateAnalyticsEventError>>
    {
        let params = analytics_api::CreateAnalyticsEventParams {
            analytics_event: schema,
        };
        execute_wrapper!(self, analytics_api::create_analytics_event, params)
    }

    /// Retrieve the most recent analytics events for a specific user and analytics rule name.
    ///
    /// # Arguments
    /// * `params` - `GetAnalyticsEventsParams`.
    pub async fn retrieve(
        &self,
        params: models::GetAnalyticsEventsParams<'_>,
    ) -> Result<models::AnalyticsEventsResponse, Error<analytics_api::GetAnalyticsEventsError>>
    {
        execute_wrapper!(self, analytics_api::get_analytics_events, params)
    }
}
