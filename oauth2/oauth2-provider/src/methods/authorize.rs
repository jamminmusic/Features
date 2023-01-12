#[allow(unused_imports)]
use oauth2_interface::{ AuthorizeUserRequest, AuthorizeUserResponse };
use oauth2::basic::BasicClient;
use oauth2::devicecode::StandardDeviceAuthorizationResponse;
use oauth2::reqwest::async_http_client;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, DeviceAuthorizationUrl,
    PkceCodeChallenge, PkceCodeVerifier, RedirectUrl, Scope, TokenResponse, TokenUrl,
};
use url::Url;

#[derive(Default)]
pub struct AuthUserBuilder {
    success: bool, 
    error: Option<String>, 
    access_token: Option<String>, 
    refresh_token: Option<String>, 
    user_id: Option<String>, 
    device_id: Option<String>, 
    scope: Option<String>,
    expire: Option<u64>,
}

impl AuthUserBuilder {
    pub fn new() -> Self {
        Self {
            success: false, 
            error: None, 
            access_token: None, 
            refresh_token: None, 
            user_id: None, 
            device_id: None, 
            scope: None,
            expire: None
        }
    }

    async fn compare_csrf_state(){
        todo!();

        // if csrf_state == state {
        //     // OK
        // } else {
        //     // Error
        // }
    }


    // Only supporting Bearer Tokens
    async fn token_exchange(){
        todo!();

        // client here  = (auth_url, csrf_token)
        // let token_result = client
        //     .exchange_code(AuthorizationCode::new("some authorization code".to_string()))
        //     // Set the PKCE code verifier.
        //     .set_pkce_verifier(pkce_verifier)
        //     .request_async(async_http_client)
        //     .await?;
    }

    async fn device_token_exchange(){
        todo!();
        // let details: StandardDeviceAuthorizationResponse = client
        // .exchange_device_code()?
        // .add_scope(Scope::new("read".to_string()))
        // .request(http_client)?;
        // let details: StandardDeviceAuthorizationResponse = client
        // .exchange_device_code()?
        // .add_scope(Scope::new("read".to_string()))
        // .request(http_client)?;

        // println!(
        //     "Open this URL in your browser:\n{}\nand enter the code: {}",
        //     details.verification_url().to_string(),
        //     details.user_code().secret().to_string()
        // );

        // let token_result =
        //     client
        //     .exchange_device_access_token(&details)
        //     .request(http_client, std::thread::sleep, None)?;
    }

    // TODO
    pub fn build(self) -> AuthorizeUserResponse {
        AuthorizeUserResponse { 
            success: self.success, error: self.error, 
            access_token: self.access_token.unwrap(), refresh_token: self.refresh_token, 
            user_id: self.user_id, device_id: self.device_id, scope: self.scope.unwrap(), expire: self.expire.unwrap(),
        }
    }
}
