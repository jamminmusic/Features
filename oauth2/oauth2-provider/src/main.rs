//! Oauth2 capability provider
//!
//!
// Device Flow is not yet implemented
use oauth2_provider::Oauth2Provider;

#[allow(unused_imports)]
use wasmbus_rpc::provider::prelude::*;

// main (via provider_main) initializes the threaded tokio executor,
// listens to lattice rpcs, handles actor links,
// and returns only when it receives a shutdown message
//
fn main() -> Result<(), Box<dyn std::error::Error>> {
    provider_main(Oauth2Provider::default(), Some("Oauth2".to_string()))?;

    eprintln!("Oauth2 provider exiting");
    Ok(())
}
