use oauth2::basic::BasicClient;
use oauth2::devicecode::StandardDeviceAuthorizationResponse;
use oauth2::reqwest::async_http_client;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, DeviceAuthorizationUrl,
    PkceCodeChallenge, PkceCodeVerifier, RedirectUrl, Scope, TokenResponse, TokenUrl,
};
#[allow(unused_imports)]
use oauth2_interface::{GetAuthUrlRequest, GetAuthUrlResponse};
use url::Url;

// TODO - Convert fn outputs to Result<Self, Box<dyn Error>> and use anyhow for errors instead of .expect().

#[derive(Default)]
pub struct AuthUrlBuilder {
    // Basic Client Struct - Client<BasicErrorResponse, BasicTokenResponse, BasicTokenType, BasicTokenIntrospectionResponse, StandardRevocableToken, BasicRevocationErrorResponse>
    // Client Struct - { client_id: ClientId, client_secret: Option<ClientSecret>, auth_url: AuthUrl, auth_type: AuthType, token_url: Option<TokenUrl>, redirect_url: Option<RedirectUrl>, introspection_url: Option<IntrospectionUrl>, revocation_url: Option<RevocationUrl>, device_authorization_url: Option<DeviceAuthorizationUrl>, phantom: PhantomData<(TE, TR, TT, TIR, RT, TRE)>}
    client: Option<oauth2::basic::BasicClient>,
    auth_url: Option<(Url, CsrfToken)>,
    pkce: Option<PkceCodeChallenge>,
    pkce_verifier: Option<PkceCodeVerifier>,
    success: bool,
    error: Option<String>,
    // (verification_url, UserCode, expire_time)
    // device_auth_url: Option<(Url, String, u64)>
}

impl AuthUrlBuilder {
    // TODO - MAKE SURE THAT THERE IS LOGIC PREVENTING BUILDER STEPS FROM BEING CALLED FROM ONES THEY DON'T PAIR WITH
    // TODO - Implement Device Flow

    pub fn new() -> AuthUrlBuilder {
        AuthUrlBuilder {
            client: None,
            auth_url: None,
            pkce: None,
            pkce_verifier: None,
            success: false,
            error: None,
        }
    }

    pub fn create_client(mut self, req: &GetAuthUrlRequest) -> Self {
        self.client = Some(BasicClient::new(
            ClientId::new(req.client_id.to_owned()),
            Some(ClientSecret::new(req.client_secret.to_owned().unwrap())),
            AuthUrl::new(req.auth_url.to_owned()).expect("Invalid authorization endpoint URL"),
            Some(
                TokenUrl::new(req.token_url.to_owned())
                    .expect("Invalid authorization endpoint URL"),
            ),
        ));
        self
    }

    pub fn set_redirect_url(mut self, req: &GetAuthUrlRequest) -> Self {
        self.client = Some(self.client.unwrap().set_redirect_uri(
            RedirectUrl::new(req.redirect_url.to_owned().unwrap()).expect("Invalid redirect URL"),
        ));
        self
    }

    pub fn generate_auth_url(mut self, req: &GetAuthUrlRequest) -> Self {
        self.auth_url = Some(
            self.client
                .to_owned()
                .unwrap()
                .authorize_url(CsrfToken::new_random)
                .add_scope(Scope::new(req.scope.to_owned()))
                .url(),
        );

        // TODO - how to validate auth_url to return true
        self.success = true;
        self
    }

    pub fn generate_pkce(mut self) -> Self {
        let pkce_tuple = PkceCodeChallenge::new_random_sha256();
        self.pkce = Some(pkce_tuple.0);
        self.pkce_verifier = Some(pkce_tuple.1);
        self
    }

    pub fn generate_auth_url_pkce(mut self, req: &GetAuthUrlRequest) -> Self {
        self.auth_url = Some(
            self.client
                .to_owned()
                .unwrap()
                .authorize_url(CsrfToken::new_random)
                .add_scope(Scope::new(req.scope.to_owned()))
                .set_pkce_challenge(self.pkce.to_owned().unwrap())
                .url(),
        );

        // TODO - how to validate auth_url to return true
        self.success = true;
        self
    }

    // pub fn set_device_url(mut self, req: &GetAuthUrlRequest) -> Self {
    //     todo!();

    //     // TODO - https://docs.rs/oauth2/latest/oauth2/#device-code-flow
    //     self.client = Some( BasicClient::new(
    //         ClientId::new(req.client_id.to_owned()),
    //         Some(ClientSecret::new(req.client_secret.to_owned().unwrap())),
    //         AuthUrl::new(req.auth_url.to_owned()).expect("Invalid authorization endpoint URL"),
    //         Some(TokenUrl::new(req.token_url.to_owned()).expect("Invalid authorization endpoint URL")),
    //     ).set_device_authorization_url(DeviceAuthorizationUrl::new(req.device_auth_url.to_owned()).expect("Invalid device authorization endpoint URL")).expect("Invalid redirect URL"));
    //     self
    // }

    // pub fn generate_device_auth_url(mut self, req: &GetAuthUrlRequest) -> Self {
    //      todo!();
    //     // TODO - https://docs.rs/oauth2/latest/oauth2/#device-code-flow
    //     // DeviceAuthorizationResponse struct - { device_code: DeviceCode, user_code: UserCode, verification_url: EndUserVerificationUrl, verification_url_complete: Option<VerificationUrlComplete>, expires_in: u64, interval: u64, extra_fields: EF }
    //     let details: StandardDeviceAuthorizationResponse = self.client
    //         // The device verification code
    //         .exchange_device_code()
    //         .add_scope(Scope::new(req.scope.to_owned()))
    //         // look this up
    //         .request_async(http_client)
    //         .await;
    //     // need to send verification url and user code to user - the rest needs to be stored for later for token exhange.
    //     self.auth_url = Some(details);
    //     // TODO - how to validate auth_url to return true
    //     self.success = true;
    //     self
    // }

    pub fn build(self) -> GetAuthUrlResponse {
        GetAuthUrlResponse {
            success: self.success,
            error: self.error,
            // need to create a smithy model for CsrfToken, or create one as a string - https://docs.rs/oauth2/latest/oauth2/struct.CsrfToken.html#method.secret
            url: Some(self.auth_url.to_owned().unwrap().0.to_string()),
            csrf_state: self.auth_url.unwrap().1.secret().to_string(),
            device_url: None,
            device_code: None,
            device_code_expire: None,
        }
    }
}
