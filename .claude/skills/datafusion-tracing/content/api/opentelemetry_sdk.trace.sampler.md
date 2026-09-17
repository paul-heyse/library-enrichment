# `opentelemetry_sdk::trace::sampler`

Crate `opentelemetry_sdk` · 3 public items · structured records in [`model/opentelemetry_sdk.trace.sampler.json`](../model/opentelemetry_sdk.trace.sampler.json)

## Sampler

`enum` · `opentelemetry_sdk::trace::sampler::Sampler`

Also reachable as `opentelemetry_sdk::trace::Sampler`

```rust
enum Sampler
```

**Variants**: `AlwaysOn`, `AlwaysOff`, `ParentBased`, `TraceIdRatioBased`, `JaegerRemote`

**Implements**: `opentelemetry_sdk::trace::sampler::ShouldSample`

**Derives**: Clone, Debug

**Methods** (1)

```rust
fn jaeger_remote<C, Sampler, R, Svc>(runtime: R, http_client: C, default_sampler: Sampler, service_name: Svc) -> JaegerRemoteSamplerBuilder<C, Sampler, R> where C: HttpClient + 'static, Sampler: ShouldSample, R: runtime::RuntimeChannel, Svc: Into<String>
```

**via `opentelemetry_sdk::trace::sampler::ShouldSample`**

```rust
fn should_sample(&self, parent_context: Option<&Context>, trace_id: TraceId, name: &str, span_kind: &SpanKind, attributes: &[KeyValue], links: &[Link]) -> SamplingResult
```

Default Sampling options

The [built-in samplers] allow for simple decisions. For more complex scenarios consider
implementing your own sampler using [`ShouldSample`] trait.

[built-in samplers]: https://github.com/open-telemetry/opentelemetry-specification/blob/main/specification/trace/sdk.md#built-in-samplers

---

## CloneShouldSample

`trait` · `opentelemetry_sdk::trace::sampler::CloneShouldSample`

```rust
trait CloneShouldSample
```

**Methods** (1)

```rust
fn box_clone(&self) -> Box<dyn ShouldSample>
```

This trait should not be used directly instead users should use [`ShouldSample`].

---

## ShouldSample

`trait` · `opentelemetry_sdk::trace::sampler::ShouldSample`

Also reachable as `opentelemetry_sdk::trace::ShouldSample`

```rust
trait ShouldSample: CloneShouldSample + Send + Sync + std::fmt::Debug
```

**Implementors** (2)

- `opentelemetry_sdk::trace::sampler::Sampler`
- `opentelemetry_sdk::trace::sampler::jaeger_remote::sampler::JaegerRemoteSampler`

**Methods** (1)

```rust
fn should_sample(&self, parent_context: Option<&Context>, trace_id: TraceId, name: &str, span_kind: &SpanKind, attributes: &[KeyValue], links: &[Link]) -> SamplingResult
```

The [`ShouldSample`] interface allows implementations to provide samplers
which will return a sampling [`SamplingResult`] based on information that
is typically available just before the [`Span`] was created.

# Sampling

Sampling is a mechanism to control the noise and overhead introduced by
OpenTelemetry by reducing the number of samples of traces collected and
sent to the backend.

Sampling may be implemented on different stages of a trace collection.
[OpenTelemetry SDK] defines a [`ShouldSample`] interface that can be used at
instrumentation points by libraries to check the sampling [`SamplingDecision`]
early and optimize the amount of telemetry that needs to be collected.

All other sampling algorithms may be implemented on SDK layer in exporters,
or even out of process in Agent or Collector.

The OpenTelemetry API has two properties responsible for the data collection:

* [`Span::is_recording()`]. If `true` the current [`Span`] records
  tracing events (attributes, events, status, etc.), otherwise all tracing
  events are dropped. Users can use this property to determine if expensive
  trace events can be avoided. [`SpanProcessor`]s will receive
  all spans with this flag set. However, [`SpanExporter`]s will
  not receive them unless the `Sampled` flag was set.
* `Sampled` flag in [`SpanContext::trace_flags()`]. This flag is propagated
  via the [`SpanContext`] to child Spans. For more details see the [W3C
  specification](https://w3c.github.io/trace-context/). This flag indicates
  that the [`Span`] has been `sampled` and will be exported. [`SpanProcessor`]s
  and [`SpanExporter`]s will receive spans with the `Sampled` flag set for
  processing.

The flag combination `Sampled == false` and `is_recording == true` means
that the current `Span` does record information, but most likely the child
`Span` will not.

The flag combination `Sampled == true` and `is_recording == false` could
cause gaps in the distributed trace, and because of this OpenTelemetry API
MUST NOT allow this combination.

[OpenTelemetry SDK]: https://github.com/open-telemetry/opentelemetry-specification/blob/main/specification/trace/sdk.md#sampling
[`SpanContext`]: opentelemetry::trace::SpanContext
[`SpanContext::trace_flags()`]: opentelemetry::trace::SpanContext#method.trace_flags
[`SpanExporter`]: crate::trace::SpanExporter
[`SpanProcessor`]: crate::trace::SpanProcessor
[`Span`]: opentelemetry::trace::Span
[`Span::is_recording()`]: opentelemetry::trace::Span#tymethod.is_recording

---
