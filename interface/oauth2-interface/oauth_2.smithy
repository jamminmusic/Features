// oauth_2.smithy
// A simple service that calculates the factorial of a whole number


// Tell the code generator how to reference symbols defined in this namespace
metadata package = [ { namespace: "org.jammin.interfaces.oauth_2", crate: "oauth_2_interface" } ]

namespace org.jammin.interfaces.oauth_2

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
  operations: [ Calculate ]
}

/// Calculates the factorial (n!) of the input parameter
operation Calculate {
  input: U32,
  output: U64
}

