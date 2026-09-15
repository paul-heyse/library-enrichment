# `arrow_flight`

Crate `arrow-flight` · 2 public items · structured records in [`model/arrow_flight.json`](../model/arrow_flight.json)

## IpcMessage

`struct` · `arrow_flight::IpcMessage`

```rust
struct IpcMessage
```

**Implements**: `core::convert::TryFrom`, `core::ops::deref::Deref`

**Derives**: Debug

**via `core::convert::TryFrom`**

```rust
fn try_from(schema_ipc: SchemaAsIpc<'_>) -> std::result::Result<Self, arrow_schema::ArrowError>
```

**via `core::ops::deref::Deref`**

```rust
fn deref(&self) -> &Self::Target
```

IpcMessage represents a `Schema` in the format expected in
`FlightInfo.schema`

---

## SchemaAsIpc

`struct` · `arrow_flight::SchemaAsIpc`

```rust
struct SchemaAsIpc<'a>
```

**Fields**: `pair`

**Implements**: `core::ops::deref::Deref`

**Methods** (1)

```rust
fn new(schema: &'a Schema, options: &'a IpcWriteOptions) -> Self
```

**via `core::ops::deref::Deref`**

```rust
fn deref(&self) -> &Self::Target
```

SchemaAsIpc represents a pairing of a `Schema` with IpcWriteOptions

---
