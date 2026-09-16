# `pyrefly_types::polars_dtype`

Crate `pyrefly_types` · 3 public items · structured records in [`model/pyrefly_types.polars_dtype.json`](../model/pyrefly_types.polars_dtype.json)

## PolarsArrayShape

`enum` · `pyrefly_types::polars_dtype::PolarsArrayShape`

```rust
enum PolarsArrayShape
```

**Variants**: `Known`, `Unknown`

**Implements**: `pyrefly_types::equality::TypeEq`, `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `pyrefly_types::equality::TypeEq`**

```rust
fn type_eq(&self, other: &Self, ctx: &mut equality::TypeEqCtx) -> bool
```

**via `pyrefly_util::visit::Visit`**

```rust
fn recurse<'a>(&'a self, f: &mut dyn FnMut(&'a To))
```

**via `pyrefly_util::visit::VisitMut`**

```rust
fn recurse_mut(&mut self, f: &mut dyn FnMut(&mut To))
```

The dimensions of a Polars array dtype.

---

## PolarsDType

`enum` · `pyrefly_types::polars_dtype::PolarsDType`

```rust
enum PolarsDType
```

**Variants**: `Scalar`, `List`, `Array`, `Struct`, `Unknown`

**Implements**: `core::fmt::Display`, `pyrefly_types::equality::TypeEq`, `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (7)

```rust
fn from_polars_name(name: &str) -> Option<Self>
fn int_bounds(&self) -> Option<(i128, i128)>
fn is_float(&self) -> bool
fn is_integer(&self) -> bool
fn is_numeric(&self) -> bool
fn is_signed_int(&self) -> bool
fn supertype(self, other: Self) -> Option<Self>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `pyrefly_types::equality::TypeEq`**

```rust
fn type_eq(&self, other: &Self, ctx: &mut equality::TypeEqCtx) -> bool
```

**via `pyrefly_util::visit::Visit`**

```rust
fn recurse<'a>(&'a self, _: &mut dyn FnMut(&'a To))
```

**via `pyrefly_util::visit::VisitMut`**

```rust
fn recurse_mut(&mut self, _: &mut dyn FnMut(&mut To))
```

A column dtype. `Unknown` represents a column whose dtype cannot be determined.

---

## PolarsScalarDType

`enum` · `pyrefly_types::polars_dtype::PolarsScalarDType`

```rust
enum PolarsScalarDType
```

**Variants**: `Boolean`, `Int8`, `Int16`, `Int32`, `Int64`, `Int128`, `UInt8`, `UInt16`, `UInt32`, `UInt64`, `UInt128`, `Float32`, `Float64`, `String`, `Binary`, `Date`, `Datetime`, `Duration`, `Time`, `Null`, `Object`

**Implements**: `pyrefly_types::equality::TypeEq`, `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`

**Derives**: Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (1)

```rust
fn name(self) -> &'static str
```

**via `pyrefly_types::equality::TypeEq`**

```rust
fn type_eq(&self, other: &Self, ctx: &mut equality::TypeEqCtx) -> bool
```

**via `pyrefly_util::visit::Visit`**

```rust
fn recurse<'a>(&'a self, f: &mut dyn FnMut(&'a To))
```

**via `pyrefly_util::visit::VisitMut`**

```rust
fn recurse_mut(&mut self, f: &mut dyn FnMut(&mut To))
```

A non-recursive Polars dtype used as a leaf in composite column dtypes.

---
