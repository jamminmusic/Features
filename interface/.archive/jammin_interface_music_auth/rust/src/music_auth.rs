// This file is @generated by wasmcloud/weld-codegen 0.6.0.
// It is not intended for manual editing.
// namespace: jammin.service.music_auth

#[allow(unused_imports)]
use async_trait::async_trait;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::{borrow::Borrow, borrow::Cow, io::Write, string::ToString};
#[allow(unused_imports)]
use wasmbus_rpc::{
    cbor::*,
    common::{
        deserialize, message_format, serialize, Context, Message, MessageDispatch, MessageFormat,
        SendOpts, Transport,
    },
    error::{RpcError, RpcResult},
    Timestamp,
};

#[allow(dead_code)]
pub const SMITHY_VERSION: &str = "1.0";

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct ConnectProviderRequest {
    #[serde(default)]
    pub provider: String,
    /// OAuth2 Options: AuthorizationCode, PKCE, Refresh, ClientCredentials, DeviceCode
    #[serde(rename = "grantType")]
    #[serde(default)]
    pub grant_type: String,
    /// Client request origin needed for auth redirect, get x-forwarded-for header.
    pub header: HeaderMap,
}

// Encode ConnectProviderRequest as CBOR and append to output stream
#[doc(hidden)]
#[allow(unused_mut)]
pub fn encode_connect_provider_request<W: wasmbus_rpc::cbor::Write>(
    mut e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &ConnectProviderRequest,
) -> RpcResult<()>
where
    <W as wasmbus_rpc::cbor::Write>::Error: std::fmt::Display,
{
    e.array(3)?;
    e.str(&val.provider)?;
    e.str(&val.grant_type)?;
    encode_header_map(e, &val.header)?;
    Ok(())
}

// Decode ConnectProviderRequest from cbor input stream
#[doc(hidden)]
pub fn decode_connect_provider_request(
    d: &mut wasmbus_rpc::cbor::Decoder<'_>,
) -> Result<ConnectProviderRequest, RpcError> {
    let __result = {
        let mut provider: Option<String> = None;
        let mut grant_type: Option<String> = None;
        let mut header: Option<HeaderMap> = None;

        let is_array = match d.datatype()? {
            wasmbus_rpc::cbor::Type::Array => true,
            wasmbus_rpc::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct ConnectProviderRequest, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.fixed_array()?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => provider = Some(d.str()?.to_string()),
                    1 => grant_type = Some(d.str()?.to_string()),
                    2 => {
                        header = Some(decode_header_map(d).map_err(|e| {
                            format!("decoding 'jammin.service.music_auth#HeaderMap': {}", e)
                        })?)
                    }
                    _ => d.skip()?,
                }
            }
        } else {
            let len = d.fixed_map()?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "provider" => provider = Some(d.str()?.to_string()),
                    "grantType" => grant_type = Some(d.str()?.to_string()),
                    "header" => {
                        header = Some(decode_header_map(d).map_err(|e| {
                            format!("decoding 'jammin.service.music_auth#HeaderMap': {}", e)
                        })?)
                    }
                    _ => d.skip()?,
                }
            }
        }
        ConnectProviderRequest {
            provider: if let Some(__x) = provider {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field ConnectProviderRequest.provider (#0)".to_string(),
                ));
            },

            grant_type: if let Some(__x) = grant_type {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field ConnectProviderRequest.grant_type (#1)".to_string(),
                ));
            },

            header: if let Some(__x) = header {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field ConnectProviderRequest.header (#2)".to_string(),
                ));
            },
        }
    };
    Ok(__result)
}
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct ConnectProviderResponse {
    /// indication whether the request was successful((u9]@BHwE}1JnWpu9Ii
    #[serde(default)]
    pub success: bool,
    /// If success is false, this may contain an error
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

// Encode ConnectProviderResponse as CBOR and append to output stream
#[doc(hidden)]
#[allow(unused_mut)]
pub fn encode_connect_provider_response<W: wasmbus_rpc::cbor::Write>(
    mut e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &ConnectProviderResponse,
) -> RpcResult<()>
where
    <W as wasmbus_rpc::cbor::Write>::Error: std::fmt::Display,
{
    e.array(2)?;
    e.bool(val.success)?;
    if let Some(val) = val.error.as_ref() {
        e.str(val)?;
    } else {
        e.null()?;
    }
    Ok(())
}

// Decode ConnectProviderResponse from cbor input stream
#[doc(hidden)]
pub fn decode_connect_provider_response(
    d: &mut wasmbus_rpc::cbor::Decoder<'_>,
) -> Result<ConnectProviderResponse, RpcError> {
    let __result = {
        let mut success: Option<bool> = None;
        let mut error: Option<Option<String>> = Some(None);

        let is_array = match d.datatype()? {
            wasmbus_rpc::cbor::Type::Array => true,
            wasmbus_rpc::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct ConnectProviderResponse, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.fixed_array()?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => success = Some(d.bool()?),
                    1 => {
                        error = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.str()?.to_string()))
                        }
                    }

                    _ => d.skip()?,
                }
            }
        } else {
            let len = d.fixed_map()?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "success" => success = Some(d.bool()?),
                    "error" => {
                        error = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.str()?.to_string()))
                        }
                    }
                    _ => d.skip()?,
                }
            }
        }
        ConnectProviderResponse {
            success: if let Some(__x) = success {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field ConnectProviderResponse.success (#0)".to_string(),
                ));
            },
            error: error.unwrap(),
        }
    };
    Ok(__result)
}
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct DisconnectProviderRequest {
    #[serde(default)]
    pub provider: String,
}

// Encode DisconnectProviderRequest as CBOR and append to output stream
#[doc(hidden)]
#[allow(unused_mut)]
pub fn encode_disconnect_provider_request<W: wasmbus_rpc::cbor::Write>(
    mut e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &DisconnectProviderRequest,
) -> RpcResult<()>
where
    <W as wasmbus_rpc::cbor::Write>::Error: std::fmt::Display,
{
    e.array(1)?;
    e.str(&val.provider)?;
    Ok(())
}

// Decode DisconnectProviderRequest from cbor input stream
#[doc(hidden)]
pub fn decode_disconnect_provider_request(
    d: &mut wasmbus_rpc::cbor::Decoder<'_>,
) -> Result<DisconnectProviderRequest, RpcError> {
    let __result = {
        let mut provider: Option<String> = None;

        let is_array = match d.datatype()? {
            wasmbus_rpc::cbor::Type::Array => true,
            wasmbus_rpc::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct DisconnectProviderRequest, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.fixed_array()?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => provider = Some(d.str()?.to_string()),
                    _ => d.skip()?,
                }
            }
        } else {
            let len = d.fixed_map()?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "provider" => provider = Some(d.str()?.to_string()),
                    _ => d.skip()?,
                }
            }
        }
        DisconnectProviderRequest {
            provider: if let Some(__x) = provider {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field DisconnectProviderRequest.provider (#0)".to_string(),
                ));
            },
        }
    };
    Ok(__result)
}
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct DisconnectProviderResponse {
    /// indication whether the request was successful
    #[serde(default)]
    pub success: bool,
    /// If success is false, this may contain an error
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

// Encode DisconnectProviderResponse as CBOR and append to output stream
#[doc(hidden)]
#[allow(unused_mut)]
pub fn encode_disconnect_provider_response<W: wasmbus_rpc::cbor::Write>(
    mut e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &DisconnectProviderResponse,
) -> RpcResult<()>
where
    <W as wasmbus_rpc::cbor::Write>::Error: std::fmt::Display,
{
    e.array(2)?;
    e.bool(val.success)?;
    if let Some(val) = val.error.as_ref() {
        e.str(val)?;
    } else {
        e.null()?;
    }
    Ok(())
}

// Decode DisconnectProviderResponse from cbor input stream
#[doc(hidden)]
pub fn decode_disconnect_provider_response(
    d: &mut wasmbus_rpc::cbor::Decoder<'_>,
) -> Result<DisconnectProviderResponse, RpcError> {
    let __result = {
        let mut success: Option<bool> = None;
        let mut error: Option<Option<String>> = Some(None);

        let is_array = match d.datatype()? {
            wasmbus_rpc::cbor::Type::Array => true,
            wasmbus_rpc::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct DisconnectProviderResponse, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.fixed_array()?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => success = Some(d.bool()?),
                    1 => {
                        error = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.str()?.to_string()))
                        }
                    }

                    _ => d.skip()?,
                }
            }
        } else {
            let len = d.fixed_map()?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "success" => success = Some(d.bool()?),
                    "error" => {
                        error = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.str()?.to_string()))
                        }
                    }
                    _ => d.skip()?,
                }
            }
        }
        DisconnectProviderResponse {
            success: if let Some(__x) = success {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field DisconnectProviderResponse.success (#0)".to_string(),
                ));
            },
            error: error.unwrap(),
        }
    };
    Ok(__result)
}
/// map data structure for holding http headers
///
pub type HeaderMap = std::collections::HashMap<String, HeaderValues>;

// Encode HeaderMap as CBOR and append to output stream
#[doc(hidden)]
#[allow(unused_mut)]
pub fn encode_header_map<W: wasmbus_rpc::cbor::Write>(
    mut e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &HeaderMap,
) -> RpcResult<()>
where
    <W as wasmbus_rpc::cbor::Write>::Error: std::fmt::Display,
{
    e.map(val.len() as u64)?;
    for (k, v) in val {
        e.str(k)?;
        encode_header_values(e, v)?;
    }
    Ok(())
}

// Decode HeaderMap from cbor input stream
#[doc(hidden)]
pub fn decode_header_map(d: &mut wasmbus_rpc::cbor::Decoder<'_>) -> Result<HeaderMap, RpcError> {
    let __result = {
        {
            let map_len = d.fixed_map()? as usize;
            let mut m: std::collections::HashMap<String, HeaderValues> =
                std::collections::HashMap::with_capacity(map_len);
            for _ in 0..map_len {
                let k = d.str()?.to_string();
                let v = decode_header_values(d).map_err(|e| {
                    format!("decoding 'jammin.service.music_auth#HeaderValues': {}", e)
                })?;
                m.insert(k, v);
            }
            m
        }
    };
    Ok(__result)
}
pub type HeaderValues = Vec<String>;

// Encode HeaderValues as CBOR and append to output stream
#[doc(hidden)]
#[allow(unused_mut)]
pub fn encode_header_values<W: wasmbus_rpc::cbor::Write>(
    mut e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &HeaderValues,
) -> RpcResult<()>
where
    <W as wasmbus_rpc::cbor::Write>::Error: std::fmt::Display,
{
    e.array(val.len() as u64)?;
    for item in val.iter() {
        e.str(item)?;
    }
    Ok(())
}

// Decode HeaderValues from cbor input stream
#[doc(hidden)]
pub fn decode_header_values(
    d: &mut wasmbus_rpc::cbor::Decoder<'_>,
) -> Result<HeaderValues, RpcError> {
    let __result = {
        if let Some(n) = d.array()? {
            let mut arr: Vec<String> = Vec::with_capacity(n as usize);
            for _ in 0..(n as usize) {
                arr.push(d.str()?.to_string())
            }
            arr
        } else {
            // indefinite array
            let mut arr: Vec<String> = Vec::new();
            loop {
                match d.datatype() {
                    Err(_) => break,
                    Ok(wasmbus_rpc::cbor::Type::Break) => break,
                    Ok(_) => arr.push(d.str()?.to_string()),
                }
            }
            arr
        }
    };
    Ok(__result)
}
/// The MusicAuth service has a single method, calculate, which
/// calculates the factorial of its whole number parameter.
/// wasmbus.contractId: jammin:service:music_auth
/// wasmbus.providerReceive
/// wasmbus.actorReceive
#[async_trait]
pub trait MusicAuth {
    /// returns the capability contract id for this interface
    fn contract_id() -> &'static str {
        "jammin:service:music_auth"
    }
    async fn connect_provider(
        &self,
        ctx: &Context,
        arg: &ConnectProviderRequest,
    ) -> RpcResult<ConnectProviderResponse>;
    async fn disconnect_provider(
        &self,
        ctx: &Context,
        arg: &DisconnectProviderRequest,
    ) -> RpcResult<DisconnectProviderResponse>;
}

/// MusicAuthReceiver receives messages defined in the MusicAuth service trait
/// The MusicAuth service has a single method, calculate, which
/// calculates the factorial of its whole number parameter.
#[doc(hidden)]
#[async_trait]
pub trait MusicAuthReceiver: MessageDispatch + MusicAuth {
    async fn dispatch(&self, ctx: &Context, message: Message<'_>) -> Result<Vec<u8>, RpcError> {
        match message.method {
            "ConnectProvider" => {
                let value: ConnectProviderRequest = wasmbus_rpc::common::deserialize(&message.arg)
                    .map_err(|e| RpcError::Deser(format!("'ConnectProviderRequest': {}", e)))?;

                let resp = MusicAuth::connect_provider(self, ctx, &value).await?;
                let buf = wasmbus_rpc::common::serialize(&resp)?;

                Ok(buf)
            }
            "DisconnectProvider" => {
                let value: DisconnectProviderRequest =
                    wasmbus_rpc::common::deserialize(&message.arg).map_err(|e| {
                        RpcError::Deser(format!("'DisconnectProviderRequest': {}", e))
                    })?;

                let resp = MusicAuth::disconnect_provider(self, ctx, &value).await?;
                let buf = wasmbus_rpc::common::serialize(&resp)?;

                Ok(buf)
            }
            _ => Err(RpcError::MethodNotHandled(format!(
                "MusicAuth::{}",
                message.method
            ))),
        }
    }
}

/// MusicAuthSender sends messages to a MusicAuth service
/// The MusicAuth service has a single method, calculate, which
/// calculates the factorial of its whole number parameter.
/// client for sending MusicAuth messages
#[derive(Debug)]
pub struct MusicAuthSender<T: Transport> {
    transport: T,
}

impl<T: Transport> MusicAuthSender<T> {
    /// Constructs a MusicAuthSender with the specified transport
    pub fn via(transport: T) -> Self {
        Self { transport }
    }

    pub fn set_timeout(&self, interval: std::time::Duration) {
        self.transport.set_timeout(interval);
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl<'send> MusicAuthSender<wasmbus_rpc::provider::ProviderTransport<'send>> {
    /// Constructs a Sender using an actor's LinkDefinition,
    /// Uses the provider's HostBridge for rpc
    pub fn for_actor(ld: &'send wasmbus_rpc::core::LinkDefinition) -> Self {
        Self {
            transport: wasmbus_rpc::provider::ProviderTransport::new(ld, None),
        }
    }
}
#[cfg(target_arch = "wasm32")]
impl MusicAuthSender<wasmbus_rpc::actor::prelude::WasmHost> {
    /// Constructs a client for actor-to-actor messaging
    /// using the recipient actor's public key
    pub fn to_actor(actor_id: &str) -> Self {
        let transport =
            wasmbus_rpc::actor::prelude::WasmHost::to_actor(actor_id.to_string()).unwrap();
        Self { transport }
    }
}

#[cfg(target_arch = "wasm32")]
impl MusicAuthSender<wasmbus_rpc::actor::prelude::WasmHost> {
    /// Constructs a client for sending to a MusicAuth provider
    /// implementing the 'jammin:service:music_auth' capability contract, with the "default" link
    pub fn new() -> Self {
        let transport = wasmbus_rpc::actor::prelude::WasmHost::to_provider(
            "jammin:service:music_auth",
            "default",
        )
        .unwrap();
        Self { transport }
    }

    /// Constructs a client for sending to a MusicAuth provider
    /// implementing the 'jammin:service:music_auth' capability contract, with the specified link name
    pub fn new_with_link(link_name: &str) -> wasmbus_rpc::error::RpcResult<Self> {
        let transport = wasmbus_rpc::actor::prelude::WasmHost::to_provider(
            "jammin:service:music_auth",
            link_name,
        )?;
        Ok(Self { transport })
    }
}
#[async_trait]
impl<T: Transport + std::marker::Sync + std::marker::Send> MusicAuth for MusicAuthSender<T> {
    #[allow(unused)]
    async fn connect_provider(
        &self,
        ctx: &Context,
        arg: &ConnectProviderRequest,
    ) -> RpcResult<ConnectProviderResponse> {
        let buf = wasmbus_rpc::common::serialize(arg)?;

        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "MusicAuth.ConnectProvider",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;

        let value: ConnectProviderResponse = wasmbus_rpc::common::deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("'{}': ConnectProviderResponse", e)))?;
        Ok(value)
    }
    #[allow(unused)]
    async fn disconnect_provider(
        &self,
        ctx: &Context,
        arg: &DisconnectProviderRequest,
    ) -> RpcResult<DisconnectProviderResponse> {
        let buf = wasmbus_rpc::common::serialize(arg)?;

        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "MusicAuth.DisconnectProvider",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;

        let value: DisconnectProviderResponse = wasmbus_rpc::common::deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("'{}': DisconnectProviderResponse", e)))?;
        Ok(value)
    }
}
