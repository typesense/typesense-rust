//! Provides access to the API endpoints for managing a single analytics rule.
//!
//! An `Rule` instance is created via the `Client::analytics().rule("rule_name")` method.

use super::{Client, Error};
use std::sync::Arc;
use typesense_codegen::{
    apis::{analytics_api, configuration},
    models,
};

/// Provides methods for interacting with a specific analytics rule.
///
/// This struct is created by calling `analytics.rule("rule_name")`.
pub struct Rule<'a> {
    pub(super) client: &'a Client,
    pub(super) rule_name: &'a str,
}

impl<'a> Rule<'a> {
    /// Creates a new `Rule` instance for a specific rule name.
    pub(super) fn new(client: &'a Client, rule_name: &'a str) -> Self {
        Self { client, rule_name }
    }

    /// Retrieves the details of this specific analytics rule.
    pub async fn retrieve(
        &self,
    ) -> Result<models::AnalyticsRuleSchema, Error<analytics_api::RetrieveAnalyticsRuleError>> {
        let params = analytics_api::RetrieveAnalyticsRuleParams {
            rule_name: self.rule_name.to_string(),
        };
        self.client
            .execute(|config: Arc<configuration::Configuration>| {
                let params_for_move = params.clone();
                async move { analytics_api::retrieve_analytics_rule(&config, params_for_move).await }
            })
            .await
    }

    /// Permanently deletes this specific analytics rule.
    pub async fn delete(
        &self,
    ) -> Result<models::AnalyticsRuleDeleteResponse, Error<analytics_api::DeleteAnalyticsRuleError>>
    {
        let params = analytics_api::DeleteAnalyticsRuleParams {
            rule_name: self.rule_name.to_string(),
        };
        self.client
            .execute(|config: Arc<configuration::Configuration>| {
                let params_for_move = params.clone();
                async move { analytics_api::delete_analytics_rule(&config, params_for_move).await }
            })
            .await
    }
}
