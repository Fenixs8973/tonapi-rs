use serde::{Deserialize, Serialize};

/*
 * REST api to TON blockchain explorer
 *
 * Provide access to indexed TON blockchain
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: support@tonkeeper.com
 * Generated by: https://openapi-generator.tech
 */

use reqwest;
use tonlib::address::TonAddress;

use super::{configuration, Error};
use crate::rest_api::codegen::apis::ResponseContent;

/// struct for passing parameters to the method [`get_jetton_holders`]
#[derive(Clone, Debug)]
pub struct GetJettonHoldersParams {
    /// account ID
    pub account_id: TonAddress,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

/// struct for passing parameters to the method [`get_jetton_info`]
#[derive(Clone, Debug)]
pub struct GetJettonInfoParams {
    /// account ID
    pub account_id: TonAddress,
}

/// struct for passing parameters to the method [`get_jettons`]
#[derive(Clone, Debug)]
pub struct GetJettonsParams {
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

/// struct for passing parameters to the method [`get_jettons_events`]
#[derive(Clone, Debug)]
pub struct GetJettonsEventsParams {
    /// event ID or transaction hash in hex (without 0x) or base64url format
    pub event_id: String,
    pub accept_language: Option<String>,
}

/// struct for typed errors of method [`get_jetton_holders`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetJettonHoldersError {
    DefaultResponse(crate::rest_api::codegen::models::GetBlockchainBlockDefaultResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_jetton_info`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetJettonInfoError {
    DefaultResponse(crate::rest_api::codegen::models::GetBlockchainBlockDefaultResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_jettons`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetJettonsError {
    DefaultResponse(crate::rest_api::codegen::models::GetBlockchainBlockDefaultResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_jettons_events`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetJettonsEventsError {
    DefaultResponse(crate::rest_api::codegen::models::GetBlockchainBlockDefaultResponse),
    UnknownValue(serde_json::Value),
}

/// Get jetton's holders
pub async fn get_jetton_holders(
    configuration: &configuration::Configuration,
    params: GetJettonHoldersParams,
) -> Result<crate::rest_api::codegen::models::JettonHolders, Error<GetJettonHoldersError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let account_id = params.account_id;
    let limit = params.limit;
    let offset = params.offset;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/v2/jettons/{account_id}/holders",
        local_var_configuration.base_path,
        account_id = crate::rest_api::codegen::apis::urlencode(account_id.to_base64_url())
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = limit {
        local_var_req_builder =
            local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = offset {
        local_var_req_builder =
            local_var_req_builder.query(&[("offset", &local_var_str.to_string())]);
    }
    if let Some(ref bearer_access_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(bearer_access_token);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetJettonHoldersError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get jetton metadata by jetton master address
pub async fn get_jetton_info(
    configuration: &configuration::Configuration,
    params: GetJettonInfoParams,
) -> Result<crate::rest_api::codegen::models::JettonInfo, Error<GetJettonInfoError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let account_id = params.account_id;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/v2/jettons/{account_id}",
        local_var_configuration.base_path,
        account_id = crate::rest_api::codegen::apis::urlencode(account_id.to_base64_url())
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref bearer_access_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(bearer_access_token);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetJettonInfoError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get a list of all indexed jetton masters in the blockchain.
pub async fn get_jettons(
    configuration: &configuration::Configuration,
    params: GetJettonsParams,
) -> Result<crate::rest_api::codegen::models::Jettons, Error<GetJettonsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let limit = params.limit;
    let offset = params.offset;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v2/jettons", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = limit {
        local_var_req_builder =
            local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = offset {
        local_var_req_builder =
            local_var_req_builder.query(&[("offset", &local_var_str.to_string())]);
    }
    if let Some(ref bearer_access_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(bearer_access_token);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetJettonsError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get only jetton transfers in the event
pub async fn get_jettons_events(
    configuration: &configuration::Configuration,
    params: GetJettonsEventsParams,
) -> Result<crate::rest_api::codegen::models::Event, Error<GetJettonsEventsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let event_id = params.event_id;
    let accept_language = params.accept_language;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/v2/events/{event_id}/jettons",
        local_var_configuration.base_path,
        event_id = crate::rest_api::codegen::apis::urlencode(event_id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref bearer_access_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(bearer_access_token);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = accept_language {
        local_var_req_builder =
            local_var_req_builder.header("Accept-Language", local_var_param_value.to_string());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetJettonsEventsError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
