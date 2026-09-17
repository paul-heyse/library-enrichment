# `opentelemetry_sdk::metrics::meter_provider`

Crate `opentelemetry_sdk` · 2 public items · structured records in [`model/opentelemetry_sdk.metrics.meter_provider.json`](../model/opentelemetry_sdk.metrics.meter_provider.json)

## MeterProviderBuilder

`struct` · `opentelemetry_sdk::metrics::meter_provider::MeterProviderBuilder`

Also reachable as `opentelemetry_sdk::metrics::MeterProviderBuilder`

```rust
struct MeterProviderBuilder
```

**Derives**: Debug, Default

**Methods** (5)

```rust
fn build(self) -> SdkMeterProvider
fn with_periodic_exporter<T>(self, exporter: T) -> Self where T: PushMetricExporter
fn with_reader<T: MetricReader>(self, reader: T) -> Self
fn with_resource(self, resource: Resource) -> Self
fn with_view<T>(self, view: T) -> Self where T: Fn(&Instrument) -> Option<Stream> + Send + Sync + 'static
```

Configuration options for a [MeterProvider].

---

## SdkMeterProvider

`struct` · `opentelemetry_sdk::metrics::meter_provider::SdkMeterProvider`

Also reachable as `opentelemetry_sdk::metrics::SdkMeterProvider`

```rust
struct SdkMeterProvider
```

**Implements**: `opentelemetry::metrics::meter::MeterProvider`

**Derives**: Clone, Debug, Default

**Methods** (4)

```rust
fn builder() -> MeterProviderBuilder
fn force_flush(&self) -> OTelSdkResult
fn shutdown(&self) -> OTelSdkResult
fn shutdown_with_timeout(&self, _timeout: Duration) -> OTelSdkResult
```

**via `opentelemetry::metrics::meter::MeterProvider`**

```rust
fn meter(&self, name: &'static str) -> Meter
fn meter_with_scope(&self, scope: InstrumentationScope) -> Meter
```

Handles the creation and coordination of [Meter]s.

All `Meter`s created by a `MeterProvider` will be associated with the same
[Resource], have the same views applied to them, and have their produced
metric telemetry passed to the configured [MetricReader]s. This is a
clonable handle to the MeterProvider implementation itself, and cloning it
will create a new reference, not a new instance of a MeterProvider. Dropping
the last reference to it will trigger shutdown of the provider. Shutdown can
also be triggered manually by calling the `shutdown` method.
[Meter]: opentelemetry::metrics::Meter

---
