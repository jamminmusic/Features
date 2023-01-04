use wasmbus_rpc::provider::prelude::*;
use oauth2_provider::Oauth2Provider;
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

#[tokio::test]
async fn run_all() {
    let opts = TestOptions::default();
    let res = run_selected_spawn!(
        &opts, health_check, 
        authorization_code_url_test, pkce_url_test, refresh_url_test, client_credentials_url_test, device_code_url_test,
        authorization_code_auth_test, pkce_auth_test, refresh_auth_test, client_credentials_auth_test, device_code_auth_test,
        authorization_code_unauth_test, pkce_unauth_test, refresh_unauth_test, client_credentials_unauth_test, device_code_unauth_test,
        case_insensitive_test,
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
async fn authorization_code_url_test(_opt: &TestOptions) -> RpcResult<()>{
    let key = "PROVIDER_TEST_CONFIG";
    env::set_var(key, "./config/authorization_code_test_config.toml");
    let ctx = wasmbus_rpc::common::Context::default();

    let prov = test_provider().await;
    env_logger::try_init().ok();

    // use mock secrets to verify pulling correctly, then pull real secrets below.
    let secrets = SecretsManager::load("config/secure/AuthCode_GetAuthUrl.json", KeySource::File(Path::new("config/secure/AuthCode_GetAuthUrl.json")))
    .expect("Failed to load SecureStore vault!");

    let req = GetAuthUrlRequest {
        grant_type: "AuthorizationCode".to_string(),
        client_id: secrets.get("client_id:password").expect("secure store error"),
        device_code: None,
        client_secret: Some(secrets.get("client_secret:password").expect("secure store error")),
        auth_url: secrets.get("auth_url:username").expect("secure store error"),
        token_url: secrets.get("token_url:username").expect("secure store error"),
        redirect_url: Some(secrets.get("redirect_url:username").expect("secure store error")),
        scope: secrets.get("scope:username").expect("secure store error"),
        device_auth_url:  None,
    };

    let auth_url = <Oauth2Provider as Oauth2>::get_auth_url(&ctx, &req).await.unwrap();

    println!("{:?}", auth_url);
    Ok(())
}

async fn pkce_url_test(_opt: &TestOptions) -> RpcResult<()> {
    todo!();
    // let key = "PROVIDER_TEST_CONFIG";
    // env::set_var(key, "./config/pkce_test_config.toml");
    // let ctx = wasmbus_rpc::common::Context::default();

    // let prov = test_provider().await;
    // env_logger::try_init().ok();
}

async fn refresh_url_test(_opt: &TestOptions) -> RpcResult<()> {
    todo!();
    // let key = "PROVIDER_TEST_CONFIG";
    // env::set_var(key, "./config/refresh_test_config.toml");
    // let ctx = wasmbus_rpc::common::Context::default();

    // let prov = test_provider().await;
    // env_logger::try_init().ok();
}

async fn client_credentials_url_test(_opt: &TestOptions) -> RpcResult<()> {
    todo!();
    // let key = "PROVIDER_TEST_CONFIG";
    // env::set_var(key, "./config/client_credentials_test_config.toml");
    // let ctx = wasmbus_rpc::common::Context::default();

    // let prov = test_provider().await;
    // env_logger::try_init().ok();
}

async fn device_code_url_test(_opt: &TestOptions) -> RpcResult<()> {
    todo!();
    // let key = "PROVIDER_TEST_CONFIG";
    // env::set_var(key, "./config/device_code_test_config.toml");
    // let ctx = wasmbus_rpc::common::Context::default();

    // let prov = test_provider().await;
    // env_logger::try_init().ok();
}

/// tests for available grant types with AuthorizeUser method 
async fn authorization_code_auth_test(_opt: &TestOptions) -> RpcResult<()> {
    todo!();
    // let key = "PROVIDER_TEST_CONFIG";
    // env::set_var(key, "./config/authorization_code_test_config.toml");
    // let ctx = wasmbus_rpc::common::Context::default();

    // let prov = test_provider().await;
    // env_logger::try_init().ok();
}

async fn pkce_auth_test(_opt: &TestOptions) -> RpcResult<()> {
    todo!();
    // let key = "PROVIDER_TEST_CONFIG";
    // env::set_var(key, "./config/pkce_test_config.toml");
    // let ctx = wasmbus_rpc::common::Context::default();

    // let prov = test_provider().await;
    // env_logger::try_init().ok();
}

async fn refresh_auth_test(_opt: &TestOptions) -> RpcResult<()> {
    todo!();
    // let key = "PROVIDER_TEST_CONFIG";
    // env::set_var(key, "./config/refresh_test_config.toml");
    // let ctx = wasmbus_rpc::common::Context::default();

    // let prov = test_provider().await;
    // env_logger::try_init().ok();
}

async fn client_credentials_auth_test(_opt: &TestOptions) -> RpcResult<()> {
    todo!();
    // let key = "PROVIDER_TEST_CONFIG";
    // env::set_var(key, "./config/client_credentials_test_config.toml");
    // let ctx = wasmbus_rpc::common::Context::default();

    // let prov = test_provider().await;
    // env_logger::try_init().ok();
}

async fn device_code_auth_test(_opt: &TestOptions) -> RpcResult<()> {
    todo!();
    // let key = "PROVIDER_TEST_CONFIG";
    // env::set_var(key, "./config/device_code_test_config.toml");
    // let ctx = wasmbus_rpc::common::Context::default();

    // let prov = test_provider().await;
    // env_logger::try_init().ok();
}

/// tests for available grant types with UnauthorizeUser method 
async fn authorization_code_unauth_test(_opt: &TestOptions) -> RpcResult<()> {
    todo!();
    // let key = "PROVIDER_TEST_CONFIG";
    // env::set_var(key, "./config/authorization_code_test_config.toml");
    // let ctx = wasmbus_rpc::common::Context::default();

    // let prov = test_provider().await;
    // env_logger::try_init().ok();
}

async fn pkce_unauth_test(_opt: &TestOptions) -> RpcResult<()> {
    todo!();
    // let key = "PROVIDER_TEST_CONFIG";
    // env::set_var(key, "./config/pkce_test_config.toml");
    // let ctx = wasmbus_rpc::common::Context::default();

    // let prov = test_provider().await;
    // env_logger::try_init().ok();
}

async fn refresh_unauth_test(_opt: &TestOptions) -> RpcResult<()> {
    todo!();
    // let key = "PROVIDER_TEST_CONFIG";
    // env::set_var(key, "./config/refresh_test_config.toml");
    // let ctx = wasmbus_rpc::common::Context::default();

    // let prov = test_provider().await;
    // env_logger::try_init().ok();
}

async fn client_credentials_unauth_test(_opt: &TestOptions) -> RpcResult<()> {
    todo!();
    // let key = "PROVIDER_TEST_CONFIG";
    // env::set_var(key, "./config/client_credentials_test_config.toml");
    // let ctx = wasmbus_rpc::common::Context::default();

    // let prov = test_provider().await;
    // env_logger::try_init().ok();
}

async fn device_code_unauth_test(_opt: &TestOptions) -> RpcResult<()> {
    todo!();
    // let key = "PROVIDER_TEST_CONFIG";
    // env::set_var(key, "./config/device_code_test_config.toml");
    // let ctx = wasmbus_rpc::common::Context::default();

    // let prov = test_provider().await;
    // env_logger::try_init().ok();
}

// test case insensitive grant type value.
async fn case_insensitive_test(_opt: &TestOptions) -> RpcResult<()> {
    todo!();
    // let key = "PROVIDER_TEST_CONFIG";
    // env::set_var(key, "./config/case_insensitive_test_config.toml");
    // let ctx = wasmbus_rpc::common::Context::default();

    // let prov = test_provider().await;
    // env_logger::try_init().ok();
}
