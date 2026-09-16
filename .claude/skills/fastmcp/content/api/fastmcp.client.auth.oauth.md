# `fastmcp.client.auth.oauth`

Distribution: `fastmcp`

## __all__

`fastmcp.client.auth.oauth.__all__`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
__all__ = ['OAuth']
```

## logger

`fastmcp.client.auth.oauth.logger`

```python
logger = get_logger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## ClientNotFoundError

`fastmcp.client.auth.oauth.ClientNotFoundError`

```python
class ClientNotFoundError(Exception)
```

**Bases** `Exception`

Raised when OAuth client credentials are not found on the server.


## ExpiredClientRegistrationError

`fastmcp.client.auth.oauth.ExpiredClientRegistrationError`

```python
class ExpiredClientRegistrationError(Exception)
```

**Bases** `Exception`

Raised when dynamic registration returns an expired client secret.


## OAuth

Import as `fastmcp.client.OAuth`  ·  defined at `fastmcp.client.auth.oauth.OAuth`

```python
class OAuth(OAuthClientProvider)
```

**Also exported as** `fastmcp.client.OAuth`, `fastmcp.client.auth.OAuth`

_4 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `OAuthClientProvider`

**Declared members (4)**

- `async def async_auth_flow(self, request: httpx2.Request) -> AsyncGenerator[httpx2.Request, httpx2.Response]`  _async_
  HTTPX auth flow with automatic retry on stale cached credentials.
- `async def callback_handler(self) -> AuthorizationCodeResult`  _async_
  Handle OAuth callback and return the authorization code result.
- `httpx_client_factory = httpx_client_factory or httpx2.AsyncClient`  _instance-attribute_
- `async def redirect_handler(self, authorization_url: str) -> None`  _async_
  Open browser for authorization, with pre-flight check for invalid client.

OAuth client provider for MCP servers with browser-based authentication.

This class provides OAuth authentication for FastMCP clients by opening
a browser for user authorization and running a local callback server.


## TokenStorageAdapter

Import as `fastmcp.client.auth.client_credentials.TokenStorageAdapter`  ·  defined at `fastmcp.client.auth.oauth.TokenStorageAdapter`

```python
class TokenStorageAdapter(TokenStorage)
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `TokenStorage`

**Declared members (6)**

- `async def clear(self) -> None`  _async_
- `async def get_client_info(self) -> OAuthClientInformationFull | None`  _async_
- `async def get_token_expiry(self) -> float | None`  _async_
- `async def get_tokens(self) -> OAuthToken | None`  _async_
- `async def set_client_info(self, client_info: OAuthClientInformationFull) -> None`  _async_
- `async def set_tokens(self, tokens: OAuthToken) -> None`  _async_

## _format_callback_host_for_url

`fastmcp.client.auth.oauth._format_callback_host_for_url`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _format_callback_host_for_url(host: str) -> str
```

## _normalize_callback_host_for_bind

`fastmcp.client.auth.oauth._normalize_callback_host_for_bind`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _normalize_callback_host_for_bind(host: str) -> str
```

## check_if_auth_required

`fastmcp.client.auth.oauth.check_if_auth_required`

```python
async def check_if_auth_required(mcp_url: str, httpx_kwargs: dict[str, Any] | None = None) -> bool
```

Check if the MCP endpoint requires authentication by making a test request.

Returns:
    True if auth appears to be required, False otherwise


