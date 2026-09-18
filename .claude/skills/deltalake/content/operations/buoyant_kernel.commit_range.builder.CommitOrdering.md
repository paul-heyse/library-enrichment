# `buoyant_kernel::commit_range::builder::CommitOrdering`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.commit_range.builder.CommitOrdering.json).

<a id="op-406de1722baf5b7d3ee90ca9"></a>
## CommitOrdering

`enum` · `buoyant_kernel::commit_range::builder::CommitOrdering` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
enum CommitOrdering
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/commit_range/builder.rs#L129).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/commit_range/builder.rs:129`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Direction in which [`CommitRange::commits`](../operations/buoyant_kernel.commit_range.CommitRange.md#op-82385bf443cf1811dcd55fe8) yields commits.
Default is [`CommitOrdering::AscendingOrder`](../operations/buoyant_kernel.commit_range.builder.CommitOrdering.md#op-8b295ffda3eae152a8fae3b3)

<a id="op-8b295ffda3eae152a8fae3b3"></a>
## AscendingOrder

`variant` · `buoyant_kernel::commit_range::builder::CommitOrdering::AscendingOrder` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
AscendingOrder
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/commit_range/builder.rs#L131).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/commit_range/builder.rs:131`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Yield commits in increasing version order (e.g. `v=0, v=1, v=2, ...`).

<a id="op-75145b6f3d62cc4e2e0418a5"></a>
## DescendingOrder

`variant` · `buoyant_kernel::commit_range::builder::CommitOrdering::DescendingOrder` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
DescendingOrder
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/commit_range/builder.rs#L133).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/commit_range/builder.rs:133`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Yield commits in decreasing version order (e.g. `v=N, v=N-1, ..., v=0`).

<a id="op-690fd414e48dc5caa5b31df5"></a>
## clone

`function` · `buoyant_kernel::commit_range::builder::CommitOrdering::clone` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> CommitOrdering
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/commit_range/builder.rs#L128).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::commit_range::builder::CommitOrdering", "path": "CommitOrdering"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [128, 17], "end": [128, 22], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/commit_range/builder.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/commit_range/builder.rs:128`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0b2a4e08b2d3e1f0b828da54"></a>
## eq

`function` · `buoyant_kernel::commit_range::builder::CommitOrdering::eq` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eq(&self, other: &CommitOrdering) -> bool
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/commit_range/builder.rs#L128).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::commit_range::builder::CommitOrdering", "path": "CommitOrdering"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [128, 30], "end": [128, 39], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/commit_range/builder.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/commit_range/builder.rs:128`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2f3972e67321e3891f6a547d"></a>
## fmt

`function` · `buoyant_kernel::commit_range::builder::CommitOrdering::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/commit_range/builder.rs#L128).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::commit_range::builder::CommitOrdering", "path": "CommitOrdering"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [128, 10], "end": [128, 15], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/commit_range/builder.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/commit_range/builder.rs:128`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.
