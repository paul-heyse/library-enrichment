# `mcp.client.sse`

Distribution: `mcp`

## logger

`mcp.client.sse.logger`

```python
logger = logging.getLogger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## _extract_session_id_from_endpoint

`mcp.client.sse._extract_session_id_from_endpoint`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _extract_session_id_from_endpoint(endpoint_url: str) -> str | None
```

## remove_request_params

`mcp.client.sse.remove_request_params`

```python
def remove_request_params(url: str) -> str
```

## sse_client

Import as `mcp.client.session_group.sse_client`  ·  defined at `mcp.client.sse.sse_client`

```python
async def sse_client(url: str, headers: dict[str, Any] | None = None, timeout: float = 5.0, sse_read_timeout: float = 300.0, httpx_client_factory: McpHttpClientFactory = create_mcp_http_client, auth: httpx2.Auth | None = None, on_session_created: Callable[[str], None] | None = None)
```

_3 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Client transport for SSE.

`sse_read_timeout` determines how long (in seconds) the client will wait for a new
event before disconnecting. All other HTTP operations are controlled by `timeout`.

Args:
    url: The SSE endpoint URL.
    headers: Optional headers to include in requests.
    timeout: HTTP timeout for regular operations (in seconds).
    sse_read_timeout: Timeout for SSE read operations (in seconds).
    httpx_client_factory: Factory function for creating the httpx2 client. Whichever client it
        returns, MCP requests follow a redirect only when it stays on the endpoint's origin
        (same scheme, host and port, or http to https on the same host with default ports) and
        keeps the request method (any status for the SSE GET, 307/308 for a message POST); any
        other redirect is not followed, so connecting fails with
        `httpx2.HTTPStatusError` for the redirect response. The client's `follow_redirects`
        setting is not consulted; the SDK's OAuth providers apply the same rule to the requests
        they make.
    auth: Optional httpx2 authentication handler.
    on_session_created: Optional callback invoked with the session ID when received.


