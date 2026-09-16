# `fastmcp.server.auth.providers.auth0`

Distribution: `fastmcp`

## logger

`fastmcp.server.auth.providers.auth0.logger`

```python
logger = get_logger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## Auth0JWTVerifier

`fastmcp.server.auth.providers.auth0.Auth0JWTVerifier`

```python
class Auth0JWTVerifier(JWTVerifier)
```

**Bases** `JWTVerifier`

**Inherited (19)**

- from `fastmcp.server.auth.auth.AuthProvider`: `base_url`, `challenge_scopes`, `get_challenge_scopes`, `get_middleware`, `get_routes`, `get_well_known_routes`, `required_scopes`, `resource_base_url`, `scopes_supported`, `set_mcp_path`
- from `fastmcp.server.auth.providers.jwt.JWTVerifier`: `algorithm`, `audience`, `issuer`, `jwks_uri`, `load_access_token`, `logger`, `public_key`, `ssrf_safe`, `verify_token`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

JWT verifier for Auth0 MCP access tokens.

Auth0's ``rfc9068_profile_authz`` token dialect exposes API permissions in
the ``permissions`` claim. Standard OAuth ``scope``/``scp`` claims are checked
first; ``permissions`` is included when present.


## Auth0MCPProvider

`fastmcp.server.auth.providers.auth0.Auth0MCPProvider`

```python
class Auth0MCPProvider(RemoteAuthProvider)
```

**Bases** `RemoteAuthProvider`

**Declared members (4)**

- `base_url = AnyHttpUrl(str(base_url).rstrip('/'))`  _instance-attribute_
- `def get_routes(self, mcp_path: str | None = None) -> list[Route]`
  Protected resource routes plus Auth0 authorization server metadata.
- `issuer = str(oidc_config.issuer).rstrip('/')`  _instance-attribute_
- `def set_mcp_path(self, mcp_path: str | None) -> None`
  Bind the default verifier's audience to this server's resource URL.

**Inherited (12)**

- from `fastmcp.server.auth.auth.AuthProvider`: `challenge_scopes`, `get_middleware`, `get_well_known_routes`, `required_scopes`, `resource_base_url`
- from `fastmcp.server.auth.auth.RemoteAuthProvider`: `authorization_servers`, `get_challenge_scopes`, `resource_documentation`, `resource_name`, `scopes_supported`, `token_verifier`, `verify_token`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Auth0 resource server provider for Auth for MCP (DCR/CIMD).

FastMCP validates access tokens issued by Auth0 while Auth0 handles OAuth,
dynamic client registration, and CIMD approval in the tenant dashboard.

Enable the Resource Parameter Compatibility Profile in Auth0 and create an
API whose identifier matches this server's resource URL (logged at startup).

Example:
    ```python
    from fastmcp.server.auth.providers.auth0 import Auth0MCPProvider

    auth = Auth0MCPProvider(
        config_url="https://your-tenant.auth0.com/.well-known/openid-configuration",
        base_url="http://127.0.0.1:8000",
    )
    ```


## Auth0Provider

`fastmcp.server.auth.providers.auth0.Auth0Provider`

```python
class Auth0Provider(OIDCProxy)
```

**Bases** `OIDCProxy`

**Inherited (31)**

- from `fastmcp.server.auth.auth.AuthProvider`: `base_url`, `challenge_scopes`, `get_challenge_scopes`, `get_middleware`, `resource_base_url`
- from `fastmcp.server.auth.auth.OAuthProvider`: `client_registration_options`, `get_well_known_routes`, `issuer_url`, `revocation_options`, `scopes_supported`, `service_documentation_url`, `verify_token`
- from `fastmcp.server.auth.oauth_proxy.proxy.OAuthProxy`: `authorize`, `exchange_authorization_code`, `exchange_identity_assertion`, `exchange_refresh_token`, `get_client`, `get_routes`, `jwt_issuer`, `load_access_token`, `load_authorization_code`, `load_refresh_token`, `register_client`, `revoke_token`, `set_mcp_path`, `token_endpoint_url`, `update_default_scopes`
- from `fastmcp.server.auth.oidc_proxy.OIDCProxy`: `get_oidc_configuration`, `get_token_verifier`, `oidc_config`, `required_scopes`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

An Auth0 provider implementation for FastMCP.

This provider is a complete Auth0 integration that's ready to use with
just the configuration URL, client ID, client secret, audience, and base URL.

Example:
    ```python
    from fastmcp import FastMCP
    from fastmcp.server.auth.providers.auth0 import Auth0Provider

    # Simple Auth0 OAuth protection
    auth = Auth0Provider(
        config_url="https://auth0.config.url",
        client_id="your-auth0-client-id",
        client_secret="your-auth0-client-secret",
        audience="your-auth0-api-audience",
        base_url="http://localhost:8000",
    )

    mcp = FastMCP("My Protected Server", auth=auth)
    ```


