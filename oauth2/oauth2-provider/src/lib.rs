#[allow(unused_imports)]
use anyhow;
use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use oauth2::{
    url, AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, HttpRequest,
    PkceCodeChallenge, PkceCodeVerifier, RedirectUrl, Scope, TokenResponse, TokenUrl,
};
use std::collections::HashMap;
use std::str::FromStr;
use wasmbus_rpc::provider::prelude::*;


// client origin must be passed in here
async fn create_client(req: &GetAuthUriRequest) -> Result<BasicClient, Error> {
    let client = BasicClient::new(
        ClientId::new(req.client_id.unwrap()),
        Some(ClientSecret::new(req.client_secret.unwrap())),
        AuthUrl::new(req.auth_url.unwrap()).expect("Invalid authorization endpoint URL"),
        Some(TokenUrl::new(req.token_url.unwrap()).expect("Invalid authorization endpoint URL")),
    )
    .set_redirect_uri(
        RedirectUrl::new(req.auth_url.unwrap()).expect("Invalid redirect URL"),
    );
    Ok(client)
}

async fn generate_pkce() -> Result<(PkceCodeChallenge, PkceCodeVerifier), Error> {
    let (pkce_code_challenge, pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();
    // what to return?
    Ok((pkce_code_challenge, pkce_code_verifier))
}

async fn generate_auth_url(
    client: BasicClient,
    pkce_code_challenge: Option<PkceCodeChallenge>,
    config: TBD,
) -> Result<(url::Url, CsrfToken), Error> {
    let (authorize_url, csrf_state) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new(config.scope.unwrap()))
        // Use builder pattern here to make set pkce an optional chained method
        .set_pkce_challenge(pkce_code_challenge.unwrap())
        .url();
    Ok((authorize_url, csrf_state))
}

// Use NATS Provider HERE
async fn client_redirect(client: BasicClient, auth_url: url::Url) {
    // Once the user has been redirected to the redirect URL, you'll have access to the
    // authorization code. For security reasons, your code should verify that the `state`
    // parameter returned by the server matches `csrf_state`.
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
