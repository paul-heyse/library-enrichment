# `buoyant_kernel::action_reconciliation::log_replay::ActionReconciliationIterator`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.action_reconciliation.log_replay.ActionReconciliationIterator.json).

<a id="op-e9d4036b573b4f1725f0c862"></a>
## ActionReconciliationIterator

`struct` · `buoyant_kernel::action_reconciliation::log_replay::ActionReconciliationIterator` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct ActionReconciliationIterator
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/action_reconciliation/log_replay.rs#L131).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/action_reconciliation/log_replay.rs:131`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Iterator over action reconciliation data.

This iterator yields a stream of [`FilteredEngineData`](../operations/buoyant_kernel.engine_data.FilteredEngineData.md#op-62b3837332f86d6b0fa41aab) items while, tracking action
counts. Used by both checkpoint and log compaction workflows.

<a id="op-f89dc83d0eaeec78ce57ac44"></a>
## Item

`assoc_type` · `buoyant_kernel::action_reconciliation::log_replay::ActionReconciliationIterator::Item` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type Item = Result<FilteredEngineData, Error>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/action_reconciliation/log_replay.rs#L180).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::action_reconciliation::log_replay::ActionReconciliationIterator", "path": "ActionReconciliationIterator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [179, 1], "end": [186, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/action_reconciliation/log_replay.rs"}, "trait": {"args": null, "id": "core::iter::traits::iterator::Iterator", "path": "Iterator"}, "trait_path": "core::iter::traits::iterator::Iterator"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/action_reconciliation/log_replay.rs:180`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-45bb635096731e82de70f947"></a>
## fmt

`function` · `buoyant_kernel::action_reconciliation::log_replay::ActionReconciliationIterator::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/action_reconciliation/log_replay.rs#L172).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::action_reconciliation::log_replay::ActionReconciliationIterator", "path": "ActionReconciliationIterator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [171, 1], "end": [177, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/action_reconciliation/log_replay.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/action_reconciliation/log_replay.rs:172`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-34d36fa66a8bba9d914da21d"></a>
## next

`function` · `buoyant_kernel::action_reconciliation::log_replay::ActionReconciliationIterator::next` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn next(&mut self) -> Option<Self::Item>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/action_reconciliation/log_replay.rs#L182).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::action_reconciliation::log_replay::ActionReconciliationIterator", "path": "ActionReconciliationIterator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [179, 1], "end": [186, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/action_reconciliation/log_replay.rs"}, "trait": {"args": null, "id": "core::iter::traits::iterator::Iterator", "path": "Iterator"}, "trait_path": "core::iter::traits::iterator::Iterator"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/action_reconciliation/log_replay.rs:182`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a293d81ea9f984def192499b"></a>
## state

`function` · `buoyant_kernel::action_reconciliation::log_replay::ActionReconciliationIterator::state` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn state(&self) -> Arc<ActionReconciliationIteratorState>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/action_reconciliation/log_replay.rs#L146).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::action_reconciliation::log_replay::ActionReconciliationIterator", "path": "ActionReconciliationIterator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [136, 1], "end": [169, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/action_reconciliation/log_replay.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/action_reconciliation/log_replay.rs:146`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Get the shared state. This allows sharing of stats.

<a id="op-24d5eafd6baa3210550b8b08"></a>
## inner

`struct_field` · `buoyant_kernel::action_reconciliation::log_replay::ActionReconciliationIterator::inner` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
inner: DeltaResultIteratorStatic<ActionReconciliationBatch>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/action_reconciliation/log_replay.rs#L132).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/action_reconciliation/log_replay.rs:132`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8f930bfcf91a21312ca441b8"></a>
## state

`struct_field` · `buoyant_kernel::action_reconciliation::log_replay::ActionReconciliationIterator::state` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
state: std::sync::Arc<ActionReconciliationIteratorState>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/action_reconciliation/log_replay.rs#L133).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/action_reconciliation/log_replay.rs:133`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.
