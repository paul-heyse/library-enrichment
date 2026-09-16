# `ruff_python_semantic::branches`

Crate `ruff_python_semantic` · 1 public items · structured records in [`model/ruff_python_semantic.branches.json`](../model/ruff_python_semantic.branches.json)

## BranchId

`struct` · `ruff_python_semantic::branches::BranchId`

Also reachable as `ruff_python_semantic::BranchId`

```rust
struct BranchId
```

**Implements**: `core::convert::From`, `core::ops::arith::Add`, `ruff_index::idx::Idx`

**Derives**: Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

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

ID uniquely identifying a branch in a program.

For example, given:
```python
if x > 0:
    pass
elif x > 1:
    pass
else:
    pass
```

Each of the three arms of the `if`-`elif`-`else` would be considered a branch, and would be
assigned their own unique [`BranchId`].

---
