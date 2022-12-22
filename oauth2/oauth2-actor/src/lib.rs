use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_httpserver::{HttpRequest, HttpResponse, HttpServer, HttpServerReceiver};
// Needs Vault link
use wasmcloud_interface_keyvalue::{KeyValue, KeyValueSender};
// Determine what is needed for interface, and remove the others.
// Needs NATS link
use wasmcloud_interface_messaging::{MessageSubscriber, MessageSubscriberReceiver, SubMessage, MessagingSender, PubMessage, RequestMessage};

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, HttpServer)]
struct Oauth2Actor {}

/// Implementation of HttpServer trait methods
#[async_trait]
impl HttpServer for Oauth2Actor {
    /// Returns a greeting, "Hello World", in the response body.
    /// If the request contains a query parameter 'name=NAME', the
    /// response is changed to "Hello NAME"
    async fn handle_request(&self, _ctx: &Context, req: &HttpRequest) -> RpcResult<HttpResponse> {
        let text = form_urlencoded::parse(req.query_string.as_bytes())
            .find(|(n, _)| n == "name")
            .map(|(_, v)| v.to_string())
            .unwrap_or_else(|| "World".to_string());

        Ok(HttpResponse {
            body: format!("Hello {}", text).as_bytes().to_vec(),
            ..Default::default()
        })
    }
}

// TODO
// example - Implementing the Messaging.HandleMessage operation
// use wasmbus_rpc::actor::prelude::*;
// use wasmcloud_interface_logging::info;
// use wasmcloud_interface_messaging::{MessageSubscriber, MessageSubscriberReceiver, SubMessage};

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, MessageSubscriber)]

#[async_trait]
impl MessageSubscriber for Oauth2Actor {
    /// Handle a message received on a subscription
    async fn handle_message(&self, _ctx: &Context, msg: &SubMessage) -> RpcResult<()> {
        unimplemented!();

        info!("Received message: {:?}", msg);
        Ok(())
    }
}

// TODO
// example - Sending a message via a wasmcloud:messaging provider, waiting one second for a reply
// use wasmcloud_interface_messaging::{Messaging, MessagingSender, RequestMessage};
async fn message_request(ctx: &Context, subject: &str, body: &[u8]) -> RpcResult<()> {
    unimplemented!();

    let provider = MessagingSender::new();
    if let Err(e) = provider
        .request(
            ctx,
            &RequestMessage {
                body: body.to_vec(),
                subject: subject.to_owned(),
                timeout_ms: 1_000,
            },
        )
        .await
    {
        Err(format!("Could not request message {}", e.to_string()).into())
    } else {
        Ok(())
    }
}

// TODO
// example - check if value exists in kvstore
async fn key_exists(ctx: &Context, key: &str) -> bool {
    unimplemented!();

    KeyValueSender::new().contains(ctx, key).await.is_ok()
}