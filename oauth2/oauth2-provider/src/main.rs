//! Oauth2 capability provider
//!
//!
use oauth2_interface::{
    AuthorizeUserRequest, AuthorizeUserResponse, 
    GetAuthUriRequest, GetAuthUriResponse, 
    Oauth2, Oauth2Receiver, 
    UnauthorizeUserRequest, UnauthorizeUserResponse 
};
use strum::EnumString;
use std::str::FromStr;
use anyhow::Error;
use serde::Deserialize;
use oauth2_provider::AuthUriBuilder;

#[allow(unused_imports)]
use wasmbus_rpc::provider::prelude::*;
// main (via provider_main) initializes the threaded tokio executor,
// listens to lattice rpcs, handles actor links,
// and returns only when it receives a shutdown message
//
fn main() -> Result<(), Box<dyn std::error::Error>> {
    provider_main(Oauth2Provider::default(), Some("Oauth2".to_string()))?;

    eprintln!("Oauth2 provider exiting");
    Ok(())
}

// available Oauth2 Grant Types - Legacy Grant Types not supported
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
    pub async fn get_auth_uri(&self, req: &GetAuthUriRequest) -> Result<GetAuthUriResponse, Error>{
        let auth_uri_response = match self {
            // User Flow - User interaction with auth_uri needed.
            //Just remove PKCE code from https://docs.rs/oauth2/4.3.0/oauth2/#getting-started-authorization-code-grant-w-pkce
            GrantType::AuthorizationCode => AuthUriBuilder::new().create_client(req).set_redirect_uri(req).generate_auth_uri(req).build(),
            // User Flow + Pkce - User interaction with auth_uri needed and will contain code challenge. Most secure User Flow.
            // https://docs.rs/oauth2/4.3.0/oauth2/#getting-started-authorization-code-grant-w-pkce
            GrantType::Pkce => AuthUriBuilder::new().create_client(req).set_redirect_uri(req).generate_pkce().generate_auth_uri_pkce(req).build(),
            // Refresh Flow - If client was issued a secret User interaction with auth_uri needed, otherwise User interaction with auth_uri not needed. 
            // https://docs.rs/oauth2/4.3.0/oauth2/trait.TokenResponse.html#tymethod.refresh_token
            GrantType::Refresh => AuthUriBuilder::new().create_client(req).generate_auth_uri(req).build(),
            // Application Flow - User interaction with auth_uri not needed. Application as a client will pass client_id and secret for authentication.
            // https://docs.rs/oauth2/4.3.0/oauth2/#client-credentials-grant
            // client_secret is an option when creating the Client, therefore we can handle whether a user needs to interact outside of URI generation.
            GrantType::ClientCredentials => AuthUriBuilder::new().create_client(req).generate_auth_uri(req).build(),
            // Device Flow - User interaction with auth_uri needed - authenticate on browserless or input-constrained devices.
            // https://docs.rs/oauth2/4.3.0/oauth2/#device-code-flow
            GrantType::DeviceCode => AuthUriBuilder::new().create_client(req).set_device_uri(req).generate_device_auth_uri().build(),
        };
        // Response Struct - { success: boolean, error: String, uri: String, csrf_state: String }       
        Ok(auth_uri_response)
    }
    // TODO
    pub async fn authorize_user(&self, req: &AuthorizeUserRequest) -> Result<AuthorizeUserResponse, Error> {
        unimplemented!();

        // let authorize_user_response = match self {
        //     // 
        //     GrantType::AuthorizationCode => "Define functions to call",
        //     // 
        //     GrantType::Pkce => "Define functions to call",
        //     // 
        //     GrantType::Refresh => "Define functions to call",
        //     // 
        //     GrantType::ClientCredentials => "Define functions to call",
        //     // 
        //     GrantType::DeviceCode => "Define functions to call",
        // };
        // // Response Struct - { success: boolean, error: String, access_token: String, refresh_token: String, 
        // // user_id: String, device_id: String, scope: String } 
        // Ok(AuthorizeUserResponse)
    }
    // TODO
    pub async fn unauthorize_user(&self, req: &UnauthorizeUserRequest) -> Result<UnauthorizeUserResponse, Error> {
        unimplemented!();

        // let unauthorize_user_response = match self {
        //     // 
        //     GrantType::AuthorizationCode => "Define functions to call",
        //     // 
        //     GrantType::Pkce => "Define functions to call",
        //     // 
        //     GrantType::Refresh => "Define functions to call",
        //     // 
        //     GrantType::ClientCredentials => "Define functions to call",
        //     // 
        //     GrantType::DeviceCode => "Define functions to call",
        // };
        // // Response Struct - { success: boolean, error: String } 
        // Ok(UnauthorizeUserResponse)
    }
}

/// Oauth2 capability provider implementation
#[derive(Default, Clone, Provider)]
#[services(Oauth2)]
struct Oauth2Provider {}

/// use default implementations of provider message handlers
impl ProviderDispatch for Oauth2Provider {}
impl ProviderHandler for Oauth2Provider {}

/// Handle Oauth2 methods
#[async_trait]
impl Oauth2 for Oauth2Provider {
    async fn get_auth_uri(
        &self,
        _ctx: &Context,
        _req: &GetAuthUriRequest,
    ) -> RpcResult<GetAuthUriResponse> {
        // Request Struct - { grant_type: String, client_id: String, device_code: String, client_secret: String, 
        //                    auth_url: String, token_url: String, redirect_url: String, scope: String, device_auth_uri: String }
        // Response Struct - { success: Boolean, error: String, uri: String, csrf_state: String, device_uri: String, device_usercode: String }
        let response = GrantType::from_str(&_req.grant_type).expect("Grant type not found").get_auth_uri(&_req).await.unwrap();
        Ok(response)
    }

    async fn authorize_user(
        &self,
        _ctx: &Context,
        _req: &AuthorizeUserRequest,
    ) -> RpcResult<AuthorizeUserResponse> {
        unimplemented!();
        // Request Struct - { grant_type: String, auth_code: String, state: String, csrf_state: String }
        // Response Struct - { success: Boolean, error: String, access_token: String, refresh_token: String, 
        //                     user_id: String, device_id: String, expire_date: String, scope: String }
        // let response = GrantType::from_str(GetAuthUriRequest.grant_type).authorize_user().unwrap().await;
        // Ok(response)
    }

    async fn unauthorize_user(
        &self,
        _ctx: &Context,
        _req: &UnauthorizeUserRequest,
    ) -> RpcResult<UnauthorizeUserResponse> {
        unimplemented!();
        // Request Struct - { user: String, device_id: String }
        // Response Struct - { success: Boolean, error: String }
        // let response = GrantType::from_str(GetAuthUriRequest.grant_type).unauthorize_user().unwrap().await;
        // Ok(response)
    }
}
