#[allow(unused_imports)]
use wasmbus_rpc::provider::prelude::*;
use wasmcloud_test_util::{
    check,
    cli::print_test_results,
    provider_test::test_provider,
    run_selected, run_selected_spawn,
    testing::{TestOptions, TestResult},
};

use jammin_provider_music_auth::config::{MusicAuthConfig, MusicAuthProviders};

use jammin_interface_music_auth::*;
use securestore::{KeySource, SecretsManager};
use std::path::Path;
use tempfile::NamedTempFile;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;


#[tokio::test]
async fn run_all() {
    let opts = TestOptions::default();
    let res = run_selected_spawn!(&opts, health_check);
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

// async fn config_from_file(_opt: &TestOptions) -> RpcResult<()> {
//     let prov = test_provider().await;
//     let value_map = HashMap::from{[
//             ("provider".to_string(), "spotify".to_string()),
//         ]};
//     let values = prov.link_to_test(value_map).await;
// }

// async fn config_from_link(_opt: &TestOptions) -> RpcResult<()> {
//     let prov = test_provider().await;
//     let value_map = HashMap::from{[
//         ("provider".to_string(), "other".to_string()),
//         ("client_id".to_string(), "931a320fb123fehsa53680ababdc3132".to_string()),
//         ("client_secret".to_string(), "deb2f4fe2438cb75f0bedd88dd".to_string()),
//         ("auth_url".to_string(), "https://accounts.spotify.com/authorize".to_string()),
//         ("token_url".to_string(), "https://accounts.spotify.com/api/token".to_string()),
//         ("scope".to_string(), "user-read-currently-playing  playlist-modify-private".to_string())
//     ]};
//     let values = prov.link_to_test(value_map).await;
// }
