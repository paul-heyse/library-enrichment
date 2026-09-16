# `ruff_python_semantic::nodes`

Crate `ruff_python_semantic` · 3 public items · structured records in [`model/ruff_python_semantic.nodes.json`](../model/ruff_python_semantic.nodes.json)

## NodeRef

`enum` · `ruff_python_semantic::nodes::NodeRef`

Also reachable as `ruff_python_semantic::NodeRef`

```rust
enum NodeRef<'a>
```

**Variants**: `Stmt`, `Expr`

**Implements**: `core::convert::From`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Copy, Debug, PartialEq, StructuralPartialEq

**via `core::convert::From`**

```rust
fn from(expr: &'a Expr) -> Self
fn from(stmt: &'a Stmt) -> Self
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> TextRange
```

A reference to an AST node. Like [`ruff_python_ast::AnyNodeRef`], but wraps the node
itself (like [`Stmt`]) rather than the narrowed type (like [`ruff_python_ast::StmtAssign`]).

TODO(charlie): Replace with [`ruff_python_ast::AnyNodeRef`]. This requires migrating
the rest of the codebase to use [`ruff_python_ast::AnyNodeRef`] and related abstractions,
like [`ruff_python_ast::ExprRef`] instead of [`Expr`].

---

## NodeId

`struct` · `ruff_python_semantic::nodes::NodeId`

Also reachable as `ruff_python_semantic::NodeId`

```rust
struct NodeId
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

Id uniquely identifying an AST node in a program.

Using a `u32` is sufficient because Ruff only supports parsing documents with a size of max
`u32::max` and it is impossible to have more nodes than characters in the file. We use a
`NonZeroU32` to take advantage of memory layout optimizations.

---

## Nodes

`struct` · `ruff_python_semantic::nodes::Nodes`

Also reachable as `ruff_python_semantic::Nodes`

```rust
struct Nodes<'a>
```

**Implements**: `core::ops::index::Index`

**Derives**: Debug, Default

**via `core::ops::index::Index`**

```rust
fn index(&self, index: NodeId) -> &Self::Output
```

The nodes of a program indexed by [`NodeId`]

---
