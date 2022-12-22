#[allow(unused_imports)]
use anyhow;
use oauth2_interface::GetAuthUriRequest;
use oauth2::{
    AuthorizationCode,
    AuthUrl,
    ClientId,
    ClientSecret,
    CsrfToken,
    DeviceAuthorizationUrl,
    PkceCodeChallenge,
    PkceCodeVerifier,
    RedirectUrl,
    Scope,
    TokenResponse,
    TokenUrl
};
use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use oauth2::devicecode::StandardDeviceAuthorizationResponse;
use url::Url;
use std::collections::HashMap;
use std::str::FromStr;
use wasmbus_rpc::provider::prelude::*;


#[derive(Debug, PartialEq)]
pub struct AuthUri {
    success: bool, 
    error: Option<String>, 
    uri: String, 
    csrf_state: String
}

impl AuthUri {
    pub fn builder() -> AuthUriBuilder {
        AuthUriBuilder::default()
    }
}

#[derive(Default)]
pub struct AuthUriBuilder {
    client: Option<oauth2::basic::BasicClient>,
    // may need another field for device uri
    auth_uri: Option<(Url, CsrfToken)>,
    pkce: Option<(PkceCodeChallenge, PkceCodeVerifier)>,
    success: bool,
    error: Option<String>
}

impl AuthUriBuilder {
    pub fn new() -> AuthUriBuilder {
        AuthUriBuilder {
            client: None,
            auth_uri: None,
            pkce: None,
            success: false,
            error: None,
        }
    }

    pub fn create_client(mut self, req: &GetAuthUriRequest) -> Self {
        self.client = Some(BasicClient::new(
            ClientId::new(req.client_id),
            Some(ClientSecret::new(req.client_secret.unwrap())),
            AuthUrl::new(req.auth_uri).expect("Invalid authorization endpoint URL"),
            Some(TokenUrl::new(req.token_uri).expect("Invalid authorization endpoint URL")),
        ));
        self
    }

    pub fn set_redirect_uri(mut self, req: &GetAuthUriRequest) -> Self {
        self.client = Some(self.client.unwrap().set_redirect_uri(
                    RedirectUrl::new(req.auth_uri).expect("Invalid redirect URL")));
        self    
    }

    pub fn generate_auth_uri(mut self, req: &GetAuthUriRequest) -> Self {
        self.auth_uri = Some(self.client.unwrap()
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new(req.scope))
            .url());

        // TODO - how to validate auth_uri to return true
        self.success = true; 
        self 
    }

    pub fn generate_pkce(mut self) -> Self {
        self.pkce = Some(PkceCodeChallenge::new_random_sha256());
        self
    }

    pub fn generate_auth_uri_pkce(mut self, req: &GetAuthUriRequest) -> Self {
        self.auth_uri = Some(self.client.unwrap()
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new(req.scope))
            .set_pkce_challenge(self.pkce.unwrap().0)
            .url());

        // TODO - how to validate auth_uri to return true
        self.success = true;  
        self 
    }

    pub fn set_device_uri(mut self, req: &GetAuthUriRequest) -> Self {
        unimplemented!();

        // TODO - https://docs.rs/oauth2/latest/oauth2/#device-code-flow
        // self.client = Some(self.client.unwrap().set_device_authorization_url(DeviceAuthorizationUrl::new(req.auth_uri)).expect("Invalid redirect URL"));
        // self        
    }

    pub fn generate_device_auth_uri(mut self) -> Self {
        unimplemented!();

        // // TODO - https://docs.rs/oauth2/latest/oauth2/#device-code-flow
        // self.auth_uri = Some(self
        //     // look this up
        //     .exchange_device_code()
        //     .add_scope(Scope::new("read".to_string()))
        //     // look this up
        //     .request(http_client));

        // // TODO - how to validate auth_uri to return true
        // self.success = true; 
        // self 
    }
    pub fn build(self) -> AuthUri {
        AuthUri { 
            success: self.success, error: self.error,
            // need to create a smithy model for CsrfToken, or create one as a string - https://docs.rs/oauth2/latest/oauth2/struct.CsrfToken.html#method.secret
            uri: self.auth_uri.unwrap().0.to_string(), csrf_state: self.auth_uri.unwrap().1 
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct AuthUser {
    success: bool, 
    error: Option<String>, 
    access_token: Option<String>, 
    refresh_token: Option<String>, 
    user_id: Option<String>, 
    device_id: Option<String>, 
    scope: Option<String>
}

impl AuthUser {
    pub fn builder() -> AuthUserBuilder {
        AuthUserBuilder::default()
    }
}

#[derive(Default)]
pub struct AuthUserBuilder {
    success: bool, 
    error: Option<String>, 
    access_token: Option<String>, 
    refresh_token: Option<String>, 
    user_id: Option<String>, 
    device_id: Option<String>, 
    scope: Option<String>
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
            scope: None
        }
    }
    // TODO
    async fn compare_csrf_state(){
        unimplemented!();

        // if csrf_state == state {
        //     // OK
        // } else {
        //     // Error
        // }
    }


    // TODO
    async fn token_exchange(){
        unimplemented!();

        // let token_result = client
        //     .exchange_code(AuthorizationCode::new("some authorization code".to_string()))
        //     // Set the PKCE code verifier.
        //     .set_pkce_verifier(pkce_verifier)
        //     .request_async(async_http_client)
        //     .await?;
    }

    // TODO
    async fn device_token_exchange(){

        unimplemented!();
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
        //     details.verification_uri().to_string(),
        //     details.user_code().secret().to_string()
        // );

        // let token_result =
        //     client
        //     .exchange_device_access_token(&details)
        //     .request(http_client, std::thread::sleep, None)?;
    }

    // TODO
    pub fn build(self) -> AuthUser {
        AuthUser { 
            success: self.success, error: self.error, 
            access_token: self.access_token, refresh_token: self.refresh_token, 
            user_id: self.user_id, device_id: self.device_id, scope: self.scope 
        }
    }
}







// TODO
//-----------UNIT TESTS-----------------
#[test]
fn builder_test() {
    let foo = Foo {
        bar: String::from("Y"),
    };
    let foo_from_builder: Foo = FooBuilder::new().name(String::from("Y")).build();
    assert_eq!(foo, foo_from_builder);
}