# `ruff_python_semantic::globals`

Crate `ruff_python_semantic` · 2 public items · structured records in [`model/ruff_python_semantic.globals.json`](../model/ruff_python_semantic.globals.json)

## Globals

`struct` · `ruff_python_semantic::globals::Globals`

Also reachable as `ruff_python_semantic::Globals`

```rust
struct Globals<'a>
```

**Derives**: Debug

**Methods** (1)

```rust
fn from_body(body: &'a [Stmt]) -> Option<Self>
```

The set of global names for a given scope, represented as a map from the name of the global to
the range of the declaration in the source code.

---

## GlobalsId

`struct` · `ruff_python_semantic::globals::GlobalsId`

Also reachable as `ruff_python_semantic::GlobalsId`

```rust
struct GlobalsId
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

Id uniquely identifying the set of global names for a given scope.

---
