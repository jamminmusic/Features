// use tracing::debug;
use wasmbus_rpc::{
    provider::prelude::*, 
    common::{Context, Message, Transport}, 
    core::LinkDefinition, 
    error::{RpcError, RpcResult}
};
use std::io::Read;
use std::fs::File;
use std::str::FromStr;
use strum::EnumString;
use std::collections::HashMap;
use serde::Deserialize;


// To reference secrets
use securestore::{KeySource, SecretsManager};
use std::path::Path;
use tempfile::NamedTempFile;


#[derive(Clone, Default, Deserialize, Debug)]
pub struct MusicAuthConfig {
    /// client id
    pub client_id: Option<String>,
    /// client secret
    pub client_secret: Option<String>,
    /// authorize url
    pub auth_url: Option<String>,
    /// token url
    pub token_url: Option<String>,
    /// scope
    pub scope: Option<String>,
}

/// OAuth2 providers with available configurations
#[derive(Clone, Deserialize, Debug, PartialEq, EnumString)]
pub enum MusicAuthProviders {
    #[strum(ascii_case_insensitive)]
    Google,
    #[strum(ascii_case_insensitive)]
    Spotify, 
    #[strum(ascii_case_insensitive)]
    Apple,
    #[strum(ascii_case_insensitive)]
    Amazon,
    #[strum(ascii_case_insensitive)]
    Deezer,
    #[strum(ascii_case_insensitive)]
    Other
}

impl MusicAuthProviders {
    pub async fn get_config(&self, req: Option<&HashMap<String,String>>) -> RpcResult<SecretsManager> {
        let secrets = match self {
            MusicAuthProviders::Google => SecretsManager::load("secure/google.json", KeySource::File(Path::new("secure/google.key")))
                                            .expect("Failed to load SecureStore vault!"),

            MusicAuthProviders::Spotify => SecretsManager::load("secure/spotify.json", KeySource::File(Path::new("secure/spotify.key")))
                                            .expect("Failed to load SecureStore vault!"),

            MusicAuthProviders::Apple => SecretsManager::load("secure/apple.json", KeySource::File(Path::new("secure/apple.key")))
                                            .expect("Failed to load SecureStore vault!"),

            MusicAuthProviders::Amazon => SecretsManager::load("secure/amazon.json", KeySource::File(Path::new("secure/amazon.key")))
                                            .expect("Failed to load SecureStore vault!"),

            MusicAuthProviders::Deezer => SecretsManager::load("secure/deezer.json", KeySource::File(Path::new("secure/deezer.key")))
                                            .expect("Failed to load SecureStore vault!"),

            MusicAuthProviders::Other => secrets_from_request(&req.unwrap()).await.expect("Failed to load SecureStore vault!"),
        };
        Ok(secrets)     
    }

    pub async fn set_config(&self, secrets: &SecretsManager) -> Result<MusicAuthConfig, securestore::Error>{
        let config = MusicAuthConfig {
                client_id: Some(secrets.get("client_id:password")?),
                client_secret: Some(secrets.get("client_secret:password")?),
                auth_url: Some(secrets.get("auth_url:username")?),
                token_url: Some(secrets.get("token_url:username")?),
                scope: Some(secrets.get("scope:username")?),
        };
        Ok(config)
    }
}

pub async fn secrets_from_request(req: &HashMap<String,String>) -> RpcResult<SecretsManager> {
    // Create temp file for link def values
    let vault = NamedTempFile::new().unwrap().into_temp_path();
    let keyfile = NamedTempFile::new().unwrap().into_temp_path();

    // Create a vault, write to it, and export it and its keys
    let mut sman = SecretsManager::new(KeySource::Csprng).unwrap();

    // link definition values go here
    sman.set("client_id", req.get(&"client_id" as &str).expect("could not find value").to_string());
    sman.set("client_secret", req.get("client_secret").expect("could not find value").to_string());
    sman.set("auth_url", req.get("auth_url").expect("could not find value").to_string());
    sman.set("token_url", req.get("token_url").expect("could not find value").to_string());
    sman.set("scope", req.get("scope").expect("could not find value").to_string());
    sman.export_key(&keyfile).unwrap();
    sman.save_as(&vault).unwrap();

    let keysource = KeySource::File(Path::new(&keyfile));

    Ok(
        SecretsManager::load(vault, keysource)
        .expect("Failed to load SecureStore vault!")
    )
}

// Test that checks config files and securestore are functioning correctly
#[tokio::test]
async fn verify_config_file() -> Result<(), securestore::Error> {
    let expected_config = MusicAuthConfig {
            client_id: Some("931a320fb123fehsa53680ababdc3132".to_string()),
            client_secret: Some("0deb2f4fe2438cb75f0bedd88dd".to_string()),
            auth_url: Some("https://accounts.spotify.com/authorize".to_string()),
            token_url: Some("https://accounts.spotify.com/api/token".to_string()),
            scope: Some("user-read-currently-playing  playlist-modify-private".to_string()),
    };

    let load_test_config = SecretsManager::load(
        "secure/test.json",
        KeySource::File(Path::new("secure/test.key")),
    )
    .expect("Failed to load SecureStore vault!");

    let actual_config = MusicAuthProviders::Spotify
    .set_config(&load_test_config)
    .await
    .unwrap();

    assert_eq!(expected_config.client_id, actual_config.client_id);
    assert_eq!(expected_config.client_secret, actual_config.client_secret);
    assert_eq!(expected_config.auth_url, actual_config.auth_url);
    assert_eq!(expected_config.token_url, actual_config.token_url);
    assert_eq!(expected_config.scope, actual_config.scope);
    return Ok(());
}
