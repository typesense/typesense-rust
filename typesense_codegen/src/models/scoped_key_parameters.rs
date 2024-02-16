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
pub struct ScopedKeyParameters {
    #[serde(rename = "filter_by", skip_serializing_if = "Option::is_none")]
    pub filter_by: Option<String>,
    #[serde(rename = "expires_at", skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<i64>,
}

impl ScopedKeyParameters {
    pub fn new() -> ScopedKeyParameters {
        ScopedKeyParameters {
            filter_by: None,
            expires_at: None,
        }
    }
}
