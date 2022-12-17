# Oauth2 Provider

``` mermaid
sequenceDiagram
    %% Actors
    participant Client
    participant Oauth2Actor[http/nats]
    participant Oauth2Provider
    participant AuthCache
    participant ngs
    participant SocialProvider

    critical User Connects Social Service
        Client ->> Oauth2Actor[http/nats]: click /login/${Social provider}
        Oauth2Actor[http/nats] ->> Oauth2Provider: GetAuthUriRequest
        Oauth2Provider -->> Oauth2Actor[http/nats]: GetAuthUriResponse
        Oauth2Actor[http/nats] ->> AuthCache: store csrf_token
        Oauth2Actor[http/nats] ->> ngs: push auth_uri
        Client -->> ngs: pull auth_uri
        Client ->> Client: confirm authentication
        Client ->> ngs: push auth_code, state
        Oauth2Actor[http/nats] -->> ngs: pull auth_code, state
        Oauth2Actor[http/nats] -->> AuthCache: get csrf_state
        Oauth2Actor[http/nats] ->> Oauth2Provider: AuthorizeRequest
        Oauth2Provider ->> Oauth2Provider: compare state, csrf_state
        Oauth2Provider ->> SocialProvider: auth_code, option(pkce_challenge)
        SocialProvider -->> Oauth2Provider: token exchange
        Oauth2Provider -->> Oauth2Actor[http/nats]: AuthorizeResponse
        Oauth2Actor[http/nats] ->> AuthCache: Token, UserID
        Oauth2Actor[http/nats] -->> Client: Authenticated
    end
```
