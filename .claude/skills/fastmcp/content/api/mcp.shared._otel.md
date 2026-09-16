# `mcp.shared._otel`

Distribution: `mcp`

## _tracer

`mcp.shared._otel._tracer`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_tracer = get_tracer('mcp-python-sdk')
```

## extract_trace_context

`mcp.shared._otel.extract_trace_context`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def extract_trace_context(meta: Mapping[str, Any] | None) -> Context | None
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Extract W3C trace context from a `_meta` dict.

Returns `None` when the carrier is absent, malformed, or carries no
valid `traceparent`, so callers fall through to ambient parenting; an
explicit empty `Context` would orphan the span instead of nesting under
the current one.


## inject_trace_context

Import as `mcp.shared.jsonrpc_dispatcher.inject_trace_context`  ·  defined at `mcp.shared._otel.inject_trace_context`

```python
def inject_trace_context(meta: dict[str, Any]) -> None
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Inject W3C trace context (traceparent/tracestate) into a `_meta` dict.


## otel_span

Import as `mcp.shared.jsonrpc_dispatcher.otel_span`  ·  defined at `mcp.shared._otel.otel_span`

```python
def otel_span(name: str, kind: SpanKind, attributes: dict[str, Any] | None = None, context: Context | None = None, record_exception: bool = True, set_status_on_exception: bool = True) -> Generator[Span]
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Create an OTel span.


