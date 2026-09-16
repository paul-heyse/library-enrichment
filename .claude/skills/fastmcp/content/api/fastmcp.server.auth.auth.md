# `fastmcp.server.auth.auth`

Distribution: `fastmcp`

## JWT_BEARER_ASSERTION_TYPE

`fastmcp.server.auth.auth.JWT_BEARER_ASSERTION_TYPE`

```python
JWT_BEARER_ASSERTION_TYPE = 'urn:ietf:params:oauth:client-assertion-type:jwt-bearer'
```

**Inferred type** (`ty`, not declared in the source): `Literal["urn:ietf:params:oauth:client-assertion-type:jwt-bearer"]`

## logger

`fastmcp.server.auth.auth.logger`

```python
logger = get_logger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## AccessToken

Import as `fastmcp.server.auth.AccessToken`  ·  defined at `fastmcp.server.auth.auth.AccessToken`

```python
class AccessToken(_SDKAccessToken)
```

**Also exported as** `fastmcp.server.auth.AccessToken`, `fastmcp.server.dependencies.AccessToken`

_13 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `_SDKAccessToken`

**Declared members (1)**

- `claims: dict[str, Any] = Field(default_factory=dict)`  _class-attribute, instance-attribute_

AccessToken that includes all JWT claims.


## AuthProvider

Import as `fastmcp.server.auth.AuthProvider`  ·  defined at `fastmcp.server.auth.auth.AuthProvider`

```python
class AuthProvider(TokenVerifierProtocol)
```

**Also exported as** `fastmcp.server.auth.AuthProvider`

_3 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `TokenVerifierProtocol`

**Declared members (11)**

- `base_url = base_url`  _instance-attribute_
- `challenge_scopes: list[str]`  _property_
  Scopes clients must request to access this resource.
- `def get_challenge_scopes(self, required_scopes: list[str] | None = None) -> list[str]`
  Translate validation scopes into scopes clients should request.
- `def get_middleware(self) -> list`
  Get HTTP application-level middleware for this auth provider.
- `def get_routes(self, mcp_path: str | None = None) -> list[Route]`
  Get all routes for this authentication provider.
- `def get_well_known_routes(self, mcp_path: str | None = None) -> list[Route]`
  Get well-known discovery routes for this authentication provider.
- `required_scopes = required_scopes or []`  _instance-attribute_
- `resource_base_url = resource_base_url`  _instance-attribute_
- `scopes_supported: list[str]`  _property_
  Scopes advertised in protected resource metadata.
- `def set_mcp_path(self, mcp_path: str | None) -> None`
  Set the MCP endpoint path and compute resource URL.
- `async def verify_token(self, token: str) -> AccessToken | None`  _async_
  Verify a bearer token and return access info if valid.

Base class for all FastMCP authentication providers.

This class provides a unified interface for all authentication providers,
whether they are simple token verifiers or full OAuth authorization servers.
All providers must be able to verify tokens and can optionally provide
custom authentication routes.


## MultiAuth

Import as `fastmcp.server.auth.MultiAuth`  ·  defined at `fastmcp.server.auth.auth.MultiAuth`

```python
class MultiAuth(AuthProvider)
```

**Also exported as** `fastmcp.server.auth.MultiAuth`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `AuthProvider`

**Declared members (8)**

- `def get_challenge_scopes(self, required_scopes: list[str] | None = None) -> list[str]`
  Translate effective scopes through an unambiguous auth source.
- `def get_routes(self, mcp_path: str | None = None) -> list[Route]`
  Delegate route creation to the server.
- `def get_well_known_routes(self, mcp_path: str | None = None) -> list[Route]`
  Delegate well-known route creation to the server.
- `scopes_supported: list[str]`  _property_
  Scopes advertised by the delegated auth server.
- `server = server`  _instance-attribute_
- `def set_mcp_path(self, mcp_path: str | None) -> None`
  Propagate MCP path to the server and all verifiers.
- `verifiers = list(verifiers)`  _instance-attribute_
- `async def verify_token(self, token: str) -> AccessToken | None`  _async_
  Verify a token by trying the server, then each verifier in order.

**Inherited (5)**

- from `fastmcp.server.auth.auth.AuthProvider`: `base_url`, `challenge_scopes`, `get_middleware`, `required_scopes`, `resource_base_url`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Composes an optional auth server with additional token verifiers.

Use this when a single server needs to accept tokens from multiple sources.
For example, an OAuth proxy for interactive clients combined with a JWT
verifier for machine-to-machine tokens.

Token verification tries the server first (if present), then each verifier
in order, returning the first successful result. Routes and OAuth metadata
come from the server; verifiers contribute only token verification.

Example:
    ```python
    from fastmcp import FastMCP
    from fastmcp.server.auth import MultiAuth, JWTVerifier, OAuthProxy

    upstream = OAuthProxy(
        upstream_authorization_endpoint="https://login.example.com/oauth/authorize",
        upstream_token_endpoint="https://login.example.com/oauth/token",
        upstream_client_id="my-app",
        upstream_client_secret="secret",
        token_verifier=JWTVerifier(
            jwks_uri="https://login.example.com/.well-known/jwks.json"
        ),
        base_url="https://my-server.com",
    )

    auth = MultiAuth(
        server=upstream,
        verifiers=[JWTVerifier(jwks_uri="https://example.com/.well-known/jwks.json")],
    )
    mcp = FastMCP("my-server", auth=auth)
    ```


## OAuthProvider

Import as `fastmcp.server.auth.OAuthProvider`  ·  defined at `fastmcp.server.auth.auth.OAuthProvider`

```python
class OAuthProvider(AuthProvider, OAuthAuthorizationServerProvider[AuthorizationCode, RefreshToken, AccessToken])
```

**Also exported as** `fastmcp.server.auth.OAuthProvider`

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `AuthProvider`, `OAuthAuthorizationServerProvider[AuthorizationCode, RefreshToken, AccessToken]`

**Declared members (8)**

- `client_registration_options = client_registration_options`  _instance-attribute_
- `def get_routes(self, mcp_path: str | None = None) -> list[Route]`
  Get OAuth authorization server routes and optional protected resource routes.
- `def get_well_known_routes(self, mcp_path: str | None = None) -> list[Route]`
  Get well-known discovery routes with RFC 8414 path-aware support.
- `issuer_url = self.base_url`  _instance-attribute_
- `revocation_options = revocation_options`  _instance-attribute_
- `scopes_supported: list[str]`  _property_
  Scopes advertised by this authorization server.
- `service_documentation_url = service_documentation_url`  _instance-attribute_
- `async def verify_token(self, token: str) -> AccessToken | None`  _async_
  Verify a bearer token and return access info if valid.

**Inherited (7)**

- from `fastmcp.server.auth.auth.AuthProvider`: `base_url`, `challenge_scopes`, `get_challenge_scopes`, `get_middleware`, `required_scopes`, `resource_base_url`, `set_mcp_path`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

OAuth Authorization Server provider.

This class provides full OAuth server functionality including client registration,
authorization flows, token issuance, and token verification.


## PrivateKeyJWTClientAuthenticator

Import as `fastmcp.server.auth.oauth_proxy.proxy.PrivateKeyJWTClientAuthenticator`  ·  defined at `fastmcp.server.auth.auth.PrivateKeyJWTClientAuthenticator`

```python
class PrivateKeyJWTClientAuthenticator(_SDKClientAuthenticator)
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `_SDKClientAuthenticator`

**Declared members (1)**

- `async def authenticate_request(self, request: Request) -> OAuthClientInformationFull`  _async_
  Authenticate a client from an HTTP request.

Client authenticator with private_key_jwt support for CIMD clients.

Extends the SDK's ClientAuthenticator to add support for the `private_key_jwt`
authentication method per RFC 7523. This is required for CIMD (Client ID Metadata
Document) clients that use asymmetric keys for authentication.

The authenticator:
1. Delegates to SDK for standard methods (client_secret_basic, client_secret_post, none)
2. Adds private_key_jwt handling for CIMD clients
3. Validates JWT assertions against client's JWKS


## RemoteAuthProvider

Import as `fastmcp.server.auth.RemoteAuthProvider`  ·  defined at `fastmcp.server.auth.auth.RemoteAuthProvider`

```python
class RemoteAuthProvider(AuthProvider)
```

**Also exported as** `fastmcp.server.auth.RemoteAuthProvider`

_7 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `AuthProvider`

**Declared members (9)**

- `authorization_servers = authorization_servers`  _instance-attribute_
- `base_url: AnyHttpUrl`  _instance-attribute_
- `def get_challenge_scopes(self, required_scopes: list[str] | None = None) -> list[str]`
  Translate effective validation scopes for the authorization server.
- `def get_routes(self, mcp_path: str | None = None) -> list[Route]`
  Get routes for this provider.
- `resource_documentation = resource_documentation`  _instance-attribute_
- `resource_name = resource_name`  _instance-attribute_
- `scopes_supported: list[str]`  _property_
  Scopes advertised in protected resource metadata.
- `token_verifier = token_verifier`  _instance-attribute_
- `async def verify_token(self, token: str) -> AccessToken | None`  _async_
  Verify token using the configured token verifier.

**Inherited (6)**

- from `fastmcp.server.auth.auth.AuthProvider`: `challenge_scopes`, `get_middleware`, `get_well_known_routes`, `required_scopes`, `resource_base_url`, `set_mcp_path`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Authentication provider for resource servers that verify tokens from known authorization servers.

This provider composes a TokenVerifier with authorization server metadata to create
standardized OAuth 2.0 Protected Resource endpoints (RFC 9728). Perfect for:
- JWT verification with known issuers
- Remote token introspection services
- Any resource server that knows where its tokens come from

Use this when you have token verification logic and want to advertise
the authorization servers that issue valid tokens.


## TokenHandler

Import as `fastmcp.server.auth.oauth_proxy.proxy.TokenHandler`  ·  defined at `fastmcp.server.auth.auth.TokenHandler`

```python
class TokenHandler(_SDKTokenHandler)
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `_SDKTokenHandler`

**Declared members (1)**

- `async def handle(self, request: Any)`  _async_
  Wrap SDK handle() and transform auth error responses.

TokenHandler that returns MCP-compliant error responses.

This handler addresses two SDK issues:

1. Error code: The SDK returns `unauthorized_client` for client authentication
   failures, but RFC 6749 Section 5.2 requires `invalid_client` with HTTP 401.
   This distinction matters for client re-registration behavior.

2. Status code: The SDK returns HTTP 400 for all token errors including
   `invalid_grant` (expired/invalid tokens). However, the MCP spec requires:
   "Invalid or expired tokens MUST receive a HTTP 401 response."

This handler transforms responses to be compliant with both OAuth 2.1 and MCP specs.


## TokenVerifier

Import as `fastmcp.server.auth.TokenVerifier`  ·  defined at `fastmcp.server.auth.auth.TokenVerifier`

```python
class TokenVerifier(AuthProvider)
```

**Also exported as** `fastmcp.server.auth.TokenVerifier`

_16 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `AuthProvider`

**Declared members (1)**

- `async def verify_token(self, token: str) -> AccessToken | None`  _async_
  Verify a bearer token and return access info if valid.

**Inherited (10)**

- from `fastmcp.server.auth.auth.AuthProvider`: `base_url`, `challenge_scopes`, `get_challenge_scopes`, `get_middleware`, `get_routes`, `get_well_known_routes`, `required_scopes`, `resource_base_url`, `scopes_supported`, `set_mcp_path`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Base class for token verifiers (Resource Servers).

This class provides token verification capability without OAuth server functionality.
Token verifiers typically don't provide authentication routes by default.


