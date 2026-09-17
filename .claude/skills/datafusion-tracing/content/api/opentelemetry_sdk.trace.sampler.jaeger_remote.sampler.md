# `opentelemetry_sdk::trace::sampler::jaeger_remote::sampler`

Crate `opentelemetry_sdk` · 2 public items · structured records in [`model/opentelemetry_sdk.trace.sampler.jaeger_remote.sampler.json`](../model/opentelemetry_sdk.trace.sampler.jaeger_remote.sampler.json)

## JaegerRemoteSampler

`struct` · `opentelemetry_sdk::trace::sampler::jaeger_remote::sampler::JaegerRemoteSampler`

Also reachable as `opentelemetry_sdk::trace::JaegerRemoteSampler`

```rust
struct JaegerRemoteSampler
```

**Implements**: `opentelemetry_sdk::trace::sampler::ShouldSample`

**Derives**: Clone, Debug

**via `opentelemetry_sdk::trace::sampler::ShouldSample`**

```rust
fn should_sample(&self, parent_context: Option<&Context>, trace_id: TraceId, name: &str, span_kind: &SpanKind, attributes: &[KeyValue], links: &[Link]) -> SamplingResult
```

Sampler that fetches the sampling configuration from remotes.

It offers the following sampling strategies:
- **Probabilistic**, fetch a probability between [0.0, 1.0] from remotes and use it to sample traces. If the probability is 0.0, it will never sample traces. If the probability is 1.0, it will always sample traces.
- **Rate limiting**, ses a leaky bucket rate limiter to ensure that traces are sampled with a certain constant rate.
- **Per Operations**, instead of sampling all traces, it samples traces based on the span name. Only probabilistic sampling is supported at the moment.

User can build a [`JaegerRemoteSampler`] by getting a [`JaegerRemoteSamplerBuilder`] from [`Sampler::jaeger_remote`].

Note that the backend doesn't need to be Jaeger so long as it supports jaeger remote sampling
protocol.

---

## JaegerRemoteSamplerBuilder

`struct` · `opentelemetry_sdk::trace::sampler::jaeger_remote::sampler::JaegerRemoteSamplerBuilder`

Also reachable as `opentelemetry_sdk::trace::JaegerRemoteSamplerBuilder`

```rust
struct JaegerRemoteSamplerBuilder<C, S, R> where R: RuntimeChannel, C: HttpClient + 'static, S: ShouldSample + 'static
```

**Derives**: Debug

**Methods** (4)

```rust
fn build(self) -> Result<Sampler, TraceError>
fn with_endpoint<Str: Into<String>>(self, endpoint: Str) -> Self
fn with_leaky_bucket_size(self, size: f64) -> Self
fn with_update_interval(self, interval: Duration) -> Self
```

Builder for [`JaegerRemoteSampler`].
See [Sampler::jaeger_remote] for details.

---
