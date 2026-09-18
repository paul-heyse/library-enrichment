# `datafusion_common::types::native`

Crate `datafusion-common` · 1 public items · structured records in [`model/datafusion_common.types.native.json`](../model/datafusion_common.types.native.json)

## NativeType

`enum` · `datafusion_common::types::native::NativeType`

```rust
enum NativeType
```

**Variants**: `Null`, `Boolean`, `Int8`, `Int16`, `Int32`, `Int64`, `UInt8`, `UInt16`, `UInt32`, `UInt64`, `Float16`, `Float32`, `Float64`, `Timestamp`, `Date`, `Time`, `Duration`, `Interval`, `Binary`, `FixedSizeBinary`, `String`, `List`, `FixedSizeList`, `Struct`, `Union`, `Decimal`, `Map`

**Implements**: `core::convert::From`, `core::fmt::Display`, `datafusion_common::types::logical::LogicalType`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (11)

```rust
fn is_binary(&self) -> bool
fn is_date(&self) -> bool
fn is_decimal(&self) -> bool
fn is_duration(&self) -> bool
fn is_float(&self) -> bool
fn is_integer(&self) -> bool
fn is_interval(&self) -> bool
fn is_null(&self) -> bool
fn is_numeric(&self) -> bool
fn is_time(&self) -> bool
fn is_timestamp(&self) -> bool
```

**via `core::convert::From`**

```rust
fn from(value: &DataType) -> Self
fn from(value: DataType) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `datafusion_common::types::logical::LogicalType`**

```rust
fn default_cast_for(&self, origin: &DataType) -> Result<DataType>
fn native(&self) -> &NativeType
fn signature(&self) -> TypeSignature<'_>
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.types.native.NativeType.md).


Representation of a type that DataFusion can handle natively. It is a subset
of the physical variants in Arrow's native [`DataType`].

---
