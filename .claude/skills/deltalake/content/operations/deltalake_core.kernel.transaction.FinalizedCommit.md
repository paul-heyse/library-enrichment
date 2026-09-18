# `deltalake_core::kernel::transaction::FinalizedCommit`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.kernel.transaction.FinalizedCommit.json).

<a id="op-c7dc3c28452e5b512659d272"></a>
## FinalizedCommit

`struct` · `deltalake_core::kernel::transaction::FinalizedCommit` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct FinalizedCommit
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/mod.rs#L1170).

Source: `crates/core/src/kernel/transaction/mod.rs:1170`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

A commit that successfully completed

<a id="op-919b03f426e13a18fae0dfd8"></a>
## fmt

`function` · `deltalake_core::kernel::transaction::FinalizedCommit::fmt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/mod.rs#L1182).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::transaction::FinalizedCommit", "path": "FinalizedCommit"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1181, 1], "end": [1188, 2], "filename": "crates/core/src/kernel/transaction/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/core/src/kernel/transaction/mod.rs:1182`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ceba97098e6d79b6cecfccac"></a>
## metrics

`struct_field` · `deltalake_core::kernel::transaction::FinalizedCommit::metrics` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
metrics: Metrics
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/mod.rs#L1178).

Source: `crates/core/src/kernel/transaction/mod.rs:1178`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Metrics associated with the commit operation

<a id="op-7ec7423e06535ded995b5baa"></a>
## snapshot

`function` · `deltalake_core::kernel::transaction::FinalizedCommit::snapshot` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn snapshot(&self) -> DeltaTableState
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/mod.rs#L1192).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::transaction::FinalizedCommit", "path": "FinalizedCommit"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1190, 1], "end": [1199, 2], "filename": "crates/core/src/kernel/transaction/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/kernel/transaction/mod.rs:1192`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The new table state after a commit

<a id="op-869b547997efe0989d721402"></a>
## snapshot

`struct_field` · `deltalake_core::kernel::transaction::FinalizedCommit::snapshot` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
snapshot: table::state::DeltaTableState
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/mod.rs#L1172).

Source: `crates/core/src/kernel/transaction/mod.rs:1172`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The new table state after a commit

<a id="op-20cc84a1e1a4748c5522aa4a"></a>
## version

`struct_field` · `deltalake_core::kernel::transaction::FinalizedCommit::version` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
version: kernel::Version
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/mod.rs#L1175).

Source: `crates/core/src/kernel/transaction/mod.rs:1175`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Version of the finalized commit

<a id="op-c61f7ccb0ffc3a2f4c6839f5"></a>
## version

`function` · `deltalake_core::kernel::transaction::FinalizedCommit::version` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn version(&self) -> Version
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/mod.rs#L1196).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::transaction::FinalizedCommit", "path": "FinalizedCommit"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1190, 1], "end": [1199, 2], "filename": "crates/core/src/kernel/transaction/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/kernel/transaction/mod.rs:1196`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Version of the finalized commit
