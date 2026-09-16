# `fastmcp.client.telemetry`

Distribution: `fastmcp`

## __all__

`fastmcp.client.telemetry.__all__`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
__all__ = ['client_span']
```

## client_span

Import as `fastmcp.client.mixins.tools.client_span`  ·  defined at `fastmcp.client.telemetry.client_span`

```python
def client_span(name: str, method: str, component_key: str, session_id: str | None = None, resource_uri: str | None = None, tool_name: str | None = None, prompt_name: str | None = None) -> Generator[Span, None, None]
```

_4 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Create a CLIENT span with standard MCP attributes.

Automatically records any exception on the span and sets error status.


