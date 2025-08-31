use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DocumentIndexParameters {
    #[serde(rename = "dirty_values", skip_serializing_if = "Option::is_none")]
    pub dirty_values: Option<typesense_codegen::models::DirtyValues>,
}

impl DocumentIndexParameters {
    pub fn new() -> DocumentIndexParameters {
        DocumentIndexParameters { dirty_values: None }
    }
}
