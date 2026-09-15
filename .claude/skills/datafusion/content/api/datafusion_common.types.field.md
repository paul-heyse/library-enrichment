# `datafusion_common::types::field`

Crate `datafusion-common` · 4 public items · structured records in [`model/datafusion_common.types.field.json`](../model/datafusion_common.types.field.json)

## LogicalField

`struct` · `datafusion_common::types::field::LogicalField`

```rust
struct LogicalField
```

**Fields**: `name`, `logical_type`, `nullable`

**Implements**: `core::convert::From`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd

**via `core::convert::From`**

```rust
fn from(value: &Field) -> Self
```

A record of a logical type, its name and its nullability.

---

## LogicalFields

`struct` · `datafusion_common::types::field::LogicalFields`

```rust
struct LogicalFields
```

**Implements**: `core::convert::From`, `core::iter::traits::collect::FromIterator`, `core::ops::deref::Deref`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `core::convert::From`**

```rust
fn from(value: &Fields) -> Self
```

**via `core::iter::traits::collect::FromIterator`**

```rust
fn from_iter<T: IntoIterator<Item = LogicalFieldRef>>(iter: T) -> Self
```

**via `core::ops::deref::Deref`**

```rust
fn deref(&self) -> &Self::Target
```

A cheaply cloneable, owned collection of [`LogicalFieldRef`].

---

## LogicalUnionFields

`struct` · `datafusion_common::types::field::LogicalUnionFields`

```rust
struct LogicalUnionFields
```

**Implements**: `core::convert::From`, `core::iter::traits::collect::FromIterator`, `core::ops::deref::Deref`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `core::convert::From`**

```rust
fn from(value: &UnionFields) -> Self
```

**via `core::iter::traits::collect::FromIterator`**

```rust
fn from_iter<T: IntoIterator<Item = (i8, LogicalFieldRef)>>(iter: T) -> Self
```

**via `core::ops::deref::Deref`**

```rust
fn deref(&self) -> &Self::Target
```

A cheaply cloneable, owned collection of [`LogicalFieldRef`] and their
corresponding type ids.

---

## LogicalFieldRef

`type_alias` · `datafusion_common::types::field::LogicalFieldRef`

```rust
type LogicalFieldRef = std::sync::Arc<LogicalField>
```

A reference counted [`LogicalField`].

---
