//! oauth2 capability provider
//!
//!
use oauth2_interface::{
    AuthorizeUserRequest, AuthorizeUserResponse, 
    GetAuthUriRequest, GetAuthUriResponse, 
    Oauth2, Oauth2Receiver, 
    UnauthorizeUserRequest, UnauthorizeUserResponse 
};
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
        let x = GetAuthUriResponse {
            success: true,
            error: Some("words".to_string()),
            uri: Some("words".to_string()),
            csrf_state: Some("words".to_string()),
        };
        Ok(x)
    }

    async fn authorize_user(
        &self,
        _ctx: &Context,
        _req: &AuthorizeUserRequest,
    ) -> RpcResult<AuthorizeUserResponse> {
        let x = AuthorizeUserResponse {
            success: true,
            error: Some("words".to_string()),
            token: Some("words".to_string()),
        };
        Ok(x)
    }

    async fn unauthorize_user(
        &self,
        _ctx: &Context,
        _req: &UnauthorizeUserRequest,
    ) -> RpcResult<UnauthorizeUserResponse> {
        let x = UnauthorizeUserResponse {
            success: true,
            error: Some("words".to_string()),
        };
        Ok(x)
    }
}
