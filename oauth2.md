# Oauth2 Provider

``` mermaid
sequenceDiagram
    %% Actors
    participant Client
    participant Oauth2Actor
    participant Oauth2Provider
    participant AuthCache
    participant ngs
    participant SocialProvider

    critical User Connects Social Service
        Client ->> Oauth2AuthActor: click /login/${Social provider}
        Oauth2Actor ->> Oauth2Provider: GetAuthUriRequest
        Oauth2Provider -->> Oauth2Actor: GetAuthUriResponse
        Oauth2Actor ->> AuthCache: store csrf_token
        Oauth2Actor ->> ngs: push auth_uri
        client -->> ngs: pull auth_uri
        client ->> client: confirm authentication
        client ->> ngs: push auth_code, state
        Oauth2Actor -->> ngs: pull auth_code, state
        Oauth2Actor -->> AuthCache: get csrf_state
        Oauth2Actor ->> Oauth2Provider: AuthorizeRequest
        Oauth2Provider -->> Oauth2Actor: AuthorizeResponse
        Oauth2Actor ->> AuthCache: Token
    end
```
