# `ruff_python_stdlib::open_mode`

Crate `ruff_python_stdlib` · 1 public items · structured records in [`model/ruff_python_stdlib.open_mode.json`](../model/ruff_python_stdlib.open_mode.json)

## OpenMode

`struct` · `ruff_python_stdlib::open_mode::OpenMode`

```rust
struct OpenMode
```

**Implements**: `bitflags::traits::Flags`, `bitflags::traits::PublicFlags`, `core::convert::TryFrom`, `core::fmt::Binary`, `core::fmt::Display`, `core::fmt::LowerHex`, `core::fmt::Octal`, `core::fmt::UpperHex`, `core::iter::traits::collect::Extend`, `core::iter::traits::collect::FromIterator`, `core::iter::traits::collect::IntoIterator`, `core::ops::arith::Sub`, `core::ops::arith::SubAssign`, `core::ops::bit::BitAnd`, `core::ops::bit::BitAndAssign`, `core::ops::bit::BitOr`, `core::ops::bit::BitOrAssign`, `core::ops::bit::BitXor`, `core::ops::bit::BitXorAssign`, `core::ops::bit::Not`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (24)

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
fn from_chars(chars: impl Iterator<Item = char>) -> Result<Self, String>
fn from_name(name: &str) -> __private::core::option::Option<Self>
fn insert(&mut self, other: Self)
const fn intersection(self, other: Self) -> Self
const fn intersects(&self, other: Self) -> bool
const fn is_all(&self) -> bool
const fn is_empty(&self) -> bool
const fn iter(&self) -> iter::Iter<OpenMode>
const fn iter_names(&self) -> iter::IterNames<OpenMode>
fn reduce(self) -> Self
fn remove(&mut self, other: Self)
fn set(&mut self, other: Self, value: bool)
const fn symmetric_difference(self, other: Self) -> Self
fn toggle(&mut self, other: Self)
const fn union(self, other: Self) -> Self
```

**via `bitflags::traits::Flags`**

```rust
fn all_named() -> OpenMode
fn bits(&self) -> u8
fn from_bits_retain(bits: u8) -> OpenMode
```

**via `core::convert::TryFrom`**

```rust
fn try_from(value: &str) -> Result<Self, Self::Error>
fn try_from(value: char) -> Result<Self, Self::Error>
```

**via `core::fmt::Binary`**

```rust
fn fmt(&self, f: &mut __private::core::fmt::Formatter<'_>) -> __private::core::fmt::Result
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
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
fn bitor(self, other: OpenMode) -> Self
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
