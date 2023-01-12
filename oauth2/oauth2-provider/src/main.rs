//! Oauth2 capability provider
//!
//!
// Device Flow is not yet implemented

use oauth2_interface::*;
use std::str::FromStr;
use oauth2_provider::GrantType;

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

/// Oauth2 capability provider implementation
#[derive(Default, Clone, Provider)]
#[services(Oauth2)]
pub struct Oauth2Provider {}

/// use default implementations of provider message handlers
impl ProviderDispatch for Oauth2Provider {}
impl ProviderHandler for Oauth2Provider {}

/// Handle Oauth2 methods
#[async_trait]
impl Oauth2 for Oauth2Provider {
    async fn get_auth_url(
        &self,
        _ctx: &Context,
        _req: &GetAuthUrlRequest,
    ) -> RpcResult<GetAuthUrlResponse> {
        // Request Struct - { grant_type: String, client_id: String, device_code: String, client_secret: String, 
        //                    auth_url: String, token_url: String, redirect_url: String, scope: String, device_auth_url: String }
        // Response Struct - { success: Boolean, error: String, url: String, csrf_state: String, device_url: String, device_code: String, device_code_expire: U64 }
        let response = GrantType::from_str(&_req.grant_type).expect("Grant type not found").get_auth_url(_req).await.unwrap();
        Ok(response)
    }

    async fn authorize_user(
        &self,
        _ctx: &Context,
        _req: &AuthorizeUserRequest,
    ) -> RpcResult<AuthorizeUserResponse> {
        todo!();
        // Request Struct - { grant_type: String, auth_code: String, state: String, csrf_state: String }
        // Response Struct - { success: Boolean, error: String, access_token: String, refresh_token: String, 
        //                     user_id: String, device_id: String, expire: String, scope: String }
        // let response = GrantType::from_str(GetAuthUrlRequest.grant_type).authorize_user().unwrap().await;
        // Ok(response)
    }

    async fn unauthorize_user(
        &self,
        _ctx: &Context,
        _req: &UnauthorizeUserRequest,
    ) -> RpcResult<UnauthorizeUserResponse> {
        todo!();
        // Request Struct - { user: String, device_id: String }
        // Response Struct - { success: Boolean, error: String }
        // let response = GrantType::from_str(GetAuthUrlRequest.grant_type).unauthorize_user().unwrap().await;
        // Ok(response)
    }
}
