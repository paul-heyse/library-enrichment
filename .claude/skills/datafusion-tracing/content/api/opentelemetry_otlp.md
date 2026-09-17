# `opentelemetry_otlp`

Crate `opentelemetry-otlp` · 4 public items · structured records in [`model/opentelemetry_otlp.json`](../model/opentelemetry_otlp.json)

## Protocol

`enum` · `opentelemetry_otlp::Protocol`

```rust
enum Protocol
```

**Variants**: `Grpc`, `HttpBinary`, `HttpJson`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private226::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private226::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

The communication protocol to use when exporting data.

---

## HttpExporterBuilderSet

`struct` · `opentelemetry_otlp::HttpExporterBuilderSet`

```rust
struct HttpExporterBuilderSet
```

**Derives**: Debug, Default

Type to hold the [HttpExporterBuilder] and indicate it has been set.

Allowing access to [HttpExporterBuilder] specific configuration methods.

---

## NoExporterBuilderSet

`struct` · `opentelemetry_otlp::NoExporterBuilderSet`

```rust
struct NoExporterBuilderSet
```

**Derives**: Clone, Debug, Default

Type to indicate the builder does not have a client set.

---

## TonicExporterBuilderSet

`struct` · `opentelemetry_otlp::TonicExporterBuilderSet`

```rust
struct TonicExporterBuilderSet
```

**Derives**: Debug, Default

Type to hold the [TonicExporterBuilder] and indicate it has been set.

Allowing access to [TonicExporterBuilder] specific configuration methods.

---
