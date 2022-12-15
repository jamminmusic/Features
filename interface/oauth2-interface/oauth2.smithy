// oauth_2.smithy
// A simple service that calculates the factorial of a whole number


// Tell the code generator how to reference symbols defined in this namespace
metadata package = [ { namespace: "org.jammin.interfaces.oauth2", crate: "oauth2_interface" } ]

namespace org.jammin.interfaces.oauth2

use org.wasmcloud.model#wasmbus
use org.wasmcloud.model#U32
use org.wasmcloud.model#U64

/// The Oauth2 service has a single method, calculate, which
/// calculates the factorial of its whole number parameter.
@wasmbus(
    contractId: "jammin:interfaces:oauth2",
    actorReceive: true,
    providerReceive: true )
service Oauth2 {
  version: "0.1",
  operations: [ Authorize, Unauthorize ]
}

operation Authorize {
  input: AuthorizeRequest,
  output: AuthorizeResponse,
}

structure AuthorizeRequest {
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
}

structure AuthorizeResponse {
  /// indication whether the request was successful
  @n(0)
  @required
  success: Boolean

  /// If success is false, this may contain an error
  @n(1)
  error: String
}

operation Unauthorize {
  input: UnauthorizeRequest,
  output: UnauthorizeResponse,
}

structure UnauthorizeRequest {
  @n(0)
  @required
  provider: String
}

structure UnauthorizeResponse {
  /// indication whether the request was successful
  @n(0)
  @required
  success: Boolean

  /// If success is false, this may contain an error
  @n(1)
  error: String
}
