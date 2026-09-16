# `fastmcp.server.auth.oauth_proxy.upstream`

Distribution: `fastmcp`

## _DEFAULT_TOKEN_HEADERS

`fastmcp.server.auth.oauth_proxy.upstream._DEFAULT_TOKEN_HEADERS`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_DEFAULT_TOKEN_HEADERS = {'Accept': 'application/json', 'Content-Type': 'application/x-www-form-urlencoded;charset=UTF-8'}
```

## __all__

`fastmcp.server.auth.oauth_proxy.upstream.__all__`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
__all__ = ['AsyncOAuth2Client', 'OAuthError']
```

## AsyncOAuth2Client

Import as `fastmcp.server.auth.oauth_proxy.proxy.AsyncOAuth2Client`  ·  defined at `fastmcp.server.auth.oauth_proxy.upstream.AsyncOAuth2Client`

```python
class AsyncOAuth2Client
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (6)**

- `async def aclose(self) -> None`  _async_
- `client_id = client_id`  _instance-attribute_
- `client_secret = client_secret`  _instance-attribute_
- `async def fetch_token(self, url: str, grant_type: str = 'authorization_code', params: Any = {}) -> dict[str, Any]`  _async_
  Exchange an authorization grant for tokens at the token endpoint.
- `async def refresh_token(self, url: str, refresh_token: str | None = None, params: Any = {}) -> dict[str, Any]`  _async_
  Fetch a new access token using a refresh token.
- `token_endpoint_auth_method = token_endpoint_auth_method or 'client_secret_basic'`  _instance-attribute_

Minimal async OAuth2 client for upstream token-endpoint interactions.

Drop-in replacement for the slice of authlib's `AsyncOAuth2Client` that
`OAuthProxy` uses. Subclasses of `OAuthProxy` that override
`_create_upstream_oauth_client` may return any object with the same
`fetch_token`/`refresh_token`/`client_secret`/`aclose` surface.


## OAuthError

`fastmcp.server.auth.oauth_proxy.upstream.OAuthError`

```python
OAuthError  # re-exported from authlib.integrations.base_client.OAuthError
```

