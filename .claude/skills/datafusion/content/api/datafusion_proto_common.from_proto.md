# `datafusion_proto_common::from_proto`

Crate `datafusion-proto-common` · 5 public items · structured records in [`model/datafusion_proto_common.from_proto.json`](../model/datafusion_proto_common.from_proto.json)

## Error

`enum` · `datafusion_proto_common::from_proto::Error`

Also reachable as `datafusion_proto::protobuf::FromProtoError`, `datafusion_proto_common::FromProtoError`

```rust
enum Error
```

**Variants**: `General`, `DataFusionError`, `MissingRequiredField`, `AtLeastOneValue`, `UnknownEnumVariant`

**Implements**: `core::convert::From`, `core::error::Error`, `core::fmt::Display`

**Derives**: Debug

**Methods** (2)

```rust
fn required(field: impl Into<String>) -> Error
fn unknown(name: impl Into<String>, value: i32) -> Error
```

**via `core::convert::From`**

```rust
fn from(e: DataFusionError) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_common.from_proto.Error.md).


---

## parse_i32_to_interval_unit

`function` · `datafusion_proto_common::from_proto::parse_i32_to_interval_unit`

```rust
fn parse_i32_to_interval_unit(value: &i32) -> datafusion_common::Result<arrow::datatypes::IntervalUnit, Error>
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_common.from_proto.parse_i32_to_interval_unit.md).


---

## parse_i32_to_time_unit

`function` · `datafusion_proto_common::from_proto::parse_i32_to_time_unit`

```rust
fn parse_i32_to_time_unit(value: &i32) -> datafusion_common::Result<arrow::datatypes::TimeUnit, Error>
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_common.from_proto.parse_i32_to_time_unit.md).


---

## parse_proto_fields_to_fields

`function` · `datafusion_proto_common::from_proto::parse_proto_fields_to_fields`

```rust
fn parse_proto_fields_to_fields<'a, I>(fields: I) -> Result<Vec<arrow::datatypes::Field>, Error> where I: IntoIterator<Item = &'a protobuf::Field>
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_common.from_proto.parse_proto_fields_to_fields.md).


Converts a vector of `protobuf::Field`s to `Arc<arrow::Field>`s.

---

## FromOptionalField

`trait` · `datafusion_proto_common::from_proto::FromOptionalField`

```rust
trait FromOptionalField<T>
```

**Implementors** (1)

- `core::option::Option`

**Methods** (2)

```rust
fn optional(self) -> datafusion_common::Result<Option<T>, Error>
fn required(self, field: impl Into<String>) -> datafusion_common::Result<T, Error>
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_common.from_proto.FromOptionalField.md).


An extension trait that adds the methods `optional` and `required` to any
Option containing a type implementing `TryInto<U, Error = Error>`

---
