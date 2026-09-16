# `mcp.shared._httpx_utils`

Distribution: `mcp`

## MCP_DEFAULT_SSE_READ_TIMEOUT

`mcp.shared._httpx_utils.MCP_DEFAULT_SSE_READ_TIMEOUT`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
MCP_DEFAULT_SSE_READ_TIMEOUT = 300.0
```

## MCP_DEFAULT_TIMEOUT

`mcp.shared._httpx_utils.MCP_DEFAULT_TIMEOUT`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
MCP_DEFAULT_TIMEOUT = 30.0
```

## _AUTH_REDIRECT_LIMIT

`mcp.shared._httpx_utils._AUTH_REDIRECT_LIMIT`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_AUTH_REDIRECT_LIMIT = 5
```

## _SSE_HEADERS

`mcp.shared._httpx_utils._SSE_HEADERS`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_SSE_HEADERS = {'Accept': 'text/event-stream', 'Cache-Control': 'no-store'}
```

## __all__

`mcp.shared._httpx_utils.__all__`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
__all__ = ['create_mcp_http_client', 'MCP_DEFAULT_TIMEOUT', 'MCP_DEFAULT_SSE_READ_TIMEOUT']
```

## McpHttpClientFactory

Import as `mcp.client.sse.McpHttpClientFactory`  ·  defined at `mcp.shared._httpx_utils.McpHttpClientFactory`

```python
class McpHttpClientFactory(Protocol)
```

_4 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `Protocol`

## RedirectAwareAuth

Import as `mcp.client.auth.oauth2.RedirectAwareAuth`  ·  defined at `mcp.shared._httpx_utils.RedirectAwareAuth`

```python
class RedirectAwareAuth(ABC, httpx2.Auth)
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `ABC`, `httpx2.Auth`

**Declared members (1)**

- `async def async_auth_flow(self, request: httpx2.Request) -> AsyncGenerator[httpx2.Request, httpx2.Response]`  _async_

An `httpx2.Auth` whose own requests follow redirects the way MCP transport requests do.

The transports send every request with redirect following off and follow a
redirect themselves only within the endpoint's origin (`stream_within_origin`).
httpx2 applies that per-request setting to the requests an auth flow makes
too (metadata discovery, registration, token), so on their own those would
follow nothing. Subclasses write their flow as `_auth_flow`; this class
drives it and, for each request the flow makes other than the one being
authenticated, follows a redirect that `next_request_within_origin` accepts,
up to `_AUTH_REDIRECT_LIMIT` times. Any other redirect response is handed
to the flow as it is.


## _within_origin

`mcp.shared._httpx_utils._within_origin`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _within_origin(url: httpx2.URL, location: httpx2.URL) -> bool
```

Whether `location` is on `url`'s origin, or is its https upgrade on the default ports.

httpx2 normalises a scheme's default port to None and lower-cases hosts, so
plain tuple comparison is exact. The upgrade rule is the one httpx2 itself
uses to decide a redirect has not left the origin (`_is_https_redirect`).


## create_mcp_http_client

Import as `mcp.client.sse.create_mcp_http_client`  ·  defined at `mcp.shared._httpx_utils.create_mcp_http_client`

```python
def create_mcp_http_client(headers: dict[str, str] | None = None, timeout: httpx2.Timeout | None = None, auth: httpx2.Auth | None = None) -> httpx2.AsyncClient
```

_4 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Create an httpx2 AsyncClient with the MCP transports' default timeouts.

The client uses a 30-second timeout for connect/write/pool and a 300-second
read timeout, because a server may hold a response stream open. Redirect
following is left at the httpx2 default (off): the MCP transports follow
redirects within the endpoint's origin themselves, see `stream_within_origin`.

Args:
    headers: Optional headers to include with all requests.
    timeout: Request timeout as httpx2.Timeout object. Defaults to 30s for
        connect/write/pool and 300s for read (for long-lived SSE streams).
    auth: Optional authentication handler.

Returns:
    Configured httpx2.AsyncClient instance.

Note:
    The returned AsyncClient must be used as a context manager to ensure
    proper cleanup of connections.


## next_request_within_origin

`mcp.shared._httpx_utils.next_request_within_origin`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def next_request_within_origin(response: httpx2.Response) -> httpx2.Request | None
```

The request that follows `response`'s redirect, if it is one the MCP transports follow.

That is when httpx2 built a next request for it (a redirect status with a
Location), the next request keeps the method (307/308, or any redirect of a
GET: httpx2 turns a POST into a body-less GET for 301/302/303, which would
drop the message), its URL stays within the origin of the request just sent
(same scheme, host and port, or http to https on the same host with default
ports), and the Location does not bring userinfo of its own (which httpx2
would otherwise send as Basic auth; userinfo the configured URL already had
is kept by a relative Location and is fine). None for anything else,
including a non-redirect.


## redirect_location

Import as `mcp.client.streamable_http.redirect_location`  ·  defined at `mcp.shared._httpx_utils.redirect_location`

```python
def redirect_location(response: httpx2.Response) -> httpx2.URL | None
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Where `response` redirects to, for use in a message: without userinfo, query or fragment,
which can carry state that does not belong in an error or a log line. None if not a redirect.


## redirect_note

Import as `mcp.client.auth.utils.redirect_note`  ·  defined at `mcp.shared._httpx_utils.redirect_note`

```python
def redirect_note(response: httpx2.Response) -> str
```

_3 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

A suffix naming the location of a redirect response that was not followed, else empty.


## request_within_origin

Import as `mcp.client.sse.request_within_origin`  ·  defined at `mcp.shared._httpx_utils.request_within_origin`

```python
async def request_within_origin(client: httpx2.AsyncClient, method: str, url: httpx2.URL | str, kwargs: Any = {}) -> httpx2.Response
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

`client.request(...)` with the redirect handling of `stream_within_origin`.


## sse_within_origin

Import as `mcp.client.sse.sse_within_origin`  ·  defined at `mcp.shared._httpx_utils.sse_within_origin`

```python
async def sse_within_origin(client: httpx2.AsyncClient, url: httpx2.URL | str, headers: dict[str, str] | None = None) -> AsyncGenerator[httpx2.EventSource]
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

`client.sse(url)` with the redirect handling of `stream_within_origin`.


## stream_within_origin

Import as `mcp.client.streamable_http.stream_within_origin`  ·  defined at `mcp.shared._httpx_utils.stream_within_origin`

```python
async def stream_within_origin(client: httpx2.AsyncClient, method: str, url: httpx2.URL | str, kwargs: Any = {}) -> AsyncGenerator[httpx2.Response]
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

`client.stream(...)`, following redirects only while they stay within the request's origin.

An MCP transport talks to one configured endpoint, and everything on a request
(headers, auth, body) was configured for that endpoint. A redirect that
`next_request_within_origin` accepts, such as a 307/308 trailing-slash
normalisation, is followed, at most `client.max_redirects` times. Any other
redirect (or one past that budget) is not followed: the redirect response
itself is yielded, the way httpx2 hands one back when `follow_redirects` is
off, and the caller treats it as the non-success it is. The client's own
`follow_redirects` setting is not consulted. Requests an `httpx2.Auth` flow
makes during the call are sent without following either; the SDK's OAuth
providers apply the same rule to their own requests.


