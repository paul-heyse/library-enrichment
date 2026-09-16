# `buoyant_kernel::action_reconciliation::log_replay`

Crate `buoyant_kernel` · 2 public items · structured records in [`model/buoyant_kernel.action_reconciliation.log_replay.json`](../model/buoyant_kernel.action_reconciliation.log_replay.json)

## ActionReconciliationIterator

`struct` · `buoyant_kernel::action_reconciliation::log_replay::ActionReconciliationIterator`

Also reachable as `buoyant_kernel::ActionReconciliationIterator`, `delta_kernel::action_reconciliation::log_replay::ActionReconciliationIterator`

```rust
struct ActionReconciliationIterator
```

**Implements**: `core::iter::traits::iterator::Iterator`

**Derives**: Debug

**Methods** (1)

```rust
fn state(&self) -> Arc<ActionReconciliationIteratorState>
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<Self::Item>
```

Iterator over action reconciliation data.

This iterator yields a stream of [`FilteredEngineData`] items while, tracking action
counts. Used by both checkpoint and log compaction workflows.

---

## ActionReconciliationIteratorState

`struct` · `buoyant_kernel::action_reconciliation::log_replay::ActionReconciliationIteratorState`

Also reachable as `buoyant_kernel::ActionReconciliationIteratorState`, `delta_kernel::action_reconciliation::log_replay::ActionReconciliationIteratorState`

```rust
struct ActionReconciliationIteratorState
```

**Derives**: Debug, Default

**Methods** (3)

```rust
fn actions_count(&self) -> i64
fn add_actions_count(&self) -> i64
fn is_exhausted(&self) -> bool
```

Stats for ActionReconciliationIterator

---
