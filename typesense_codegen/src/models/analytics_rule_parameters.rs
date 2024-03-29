/*
 * Typesense API
 *
 * An open source search engine for building delightful search experiences.
 *
 * The version of the OpenAPI document: 0.25.0
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AnalyticsRuleParameters {
    #[serde(rename = "source")]
    pub source: Box<crate::models::AnalyticsRuleParametersSource>,
    #[serde(rename = "destination")]
    pub destination: Box<crate::models::AnalyticsRuleParametersDestination>,
    #[serde(rename = "limit")]
    pub limit: i32,
}

impl AnalyticsRuleParameters {
    pub fn new(
        source: crate::models::AnalyticsRuleParametersSource,
        destination: crate::models::AnalyticsRuleParametersDestination,
        limit: i32,
    ) -> AnalyticsRuleParameters {
        AnalyticsRuleParameters {
            source: Box::new(source),
            destination: Box::new(destination),
            limit,
        }
    }
}
