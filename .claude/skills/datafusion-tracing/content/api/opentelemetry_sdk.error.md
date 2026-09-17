# `opentelemetry_sdk::error`

Crate `opentelemetry_sdk` · 3 public items · structured records in [`model/opentelemetry_sdk.error.json`](../model/opentelemetry_sdk.error.json)

## OTelSdkError

`enum` · `opentelemetry_sdk::error::OTelSdkError`

```rust
enum OTelSdkError
```

**Variants**: `AlreadyShutdown`, `Timeout`, `InternalFailure`

**Implements**: `core::error::Error`, `core::fmt::Display`

**Derives**: Debug

**via `core::fmt::Display`**

```rust
fn fmt(&self, __formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

Errors that can occur during SDK operations export(), force_flush() and shutdown().

---

## ExportError

`trait` · `opentelemetry_sdk::error::ExportError`

Also reachable as `opentelemetry_sdk::ExportError`

```rust
trait ExportError: std::error::Error + Send + Sync + 'static
```

**Methods** (1)

```rust
fn exporter_name(&self) -> &'static str
```

Trait for errors returned by exporters

---

## OTelSdkResult

`type_alias` · `opentelemetry_sdk::error::OTelSdkResult`

```rust
type OTelSdkResult = std::result::Result<(), OTelSdkError>
```

A specialized `Result` type for Shutdown operations.

---
