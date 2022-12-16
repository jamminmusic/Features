//! jammin_provider_music_auth capability provider
//!
//!
#[allow(unused_imports)]
use jammin_interface_music_auth::{
    ConnectProviderRequest, ConnectProviderResponse, DisconnectProviderRequest,
    DisconnectProviderResponse, MusicAuth, MusicAuthReceiver,
};
use wasmbus_rpc::{core::LinkDefinition, provider::prelude::*};

// main (via provider_main) initializes the threaded tokio executor,
// listens to lattice rpcs, handles actor links,
// and returns only when it receives a shutdown message
//
fn main() -> Result<(), Box<dyn std::error::Error>> {
    provider_main(
        JamminProviderMusicAuth::default(),
        Some("JamminProviderMusicAuth".to_string()),
    )?;

    eprintln!("jammin_provider_music_auth provider exiting");
    Ok(())
}

/// jammin_provider_music_auth capability provider implementation
#[derive(Default, Clone, Provider)]
#[services(MusicAuth)]
struct JamminProviderMusicAuth {}

impl JamminProviderMusicAuth {}

// use default implementations of provider message handlers
impl ProviderDispatch for JamminProviderMusicAuth {}

// Handle provider control commands
impl ProviderHandler for JamminProviderMusicAuth {}

#[async_trait]
impl MusicAuth for JamminProviderMusicAuth {
    async fn connect_provider(
        &self,
        _ctx: &Context,
        _req: &ConnectProviderRequest,
    ) -> RpcResult<ConnectProviderResponse> {
        let x = ConnectProviderResponse {
            success: true,
            error: Some("words".to_string()),
        };
        Ok(x)
    }

    async fn disconnect_provider(
        &self,
        _ctx: &Context,
        _req: &DisconnectProviderRequest,
    ) -> RpcResult<DisconnectProviderResponse> {
        let x = DisconnectProviderResponse {
            success: true,
            error: Some("words".to_string()),
        };

        Ok(x)
    }
}
