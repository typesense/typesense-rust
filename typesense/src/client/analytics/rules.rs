//! Provides access to the API endpoints for managing analytics rules.
//!
//! An `Rules` instance is created via the `Client::analytics().rules()` method.

use super::{Client, Error};
use std::sync::Arc;
use typesense_codegen::{
    apis::{analytics_api, configuration},
    models,
};

/// Provides methods for interacting with a collection of analytics rules.
///
/// This struct is created by calling `client.analytics().rules()`.
pub struct Rules<'a> {
    pub(super) client: &'a Client,
}

impl<'a> Rules<'a> {
    /// Creates a new `Rules` instance.
    pub(super) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Creates a new analytics rule.
    ///
    /// # Arguments
    /// * `schema` - An `AnalyticsRuleSchema` object describing the rule to be created.
    pub async fn create(
        &self,
        schema: models::AnalyticsRuleSchema,
    ) -> Result<models::AnalyticsRuleSchema, Error<analytics_api::CreateAnalyticsRuleError>> {
        let params = analytics_api::CreateAnalyticsRuleParams {
            analytics_rule_schema: schema,
        };
        self.client
            .execute(|config: Arc<configuration::Configuration>| {
                let params_for_move = params.clone();
                async move { analytics_api::create_analytics_rule(&config, params_for_move).await }
            })
            .await
    }

    /// Creates or updates an analytics rule with the given name.
    ///
    /// # Arguments
    /// * `rule_name` - The name of the analytics rule to create or update.
    /// * `schema` - An `AnalyticsRuleUpsertSchema` object with the rule's parameters.
    pub async fn upsert(
        &self,
        rule_name: &str,
        schema: models::AnalyticsRuleUpsertSchema,
    ) -> Result<models::AnalyticsRuleSchema, Error<analytics_api::UpsertAnalyticsRuleError>> {
        let params = analytics_api::UpsertAnalyticsRuleParams {
            rule_name: rule_name.to_string(),
            analytics_rule_upsert_schema: schema,
        };
        self.client
            .execute(|config: Arc<configuration::Configuration>| {
                let params_for_move = params.clone();
                async move { analytics_api::upsert_analytics_rule(&config, params_for_move).await }
            })
            .await
    }

    /// Retrieves the details of all analytics rules.
    pub async fn retrieve(
        &self,
    ) -> Result<
        models::AnalyticsRulesRetrieveSchema,
        Error<analytics_api::RetrieveAnalyticsRulesError>,
    > {
        self.client
            .execute(|config: Arc<configuration::Configuration>| async move {
                analytics_api::retrieve_analytics_rules(&config).await
            })
            .await
    }
}
