# `buoyant_kernel::expressions::column_names`

Crate `buoyant_kernel` · 1 public items · structured records in [`model/buoyant_kernel.expressions.column_names.json`](../model/buoyant_kernel.expressions.column_names.json)

## ColumnName

`struct` · `buoyant_kernel::expressions::column_names::ColumnName`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.expressions.column_names.ColumnName.md)

Also reachable as `buoyant_kernel::expressions::ColumnName`, `delta_kernel::expressions::column_names::ColumnName`

```rust
struct ColumnName
```

**Implements**: `core::borrow::Borrow`, `core::fmt::Display`, `core::iter::traits::collect::FromIterator`, `core::iter::traits::collect::IntoIterator`, `core::ops::deref::Deref`, `core::str::traits::FromStr`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (7)

```rust
fn from_naive_str_split(name: impl AsRef<str>) -> Self
fn into_inner(self) -> Vec<String>
fn join(&self, right: &ColumnName) -> ColumnName
fn new(iter: impl CollectInto<Self>) -> Self
fn parent(&self) -> Option<ColumnName>
fn parse_column_name_list(names: impl AsRef<str>) -> DeltaResult<Vec<ColumnName>>
fn path(&self) -> &[String]
```

**via `core::borrow::Borrow`**

```rust
fn borrow(&self) -> &[String]
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

**via `core::iter::traits::collect::FromIterator`**

```rust
fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self
fn from_iter<T: IntoIterator<Item = ColumnName>>(iter: T) -> Self
```

**via `core::iter::traits::collect::IntoIterator`**

```rust
fn into_iter(self) -> Self::IntoIter
```

**via `core::ops::deref::Deref`**

```rust
fn deref(&self) -> &[String]
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(s: &str) -> DeltaResult<Self>
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

A (possibly nested) column name.

---
