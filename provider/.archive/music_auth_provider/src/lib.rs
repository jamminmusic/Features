#[allow(unused_imports)]
pub mod config;
use anyhow;
use config::{MusicAuthConfig, MusicAuthProviders};
use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use oauth2::{
    url, AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, HttpRequest,
    PkceCodeChallenge, PkceCodeVerifier, RedirectUrl, Scope, TokenResponse, TokenUrl,
};
use std::collections::HashMap;
use std::str::FromStr;
use wasmbus_rpc::provider::prelude::*;

async fn define_auth_provider(
    provider: &String,
    req: &HashMap<String, String>,
) -> RpcResult<MusicAuthConfig> {
    // 1. Pull in Client config: auth url, token url, client id, client secret
    // 2. Create Client
    // 3. Set redirect URI
    let secrets = MusicAuthProviders::from_str(provider)
        .unwrap()
        .get_config(Some(&req))
        .await
        .unwrap();
    let config = MusicAuthProviders::from_str(provider)
        .unwrap()
        .set_config(&secrets)
        .await
        .unwrap();
    Ok(config)
}

// Convert to using ngs with websocket
async fn process_request_headers(req: &HashMap<String, String>) -> RpcResult<String> {
    let client_origin = req
        .get(&"forwarded" as &str)
        .expect("Could not get forwarded header")
        .to_string();
    Ok(client_origin)
}

// client origin must be passed in here
async fn create_client(config: MusicAuthConfig, client_origin: String) -> RpcResult<BasicClient> {
    let client = BasicClient::new(
        ClientId::new(config.client_id.unwrap()),
        Some(ClientSecret::new(config.client_secret.unwrap())),
        AuthUrl::new(config.auth_url.unwrap()).expect("Invalid authorization endpoint URL"),
        Some(TokenUrl::new(config.token_url.unwrap()).expect("Invalid authorization endpoint URL")),
    )
    .set_redirect_uri(
        RedirectUrl::new("http://local.jammin.dev".to_string()).expect("Invalid redirect URL"),
    );
    Ok(client)
}

async fn generate_pkce() -> RpcResult<(PkceCodeChallenge, PkceCodeVerifier)> {
    let (pkce_code_challenge, pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();
    // what to return?
    Ok((pkce_code_challenge, pkce_code_verifier))
}

async fn generate_auth_url(
    client: BasicClient,
    pkce_code_challenge: Option<PkceCodeChallenge>,
    config: MusicAuthConfig,
) -> RpcResult<(url::Url, CsrfToken)> {
    let (authorize_url, csrf_state) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new(config.scope.unwrap()))
        .set_pkce_challenge(pkce_code_challenge.unwrap())
        .url();
    Ok((authorize_url, csrf_state))
}

async fn client_redirect(client: BasicClient, auth_url: url::Url) {
    // Once the user has been redirected to the redirect URL, you'll have access to the
    // authorization code. For security reasons, your code should verify that the `state`
    // parameter returned by the server matches `csrf_state`.

    // let client = reqwest::Client::new();
    // let res = client
    //     .post("http://local.jammin.dev")
    //     .body(auth_url.to_string())
    //     .send()
    //     .await;

    // USE HTTP POLLING FROM CLIENT TO RETURN THE AUTH URL INSTEAD OF TRYING TO INITIATE AN OUTBOUND REQUEST FROM THE PROVIDER.

}

// async fn compare_csrf_state (auth_code: String, csrf_state: CsrfToken, csrf_response: ){
//     if csrf_state == csrf_response {
//         // OK
//     } else {
//         // Error
//     }
// }

// async fn token_exchange(authorization_code){
//     let token_result = client
//         .exchange_code(AuthorizationCode::new("some authorization code".to_string()))
//         // Set the PKCE code verifier.
//         .set_pkce_verifier(pkce_verifier)
//         .request_async(async_http_client)
//         .await?;
// }
