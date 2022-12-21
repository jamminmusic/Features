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

#[derive(Default)]
pub struct AuthUriBuilder {
    // Probably lots of optional fields.
    client: &BasicClient,
    pkce_code_challenge: Option<&PkceCodeChallenge>,
    scope: &GetAuthUriRequest.scope
}

impl AuthUriBuilder {
    fn new() -> AuthUriBuilder {
        AuthUriBuilder {
            client: BasicClient,
            redirect_uri: String,
            auth_uri: (Url, CsrfToken),
            pkce: (PkceCodeChallenge, PkceCodeVerifier),
            auth_uri_pkce: (Url, CsrfToken),
            device_client: BasicClient,
            auth_uri_device: (Url, CsrfToken)
        }
    }

    fn create_client(req: &GetAuthUriRequest) -> Result<BasicClient, Error> {
        let client = BasicClient::new(
            ClientId::new(req.client_id.unwrap()),
            Some(ClientSecret::new(req.client_secret.unwrap())),
            AuthUrl::new(req.auth_url.unwrap()).expect("Invalid authorization endpoint URL"),
            Some(TokenUrl::new(req.token_url.unwrap()).expect("Invalid authorization endpoint URL")),
        );
        Ok(client)
    }

    fn set_redirect_uri(client: BasicClient) -> Result<BasicClient, Error>{
        let client = client.set_redirect_uri(
                    RedirectUrl::new(req.auth_url.unwrap()).expect("Invalid redirect URL"));
        Ok(client)        
    }

    fn generate_auth_uri(
        client: &BasicClient,
        pkce_code_challenge: Option<&PkceCodeChallenge>,
        scope: &GetAuthUriRequest.scope
    ) -> Result<(url::Url, CsrfToken), Error> {
        let (authorize_url, csrf_state) = client
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new(config.scope.unwrap()))
            .url();
        Ok((authorize_url, csrf_state))
    }

    fn generate_pkce() -> Result<(PkceCodeChallenge, PkceCodeVerifier), Error> {
        let (pkce_code_challenge, pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();
        // what to return?
        Ok((pkce_code_challenge, pkce_code_verifier))
    }

    fn generate_auth_uri_pkce(
        client: &BasicClient,
        pkce_code_challenge: Option<&PkceCodeChallenge>,
        scope: &GetAuthUriRequest.scope
    ) -> Result<(url::Url, CsrfToken), Error> {
        let (authorize_url, csrf_state) = client
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new(config.scope.unwrap()))
            .set_pkce_challenge(pkce_code_challenge.unwrap())
            .url();
        Ok((authorize_url, csrf_state))
    }

    fn set_device_uri(client: BasicClient) -> Result<BasicClient, Error>{
        let client = client.set_redirect_uri(
                    RedirectUrl::new(DeviceAuthorizationUrl::new(req.auth_url.unwrap())).expect("Invalid redirect URL"));
        Ok(client)        
    }

    fn generate_device_auth_uri(
        client: &BasicClient,
        pkce_code_challenge: Option<&PkceCodeChallenge>,
        scope: &GetAuthUriRequest.scope
    ) -> Result<(url::Url, CsrfToken), Error> {
        // csrf token and url method need to be verified to work with device flow. This may be wrong
        let (authorize_url, csrf_state) = client.authorize_url(CsrfToken::new_random).url();
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
