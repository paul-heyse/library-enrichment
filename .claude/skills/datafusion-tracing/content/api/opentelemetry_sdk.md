# `opentelemetry_sdk`

Crate `opentelemetry_sdk` · 1 public items · structured records in [`model/opentelemetry_sdk.json`](../model/opentelemetry_sdk.json)

## InMemoryExporterError

`enum` · `opentelemetry_sdk::InMemoryExporterError`

```rust
enum InMemoryExporterError
```

**Variants**: `InternalFailure`

**Implements**: `core::convert::From`, `core::error::Error`, `core::fmt::Display`

**Derives**: Debug

**via `core::convert::From`**

```rust
fn from(err: std::sync::PoisonError<T>) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, __formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

Errors that can occur during when returning telemetry from InMemoryLogExporter

---
