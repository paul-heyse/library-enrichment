# `opentelemetry_sdk::metrics::manual_reader`

Crate `opentelemetry_sdk` · 2 public items · structured records in [`model/opentelemetry_sdk.metrics.manual_reader.json`](../model/opentelemetry_sdk.metrics.manual_reader.json)

## ManualReader

`struct` · `opentelemetry_sdk::metrics::manual_reader::ManualReader`

Also reachable as `opentelemetry_sdk::metrics::ManualReader`

```rust
struct ManualReader
```

**Implements**: `opentelemetry_sdk::metrics::reader::MetricReader`

**Derives**: Debug, Default

**Methods** (1)

```rust
fn builder() -> ManualReaderBuilder
```

**via `opentelemetry_sdk::metrics::reader::MetricReader`**

```rust
fn collect(&self, rm: &mut ResourceMetrics) -> OTelSdkResult
fn force_flush(&self) -> OTelSdkResult
fn register_pipeline(&self, pipeline: Weak<Pipeline>)
fn shutdown_with_timeout(&self, _timeout: Duration) -> OTelSdkResult
fn temporality(&self, kind: super::InstrumentKind) -> Temporality
```

A simple [MetricReader] that allows an application to read metrics on demand.

See [ManualReaderBuilder] for configuration options.

# Example

```
use opentelemetry_sdk::metrics::ManualReader;

// can specify additional reader configuration
let reader = ManualReader::builder().build();
# drop(reader)
```

---

## ManualReaderBuilder

`struct` · `opentelemetry_sdk::metrics::manual_reader::ManualReaderBuilder`

Also reachable as `opentelemetry_sdk::metrics::ManualReaderBuilder`

```rust
struct ManualReaderBuilder
```

**Derives**: Debug, Default

**Methods** (3)

```rust
fn build(self) -> ManualReader
fn new() -> Self
fn with_temporality(self, temporality: Temporality) -> Self
```

Configuration for a [ManualReader]

---
