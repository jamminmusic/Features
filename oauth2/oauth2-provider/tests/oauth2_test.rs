use wasmbus_rpc::provider::prelude::*;
use oauth2_interface::*;
// AuthorizeUserRequest, AuthorizeUserResponse, 
// GetAuthUrlRequest, GetAuthUrlResponse, 
// Oauth2, Oauth2Receiver, 
// UnauthorizeUserRequest, UnauthorizeUserResponse 
use wasmcloud_test_util::{
    check,
    cli::print_test_results,
    provider_test::test_provider,
    testing::{TestOptions, TestResult},
};
#[allow(unused_imports)]
use wasmcloud_test_util::{run_selected, run_selected_spawn};
use std::env;

// To reference secrets for tests
use securestore::{KeySource, SecretsManager};
use std::path::Path;
use tempfile::NamedTempFile;

#[tokio::test]
async fn run_all() {
    let opts = TestOptions::default();
    let res = run_selected_spawn!(
        &opts, health_check, 
        authorization_code_url_test, pkce_url_test, refresh_url_test, client_credentials_url_test, device_code_url_test,
        authorization_code_auth_test, pkce_auth_test, refresh_auth_test, client_credentials_auth_test, device_code_auth_test,
        authorization_code_unauth_test, pkce_unauth_test, refresh_unauth_test, client_credentials_unauth_test, device_code_unauth_test
    );
    print_test_results(&res);

    let passed = res.iter().filter(|tr| tr.passed).count();
    let total = res.len();
    assert_eq!(passed, total, "{} passed out of {}", passed, total);

    // try to let the provider shut dowwn gracefully
    let provider = test_provider().await;
    let _ = provider.shutdown().await;
}

/// test that health check returns healthy
async fn health_check(_opt: &TestOptions) -> RpcResult<()> {
    let prov = test_provider().await;

    // health check
    let hc = prov.health_check().await;
    check!(hc.is_ok())?;
    Ok(())
}

/// tests for available grant types with GetAuthorizationUrl method 
// test provider should load values from config as per https://github.com/wasmCloud/wasmcloud-test/blob/main/wasmcloud-test-util/src/provider_test.rs#L496
async fn authorization_code_url_test(_opt: &TestOptions) -> RpcResult<()> {
    let key = "PROVIDER_TEST_CONFIG";
    env::set_var(key, "./config/authorization_code_test_config.toml");

    let prov = test_provider().await;
    env_logger::try_init().ok();

    let secrets = SecretsManager::load("config/secure/test.json", KeySource::File(Path::new("config/secure/spotify.key")))
    .expect("Failed to load SecureStore vault!")

    let req = GetAuthorizationUrlRequest {
        // required
        grant_type: "AuthorizationCode",
        // required + sensitive
        client_id: Some(secrets.get("client_id:password")?),
        // sensitive
        device_code: None,
        // sensitive
        client_secret: Some(secrets.get("client_secret:password")?),
        // required
        auth_url: Some(secrets.get("auth_url:username")?),
        // required
        token_url: Some(secrets.get("token_url:username")?),
        // no tag
        redirect_url: Some(secrets.get("redirect_url:username")?),
        // required
        scope: Some(secrets.get("scope:username")?),
        // no tag
        device_auth_url:  None,
    };
}

async fn pkce_url_test(_opt: &TestOptions) -> RpcResult<()> {
    todo!();
    let key = "PROVIDER_TEST_CONFIG";
    env::set_var(key, "./config/pkce_test_config.toml");

    let prov = test_provider().await;
    env_logger::try_init().ok();
}

async fn refresh_url_test(_opt: &TestOptions) -> RpcResult<()> {
    todo!();
    let key = "PROVIDER_TEST_CONFIG";
    env::set_var(key, "./config/refresh_test_config.toml");

    let prov = test_provider().await;
    env_logger::try_init().ok();
}

async fn client_credentials_url_test(_opt: &TestOptions) -> RpcResult<()> {
    todo!();
    let key = "PROVIDER_TEST_CONFIG";
    env::set_var(key, "./config/client_credentials_test_config.toml");

    let prov = test_provider().await;
    env_logger::try_init().ok();
}

async fn device_code_url_test(_opt: &TestOptions) -> RpcResult<()> {
    todo!();
    let key = "PROVIDER_TEST_CONFIG";
    env::set_var(key, "./config/device_code_test_config.toml");

    let prov = test_provider().await;
    env_logger::try_init().ok();
}

/// tests for available grant types with AuthorizeUser method 
async fn authorization_code_auth_test(_opt: &TestOptions) -> RpcResult<()> {
    todo!();
    let key = "PROVIDER_TEST_CONFIG";
    env::set_var(key, "./config/authorization_code_test_config.toml");

    let prov = test_provider().await;
    env_logger::try_init().ok();
}

async fn pkce_auth_test(_opt: &TestOptions) -> RpcResult<()> {
    todo!();
    let key = "PROVIDER_TEST_CONFIG";
    env::set_var(key, "./config/pkce_test_config.toml");

    let prov = test_provider().await;
    env_logger::try_init().ok();
}

async fn refresh_auth_test(_opt: &TestOptions) -> RpcResult<()> {
    todo!();
    let key = "PROVIDER_TEST_CONFIG";
    env::set_var(key, "./config/refresh_test_config.toml");

    let prov = test_provider().await;
    env_logger::try_init().ok();
}

async fn client_credentials_auth_test(_opt: &TestOptions) -> RpcResult<()> {
    todo!();
    let key = "PROVIDER_TEST_CONFIG";
    env::set_var(key, "./config/client_credentials_test_config.toml");

    let prov = test_provider().await;
    env_logger::try_init().ok();
}

async fn device_code_auth_test(_opt: &TestOptions) -> RpcResult<()> {
    todo!();
    let key = "PROVIDER_TEST_CONFIG";
    env::set_var(key, "./config/device_code_test_config.toml");

    let prov = test_provider().await;
    env_logger::try_init().ok();
}

/// tests for available grant types with UnauthorizeUser method 
async fn authorization_code_unauth_test(_opt: &TestOptions) -> RpcResult<()> {
    todo!();
    let key = "PROVIDER_TEST_CONFIG";
    env::set_var(key, "./config/authorization_code_test_config.toml");

    let prov = test_provider().await;
    env_logger::try_init().ok();
}

async fn pkce_unauth_test(_opt: &TestOptions) -> RpcResult<()> {
    todo!();
    let key = "PROVIDER_TEST_CONFIG";
    env::set_var(key, "./config/pkce_test_config.toml");

    let prov = test_provider().await;
    env_logger::try_init().ok();
}

async fn refresh_unauth_test(_opt: &TestOptions) -> RpcResult<()> {
    todo!();
    let key = "PROVIDER_TEST_CONFIG";
    env::set_var(key, "./config/refresh_test_config.toml");

    let prov = test_provider().await;
    env_logger::try_init().ok();
}

async fn client_credentials_unauth_test(_opt: &TestOptions) -> RpcResult<()> {
    todo!();
    let key = "PROVIDER_TEST_CONFIG";
    env::set_var(key, "./config/client_credentials_test_config.toml");

    let prov = test_provider().await;
    env_logger::try_init().ok();
}

async fn device_code_unauth_test(_opt: &TestOptions) -> RpcResult<()> {
    todo!();
    let key = "PROVIDER_TEST_CONFIG";
    env::set_var(key, "./config/device_code_test_config.toml");

    let prov = test_provider().await;
    env_logger::try_init().ok();
}

// test case insensitive grant type value.
async fn case_insensitive_test(_opt: &TestOptions) -> RpcResult<()> {
    todo!();
    let key = "PROVIDER_TEST_CONFIG";
    env::set_var(key, "./config/case_insensitive_test_config.toml");

    let prov = test_provider().await;
    env_logger::try_init().ok();
}
