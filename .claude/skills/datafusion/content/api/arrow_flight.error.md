# `arrow_flight::error`

Crate `arrow-flight` · 2 public items · structured records in [`model/arrow_flight.error.json`](../model/arrow_flight.error.json)

## FlightError

`enum` · `arrow_flight::error::FlightError`

```rust
enum FlightError
```

**Variants**: `Arrow`, `NotYetImplemented`, `Tonic`, `ProtocolError`, `DecodeError`, `ExternalError`

**Implements**: `core::convert::From`, `core::error::Error`, `core::fmt::Display`

**Derives**: Debug

**Methods** (2)

```rust
fn from_external_error(error: Box<dyn Error + Send + Sync>) -> Self
fn protocol(message: impl Into<String>) -> Self
```

**via `core::convert::From`**

```rust
fn from(value: ArrowError) -> Self
fn from(status: tonic::Status) -> Self
fn from(error: prost::DecodeError) -> Self
```

**via `core::error::Error`**

```rust
fn source(&self) -> Option<&dyn Error + 'static>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Errors for the Apache Arrow Flight crate

---

## Result

`type_alias` · `arrow_flight::error::Result`

```rust
type Result<T> = std::result::Result<T, FlightError>
```

Result type for the Apache Arrow Flight crate

---
