# `ruff_index::idx`

Crate `ruff_index` · 1 public items · structured records in [`model/ruff_index.idx.json`](../model/ruff_index.idx.json)

## Idx

`trait` · `ruff_index::idx::Idx`

Also reachable as `ruff_index::Idx`

```rust
trait Idx: Copy + PartialEq + Eq + Hash + std::fmt::Debug + 'static
```

**Implementors** (8)

- `ruff_python_semantic::binding::BindingId`
- `ruff_python_semantic::branches::BranchId`
- `ruff_python_semantic::cfg::graph::BlockId`
- `ruff_python_semantic::definition::DefinitionId`
- `ruff_python_semantic::globals::GlobalsId`
- `ruff_python_semantic::nodes::NodeId`
- `ruff_python_semantic::reference::ResolvedReferenceId`
- `ruff_python_semantic::scope::ScopeId`

**Methods** (2)

```rust
fn index(self) -> usize
fn new(value: usize) -> Self
```

Represents a newtype wrapper used to index into a Vec or a slice.

You can use the [`newtype_index`](crate::newtype_index) macro to define your own index.

---
