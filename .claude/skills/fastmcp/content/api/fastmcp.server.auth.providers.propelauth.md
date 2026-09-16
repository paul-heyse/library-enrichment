# `fastmcp.server.auth.providers.propelauth`

Distribution: `fastmcp`

## logger

`fastmcp.server.auth.providers.propelauth.logger`

```python
logger = get_logger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## PropelAuthProvider

`fastmcp.server.auth.providers.propelauth.PropelAuthProvider`

```python
class PropelAuthProvider(RemoteAuthProvider)
```

**Bases** `RemoteAuthProvider`

**Declared members (2)**

- `def get_routes(self, mcp_path: str | None = None) -> list[Route]`
  Get routes for this provider.
- `async def verify_token(self, token: str) -> AccessToken | None`  _async_
  Verify token and check the ``aud`` claim against the configured resource.

**Inherited (13)**

- from `fastmcp.server.auth.auth.AuthProvider`: `challenge_scopes`, `get_middleware`, `get_well_known_routes`, `required_scopes`, `resource_base_url`, `set_mcp_path`
- from `fastmcp.server.auth.auth.RemoteAuthProvider`: `authorization_servers`, `base_url`, `get_challenge_scopes`, `resource_documentation`, `resource_name`, `scopes_supported`, `token_verifier`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

PropelAuth resource server provider using OAuth 2.1 token introspection.

This provider validates access tokens via PropelAuth's introspection endpoint
and forwards authorization server metadata for OAuth discovery.

Setup:
    1. Enable MCP authentication in the PropelAuth Dashboard
    2. Configure scopes on the MCP page
    3. Select which redirect URIs to enable by picking which clients you support
    4. Generate introspection credentials (Client ID + Client Secret)

For detailed setup instructions, see:
https://docs.propelauth.com/mcp-authentication/overview

Example:
    ```python
    from fastmcp import FastMCP
    from fastmcp.server.auth.providers.propelauth import PropelAuthProvider

    auth = PropelAuthProvider(
        auth_url="https://auth.yourdomain.com",
        introspection_client_id="your-client-id",
        introspection_client_secret="your-client-secret",
        base_url="https://your-fastmcp-server.com",
        required_scopes=["read:user_data"],
    )

    mcp = FastMCP("My App", auth=auth)
    ```


## PropelAuthTokenIntrospectionOverrides

`fastmcp.server.auth.providers.propelauth.PropelAuthTokenIntrospectionOverrides`

```python
class PropelAuthTokenIntrospectionOverrides(TypedDict)
```

**Bases** `TypedDict`

**Declared members (4)**

- `cache_ttl_seconds: int | None`  _instance-attribute_
- `http_client: httpx2.AsyncClient | None`  _instance-attribute_
- `max_cache_size: int | None`  _instance-attribute_
- `timeout_seconds: int`  _instance-attribute_

