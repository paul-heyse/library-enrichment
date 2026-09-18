# `buoyant_kernel::commit_range::actions`

Crate `buoyant_kernel` · 2 public items · structured records in [`model/buoyant_kernel.commit_range.actions.json`](../model/buoyant_kernel.commit_range.actions.json)

## DeltaAction

`enum` · `buoyant_kernel::commit_range::actions::DeltaAction`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.commit_range.actions.DeltaAction.md)

Also reachable as `buoyant_kernel::commit_range::DeltaAction`, `delta_kernel::commit_range::actions::DeltaAction`

```rust
enum DeltaAction
```

**Variants**: `Add`, `Remove`, `Metadata`, `Protocol`, `CommitInfo`, `Cdc`, `DomainMetadata`, `SetTxn`, `CheckpointMetadata`, `Sidecar`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

A Delta log action kind.

Callers that need to read multiple action types pass a slice
(e.g. `&[DeltaAction::Add, DeltaAction::Remove]`).

---

## CommitAction

`struct` · `buoyant_kernel::commit_range::actions::CommitAction`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.commit_range.actions.CommitAction.md)

Also reachable as `buoyant_kernel::commit_range::CommitAction`, `delta_kernel::commit_range::actions::CommitAction`

```rust
struct CommitAction
```

**Methods** (3)

```rust
fn get_actions(&self, engine: &dyn Engine) -> DeltaResult<FileDataReadResultIterator>
fn timestamp(&self) -> i64
fn version(&self) -> Version
```

Per-commit handle returned by [`super::CommitRange::commits`].

Carries the commit's version, timestamp, and the effective (extracted from this
commit overlaid onto the iterator's accumulated state) `Protocol` / `Metadata`.
Reading the commit's action batches is lazy and re-buildable via
[`Self::get_actions`], which issues a fresh JSON read on every call.

---
