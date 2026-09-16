# `ruff_python_semantic::scope`

Crate `ruff_python_semantic` · 6 public items · structured records in [`model/ruff_python_semantic.scope.json`](../model/ruff_python_semantic.scope.json)

## GeneratorKind

`enum` · `ruff_python_semantic::scope::GeneratorKind`

Also reachable as `ruff_python_semantic::GeneratorKind`

```rust
enum GeneratorKind
```

**Variants**: `Generator`, `ListComprehension`, `DictComprehension`, `SetComprehension`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

---

## ScopeKind

`enum` · `ruff_python_semantic::scope::ScopeKind`

Also reachable as `ruff_python_semantic::ScopeKind`

```rust
enum ScopeKind<'a>
```

**Variants**: `Class`, `DunderClassCell`, `Function`, `Generator`, `Module`, `Type`, `Lambda`

**Derives**: Clone, Copy, Debug

**Methods** (19)

```rust
fn as_class(&self) -> Option<&&'a ast::StmtClassDef>
fn as_function(&self) -> Option<&&'a ast::StmtFunctionDef>
fn as_lambda(&self) -> Option<&&'a ast::ExprLambda>
fn as_mut_class(&mut self) -> Option<&mut &'a ast::StmtClassDef>
fn as_mut_function(&mut self) -> Option<&mut &'a ast::StmtFunctionDef>
fn as_mut_lambda(&mut self) -> Option<&mut &'a ast::ExprLambda>
fn class(self) -> Option<&'a ast::StmtClassDef>
fn expect_class(self) -> &'a ast::StmtClassDef where Self: ::std::fmt::Debug
fn expect_function(self) -> &'a ast::StmtFunctionDef where Self: ::std::fmt::Debug
fn expect_lambda(self) -> &'a ast::ExprLambda where Self: ::std::fmt::Debug
fn function(self) -> Option<&'a ast::StmtFunctionDef>
const fn is_class(&self) -> bool
const fn is_dunder_class_cell(&self) -> bool
const fn is_function(&self) -> bool
const fn is_generator(&self) -> bool
const fn is_lambda(&self) -> bool
const fn is_module(&self) -> bool
const fn is_type(&self) -> bool
fn lambda(self) -> Option<&'a ast::ExprLambda>
```

---

## Scope

`struct` · `ruff_python_semantic::scope::Scope`

Also reachable as `ruff_python_semantic::Scope`

```rust
struct Scope<'a>
```

**Fields**: `kind`

**Derives**: Debug

**Methods** (13)

```rust
fn add(&mut self, name: &'a str, id: BindingId) -> Option<BindingId>
fn add_star_import(&mut self, import: StarImport<'a>)
fn all_bindings(&self) -> impl Iterator<Item = (&str, BindingId)> + '_
fn binding_ids(&self) -> impl Iterator<Item = BindingId> + '_
fn bindings(&self) -> impl Iterator<Item = (&'a str, BindingId)> + '_
fn get(&self, name: &str) -> Option<BindingId>
fn get_all(&self, name: &str) -> impl Iterator<Item = BindingId> + '_
fn has(&self, name: &str) -> bool
fn set_uses_locals(&mut self)
fn shadowed_binding(&self, id: BindingId) -> Option<BindingId>
fn shadowed_bindings(&self, id: BindingId) -> impl Iterator<Item = BindingId> + '_
const fn uses_locals(&self) -> bool
fn uses_star_imports(&self) -> bool
```

---

## ScopeFlags

`struct` · `ruff_python_semantic::scope::ScopeFlags`

Also reachable as `ruff_python_semantic::ScopeFlags`

```rust
struct ScopeFlags
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
const fn iter(&self) -> iter::Iter<ScopeFlags>
const fn iter_names(&self) -> iter::IterNames<ScopeFlags>
fn remove(&mut self, other: Self)
fn set(&mut self, other: Self, value: bool)
const fn symmetric_difference(self, other: Self) -> Self
fn toggle(&mut self, other: Self)
const fn union(self, other: Self) -> Self
```

**via `bitflags::traits::Flags`**

```rust
fn all_named() -> ScopeFlags
fn bits(&self) -> u8
fn from_bits_retain(bits: u8) -> ScopeFlags
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
fn bitor(self, other: ScopeFlags) -> Self
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

Flags on a [`Scope`].

---

## ScopeId

`struct` · `ruff_python_semantic::scope::ScopeId`

Also reachable as `ruff_python_semantic::ScopeId`

```rust
struct ScopeId
```

**Implements**: `core::convert::From`, `core::ops::arith::Add`, `ruff_index::idx::Idx`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (7)

```rust
const fn as_u32(self) -> u32
const fn as_usize(self) -> usize
const fn from_u32(value: u32) -> Self
const fn from_usize(value: usize) -> Self
const fn global() -> Self
const fn index(self) -> usize
const fn is_global(&self) -> bool
```

**via `core::convert::From`**

```rust
fn from(value: usize) -> Self
fn from(value: u32) -> Self
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

Id uniquely identifying a scope in a program.

Using a `u32` is sufficient because Ruff only supports parsing documents with a size of max `u32::max`
and it is impossible to have more scopes than characters in the file (because defining a function or class
requires more than one character).

---

## Scopes

`struct` · `ruff_python_semantic::scope::Scopes`

Also reachable as `ruff_python_semantic::Scopes`

```rust
struct Scopes<'a>
```

**Implements**: `core::ops::deref::Deref`, `core::ops::deref::DerefMut`

**Derives**: Debug, Default

**Methods** (2)

```rust
fn ancestor_ids(&self, scope_id: ScopeId) -> impl Iterator<Item = ScopeId> + '_
fn ancestors(&self, scope_id: ScopeId) -> impl Iterator<Item = &Scope<'a>> + '_
```

**via `core::ops::deref::Deref`**

```rust
fn deref(&self) -> &Self::Target
```

**via `core::ops::deref::DerefMut`**

```rust
fn deref_mut(&mut self) -> &mut Self::Target
```

The scopes of a program indexed by [`ScopeId`]

---
