// music_auth.smithy

// Tell the code generator how to reference symbols defined in this namespace
metadata package = [ { namespace: "jammin.service.music_auth", crate: "jammin_interface_music_auth" } ]

namespace jammin.service.music_auth

use org.wasmcloud.model#wasmbus
use org.wasmcloud.model#U32
use org.wasmcloud.model#U64
use org.wasmcloud.model#n

/// The MusicAuth service
@wasmbus(
    contractId: "jammin:service:music_auth",
    actorReceive: true,
    providerReceive: true )
service MusicAuth {
  version: "0.1",
  operations: [ ConnectProvider, DisconnectProvider ]
}

operation ConnectProvider {
  input: ConnectProviderRequest,
  output: ConnectProviderResponse,
}

operation DisconnectProvider {
  input: DisconnectProviderRequest,
  output: DisconnectProviderResponse,
}

structure ConnectProviderRequest {
  // If using cbor serialization, and all the fields have @n references, 
  // the struct is serialized as an array (without field names), so it’s much more compact and faster.
  // Schema evolution: If you modify the struct and add a field, as long as you keep the numbers of 
  // old fields the same, it should work. If you delete a field, don’t re-use the number.
  @n(0)
  @required
  provider: String

  /// OAuth2 Options: AuthorizationCode, PKCE, Refresh, ClientCredentials, DeviceCode
  @n(1)
  @required
  grantType: String

  /// Client request origin needed for auth redirect, get x-forwarded-for header. 
  @n(2)
  @required
  header: HeaderMap
}

structure ConnectProviderResponse {
  /// indication whether the request was successful((u9]@BHwE}1JnWpu9Ii
  @n(0)
  @required
  success: Boolean

  /// If success is false, this may contain an error
  @n(1)
  error: String
}

structure DisconnectProviderRequest {
  @n(0)
  @required
  provider: String
}

structure DisconnectProviderResponse {
  /// indication whether the request was successful
  @n(0)
  @required
  success: Boolean

  /// If success is false, this may contain an error
  @n(1)
  error: String
}

/// map data structure for holding http headers
///
map HeaderMap {
    key: String,
    value: HeaderValues,
}

list HeaderValues {
    member: String
}
