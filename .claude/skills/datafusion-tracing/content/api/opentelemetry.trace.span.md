# `opentelemetry::trace::span`

Crate `opentelemetry` · 3 public items · structured records in [`model/opentelemetry.trace.span.json`](../model/opentelemetry.trace.span.json)

## SpanKind

`enum` · `opentelemetry::trace::span::SpanKind`

Also reachable as `opentelemetry::trace::SpanKind`

```rust
enum SpanKind
```

**Variants**: `Client`, `Server`, `Producer`, `Consumer`, `Internal`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

`SpanKind` describes the relationship between the [`Span`], its parents, and
its children in a trace.

`SpanKind` describes two independent properties that benefit tracing systems
during analysis:

The first property described by `SpanKind` reflects whether the span is a
"logical" remote child or parent. By "logical", we mean that the span is
logically a remote child or parent, from the point of view of the library
that is being instrumented. Spans with a remote parent are interesting
because they are sources of external load. Spans with a remote child are
interesting because they reflect a non-local system dependency.

The second property described by `SpanKind` reflects whether a child span
represents a synchronous call.  When a child span is synchronous, the parent
is expected to wait for it to complete under ordinary circumstances. It can
be useful for tracing systems to know this property, since synchronous spans
may contribute to the overall trace latency. Asynchronous scenarios can be
remote or local.

In order for `SpanKind` to be meaningful, callers should arrange that a
single span does not serve more than one purpose. For example, a server-side
span should not be used directly as the parent of another remote span. As a
simple guideline, instrumentation should create a new span prior to
extracting and serializing the SpanContext for a remote call.

Note: there are complex scenarios where a `SpanKind::Client` span may have a
child that is also logically a `SpanKind::Client` span, or a
`SpanKind::Producer` span might have a local child that is a
`SpanKind::Client` span, depending on how the various libraries that are
providing the functionality are built and instrumented. These scenarios,
when they occur, should be detailed in the semantic conventions appropriate
to the relevant libraries.

To summarize the interpretation of these kinds:

| `SpanKind` | Synchronous | Asynchronous | Remote Incoming | Remote Outgoing |
|---|---|---|---|---|
| `Client` | yes | | | yes |
| `Server` | yes | | yes | |
| `Producer` | | yes | | maybe |
| `Consumer` | | yes | maybe | |
| `Internal` | | | | |

---

## Status

`enum` · `opentelemetry::trace::span::Status`

Also reachable as `opentelemetry::trace::Status`

```rust
enum Status
```

**Variants**: `Unset`, `Error`, `Ok`

**Derives**: Clone, Debug, Default, Eq, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (1)

```rust
fn error(description: impl Into<Cow<'static, str>>) -> Self
```

The status of a [`Span`].

These values form a total order: Ok > Error > Unset. This means that setting
`Status::Ok` will override any prior or future attempts to set a status with
`Status::Error` or `Status::Unset`.

The status should remain unset, except for the following circumstances:

Generally, instrumentation libraries should not set the code to
`Status::Ok`, unless explicitly configured to do so. Instrumentation
libraries should leave the status code as unset unless there is an error.

Application developers and operators may set the status code to
`Status::Ok`.

When span status is set to `Status::Ok` it should be considered final and
any further attempts to change it should be ignored.

Analysis tools should respond to a `Status::Ok` status by suppressing any
errors they would otherwise generate. For example, to suppress noisy errors
such as 404s.

Only the value of the last call will be recorded, and implementations are
free to ignore previous calls.

---

## Span

`trait` · `opentelemetry::trace::span::Span`

Also reachable as `opentelemetry::trace::Span`

```rust
trait Span
```

**Implementors** (3)

- `opentelemetry::global::trace::BoxedSpan`
- `opentelemetry::trace::noop::NoopSpan`
- `opentelemetry_sdk::trace::span::Span`

**Methods** (12)

```rust
fn add_event<T>(&mut self, name: T, attributes: Vec<KeyValue>) where T: Into<Cow<'static, str>>
fn add_event_with_timestamp<T>(&mut self, name: T, timestamp: SystemTime, attributes: Vec<KeyValue>) where T: Into<Cow<'static, str>>
fn add_link(&mut self, span_context: SpanContext, attributes: Vec<KeyValue>)
fn end(&mut self)
fn end_with_timestamp(&mut self, timestamp: SystemTime)
fn is_recording(&self) -> bool
fn record_error(&mut self, err: &dyn Error)
fn set_attribute(&mut self, attribute: KeyValue)
fn set_attributes(&mut self, attributes: impl IntoIterator<Item = KeyValue>)
fn set_status(&mut self, status: Status)
fn span_context(&self) -> &SpanContext
fn update_name<T>(&mut self, new_name: T) where T: Into<Cow<'static, str>>
```

The interface for a single operation within a trace.

Spans can be nested to form a trace tree. Each trace contains a root span,
which typically describes the entire operation and, optionally, one or more
sub-spans for its sub-operations.

The span `name` concisely identifies the work represented by the span, for
example, an RPC method name, a function name, or the name of a subtask or
stage within a larger computation. The span name should be the most general
string that identifies a (statistically) interesting class of spans, rather
than individual span instances while still being human-readable. That is,
`"get_user"` is a reasonable name, while `"get_user/314159"`, where `"314159"` is
a user ID, is not a good name due to its high cardinality. _Generality_
should be prioritized over _human-readability_.

For example, here are potential span names for an endpoint that gets a
hypothetical account information:

| Span Name         | Guidance     |
| ----------------- | ------------ |
| `get`             | Too general  |
| `get_account/42`  | Too specific |
| `get_account`     | Good, and account_id=42 would make a nice Span attribute |
| `get_account/{accountId}` | Also good (using the "HTTP route") |

The span's start and end timestamps reflect the elapsed real time of the
operation.

For example, if a span represents a request-response cycle (e.g. HTTP or an
RPC), the span should have a start time that corresponds to the start time
of the first sub-operation, and an end time of when the final sub-operation
is complete. This includes:

* receiving the data from the request
* parsing of the data (e.g. from a binary or json format)
* any middleware or additional processing logic
* business logic
* construction of the response
* sending of the response

Child spans (or in some cases events) may be created to represent
sub-operations which require more detailed observability. Child spans should
measure the timing of the respective sub-operation, and may add additional
attributes.

---
