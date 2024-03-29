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
pub struct FieldEmbed {
    #[serde(rename = "from")]
    pub from: Vec<String>,
    #[serde(rename = "model_config")]
    pub model_config: Box<crate::models::FieldEmbedModelConfig>,
}

impl FieldEmbed {
    pub fn new(
        from: Vec<String>,
        model_config: crate::models::FieldEmbedModelConfig,
    ) -> FieldEmbed {
        FieldEmbed {
            from,
            model_config: Box::new(model_config),
        }
    }
}
