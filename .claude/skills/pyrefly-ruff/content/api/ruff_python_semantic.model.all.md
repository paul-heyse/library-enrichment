# `ruff_python_semantic::model::all`

Crate `ruff_python_semantic` · 3 public items · structured records in [`model/ruff_python_semantic.model.all.json`](../model/ruff_python_semantic.model.all.json)

## DunderAllDefinition

`struct` · `ruff_python_semantic::model::all::DunderAllDefinition`

Also reachable as `ruff_python_semantic::all::DunderAllDefinition`

```rust
struct DunderAllDefinition<'a>
```

**Implements**: `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug

**Methods** (2)

```rust
fn names(&self) -> &[DunderAllName<'a>]
fn new(range: TextRange, names: Vec<DunderAllName<'a>>) -> Self
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> TextRange
```

Abstraction for a collection of names inside an `__all__` definition,
e.g. `["foo", "bar"]` in `__all__ = ["foo", "bar"]`

---

## DunderAllFlags

`struct` · `ruff_python_semantic::model::all::DunderAllFlags`

Also reachable as `ruff_python_semantic::all::DunderAllFlags`

```rust
struct DunderAllFlags
```

**Implements**: `bitflags::traits::Flags`, `bitflags::traits::PublicFlags`, `core::fmt::Binary`, `core::fmt::LowerHex`, `core::fmt::Octal`, `core::fmt::UpperHex`, `core::iter::traits::collect::Extend`, `core::iter::traits::collect::FromIterator`, `core::iter::traits::collect::IntoIterator`, `core::ops::arith::Sub`, `core::ops::arith::SubAssign`, `core::ops::bit::BitAnd`, `core::ops::bit::BitAndAssign`, `core::ops::bit::BitOr`, `core::ops::bit::BitOrAssign`, `core::ops::bit::BitXor`, `core::ops::bit::BitXorAssign`, `core::ops::bit::Not`

**Derives**: Clone, Copy, Debug, Default, Eq, PartialEq, StructuralPartialEq

**Methods** (22)

```rust
const fn all() -> Self
const fn bits(&self) -> u8
const fn complement(self) -> Self
const fn contains(&self, other: Self) -> bool
const fn difference(self, other: Self) -> Self
const fn empty() -> Self
const fn from_bits(bits: u8) -> __private::core::option::Option<Self>
const fn from_bits_retain(bits: u8) -> Self
const fn from_bits_truncate(bits: u8) -> Self
fn from_name(name: &str) -> __private::core::option::Option<Self>
fn insert(&mut self, other: Self)
const fn intersection(self, other: Self) -> Self
const fn intersects(&self, other: Self) -> bool
const fn is_all(&self) -> bool
const fn is_empty(&self) -> bool
const fn iter(&self) -> iter::Iter<DunderAllFlags>
const fn iter_names(&self) -> iter::IterNames<DunderAllFlags>
fn remove(&mut self, other: Self)
fn set(&mut self, other: Self, value: bool)
const fn symmetric_difference(self, other: Self) -> Self
fn toggle(&mut self, other: Self)
const fn union(self, other: Self) -> Self
```

**via `bitflags::traits::Flags`**

```rust
fn all_named() -> DunderAllFlags
fn bits(&self) -> u8
fn from_bits_retain(bits: u8) -> DunderAllFlags
```

**via `core::fmt::Binary`**

```rust
fn fmt(&self, f: &mut __private::core::fmt::Formatter<'_>) -> __private::core::fmt::Result
```

**via `core::fmt::LowerHex`**

```rust
fn fmt(&self, f: &mut __private::core::fmt::Formatter<'_>) -> __private::core::fmt::Result
```

**via `core::fmt::Octal`**

```rust
fn fmt(&self, f: &mut __private::core::fmt::Formatter<'_>) -> __private::core::fmt::Result
```

**via `core::fmt::UpperHex`**

```rust
fn fmt(&self, f: &mut __private::core::fmt::Formatter<'_>) -> __private::core::fmt::Result
```

**via `core::iter::traits::collect::Extend`**

```rust
fn extend<T: __private::core::iter::IntoIterator<Item = Self>>(&mut self, iterator: T)
```

**via `core::iter::traits::collect::FromIterator`**

```rust
fn from_iter<T: __private::core::iter::IntoIterator<Item = Self>>(iterator: T) -> Self
```

**via `core::iter::traits::collect::IntoIterator`**

```rust
fn into_iter(self) -> Self::IntoIter
```

**via `core::ops::arith::Sub`**

```rust
fn sub(self, other: Self) -> Self
```

**via `core::ops::arith::SubAssign`**

```rust
fn sub_assign(&mut self, other: Self)
```

**via `core::ops::bit::BitAnd`**

```rust
fn bitand(self, other: Self) -> Self
```

**via `core::ops::bit::BitAndAssign`**

```rust
fn bitand_assign(&mut self, other: Self)
```

**via `core::ops::bit::BitOr`**

```rust
fn bitor(self, other: DunderAllFlags) -> Self
```

**via `core::ops::bit::BitOrAssign`**

```rust
fn bitor_assign(&mut self, other: Self)
```

**via `core::ops::bit::BitXor`**

```rust
fn bitxor(self, other: Self) -> Self
```

**via `core::ops::bit::BitXorAssign`**

```rust
fn bitxor_assign(&mut self, other: Self)
```

**via `core::ops::bit::Not`**

```rust
fn not(self) -> Self
```

---

## DunderAllName

`struct` · `ruff_python_semantic::model::all::DunderAllName`

Also reachable as `ruff_python_semantic::all::DunderAllName`

```rust
struct DunderAllName<'a>
```

**Implements**: `ruff_text_size::traits::Ranged`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn name(&self) -> &'a str
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> TextRange
```

Abstraction for a string inside an `__all__` definition,
e.g. `"foo"` in `__all__ = ["foo"]`

---
