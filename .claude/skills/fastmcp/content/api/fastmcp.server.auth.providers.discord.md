# `fastmcp.server.auth.providers.discord`

Distribution: `fastmcp`

## logger

`fastmcp.server.auth.providers.discord.logger`

```python
logger = get_logger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## DiscordProvider

`fastmcp.server.auth.providers.discord.DiscordProvider`

```python
class DiscordProvider(OAuthProxy)
```

**Bases** `OAuthProxy`

**Inherited (28)**

- from `fastmcp.server.auth.auth.AuthProvider`: `base_url`, `challenge_scopes`, `get_challenge_scopes`, `get_middleware`, `required_scopes`, `resource_base_url`
- from `fastmcp.server.auth.auth.OAuthProvider`: `client_registration_options`, `get_well_known_routes`, `issuer_url`, `revocation_options`, `scopes_supported`, `service_documentation_url`, `verify_token`
- from `fastmcp.server.auth.oauth_proxy.proxy.OAuthProxy`: `authorize`, `exchange_authorization_code`, `exchange_identity_assertion`, `exchange_refresh_token`, `get_client`, `get_routes`, `jwt_issuer`, `load_access_token`, `load_authorization_code`, `load_refresh_token`, `register_client`, `revoke_token`, `set_mcp_path`, `token_endpoint_url`, `update_default_scopes`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Complete Discord OAuth provider for FastMCP.

This provider makes it trivial to add Discord OAuth protection to any
FastMCP server. Just provide your Discord OAuth app credentials and
a base URL, and you're ready to go.

Features:
- Transparent OAuth proxy to Discord
- Automatic token validation via Discord's API
- User information extraction from Discord APIs
- Minimal configuration required

Example:
    ```python
    from fastmcp import FastMCP
    from fastmcp.server.auth.providers.discord import DiscordProvider

    auth = DiscordProvider(
        client_id="123456789",
        client_secret="discord-client-secret-abc123...",
        base_url="https://my-server.com"
    )

    mcp = FastMCP("My App", auth=auth)
    ```


## DiscordTokenVerifier

`fastmcp.server.auth.providers.discord.DiscordTokenVerifier`

```python
class DiscordTokenVerifier(TokenVerifier)
```

**Bases** `TokenVerifier`

**Declared members (3)**

- `expected_client_id = expected_client_id`  _instance-attribute_
- `timeout_seconds = timeout_seconds`  _instance-attribute_
- `async def verify_token(self, token: str) -> AccessToken | None`  _async_
  Verify Discord OAuth token by calling Discord's tokeninfo API.

**Inherited (10)**

- from `fastmcp.server.auth.auth.AuthProvider`: `base_url`, `challenge_scopes`, `get_challenge_scopes`, `get_middleware`, `get_routes`, `get_well_known_routes`, `required_scopes`, `resource_base_url`, `scopes_supported`, `set_mcp_path`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Token verifier for Discord OAuth tokens.

Discord OAuth tokens are opaque (not JWTs), so we verify them
by calling Discord's tokeninfo API to check if they're valid and get user info.


