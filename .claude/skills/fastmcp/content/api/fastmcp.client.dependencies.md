# `fastmcp.client.dependencies`

Distribution: `fastmcp`

## _get_forwardable_http_headers

`fastmcp.client.dependencies._get_forwardable_http_headers`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _get_forwardable_http_headers() -> dict[str, str]
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Return ambient headers safe to copy onto a new MCP connection.

MCP transport and routing headers describe one HTTP hop and must be
regenerated for the new connection. `Last-Event-ID` likewise belongs to
the inbound connection's event stream. Other headers, including
authorization and custom proxy headers, are preserved.


## get_http_headers

`fastmcp.client.dependencies.get_http_headers`

```python
def get_http_headers(include_all: bool = False, include: set[str] | None = None) -> dict[str, str]
```

Return HTTP headers from an ambient server request, when available.

The standalone client package has no server request context. When the full
FastMCP package is installed, delegate to its request-aware implementation.


