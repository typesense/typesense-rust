//! Provides access to the API endpoints for managing a single analytics rule.
//!
//! An `Rule` instance is created via the `client.analytics().rule("rule_name")` method.

use crate::{Client, Error, execute_wrapper, models};
use typesense_codegen::apis::analytics_api;

/// Provides methods for interacting with a specific analytics rule.
///
/// This struct is created by calling `client.analytics().rule("rule_name")`.
pub struct Rule<'a> {
    pub(super) client: &'a Client,
    pub(super) rule_name: &'a str,
}

impl<'a> Rule<'a> {
    /// Creates a new `Rule` instance for a specific rule name.
    #[inline]
    pub(super) fn new(client: &'a Client, rule_name: &'a str) -> Self {
        Self { client, rule_name }
    }

    /// Retrieves the details of this specific analytics rule.
    pub async fn retrieve(
        &self,
    ) -> Result<models::AnalyticsRule, Error<analytics_api::RetrieveAnalyticsRuleError>> {
        let params = analytics_api::RetrieveAnalyticsRuleParams {
            rule_name: self.rule_name.into(),
        };
        execute_wrapper!(self, analytics_api::retrieve_analytics_rule, params)
    }

    /// Permanently deletes this specific analytics rule.
    pub async fn delete(
        &self,
    ) -> Result<models::AnalyticsRule, Error<analytics_api::DeleteAnalyticsRuleError>> {
        let params = analytics_api::DeleteAnalyticsRuleParams {
            rule_name: self.rule_name.into(),
        };
        execute_wrapper!(self, analytics_api::delete_analytics_rule, params)
    }
}
