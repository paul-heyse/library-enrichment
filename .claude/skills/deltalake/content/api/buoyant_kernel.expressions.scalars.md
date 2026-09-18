# `buoyant_kernel::expressions::scalars`

Crate `buoyant_kernel` · 5 public items · structured records in [`model/buoyant_kernel.expressions.scalars.json`](../model/buoyant_kernel.expressions.scalars.json)

## Scalar

`enum` · `buoyant_kernel::expressions::scalars::Scalar`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.expressions.scalars.Scalar.md)

Also reachable as `buoyant_kernel::expressions::Scalar`, `delta_kernel::expressions::scalars::Scalar`

```rust
enum Scalar
```

**Variants**: `Integer`, `Long`, `Short`, `Byte`, `Float`, `Double`, `String`, `Boolean`, `Timestamp`, `TimestampNtz`, `TimestampNanos`, `TimestampNanosNtz`, `Date`, `Binary`, `Decimal`, `Null`, `Struct`, `Array`, `Map`

**Implements**: `core::convert::From`, `core::convert::TryFrom`, `core::fmt::Display`, `deltalake_core::kernel::scalars::ScalarExt`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**Methods** (12)

```rust
fn data_type(&self) -> DataType
fn decimal(bits: impl Into<i128>, precision: u8, scale: u8) -> DeltaResult<Self>
fn is_null(&self) -> bool
fn logical_eq(&self, other: &Self) -> bool
fn logical_partial_cmp(&self, other: &Self) -> Option<Ordering>
fn null(data_type: impl Into<DataType>) -> Self
fn physical_eq(&self, other: &Self) -> bool
fn to_array(&self, num_rows: usize) -> DeltaResult<ArrayRef>
fn try_add(&self, other: &Scalar) -> Option<Scalar>
fn try_div(&self, other: &Scalar) -> Option<Scalar>
fn try_mul(&self, other: &Scalar) -> Option<Scalar>
fn try_sub(&self, other: &Scalar) -> Option<Scalar>
```

**via `core::convert::From`**

```rust
fn from(i: i16) -> Self
fn from(i: f32) -> Self
fn from(feature: TableFeature) -> Self
fn from(s: &str) -> Self
fn from(b: &[u8]) -> Self
fn from(i: i8) -> Self
fn from(i: i64) -> Self
fn from(b: bool) -> Self
fn from(array_data: ArrayData) -> Self
fn from(t: &T) -> Self
fn from(map_data: MapData) -> Self
fn from(d: DecimalData) -> Self
fn from(i: i32) -> Self
fn from(t: Option<T>) -> Self
fn from(i: f64) -> Self
fn from(value: String) -> Self
fn from(b: bytes::Bytes) -> Self
```

**via `core::convert::TryFrom`**

```rust
fn try_from(vec: Vec<T>) -> Result<Self, Self::Error>
fn try_from(vec: Vec<Option<T>>) -> Result<Self, Self::Error>
fn try_from(opt: Option<Vec<T>>) -> Result<Self, Self::Error>
fn try_from(map: HashMap<K, V>) -> Result<Self, Self::Error>
fn try_from(map: HashMap<K, Option<V>>) -> Result<Self, Self::Error>
fn try_from(opt: Option<HashMap<K, V>>) -> Result<Self, Self::Error>
fn try_from(format: Format) -> DeltaResult<Self>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

A single value, which can be null. Used for representing literal values
in [Expressions][crate::expressions::Expression].

NOTE: `PartialEq` uses physical (structural) comparison semantics.
For SQL NULL semantics, use [`Scalar::logical_eq`] or [`Scalar::logical_partial_cmp`].

---

## ArrayData

`struct` · `buoyant_kernel::expressions::scalars::ArrayData`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.expressions.scalars.ArrayData.md)

Also reachable as `buoyant_kernel::expressions::ArrayData`, `delta_kernel::expressions::scalars::ArrayData`

```rust
struct ArrayData
```

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**Methods** (4)

```rust
fn array_elements(&self) -> &[Scalar]
fn array_type(&self) -> &ArrayType
fn to_arrow(&self) -> DeltaResult<ArrayRef>
fn try_new(tpe: ArrayType, elements: impl IntoIterator<Item = impl Into<Scalar>>) -> DeltaResult<Self>
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

---

## DecimalData

`struct` · `buoyant_kernel::expressions::scalars::DecimalData`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.expressions.scalars.DecimalData.md)

Also reachable as `buoyant_kernel::expressions::DecimalData`, `delta_kernel::expressions::scalars::DecimalData`

```rust
struct DecimalData
```

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**Methods** (5)

```rust
fn bits(&self) -> i128
fn precision(&self) -> u8
fn scale(&self) -> u8
fn try_new(bits: impl Into<i128>, ty: DecimalType) -> DeltaResult<Self>
fn ty(&self) -> &DecimalType
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

---

## MapData

`struct` · `buoyant_kernel::expressions::scalars::MapData`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.expressions.scalars.MapData.md)

Also reachable as `buoyant_kernel::expressions::MapData`, `delta_kernel::expressions::scalars::MapData`

```rust
struct MapData
```

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn map_type(&self) -> &MapType
fn pairs(&self) -> &[(Scalar, Scalar)]
fn try_new(data_type: MapType, values: impl IntoIterator<Item = (impl Into<Scalar>, impl Into<Scalar>)>) -> DeltaResult<Self>
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

---

## StructData

`struct` · `buoyant_kernel::expressions::scalars::StructData`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.expressions.scalars.StructData.md)

Also reachable as `buoyant_kernel::expressions::StructData`, `delta_kernel::expressions::scalars::StructData`

```rust
struct StructData
```

**Implements**: `deltalake_core::kernel::arrow::engine_ext::StructDataExt`, `deltalake_core::kernel::snapshot::log_data::PartitionsExt`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn fields(&self) -> &[StructField]
fn try_new(fields: Vec<StructField>, values: Vec<Scalar>) -> DeltaResult<Self>
fn values(&self) -> &[Scalar]
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

---
