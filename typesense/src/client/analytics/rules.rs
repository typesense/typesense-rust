//! Provides access to the API endpoints for managing analytics rules.
//!
//! An `Rules` instance is created via the `Client::analytics().rules()` method.

use crate::{Client, Error, execute_wrapper, models};
use ::std::borrow::Cow;
use reqwest::StatusCode;
use serde_json::json;
use typesense_codegen::apis::{ResponseContent, analytics_api};

/// Provides methods for interacting with a collection of analytics rules.
///
/// This struct is created by calling `client.analytics().rules()`.
pub struct Rules<'a> {
    pub(super) client: &'a Client,
}

impl<'a> Rules<'a> {
    /// Creates a new `Rules` instance.
    #[inline]
    pub(super) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Creates a new analytics rule.
    ///
    /// # Arguments
    /// * `schema` - An `AnalyticsRuleCreate` object describing the rule to be created.
    pub async fn create(
        &self,
        schema: models::AnalyticsRuleCreate<'_>,
    ) -> Result<models::AnalyticsRule, Error<analytics_api::CreateAnalyticsRuleError>> {
        let params = analytics_api::CreateAnalyticsRuleParams {
            create_analytics_rule_request: models::CreateAnalyticsRuleRequest::AnalyticsRuleCreate(
                Box::new(schema),
            ),
        };
        match execute_wrapper!(self, analytics_api::create_analytics_rule, params)? {
            models::CreateAnalyticsRule200Response::AnalyticsRule(rule) => Ok(*rule),
            _ => Err(Error::Api(typesense_codegen::apis::Error::ResponseError(
                ResponseContent {
                    status: StatusCode::OK,
                    content: "Unexpected response type".to_owned(),
                    entity: Some(analytics_api::CreateAnalyticsRuleError::UnknownValue(
                        json!("Expected single AnalyticsRule, not a list"),
                    )),
                },
            ))),
        }
    }

    /// Creates multiple analytics rules in a single request.
    ///
    /// # Arguments
    /// * `schema` - A `Vec<AnalyticsRuleCreate>` describing the rules to be created.
    pub async fn create_many(
        &self,
        schema: Vec<models::AnalyticsRuleCreate<'_>>,
    ) -> Result<
        Vec<models::CreateAnalyticsRule200ResponseOneOfInner>,
        Error<analytics_api::CreateAnalyticsRuleError>,
    > {
        let params = analytics_api::CreateAnalyticsRuleParams {
            create_analytics_rule_request: models::CreateAnalyticsRuleRequest::Array(schema),
        };
        match execute_wrapper!(self, analytics_api::create_analytics_rule, params)? {
            models::CreateAnalyticsRule200Response::Array(rules) => Ok(rules),
            _ => Err(Error::Api(typesense_codegen::apis::Error::ResponseError(
                ResponseContent {
                    status: StatusCode::OK,
                    content: "Unexpected response type".to_owned(),
                    entity: Some(analytics_api::CreateAnalyticsRuleError::UnknownValue(
                        json!("Expected a list of AnalyticsRule, not a single rule"),
                    )),
                },
            ))),
        }
    }

    /// Creates or updates an analytics rule with the given name.
    ///
    /// # Arguments
    /// * `rule_name` - The name of the analytics rule to create or update.
    /// * `schema` - An `AnalyticsRuleUpsertSchema` object with the rule's parameters.
    pub async fn upsert(
        &self,
        rule_name: impl Into<Cow<'_, str>>,
        schema: models::AnalyticsRuleUpdate<'_>,
    ) -> Result<models::AnalyticsRule, Error<analytics_api::UpsertAnalyticsRuleError>> {
        let params = analytics_api::UpsertAnalyticsRuleParams {
            rule_name: rule_name.into(),
            analytics_rule_update: schema,
        };
        execute_wrapper!(self, analytics_api::upsert_analytics_rule, params)
    }

    /// Retrieves the details of all analytics rules.
    pub async fn retrieve(
        &self,
        params: Option<analytics_api::RetrieveAnalyticsRulesParams<'_>>,
    ) -> Result<Vec<models::AnalyticsRule>, Error<analytics_api::RetrieveAnalyticsRulesError>> {
        let params =
            params.unwrap_or(analytics_api::RetrieveAnalyticsRulesParams { rule_tag: None });
        execute_wrapper!(self, analytics_api::retrieve_analytics_rules, params)
    }
}
