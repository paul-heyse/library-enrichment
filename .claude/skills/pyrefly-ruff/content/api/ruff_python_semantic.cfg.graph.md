# `ruff_python_semantic::cfg::graph`

Crate `ruff_python_semantic` · 5 public items · structured records in [`model/ruff_python_semantic.cfg.graph.json`](../model/ruff_python_semantic.cfg.graph.json)

## Condition

`enum` · `ruff_python_semantic::cfg::graph::Condition`

```rust
enum Condition
```

**Variants**: `Always`

**Derives**: Clone, Debug

Represents a condition to be tested in a multi-way branch

---

## build_cfg

`function` · `ruff_python_semantic::cfg::graph::build_cfg`

```rust
fn build_cfg(stmts: &[ruff_python_ast::Stmt]) -> ControlFlowGraph<'_>
```

Returns the control flow graph associated to an array of statements

---

## BlockId

`struct` · `ruff_python_semantic::cfg::graph::BlockId`

```rust
struct BlockId
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

---

## ControlFlowGraph

`struct` · `ruff_python_semantic::cfg::graph::ControlFlowGraph`

```rust
struct ControlFlowGraph<'stmt>
```

**Derives**: Debug

**Methods** (6)

```rust
fn initial(&self) -> BlockId
fn num_blocks(&self) -> usize
fn outgoing(&self, block: BlockId) -> &Edges
fn predecessors(&self, block: BlockId) -> impl ExactSizeIterator<Item = BlockId> + '_
fn range(&self, block: BlockId) -> TextRange
fn stmts(&self, block: BlockId) -> &'stmt [Stmt]
```

Control flow graph

---

## Edges

`struct` · `ruff_python_semantic::cfg::graph::Edges`

```rust
struct Edges
```

**Derives**: Clone, Debug, Default

**Methods** (1)

```rust
fn filter_targets_by_conditions<'a, T: FnMut(&Condition) -> bool + 'a>(&'a self, predicate: T) -> impl Iterator<Item = BlockId> + 'a
```

Holds a collection of edges. Each edge is determined by:
 - a [`Condition`] for traversing the edge, and
 - a target block, specified by its [`BlockId`].

The conditions and targets are kept in two separate
vectors which must always be kept the same length.

---
