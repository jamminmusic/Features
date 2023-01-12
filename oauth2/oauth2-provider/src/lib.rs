#[allow(unused_imports)]
mod methods;

use anyhow;
use anyhow::Error;
use oauth2_interface::*;
use serde::Deserialize;
use wasmbus_rpc::provider::prelude::*;
use strum::EnumString;
use crate::methods::url::AuthUrlBuilder;

// available Oauth2 Grant Types - Legacy Grant Types not supported
#[derive(Clone, Deserialize, Debug, PartialEq, EnumString)]
pub enum GrantType {
    #[strum(ascii_case_insensitive)]
    AuthorizationCode,
    #[strum(ascii_case_insensitive)]
    Pkce, 
    #[strum(ascii_case_insensitive)]
    Refresh,
    #[strum(ascii_case_insensitive)]
    ClientCredentials,
    #[strum(ascii_case_insensitive)]
    DeviceCode,
}

impl GrantType {
    pub async fn get_auth_url(&self, _req: &GetAuthUrlRequest) -> Result<GetAuthUrlResponse, Error>{
        let auth_url_response = match self {
            // User Flow - User interaction with auth_url needed.
            //Just remove PKCE code from https://docs.rs/oauth2/4.3.0/oauth2/#getting-started-authorization-code-grant-w-pkce
            GrantType::AuthorizationCode => AuthUrlBuilder::new().create_client(_req).set_redirect_url(_req).generate_auth_url(_req).build(),
            // User Flow + Pkce - User interaction with auth_url needed and will contain code challenge. Most secure User Flow.
            // https://docs.rs/oauth2/4.3.0/oauth2/#getting-started-authorization-code-grant-w-pkce
            GrantType::Pkce => AuthUrlBuilder::new().create_client(_req).set_redirect_url(_req).generate_pkce().generate_auth_url_pkce(_req).build(),
            // Refresh Flow - If client was issued a secret User interaction with auth_url needed, otherwise User interaction with auth_url not needed. 
            // https://docs.rs/oauth2/4.3.0/oauth2/trait.TokenResponse.html#tymethod.refresh_token
            GrantType::Refresh => AuthUrlBuilder::new().create_client(_req).generate_auth_url(_req).build(),
            // Application Flow - User interaction with auth_url not needed. Application as a client will pass client_id and secret for authentication.
            // https://docs.rs/oauth2/4.3.0/oauth2/#client-credentials-grant
            // client_secret is an option when creating the Client, therefore we can handle whether a user needs to interact outside of URL generation.
            GrantType::ClientCredentials => AuthUrlBuilder::new().create_client(_req).generate_auth_url(_req).build(),
            // Device Flow - User interaction with auth_url needed - authenticate on browserless or input-constrained devices.
            // https://docs.rs/oauth2/4.3.0/oauth2/#device-code-flow
            GrantType::DeviceCode => todo!(), // AuthUrlBuilder::new().create_client(req).set_device_url(req).generate_device_auth_url().build(),
        };
        Ok(auth_url_response)
    }

    pub async fn authorize_user(&self, _req: &AuthorizeUserRequest) -> Result<AuthorizeUserResponse, Error> {
        todo!();

        // let authorize_user_response = match self {
        //     // 
        //     GrantType::AuthorizationCode => "Define functions to call",
        //     // 
        //     GrantType::Pkce => "Define functions to call",
        //     // 
        //     GrantType::Refresh => "Define functions to call",
        //     // 
        //     GrantType::ClientCredentials => "Define functions to call",
        //     // 
        //     GrantType::DeviceCode => "Define functions to call",
        // };
        // Ok(AuthorizeUserResponse)
    }

    pub async fn unauthorize_user(&self, _req: &UnauthorizeUserRequest) -> Result<UnauthorizeUserResponse, Error> {
        todo!();

        // let unauthorize_user_response = match self {
        //     // 
        //     GrantType::AuthorizationCode => "Define functions to call",
        //     // 
        //     GrantType::Pkce => "Define functions to call",
        //     // 
        //     GrantType::Refresh => "Define functions to call",
        //     // 
        //     GrantType::ClientCredentials => "Define functions to call",
        //     // 
        //     GrantType::DeviceCode => "Define functions to call",
        // };
        // Ok(UnauthorizeUserResponse)
    }
}
