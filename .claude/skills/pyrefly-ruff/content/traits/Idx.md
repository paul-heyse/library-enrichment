# Idx

`ruff_index::idx::Idx`

```rust
trait Idx: Copy + PartialEq + Eq + Hash + std::fmt::Debug + 'static
```

Also reachable as `ruff_index::Idx`

Prose: [`api/ruff_index.idx.md`](../api/ruff_index.idx.md#idx) · records: [`model/ruff_index.idx.json`](../model/ruff_index.idx.json)

## Required

Every implementation must supply these.

```rust
fn index(self) -> usize
fn new(value: usize) -> Self
```

## Implementors (8)

Read one before writing your own.

- `ruff_python_semantic::binding::BindingId`
- `ruff_python_semantic::branches::BranchId`
- `ruff_python_semantic::cfg::graph::BlockId`
- `ruff_python_semantic::definition::DefinitionId`
- `ruff_python_semantic::globals::GlobalsId`
- `ruff_python_semantic::nodes::NodeId`
- `ruff_python_semantic::reference::ResolvedReferenceId`
- `ruff_python_semantic::scope::ScopeId`

## Documentation

Represents a newtype wrapper used to index into a Vec or a slice.

You can use the [`newtype_index`](crate::newtype_index) macro to define your own index.
