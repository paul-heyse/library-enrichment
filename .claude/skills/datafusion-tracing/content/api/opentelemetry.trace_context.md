# `opentelemetry::trace_context`

Crate `opentelemetry` · 3 public items · structured records in [`model/opentelemetry.trace_context.json`](../model/opentelemetry.trace_context.json)

## SpanId

`struct` · `opentelemetry::trace_context::SpanId`

Also reachable as `opentelemetry::SpanId`, `opentelemetry::trace::SpanId`

```rust
struct SpanId
```

**Implements**: `core::convert::From`, `core::fmt::Display`, `core::fmt::LowerHex`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
const fn from_bytes(bytes: [u8; 8]) -> Self
fn from_hex(hex: &str) -> Result<Self, ParseIntError>
const fn to_bytes(self) -> [u8; 8]
```

**via `core::convert::From`**

```rust
fn from(value: u64) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `core::fmt::LowerHex`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

An 8-byte value which identifies a given span.

The id is valid if it contains at least one non-zero byte.

---

## TraceFlags

`struct` · `opentelemetry::trace_context::TraceFlags`

Also reachable as `opentelemetry::TraceFlags`, `opentelemetry::trace::TraceFlags`

```rust
struct TraceFlags
```

**Implements**: `core::fmt::LowerHex`, `core::ops::bit::BitAnd`, `core::ops::bit::BitOr`, `core::ops::bit::Not`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (4)

```rust
fn is_sampled(&self) -> bool
const fn new(flags: u8) -> Self
fn to_u8(self) -> u8
fn with_sampled(&self, sampled: bool) -> Self
```

**via `core::fmt::LowerHex`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `core::ops::bit::BitAnd`**

```rust
fn bitand(self, rhs: Self) -> Self::Output
```

**via `core::ops::bit::BitOr`**

```rust
fn bitor(self, rhs: Self) -> Self::Output
```

**via `core::ops::bit::Not`**

```rust
fn not(self) -> Self::Output
```

Flags that can be set on a `SpanContext`.

The current version of the specification only supports a single flag
[`TraceFlags::SAMPLED`].

See the W3C TraceContext specification's [trace-flags] section for more
details.

[trace-flags]: https://www.w3.org/TR/trace-context/#trace-flags

---

## TraceId

`struct` · `opentelemetry::trace_context::TraceId`

Also reachable as `opentelemetry::TraceId`, `opentelemetry::trace::TraceId`

```rust
struct TraceId
```

**Implements**: `core::convert::From`, `core::fmt::Display`, `core::fmt::LowerHex`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
const fn from_bytes(bytes: [u8; 16]) -> Self
fn from_hex(hex: &str) -> Result<Self, ParseIntError>
const fn to_bytes(self) -> [u8; 16]
```

**via `core::convert::From`**

```rust
fn from(value: u128) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `core::fmt::LowerHex`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

A 16-byte value which identifies a given trace.

The id is valid if it contains at least one non-zero byte.

---
