# `opentelemetry_sdk::trace::error`

Crate `opentelemetry_sdk` · 2 public items · structured records in [`model/opentelemetry_sdk.trace.error.json`](../model/opentelemetry_sdk.trace.error.json)

## TraceError

`enum` · `opentelemetry_sdk::trace::error::TraceError`

Also reachable as `opentelemetry_sdk::trace::TraceError`

```rust
enum TraceError
```

**Variants**: `ExportFailed`, `ExportTimedOut`, `TracerProviderAlreadyShutdown`, `Other`

**Implements**: `core::convert::From`, `core::error::Error`, `core::fmt::Display`

**Derives**: Debug

**via `core::convert::From`**

```rust
fn from(err: T) -> Self
fn from(err_msg: &'static str) -> Self
fn from(source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self
fn from(err: PoisonError<T>) -> Self
fn from(err_msg: String) -> Self
```

**via `core::error::Error`**

```rust
fn source(&self) -> ::core::option::Option<&dyn ::thiserror::__private::Error + 'static>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, __formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

Errors returned by the trace API.

---

## TraceResult

`type_alias` · `opentelemetry_sdk::trace::error::TraceResult`

Also reachable as `opentelemetry_sdk::trace::TraceResult`

```rust
type TraceResult<T> = Result<T, TraceError>
```

A specialized `Result` type for trace operations.

---
