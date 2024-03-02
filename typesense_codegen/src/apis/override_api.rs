/*
 * Typesense API
 *
 * An open source search engine for building delightful search experiences.
 *
 * The version of the OpenAPI document: 0.25.0
 *
 * Generated by: https://openapi-generator.tech
 */

use super::{configuration, Error};
use crate::apis::ResponseContent;

/// struct for typed errors of method [`get_search_override`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetSearchOverrideError {
    UnknownValue(serde_json::Value),
}

/// Retrieve the details of a search override, given its id.
pub async fn get_search_override(
    configuration: &configuration::Configuration,
    collection_name: &str,
    override_id: &str,
) -> Result<crate::models::SearchOverride, Error<GetSearchOverrideError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/collections/{collectionName}/overrides/{overrideId}",
        local_var_configuration.base_path,
        collectionName = crate::apis::urlencode(collection_name),
        overrideId = crate::apis::urlencode(override_id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent);
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = &local_var_apikey.key;
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key.clone(),
        };
        local_var_req_builder =
            local_var_req_builder.header("X-TYPESENSE-API-KEY", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetSearchOverrideError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
