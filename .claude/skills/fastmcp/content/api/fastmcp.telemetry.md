# `fastmcp.telemetry`

Distribution: `fastmcp`

## INSTRUMENTATION_NAME

`fastmcp.telemetry.INSTRUMENTATION_NAME`

```python
INSTRUMENTATION_NAME = 'fastmcp'
```

**Inferred type** (`ty`, not declared in the source): `Literal["fastmcp"]`

## TRACE_PARENT_KEY

`fastmcp.telemetry.TRACE_PARENT_KEY`

```python
TRACE_PARENT_KEY = 'traceparent'
```

**Inferred type** (`ty`, not declared in the source): `Literal["traceparent"]`

## TRACE_STATE_KEY

`fastmcp.telemetry.TRACE_STATE_KEY`

```python
TRACE_STATE_KEY = 'tracestate'
```

**Inferred type** (`ty`, not declared in the source): `Literal["tracestate"]`

## _DISABLED_TRACER

`fastmcp.telemetry._DISABLED_TRACER`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_DISABLED_TRACER = _DisabledTracer()
```

## _SUPPRESS_KEY

`fastmcp.telemetry._SUPPRESS_KEY`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_SUPPRESS_KEY = otel_context.create_key('fastmcp_suppress_telemetry')
```

## __all__

`fastmcp.telemetry.__all__`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
__all__ = ['INSTRUMENTATION_NAME', 'TRACE_PARENT_KEY', 'TRACE_STATE_KEY', 'extract_trace_context', 'get_tracer', 'inject_trace_context', 'native_spans_enabled', 'record_span_error', 'restore_dropped_attributes', 'suppress_fastmcp_telemetry', 'telemetry_mode']
```

## _AttributeReadableSpan

`fastmcp.telemetry._AttributeReadableSpan`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class _AttributeReadableSpan(Protocol)
```

**Bases** `Protocol`

**Declared members (2)**

- `attributes: Mapping[str, otel_types.AttributeValue] | None`  _property_
- `dropped_attributes: int`  _property_

Structural type for spans that expose their current attribute state.

The `opentelemetry-api` `Span` ABC has no way to read attributes back —
only SDK span implementations (e.g. `opentelemetry.sdk.trace.ReadableSpan`)
expose `.attributes` and `.dropped_attributes`. FastMCP only depends on
`opentelemetry-api`, so this module can't import the SDK class to
`isinstance`-check against it. A runtime-checkable `Protocol` gets the
same structural narrowing without that import: spans that don't expose
this state (e.g. `NonRecordingSpan`) simply fail the check.


## _DisabledTracer

`fastmcp.telemetry._DisabledTracer`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class _DisabledTracer(NoOpTracer)
```

**Bases** `NoOpTracer`

**Declared members (1)**

- `def start_as_current_span(self, name: str, context: Context | None = None, kind: SpanKind = SpanKind.INTERNAL, attributes: otel_types.Attributes = None, links: Any = None, start_time: int | None = None, record_exception: bool = True, set_status_on_exception: bool = True, end_on_exit: bool = True) -> Iterator[Span]`

A tracer that neither records spans nor touches the OTel context.

When telemetry is disabled FastMCP must be fully transparent. The stock
`NoOpTracer.start_as_current_span` still *attaches* a `NonRecordingSpan` to
the current OTel context, so an enclosing application span (from ASGI/HTTP
instrumentation or a user-created span) is hidden while a FastMCP span
helper is active — `trace.get_current_span()` inside a handler would then
return that non-recording span instead of the caller's span. This tracer
yields the invalid span *without* entering it as current, leaving the
surrounding trace context untouched.


## extract_trace_context

Import as `fastmcp.server.telemetry.extract_trace_context`  ·  defined at `fastmcp.telemetry.extract_trace_context`

```python
def extract_trace_context(meta: dict[str, Any] | None) -> Context
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Extract trace context from an MCP request meta dict.

If already in a valid trace (e.g., from HTTP propagation), the existing
trace context is preserved and meta is not used.

Args:
    meta: The meta dict from an MCP request (ctx.request_context.meta)

Returns:
    An OpenTelemetry Context with the extracted trace context,
    or the current context if no trace context found or already in a trace


## get_tracer

Import as `fastmcp.client.telemetry.get_tracer`  ·  defined at `fastmcp.telemetry.get_tracer`

```python
def get_tracer(version: str | None = None) -> Tracer
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Get the FastMCP tracer for creating spans.

Instrumentation is on by default. FastMCP uses only the OpenTelemetry API,
so span creation is a no-op with negligible overhead unless an OpenTelemetry
SDK and exporter are configured. When `fastmcp.settings.telemetry_mode` is
`propagation_only` or `off` — or the caller is inside a
`suppress_fastmcp_telemetry()` block — this returns a pass-through tracer
that creates no spans and leaves the current OTel context untouched even
when an SDK is configured.

Args:
    version: Optional version string for the instrumentation

Returns:
    A tracer instance. Returns a non-attaching pass-through tracer when
    FastMCP's own spans are disabled; span creation is otherwise a no-op
    unless an SDK is configured.


## inject_trace_context

Import as `fastmcp.client.mixins.tools.inject_trace_context`  ·  defined at `fastmcp.telemetry.inject_trace_context`

```python
def inject_trace_context(meta: dict[str, Any] | None = None) -> dict[str, Any] | None
```

_4 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Inject current trace context into a meta dict for MCP request propagation.

Args:
    meta: Optional existing meta dict to merge with trace context

Returns:
    A new dict containing the original meta (if any) plus trace context keys,
    or None if no trace context to inject and meta was None


## native_spans_enabled

`fastmcp.telemetry.native_spans_enabled`

```python
def native_spans_enabled() -> bool
```

Whether FastMCP should create its own spans right now.


## record_span_error

`fastmcp.telemetry.record_span_error`

```python
def record_span_error(span: Span, exception: BaseException) -> None
```

Record an exception on a span and set error status.


## restore_dropped_attributes

Import as `fastmcp.client.telemetry.restore_dropped_attributes`  ·  defined at `fastmcp.telemetry.restore_dropped_attributes`

```python
def restore_dropped_attributes(span: Span, attrs: Mapping[str, otel_types.AttributeValue]) -> None
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Restore FastMCP attributes a non-forwarding sampler dropped entirely.

`Tracer.start_span` builds the span from `SamplingResult.attributes`, not
the `attributes=` kwarg it was given for creation — a custom `Sampler`
whose `SamplingResult.attributes` defaults to `None` silently discards
every attribute FastMCP passed at creation time. Call this immediately
after span creation to recover from that case.

The restore only fires when the span has *no* attributes at all AND the
SDK hasn't evicted anything (`dropped_attributes == 0`):

- A bare, non-forwarding sampler (the regression this exists to fix)
  leaves the span with an empty attribute mapping, so everything is
  restored.
- A sampler that supplied any attributes of its own — whether by
  forwarding ours untouched, redacting or replacing some of our values,
  or substituting its own attributes entirely (e.g. to strip component
  names or resource URIs for privacy or cardinality control) — leaves
  the span non-empty, so it is left alone entirely. This is what makes
  the gate precise: a sampler that deliberately supplies only its own
  attributes must not have them clobbered by a restore that assumes
  "no FastMCP keys" means "sampler forwarding failed."
- A sampler that forwards most of our attributes but deliberately drops
  one is still non-empty, so it's covered by the same "leave alone"
  branch — a dropped key here is indistinguishable from the SDK's
  bounded attribute map evicting it, and reinserting it would just push
  the map's bound and evict a *different* retained key, churning which
  attributes survive without changing how many are lost. No attempt is
  made to restore individual missing keys; the gate is all-or-nothing.
- A low `OTEL_SPAN_ATTRIBUTE_COUNT_LIMIT` that evicts every attribute a
  forwarding sampler passed through is indistinguishable, from the
  span's attribute state alone, from a bare non-forwarding sampler —
  both leave an empty mapping. `dropped_attributes == 0` is what tells
  them apart: eviction always increments it, so that case is correctly
  excluded from the restore and the SDK's bounded map is left as
  computed.

Callers are expected to guard this with `if span.is_recording():`; it
does no work worth skipping for non-recording spans, but the check is
kept at call sites so it reads alongside the sibling `is_recording()`
guards already in those functions.


## suppress_fastmcp_telemetry

`fastmcp.telemetry.suppress_fastmcp_telemetry`

```python
def suppress_fastmcp_telemetry() -> Iterator[None]
```

Suppress FastMCP's own spans without disabling trace propagation.

Scoped equivalent of `telemetry_mode="propagation_only"`, for callers that
embed FastMCP inside their own instrumented stack and want to own the MCP
span hierarchy for a specific block. Narrower than OpenTelemetry's global
instrumentation suppression: only FastMCP's spans are skipped, so nested
instrumentation (HTTP clients, databases) keeps emitting, and trace context
still flows through `_meta` so those spans are parented correctly.

Has no effect when `telemetry_mode` is already `off`.


## telemetry_mode

Import as `fastmcp.server.telemetry.telemetry_mode`  ·  defined at `fastmcp.telemetry.telemetry_mode`

```python
def telemetry_mode() -> TelemetryMode
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Resolve the effective telemetry mode for the current context.

This is `fastmcp.settings.telemetry_mode`, except that an active
`suppress_fastmcp_telemetry()` block downgrades `native` to
`propagation_only`. Suppression never upgrades or overrides `off`: `off`
means FastMCP touches nothing, and a narrower request to skip FastMCP's
spans cannot re-enable the context propagation `off` deliberately omits.


