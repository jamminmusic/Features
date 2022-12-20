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
    pub async fn get_auth_uri(&self, grant_type: String) -> Result(String, Error){
        let auth_uri = match self {
            // User Flow - auth_uri needed.
            GrantType::AuthorizationCode => "Define functions to call",
            // User Flow + Pkce - auth_uri needed and will contain code challenge. Most secure User Flow.
            GrantType::Pkce => "Define functions to call",
            // Refresh Flow - If client was issued a secret auth_uri needed, otherwise auth_uri not needed. 
            GrantType::Refresh => "Define functions to call",
            // Application Flow - auth_uri not needed. Application as a client will pass client_id and secret for authentication.
            GrantType::ClientCredentials => "Define functions to call",
            // Device Flow - auth_uri not needed - device will authenticate with client_id and device_code.
            GrantType::DeviceCode => "Define functions to call",
        };
        Ok(auth_uri)
    }
    pub async fn authorize_user(&self, req: String) -> Result(String, Error) {
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
        Ok(token)
    }
    pub async fn unauthorize_user(&self, req: String) -> Result(String, Error) {
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
        Ok(status)
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

        GrantType::from_str(GetAuthUriRequest.grant_type)


        let x = GetAuthUriResponse {
            success: true,
            error: Some("words".to_string()),
            uri: "words".to_string(),
            csrf_state: "words".to_string(),
        };
        Ok(x)
    }

    async fn authorize_user(
        &self,
        _ctx: &Context,
        _req: &AuthorizeUserRequest,
    ) -> RpcResult<AuthorizeUserResponse> {

        GrantType::from_str(GetAuthUriRequest.grant_type)

        let x = AuthorizeUserResponse {
            success: true,
            error: Some("words".to_string()),
            token: "words".to_string(),
            user_id: "words".to_string(),
        };
        Ok(x)
    }

    async fn unauthorize_user(
        &self,
        _ctx: &Context,
        _req: &UnauthorizeUserRequest,
    ) -> RpcResult<UnauthorizeUserResponse> {

        GrantType::from_str(GetAuthUriRequest.grant_type)
        
        let x = UnauthorizeUserResponse {
            success: true,
            error: Some("words".to_string()),
        };
        Ok(x)
    }
}
