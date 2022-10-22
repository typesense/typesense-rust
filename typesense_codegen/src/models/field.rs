/*
 * Typesense API
 *
 * An open source search engine for building delightful search experiences.
 *
 * The version of the OpenAPI document: 0.23.0
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Field {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "type")]
    pub _type: String,
    #[serde(rename = "optional", skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
    #[serde(rename = "facet", skip_serializing_if = "Option::is_none")]
    pub facet: Option<bool>,
    #[serde(rename = "index", skip_serializing_if = "Option::is_none")]
    pub index: Option<bool>,
    #[serde(rename = "locale", skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[serde(rename = "sort", skip_serializing_if = "Option::is_none")]
    pub sort: Option<bool>,
    #[serde(rename = "infix", skip_serializing_if = "Option::is_none")]
    pub infix: Option<bool>,
    #[serde(rename = "drop", skip_serializing_if = "Option::is_none")]
    pub drop: Option<bool>,
}

impl Field {
    pub fn new(name: String, _type: String) -> Field {
        Field {
            name,
            _type,
            optional: None,
            facet: None,
            index: None,
            locale: None,
            sort: None,
            infix: None,
            drop: None,
        }
    }
}
