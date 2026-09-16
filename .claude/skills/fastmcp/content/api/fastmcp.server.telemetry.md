# `fastmcp.server.telemetry`

Distribution: `fastmcp`

## SEAM_SPAN_MARKER

`fastmcp.server.telemetry.SEAM_SPAN_MARKER`

```python
SEAM_SPAN_MARKER = 'fastmcp.span.seam'
```

**Inferred type** (`ty`, not declared in the source): `Literal["fastmcp.span.seam"]`

## __all__

`fastmcp.server.telemetry.__all__`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
__all__ = ['SEAM_SPAN_MARKER', 'delegate_span', 'get_auth_span_attributes', 'get_protocol_span_attributes', 'get_session_span_attributes', 'record_span_exception', 'seam_span', 'server_span']
```

## _active_seam_span

`fastmcp.server.telemetry._active_seam_span`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_active_seam_span: ContextVar[Span | None] = ContextVar('fastmcp_active_seam_span', default=None)
```

## _build_server_span_attrs

`fastmcp.server.telemetry._build_server_span_attrs`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _build_server_span_attrs(method: str, server_name: str, component_type: str, component_key: str, resource_uri: str | None, tool_name: str | None, prompt_name: str | None) -> dict[str, str]
```

## _get_parent_trace_context

`fastmcp.server.telemetry._get_parent_trace_context`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _get_parent_trace_context() -> Context | None
```

Get parent trace context from request meta for distributed tracing.


## _propagation_only_span

`fastmcp.server.telemetry._propagation_only_span`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _propagation_only_span() -> Generator[Span, None, None]
```

Attach the incoming `_meta` trace context without creating a span.

This is what separates `propagation_only` from `off`. Both create no
FastMCP spans, but `off` is fully transparent while `propagation_only`
still has to *parent* whatever the request goes on to do: without the
attach here, the trace context carried in `_meta` would be extracted and
then thrown away, and a span created inside a tool handler — by the user or
by the outer instrumentation layer that owns the MCP hierarchy — would
start a brand new trace instead of continuing the caller's.

Yields `INVALID_SPAN`, which is non-recording, so callers' `is_recording()`
guards skip attribute and error bookkeeping on it.


## delegate_span

Import as `fastmcp.server.providers.fastmcp_provider.delegate_span`  ·  defined at `fastmcp.server.telemetry.delegate_span`

```python
def delegate_span(name: str, provider_type: str, component_key: str, method: str | None = None) -> Generator[Span, None, None]
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Create an INTERNAL span for provider delegation.

Used by FastMCPProvider when delegating to mounted servers.
Automatically records any exception on the span and sets error status.


## get_auth_span_attributes

`fastmcp.server.telemetry.get_auth_span_attributes`

```python
def get_auth_span_attributes() -> dict[str, str]
```

Get auth attributes for the current request, if authenticated.


## get_protocol_span_attributes

`fastmcp.server.telemetry.get_protocol_span_attributes`

```python
def get_protocol_span_attributes() -> dict[str, str]
```

Get the negotiated MCP protocol version for the current request.

Mirrors the `mcp.protocol.version` attribute the SDK's own
`OpenTelemetryMiddleware` sets — FastMCP drops that middleware to avoid a
duplicate SERVER span, so this restores the attribute on FastMCP's span.


## get_session_span_attributes

`fastmcp.server.telemetry.get_session_span_attributes`

```python
def get_session_span_attributes() -> dict[str, str]
```

Get session attributes for the current request.


## record_span_exception

`fastmcp.server.telemetry.record_span_exception`

```python
def record_span_exception(span: Span, e: Exception) -> None
```

Record an exception and error status on a span.


## seam_span

Import as `fastmcp.server.low_level.seam_span`  ·  defined at `fastmcp.server.telemetry.seam_span`

```python
def seam_span(method: str, server_name: str) -> Generator[Span, None, None]
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Open the per-request SERVER span at the FastMCP middleware seam.

The span is named after the method and carries the base MCP attributes
(`mcp.method.name`, `fastmcp.server.name`, auth/session context) so
seam-only methods (`logging/setLevel`, `tasks/*`, `ping`, `initialize`, ...)
are fully attributed even though they never reach the high-level path. It is
marked with `SEAM_SPAN_MARKER` so a later `server_span` call in the
high-level path enriches this span with component attributes instead of
opening a second one. Exceptions raised anywhere below the seam — including
rejections *before* the high-level path (auth, not-found, middleware vetoes)
that would otherwise produce no SERVER span at all — are recorded here.

In `propagation_only` mode no span is opened at all — this is the one place
that has to know the difference, because the seam is where the incoming
`_meta` parent context is applied for the whole request.


## server_span

Import as `fastmcp.server.server.server_span`  ·  defined at `fastmcp.server.telemetry.server_span`

```python
def server_span(name: str, method: str, server_name: str, component_type: str, component_key: str, resource_uri: str | None = None, tool_name: str | None = None, prompt_name: str | None = None) -> Generator[Span, None, None]
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Emit or enrich a SERVER span with standard MCP attributes and auth context.

When the current active span is the request's seam span (opened by
`FastMCPServerMiddleware` and marked with `SEAM_SPAN_MARKER`), this sets the
component attributes on that span and yields it *without* starting a second
span — so failures rejected before this point and the successful high-level
call share one richly-attributed SERVER span. Otherwise (non-seam contexts,
e.g. in-process `mcp.call_tool()` calls that bypass the dispatcher) it opens a
new SERVER span as before.

Automatically records any exception on the span and sets error status.

In `propagation_only` mode no span is opened or enriched. The seam has
normally already attached the incoming parent context for this request;
doing it again here is a no-op, and covers the in-process callers that
bypass the dispatcher and so never reach the seam at all.


