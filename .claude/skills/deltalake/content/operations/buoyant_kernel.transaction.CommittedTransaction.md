# `buoyant_kernel::transaction::CommittedTransaction`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.transaction.CommittedTransaction.json).

<a id="op-bac54c281be967a3d8ea6377"></a>
## CommittedTransaction

`struct` · `buoyant_kernel::transaction::CommittedTransaction` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct CommittedTransaction
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/mod.rs#L1672).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/mod.rs:1672`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

This is the result of a successfully committed [Transaction](../operations/buoyant_kernel.transaction.Transaction.md#op-fde417829ac3f4c320319f35). One can retrieve the
[post_commit_stats], [commit version], and optionally the [post-commit snapshot] from this
struct.

[post_commit_stats]: Self::post_commit_stats
[commit version]: Self::commit_version
[post-commit snapshot]: Self::post_commit_snapshot

<a id="op-b67905944ba260128138347f"></a>
## commit_version

`function` · `buoyant_kernel::transaction::CommittedTransaction::commit_version` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn commit_version(&self) -> Version
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/mod.rs#L1686).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::transaction::CommittedTransaction", "path": "CommittedTransaction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1684, 1], "end": [1699, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/mod.rs:1686`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The version of the table that was just sucessfully committed

<a id="op-86d09ebf3d1ba5d341a1487d"></a>
## fmt

`function` · `buoyant_kernel::transaction::CommittedTransaction::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/mod.rs#L1671).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::transaction::CommittedTransaction", "path": "CommittedTransaction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1671, 10], "end": [1671, 15], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/mod.rs:1671`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-72777861a6d5101638f2e1bc"></a>
## post_commit_snapshot

`function` · `buoyant_kernel::transaction::CommittedTransaction::post_commit_snapshot` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn post_commit_snapshot(&self) -> Option<&SnapshotRef>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/mod.rs#L1696).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::transaction::CommittedTransaction", "path": "CommittedTransaction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1684, 1], "end": [1699, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/mod.rs:1696`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The [`SnapshotRef`](../operations/buoyant_kernel.snapshot.SnapshotRef.md#op-7166b16a39037614007f3d0c) of the table after this transaction was committed.

<a id="op-87e38fab1f94e26af0a2058b"></a>
## post_commit_stats

`function` · `buoyant_kernel::transaction::CommittedTransaction::post_commit_stats` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn post_commit_stats(&self) -> &PostCommitStats
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/mod.rs#L1691).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::transaction::CommittedTransaction", "path": "CommittedTransaction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1684, 1], "end": [1699, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/mod.rs:1691`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The [`PostCommitStats`](../operations/buoyant_kernel.transaction.PostCommitStats.md#op-f3e0479e1d3f22611f0bcde0) for this transaction

<a id="op-abed907d9a9fd6a0b9ea2097"></a>
## commit_version

`struct_field` · `buoyant_kernel::transaction::CommittedTransaction::commit_version` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
commit_version: Version
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/mod.rs#L1674).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/mod.rs:1674`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The version of the table that was just committed.

<a id="op-ecab91faedbf7766259a839a"></a>
## post_commit_snapshot

`struct_field` · `buoyant_kernel::transaction::CommittedTransaction::post_commit_snapshot` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
post_commit_snapshot: Option<snapshot::SnapshotRef>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/mod.rs#L1681).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/mod.rs:1681`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The [`SnapshotRef`](../operations/buoyant_kernel.snapshot.SnapshotRef.md#op-7166b16a39037614007f3d0c) of the table after this transaction was committed.

This is optional to allow incremental development of new features (e.g., table creation,
transaction retries) without blocking on implementing post-commit snapshot support.

<a id="op-b8d00e12ec5d390f1ea88f71"></a>
## post_commit_stats

`struct_field` · `buoyant_kernel::transaction::CommittedTransaction::post_commit_stats` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
post_commit_stats: PostCommitStats
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/mod.rs#L1676).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/mod.rs:1676`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The [`PostCommitStats`](../operations/buoyant_kernel.transaction.PostCommitStats.md#op-f3e0479e1d3f22611f0bcde0) for this transaction.
