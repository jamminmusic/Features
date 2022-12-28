// oauth2.smithy
// An OAuth2 Provider


// Tell the code generator how to reference symbols defined in this namespace
metadata package = [ { namespace: "jammin.interfaces.oauth2", crate: "oauth2_interface" } ]

namespace jammin.interfaces.oauth2

use org.wasmcloud.model#wasmbus
use org.wasmcloud.model#U32
use org.wasmcloud.model#U64
use org.wasmcloud.model#n
// use org.jamminmusic.model#CsrfToken

/// The Oauth2 service has 3 Operations: To Authorize the user with OAuth Provider, To Remove Authorization, and to get the initial /Authorize URI
@wasmbus(
    contractId: "jammin:interfaces:oauth2",
    actorReceive: true,
    providerReceive: true )
service Oauth2 {
  version: "0.1",
  operations: [ AuthorizeUser, UnauthorizeUser, GetAuthUrl ]
}

operation GetAuthUrl {
  input: GetAuthUrlRequest,
  output: GetAuthUrlResponse,
}

structure GetAuthUrlRequest {
  // If using cbor serialization, and all the fields have @n references, 
  // the struct is serialized as an array (without field names), so it’s much more compact and faster.
  // Schema evolution: If you modify the struct and add a field, as long as you keep the numbers of 
  // old fields the same, it should work. If you delete a field, don’t re-use the number.

  /// OAuth2 Options: AuthorizationCode, PKCE, Refresh, ClientCredentials, DeviceCode
  @n(0)
  @required
  grant_type: String

  @n(1)
  @sensitive
  @required
  client_id: String

  @n(2)
  @sensitive
  device_code: String

  @n(3)
  @sensitive
  client_secret: String

  @n(4)
  @required
  auth_url: String

  @n(5)
  @required
  token_url: String

  @n(6)
  redirect_url: String

  @n(7)
  @required
  scope: String
  
  @n(8)
  device_auth_url: String
}

structure GetAuthUrlResponse {
  // If using cbor serialization, and all the fields have @n references, 
  // the struct is serialized as an array (without field names), so it’s much more compact and faster.
  // Schema evolution: If you modify the struct and add a field, as long as you keep the numbers of 
  // old fields the same, it should work. If you delete a field, don’t re-use the number.
  @n(0)
  @required
  success: Boolean

   @n(1)
  error: String

  @n(2)
  @sensitive
  url: String

  // store state in SurrealDB or KV to compare later
  @n(3)
  @sensitive
  @required
  csrf_state: String

  @n(4)
  @sensitive
  device_url: String

  // device user code
  @n(5)
  @sensitive
  device_code: String

  // device user code expire time
  @n(6)
  device_code_expire: U64
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

  /// OAuth2 Options: AuthorizationCode, PKCE, Refresh, ClientCredentials, DeviceCode
  @n(0)
  @required
  grant_type: String

  @n(1)
  @sensitive
  @required
  auth_code: String

  // state returned with auth_code to be compared against original csrf_state
  @n(2)
  @sensitive
  @required
  state: String

  // retrieved initially generated state - compare against returned state in provider
  @n(3)
  @sensitive
  @required
  csrf_state: String

}

structure AuthorizeUserResponse {
  /// indication whether the request was successful
  @n(0)
  @required
  success: Boolean

  /// If success is false, this may contain an error
  @n(1)
  error: String

  @n(2)
  @sensitive
  @required
  access_token: String

  @n(3)
  @sensitive
  refresh_token: String

  @n(4)
  @sensitive
  user_id: String

  @n(5)
  @sensitive
  device_id: String

  // token expire time
  @n(6)
  @required
  expire: U64

  @n(7)
  @required
  scope: String

  // Potentially add email
}


// Figure this part out later. 
operation UnauthorizeUser {
  input: UnauthorizeUserRequest,
  output: UnauthorizeUserResponse,
}

structure UnauthorizeUserRequest {
  @n(0)
  user_id: String

  @n(1)
  device_id: String
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
