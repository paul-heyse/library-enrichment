# `fastmcp.server.auth.providers.workos`

Distribution: `fastmcp`

## logger

`fastmcp.server.auth.providers.workos.logger`

```python
logger = get_logger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## AuthKitProvider

`fastmcp.server.auth.providers.workos.AuthKitProvider`

```python
class AuthKitProvider(RemoteAuthProvider)
```

**Bases** `RemoteAuthProvider`

**Declared members (4)**

- `authkit_domain = str(authkit_domain).rstrip('/')`  _instance-attribute_
- `base_url = AnyHttpUrl(str(base_url).rstrip('/'))`  _instance-attribute_
- `def get_routes(self, mcp_path: str | None = None) -> list[Route]`
  Get OAuth routes including AuthKit authorization server metadata forwarding.
- `def set_mcp_path(self, mcp_path: str | None) -> None`
  Bind the default verifier's audience to this server's resource URL.

**Inherited (12)**

- from `fastmcp.server.auth.auth.AuthProvider`: `challenge_scopes`, `get_middleware`, `get_well_known_routes`, `required_scopes`, `resource_base_url`
- from `fastmcp.server.auth.auth.RemoteAuthProvider`: `authorization_servers`, `get_challenge_scopes`, `resource_documentation`, `resource_name`, `scopes_supported`, `token_verifier`, `verify_token`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

AuthKit metadata provider for DCR (Dynamic Client Registration).

This provider implements AuthKit integration using metadata forwarding
instead of OAuth proxying. This is the recommended approach for WorkOS DCR
as it allows WorkOS to handle the OAuth flow directly while FastMCP acts
as a resource server.

IMPORTANT SETUP REQUIREMENTS:

1. Enable Dynamic Client Registration in WorkOS Dashboard:
   - Go to Applications → Configuration
   - Toggle "Dynamic Client Registration" to enabled

2. Configure your FastMCP server URL as a callback:
   - Add your server URL to the Redirects tab in WorkOS dashboard
   - Example: https://your-fastmcp-server.com/oauth2/callback

For detailed setup instructions, see:
https://workos.com/docs/authkit/mcp/integrating/token-verification

Token audience is bound to this server automatically: when the MCP
mount path becomes known (typically at ``http_app()`` construction),
``JWTVerifier.audience`` is set to the resource URL advertised in
``.well-known/oauth-protected-resource``. Enable Resource Indicators
(RFC 8707) in your WorkOS Dashboard and list that same URL — AuthKit
will then mint tokens with the matching ``aud`` claim.

Example:
    ```python
    from fastmcp.server.auth.providers.workos import AuthKitProvider

    workos_auth = AuthKitProvider(
        authkit_domain="https://your-workos-domain.authkit.app",
        base_url="https://your-fastmcp-server.com",
    )

    mcp = FastMCP("My App", auth=workos_auth)
    ```


## WorkOSProvider

`fastmcp.server.auth.providers.workos.WorkOSProvider`

```python
class WorkOSProvider(OAuthProxy)
```

**Bases** `OAuthProxy`

**Inherited (28)**

- from `fastmcp.server.auth.auth.AuthProvider`: `base_url`, `challenge_scopes`, `get_challenge_scopes`, `get_middleware`, `required_scopes`, `resource_base_url`
- from `fastmcp.server.auth.auth.OAuthProvider`: `client_registration_options`, `get_well_known_routes`, `issuer_url`, `revocation_options`, `scopes_supported`, `service_documentation_url`, `verify_token`
- from `fastmcp.server.auth.oauth_proxy.proxy.OAuthProxy`: `authorize`, `exchange_authorization_code`, `exchange_identity_assertion`, `exchange_refresh_token`, `get_client`, `get_routes`, `jwt_issuer`, `load_access_token`, `load_authorization_code`, `load_refresh_token`, `register_client`, `revoke_token`, `set_mcp_path`, `token_endpoint_url`, `update_default_scopes`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Complete WorkOS OAuth provider for FastMCP.

This provider implements WorkOS AuthKit OAuth using the OAuth Proxy pattern.
It provides OAuth2 authentication for users through WorkOS Connect applications.

Features:
- Transparent OAuth proxy to WorkOS AuthKit
- Automatic token validation via userinfo endpoint
- User information extraction from ID tokens
- Support for standard OAuth scopes (openid, profile, email)

Setup Requirements:
1. Create a WorkOS Connect application in your dashboard
2. Note your AuthKit domain (e.g., "https://your-app.authkit.app")
3. Configure redirect URI as: http://localhost:8000/auth/callback
4. Note your Client ID and Client Secret

Example:
    ```python
    from fastmcp import FastMCP
    from fastmcp.server.auth.providers.workos import WorkOSProvider

    auth = WorkOSProvider(
        client_id="client_123",
        client_secret="sk_test_456",
        authkit_domain="https://your-app.authkit.app",
        base_url="http://localhost:8000"
    )

    mcp = FastMCP("My App", auth=auth)
    ```


## WorkOSTokenVerifier

`fastmcp.server.auth.providers.workos.WorkOSTokenVerifier`

```python
class WorkOSTokenVerifier(TokenVerifier)
```

**Bases** `TokenVerifier`

**Declared members (3)**

- `authkit_domain = authkit_domain.rstrip('/')`  _instance-attribute_
- `timeout_seconds = timeout_seconds`  _instance-attribute_
- `async def verify_token(self, token: str) -> AccessToken | None`  _async_
  Verify WorkOS OAuth token by calling userinfo endpoint.

**Inherited (10)**

- from `fastmcp.server.auth.auth.AuthProvider`: `base_url`, `challenge_scopes`, `get_challenge_scopes`, `get_middleware`, `get_routes`, `get_well_known_routes`, `required_scopes`, `resource_base_url`, `scopes_supported`, `set_mcp_path`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Token verifier for WorkOS OAuth tokens.

WorkOS AuthKit tokens are opaque, so we verify them by calling
the /oauth2/userinfo endpoint to check validity and get user info.


