// oauth_2.smithy
// An OAuth2 Provider


// Tell the code generator how to reference symbols defined in this namespace
metadata package = [ { namespace: "org.jammin.interfaces.oauth2", crate: "oauth2_interface" } ]

namespace org.jammin.interfaces.oauth2

use org.wasmcloud.model#wasmbus
use org.wasmcloud.model#U32
use org.wasmcloud.model#U64

/// The Oauth2 service has 3 Operations: To Authorize the user with OAuth Provider, To Remove Authorization, and to get the initial /Authorize URI
@wasmbus(
    contractId: "jammin:interfaces:oauth2",
    actorReceive: true,
    providerReceive: true )
service Oauth2 {
  version: "0.1",
  operations: [ AuthorizeUser, UnauthorizeUser, GetAuthUri ]
}

operation GetAuthUri {
  input: GetAuthUriRequest,
  output: GetAuthUriResponse,
}

structure GetAuthUriRequest {
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
  grant_type: String

  @n(2)
  @sensitive
  @required
  client_id: String

  @n(3)
  @sensitive
  @required
  client_secret: String

  @n(4)
  @required
  auth_url: String

  @n(5)
  @required
  token_url: String

  @n(6)
  @required
  redirect_url: String

  @n(7)
  @required
  scope: String
}

structure GetAuthUriResponse {
  // If using cbor serialization, and all the fields have @n references, 
  // the struct is serialized as an array (without field names), so it’s much more compact and faster.
  // Schema evolution: If you modify the struct and add a field, as long as you keep the numbers of 
  // old fields the same, it should work. If you delete a field, don’t re-use the number.
  @n(0)
  @required
  success: Boolean

  /// OAuth2 Options: AuthorizationCode, PKCE, Refresh, ClientCredentials, DeviceCode
  @n(1)
  @required
  error: String

  @n(2)
  @sensitive
  @required
  uri: String

  @n(3)
  @sensitive
  @required
  csrf_state: String

}

operation AuthorizeUser {
  input: AuthorizeUserRequest,
  output: AuthorizeUserResponse,
}

structure AuthorizeUserRequest {
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

  @n(2)
  @sensitive
  @required
  token: String

  /// OAuth2 Options: AuthorizationCode, PKCE, Refresh, ClientCredentials, DeviceCode
  @n(3)
  @sensitive
  @required
  state: String
}

structure AuthorizeUserResponse {
  /// indication whether the request was successful
  @n(0)
  @required
  success: Boolean

  /// If success is false, this may contain an error
  @n(1)
  error: String
}

operation UnauthorizeUser {
  input: UnauthorizeUserRequest,
  output: UnauthorizeUserResponse,
}

structure UnauthorizeUserRequest {
  @n(0)
  @required
  provider: String

  @n(1)
  @required
  user: String
}

structure UnauthorizeUserResponse {
  /// indication whether the request was successful
  @n(0)
  @required
  success: Boolean

  /// If success is false, this may contain an error
  @n(1)
  error: String
}
