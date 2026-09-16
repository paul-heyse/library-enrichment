# `ruff_python_semantic::reference`

Crate `ruff_python_semantic` · 4 public items · structured records in [`model/ruff_python_semantic.reference.json`](../model/ruff_python_semantic.reference.json)

## ResolvedReference

`struct` · `ruff_python_semantic::reference::ResolvedReference`

Also reachable as `ruff_python_semantic::ResolvedReference`

```rust
struct ResolvedReference
```

**Implements**: `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug

**Methods** (13)

```rust
const fn expression_id(&self) -> Option<NodeId>
const fn in_annotated_type_alias_value(&self) -> bool
const fn in_assert_statement(&self) -> bool
const fn in_dunder_all_definition(&self) -> bool
const fn in_runtime_context(&self) -> bool
const fn in_runtime_evaluated_annotation(&self) -> bool
const fn in_string_type_definition(&self) -> bool
const fn in_type_checking_block(&self) -> bool
const fn in_type_definition(&self) -> bool
const fn in_typing_context(&self) -> bool
const fn in_typing_only_annotation(&self) -> bool
const fn is_load(&self) -> bool
const fn scope_id(&self) -> ScopeId
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> TextRange
```

A resolved read reference to a name in a program.

---

## ResolvedReferenceId

`struct` · `ruff_python_semantic::reference::ResolvedReferenceId`

Also reachable as `ruff_python_semantic::ResolvedReferenceId`

```rust
struct ResolvedReferenceId
```

**Implements**: `core::convert::From`, `core::ops::arith::Add`, `ruff_index::idx::Idx`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (5)

```rust
const fn as_u32(self) -> u32
const fn as_usize(self) -> usize
const fn from_u32(value: u32) -> Self
const fn from_usize(value: usize) -> Self
const fn index(self) -> usize
```

**via `core::convert::From`**

```rust
fn from(value: u32) -> Self
fn from(value: usize) -> Self
```

**via `core::ops::arith::Add`**

```rust
fn add(self, rhs: usize) -> Self::Output
fn add(self, rhs: Self) -> Self::Output
```

**via `ruff_index::idx::Idx`**

```rust
fn index(self) -> usize
fn new(value: usize) -> Self
```

Id uniquely identifying a read reference in a program.

---

## UnresolvedReference

`struct` · `ruff_python_semantic::reference::UnresolvedReference`

Also reachable as `ruff_python_semantic::UnresolvedReference`

```rust
struct UnresolvedReference
```

**Derives**: Clone, Debug

**Methods** (5)

```rust
const fn annotation_binding_id(&self) -> Option<BindingId>
const fn exceptions(&self) -> Exceptions
const fn is_wildcard_import(&self) -> bool
fn name<'a>(&self, source: &'a str) -> &'a str
const fn range(&self) -> TextRange
```

An unresolved read reference to a name in a program.

---

## UnresolvedReferenceFlags

`struct` · `ruff_python_semantic::reference::UnresolvedReferenceFlags`

Also reachable as `ruff_python_semantic::UnresolvedReferenceFlags`

```rust
struct UnresolvedReferenceFlags
```

**Implements**: `bitflags::traits::Flags`, `bitflags::traits::PublicFlags`, `core::fmt::Binary`, `core::fmt::LowerHex`, `core::fmt::Octal`, `core::fmt::UpperHex`, `core::iter::traits::collect::Extend`, `core::iter::traits::collect::FromIterator`, `core::iter::traits::collect::IntoIterator`, `core::ops::arith::Sub`, `core::ops::arith::SubAssign`, `core::ops::bit::BitAnd`, `core::ops::bit::BitAndAssign`, `core::ops::bit::BitOr`, `core::ops::bit::BitOrAssign`, `core::ops::bit::BitXor`, `core::ops::bit::BitXorAssign`, `core::ops::bit::Not`

**Derives**: Clone, Copy, Debug

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
const fn iter(&self) -> iter::Iter<UnresolvedReferenceFlags>
const fn iter_names(&self) -> iter::IterNames<UnresolvedReferenceFlags>
fn remove(&mut self, other: Self)
fn set(&mut self, other: Self, value: bool)
const fn symmetric_difference(self, other: Self) -> Self
fn toggle(&mut self, other: Self)
const fn union(self, other: Self) -> Self
```

**via `bitflags::traits::Flags`**

```rust
fn all_named() -> UnresolvedReferenceFlags
fn bits(&self) -> u8
fn from_bits_retain(bits: u8) -> UnresolvedReferenceFlags
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
fn bitor(self, other: UnresolvedReferenceFlags) -> Self
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
