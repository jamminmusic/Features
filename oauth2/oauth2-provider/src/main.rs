//! oauth2 capability provider
//!
//!
use oauth2_interface::{
    AuthorizeUserRequest, AuthorizeUserResponse, 
    GetAuthUriRequest, GetAuthUriResponse, 
    Oauth2, Oauth2Receiver, 
    UnauthorizeUserRequest, UnauthorizeUserResponse 
};
use strum::EnumString;
use std::collections::HashMap;
use serde::Deserialize;


#[allow(unused_imports)]
use wasmbus_rpc::provider::prelude::*;
// main (via provider_main) initializes the threaded tokio executor,
// listens to lattice rpcs, handles actor links,
// and returns only when it receives a shutdown message
//
fn main() -> Result<(), Box<dyn std::error::Error>> {
    provider_main(Oauth2Provider::default(), Some("Oauth2".to_string()))?;

    eprintln!("oauth2 provider exiting");
    Ok(())
}

// available oauth2 Grant Types - Legacy Grant Types not supported
#[derive(Clone, Deserialize, Debug, PartialEq, EnumString)]
pub enum GrantType {
    #[strum(ascii_case_insensitive)]
    AuthorizationCode,
    #[strum(ascii_case_insensitive)]
    Pkce, 
    #[strum(ascii_case_insensitive)]
    Refresh,
    #[strum(ascii_case_insensitive)]
    ClientCredentials,
    #[strum(ascii_case_insensitive)]
    DeviceCode,
}

impl GrantType {
    pub async fn get_auth_uri(&self, req: &GetAuthUriRequest) -> Result(GetAuthUriResponse, Error){
        let auth_uri = match self {
            // User Flow - User interaction auth_uri needed.
            GrantType::AuthorizationCode => AuthUriBuilder::new().create_client().generate_auth_uri(),
            // User Flow + Pkce - User interaction auth_uri needed and will contain code challenge. Most secure User Flow.
            GrantType::Pkce => AuthUriBuilder::new().create_client().generate_pkce().generate_auth_uri_pkce(),
            // Refresh Flow - If client was issued a secret User interaction auth_uri needed, otherwise User interaction auth_uri not needed. 
            GrantType::Refresh => {
                if req.client_secret != None {
                    // How to handle this one?
                    AuthUriBuilder::new().create_client().generate_auth_uri()
                } else {
                    AuthUriBuilder::new().create_client().generate_auth_uri()
                }
            },
            // Application Flow - User interaction auth_uri not needed. Application as a client will pass client_id and secret for authentication.
            GrantType::ClientCredentials => AuthUriBuilder::new().create_client().generate_auth_uri(),
            // Device Flow - User interaction with auth_uri not needed - device will authenticate with client_id and device_code.
            GrantType::DeviceCode => AuthUriBuilder::new().create_client().generate_auth_uri(),
        };
        // Response Struct - { success: boolean, error: String, uri: String, csrf_state: String }       
        Ok(GetAuthUriResponse)
    }
    pub async fn authorize_user(&self, req: &AuthorizeUserRequest) -> Result(AuthorizeUserResponse, Error) {
        let token = match self {
            // 
            GrantType::AuthorizationCode => "Define functions to call",
            // 
            GrantType::Pkce => "Define functions to call",
            // 
            GrantType::Refresh => "Define functions to call",
            // 
            GrantType::ClientCredentials => "Define functions to call",
            // 
            GrantType::DeviceCode => "Define functions to call",
        };
        // Response Struct - { success: boolean, error: String, access_token: String, refresh_token: String, user_id: String, device_id: String, device_id: String, scope: String } 
        Ok(AuthorizeUserResponse)
    }
    pub async fn unauthorize_user(&self, req: &UnauthorizeUserRequest) -> Result(UnauthorizeUserResponse, Error) {
        let status = match self {
            // 
            GrantType::AuthorizationCode => "Define functions to call",
            // 
            GrantType::Pkce => "Define functions to call",
            // 
            GrantType::Refresh => "Define functions to call",
            // 
            GrantType::ClientCredentials => "Define functions to call",
            // 
            GrantType::DeviceCode => "Define functions to call",
        };
        // Response Struct - { success: boolean, error: String } 
        Ok(UnauthorizeUserResponse)
    }
}

/// oauth2 capability provider implementation
#[derive(Default, Clone, Provider)]
#[services(Oauth2)]
struct Oauth2Provider {}

/// use default implementations of provider message handlers
impl ProviderDispatch for Oauth2Provider {}
impl ProviderHandler for Oauth2Provider {}

/// Handle Factorial methods
#[async_trait]
impl Oauth2 for Oauth2Provider {
    async fn get_auth_uri(
        &self,
        _ctx: &Context,
        _req: &GetAuthUriRequest,
    ) -> RpcResult<GetAuthUriResponse> {
        // Request Struct - { 
        // provider: String, grant_type: String, client_id: String, device_code: String, client_secret: String, 
        // auth_url: String, token_url: String, redirect_url: String, scope: String }
        // Enum Method Returns GetAuthUriResponse or Error
        let response = GrantType::from_str(GetAuthUriRequest.grant_type).get_auth_uri().unwrap().await;
        Ok(response)
    }

    async fn authorize_user(
        &self,
        _ctx: &Context,
        _req: &AuthorizeUserRequest,
    ) -> RpcResult<AuthorizeUserResponse> {
        // Request Struct - { provider: String, grant_type: String, auth_code: String, state: String, csrf_state: String }
        // Enum Method Returns AuthorizeUserResponse or Error
        let response = GrantType::from_str(GetAuthUriRequest.grant_type).authorize_user().unwrap().await;
        Ok(response)
    }

    async fn unauthorize_user(
        &self,
        _ctx: &Context,
        _req: &UnauthorizeUserRequest,
    ) -> RpcResult<UnauthorizeUserResponse> {
        // Request Struct - { provider: String, user: String, device_id: String }
        // Enum Method Returns UnauthorizeUserResponse or Error
        let response = GrantType::from_str(GetAuthUriRequest.grant_type).unauthorize_user().unwrap().await;
        Ok(response)
    }
}
