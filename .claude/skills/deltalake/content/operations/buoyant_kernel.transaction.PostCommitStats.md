# `buoyant_kernel::transaction::PostCommitStats`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.transaction.PostCommitStats.json).

<a id="op-f3e0479e1d3f22611f0bcde0"></a>
## PostCommitStats

`struct` · `buoyant_kernel::transaction::PostCommitStats` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct PostCommitStats
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/mod.rs#L1596).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/mod.rs:1596`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Kernel exposes information about the state of the table that engines might want to use to
trigger actions like checkpointing or log compaction. This struct holds that information.

<a id="op-f4b963521c4d5b7a0c0d2560"></a>
## commits_since_checkpoint

`struct_field` · `buoyant_kernel::transaction::PostCommitStats::commits_since_checkpoint` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
commits_since_checkpoint: u64
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/mod.rs#L1599).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/mod.rs:1599`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The number of commits since this table has been checkpointed. Note that commit 0 is
considered a checkpoint for the purposes of this computation.

<a id="op-069e3118ead8933c88968c58"></a>
## commits_since_log_compaction

`struct_field` · `buoyant_kernel::transaction::PostCommitStats::commits_since_log_compaction` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
commits_since_log_compaction: u64
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/mod.rs#L1603).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/mod.rs:1603`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The number of commits since the log has been compacted on this table. Note that a
checkpoint is considered a compaction for the purposes of this computation. Thus this
is really the number of commits since a compaction OR a checkpoint.

<a id="op-29c0cc45f4a8832e77b33c86"></a>
## fmt

`function` · `buoyant_kernel::transaction::PostCommitStats::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/mod.rs#L1595).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::transaction::PostCommitStats", "path": "PostCommitStats"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1595, 10], "end": [1595, 15], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/mod.rs:1595`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.
