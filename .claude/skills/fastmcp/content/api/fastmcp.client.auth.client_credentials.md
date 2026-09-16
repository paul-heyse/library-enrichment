# `fastmcp.client.auth.client_credentials`

Distribution: `fastmcp`

## __all__

`fastmcp.client.auth.client_credentials.__all__`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
__all__ = ['ClientCredentialsOAuthProvider', 'PrivateKeyJWTOAuthProvider', 'SignedJWTParameters', 'static_assertion_provider']
```

## _in_step_up

`fastmcp.client.auth.client_credentials._in_step_up`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_in_step_up: ContextVar[bool] = ContextVar('fastmcp_m2m_in_step_up', default=False)
```

## ClientCredentialsOAuthProvider

Import as `fastmcp.client.ClientCredentialsOAuthProvider`  ·  defined at `fastmcp.client.auth.client_credentials.ClientCredentialsOAuthProvider`

```python
class ClientCredentialsOAuthProvider(_SDKClientCredentialsOAuthProvider)
```

**Also exported as** `fastmcp.client.ClientCredentialsOAuthProvider`, `fastmcp.client.auth.ClientCredentialsOAuthProvider`

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `_SDKClientCredentialsOAuthProvider`

**Declared members (1)**

- `def async_auth_flow(self, request: httpx2.Request) -> AsyncGenerator[httpx2.Request, httpx2.Response]`

OAuth ``client_credentials`` provider using a client ID and secret.

This is the standard machine-to-machine flow: the client exchanges its
``client_id`` and ``client_secret`` at the authorization server's token
endpoint for an access token, which is then attached to every request. The
token endpoint is discovered from the MCP server's OAuth metadata, so callers
provide the MCP server URL rather than a raw token endpoint.

Example:
    ```python
    from fastmcp import Client
    from fastmcp.client.auth import ClientCredentialsOAuthProvider

    auth = ClientCredentialsOAuthProvider(
        client_id="my-client-id",
        client_secret="my-client-secret",
        scopes=["read", "write"],
    )

    async with Client("https://example.com/mcp", auth=auth) as client:
        await client.list_tools()
    ```


## PrivateKeyJWTOAuthProvider

Import as `fastmcp.client.PrivateKeyJWTOAuthProvider`  ·  defined at `fastmcp.client.auth.client_credentials.PrivateKeyJWTOAuthProvider`

```python
class PrivateKeyJWTOAuthProvider(_SDKPrivateKeyJWTOAuthProvider)
```

**Also exported as** `fastmcp.client.PrivateKeyJWTOAuthProvider`, `fastmcp.client.auth.PrivateKeyJWTOAuthProvider`

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `_SDKPrivateKeyJWTOAuthProvider`

**Declared members (1)**

- `def async_auth_flow(self, request: httpx2.Request) -> AsyncGenerator[httpx2.Request, httpx2.Response]`

OAuth ``client_credentials`` provider using ``private_key_jwt`` (RFC 7523).

Instead of a shared client secret, the client authenticates to the token
endpoint with a signed JWT assertion. The ``assertion_provider`` callback
receives the authorization server's issuer identifier (the required JWT
audience) and returns the assertion. Use
`SignedJWTParameters.create_assertion_provider()` to sign locally with a
private key, `static_assertion_provider()` for a pre-built JWT, or supply your
own callback for workload identity federation.

Example:
    ```python
    from pathlib import Path

    from fastmcp import Client
    from fastmcp.client.auth import (
        PrivateKeyJWTOAuthProvider,
        SignedJWTParameters,
    )

    private_key_pem = Path("client-signing-key.pem").read_text()

    jwt_params = SignedJWTParameters(
        issuer="my-client-id",
        subject="my-client-id",
        signing_key=private_key_pem,
    )
    auth = PrivateKeyJWTOAuthProvider(
        client_id="my-client-id",
        assertion_provider=jwt_params.create_assertion_provider(),
    )

    async with Client("https://example.com/mcp", auth=auth) as client:
        await client.list_tools()
    ```


## _cache_namespace

`fastmcp.client.auth.client_credentials._cache_namespace`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _cache_namespace(client_id: str, scopes: str | None) -> str
```

Namespace cached tokens by both client identity and requested scopes.

Two providers that differ in either their ``client_id`` or their requested
scopes must not share cached tokens: a token issued for one client or one
scope set is not interchangeable with another. Hashing a canonical
``(client_id, scopes)`` pair keeps the namespace unambiguous regardless of the
characters either value contains.


## _drive_flow_tracking_step_up

`fastmcp.client.auth.client_credentials._drive_flow_tracking_step_up`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
async def _drive_flow_tracking_step_up(flow: AsyncGenerator[httpx2.Request, httpx2.Response]) -> AsyncGenerator[httpx2.Request, httpx2.Response]
```

Delegate to the inherited auth flow, flagging step-up challenges.

On a 403 ``insufficient_scope`` the inherited flow unions the challenged scope
with the current one before re-requesting the token. Setting `_in_step_up`
lets `_perform_authorization` leave the accumulated scope in place instead of
re-pinning the caller's explicit scopes over it. The flag is set and reset
inside this generator, so it is scoped to exactly this flow.


## _is_insufficient_scope_challenge

`fastmcp.client.auth.client_credentials._is_insufficient_scope_challenge`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _is_insufficient_scope_challenge(response: httpx2.Response) -> bool
```

True when a response is an RFC 6750 ``insufficient_scope`` step-up challenge.


## _normalize_scopes

`fastmcp.client.auth.client_credentials._normalize_scopes`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _normalize_scopes(scopes: str | list[str] | None) -> str | None
```

Normalize scopes to a space-separated string (or None).


## _resolve_token_storage

`fastmcp.client.auth.client_credentials._resolve_token_storage`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _resolve_token_storage(token_storage: AsyncKeyValue | None, mcp_url: str, client_id: str, scopes: str | None) -> TokenStorageAdapter
```

Wrap a token store in the FastMCP adapter, defaulting to in-memory.

Unlike the interactive `OAuth` provider, M2M providers do not warn when using
in-memory storage: re-acquiring a token is a single non-interactive request,
so losing the cache on restart is cheap rather than disruptive.

The cache is namespaced by client identity and requested scopes so that
providers with different credentials or scope sets can share one store against
the same MCP endpoint without overwriting each other's tokens.


## _restore_token_expiry

`fastmcp.client.auth.client_credentials._restore_token_expiry`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
async def _restore_token_expiry(context: OAuthContext) -> None
```

Restore the persisted absolute token expiry after a token is reloaded.

The inherited initializer reloads the stored token but not its expiry, so a
provider recreated with persistent storage would treat an already-expired
token as still valid. Reading the absolute expiry back keeps `is_token_valid`
honest, prompting a fresh token request when the stored one has expired.

The restore is skipped unless the reloaded token itself declares an
`expires_in`. A token whose response omitted `expires_in` (``None``) is
non-expiring, and the store may still hold a stale expiry from a previous
token it replaced; applying that would wrongly force a re-exchange. A token
that declares `expires_in=0` is immediately expired and keeps its recorded
expiry, so it is distinguished from an omitted one.


