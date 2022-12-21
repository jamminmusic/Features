#[allow(unused_imports)]
use anyhow;
use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use oauth2::{
    url, AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, HttpRequest,
    PkceCodeChallenge, PkceCodeVerifier, RedirectUrl, Scope, TokenResponse, TokenUrl, DeviceAuthorizationUrl,
};
use std::collections::HashMap;
use std::str::FromStr;
use wasmbus_rpc::provider::prelude::*;
use oauth2::devicecode::StandardDeviceAuthorizationResponse;


#[derive(Debug, PartialEq)]
pub struct AuthUri {
    auth_uri: Option((Url, CsrfToken)),
    auth_uri_pkce: Option((Url, CsrfToken)),
    auth_uri_device: Option((Url, CsrfToken))
}

impl AuthUri {
    // This method will help users to discover the builder
    pub fn builder() -> AuthUriBuilder {
        AuthUriBuilder::default()
    }
}

#[derive(Default)]
pub struct AuthUriBuilder {
    client: Option(BasicClient),
    redirect_uri: Option(String),
    auth_uri: Option((Url, CsrfToken)),
    pkce: Option((PkceCodeChallenge, PkceCodeVerifier)),
    auth_uri_pkce: Option((Url, CsrfToken)),
    device_client: Option(BasicClient),
    auth_uri_device: Option((Url, CsrfToken))
}

impl AuthUriBuilder {
    pub fn new() -> AuthUriBuilder {
        AuthUriBuilder {
            client: None,
            redirect_uri: None,
            auth_uri: None,
            pkce: None,
            auth_uri_pkce: None,
            device_client: None,
            auth_uri_device: None
        }
    }

    pub fn create_client(mut self, req: &GetAuthUriRequest) -> Self {
        let self.client = BasicClient::new(
            ClientId::new(req.client_id.unwrap()),
            Some(ClientSecret::new(req.client_secret.unwrap())),
            AuthUrl::new(req.auth_uri.unwrap()).expect("Invalid authorization endpoint URL"),
            Some(TokenUrl::new(req.token_uri.unwrap()).expect("Invalid authorization endpoint URL")),
        );
        Some(self)
    }

    pub fn set_redirect_uri(mut self, req: &GetAuthUriRequest) -> Self {
        let self.client = self.set_redirect_uri(
                    RedirectUrl::new(req.auth_uri.unwrap()).expect("Invalid redirect URL"));
        Some(self)    
    }

    pub fn generate_auth_uri(mut self, req: &GetAuthUriRequest) -> Self {
        let self.auth_uri = self
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new(req.scope.unwrap()))
            .url();
        Some(self) 
    }

    pub fn generate_pkce(mut self) -> Self {
        let self.pkce = PkceCodeChallenge::new_random_sha256();
        Some(self)
    }

    pub fn generate_auth_uri_pkce(mut self, req: &GetAuthUriRequest) -> Self {
        let self.auth_uri_pkce = self
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new(req.scope.unwrap()))
            .set_pkce_challenge(pkce_code_challenge.unwrap())
            .url();
        Some(self) 
    }

    pub fn set_device_uri(mut self, req: &GetAuthUriRequest) -> Self {
        let self.client = self.set_redirect_uri(
                    RedirectUrl::new(DeviceAuthorizationUrl::new(req.auth_uri.unwrap())).expect("Invalid redirect URL"));
        Some(self)        
    }

    pub fn generate_device_auth_uri(mut self) -> Self {
        let self.auth_uri_device = self.authorize_url(CsrfToken::new_random).url();
        Some(self) 
    }
    pub fn build(self) -> AuthUri {
        AuthUri { auth_uri: self. auth_uri_pkce: self, auth_uri_device: self }
    }
}

async fn compare_csrf_state(auth_code: String, csrf_state: CsrfToken, csrf_response: ){
    if csrf_state == csrf_response {
        // OK
    } else {
        // Error
    }
}

async fn token_exchange(authorization_code){
    let token_result = client
        .exchange_code(AuthorizationCode::new("some authorization code".to_string()))
        // Set the PKCE code verifier.
        .set_pkce_verifier(pkce_verifier)
        .request_async(async_http_client)
        .await?;
}

async fn device_token_exchange(authorization_code){

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




//-----------UNIT TESTS-----------------
#[test]
fn builder_test() {
    let foo = Foo {
        bar: String::from("Y"),
    };
    let foo_from_builder: Foo = FooBuilder::new().name(String::from("Y")).build();
    assert_eq!(foo, foo_from_builder);
}