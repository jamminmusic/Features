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

#[tokio::test]
async fn run_all() {
    let opts = TestOptions::default();
    let res = run_selected_spawn!(&opts, health_check, authorization_code_url_test, pkce_url_test, refresh_url_test, client_credentials_url_test, device_code_url_test);
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
async fn authorization_code_url_test(_opt: &TestOptions) -> RpcResult<()> {
    let prov = test_provider().await;
    env_logger::try_init().ok();
}

async fn pkce_url_test(_opt: &TestOptions) -> RpcResult<()> {
    todo!();
}

async fn refresh_url_test(_opt: &TestOptions) -> RpcResult<()> {
    todo!();
}

async fn client_credentials_url_test(_opt: &TestOptions) -> RpcResult<()> {
    todo!();
}

async fn device_code_url_test(_opt: &TestOptions) -> RpcResult<()> {
    todo!();
}
