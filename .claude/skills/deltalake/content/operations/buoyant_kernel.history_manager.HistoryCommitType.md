# `buoyant_kernel::history_manager::HistoryCommitType`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.history_manager.HistoryCommitType.json).

<a id="op-857723cc9241a5a49db3b8e1"></a>
## HistoryCommitType

`enum` · `buoyant_kernel::history_manager::HistoryCommitType` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
enum HistoryCommitType
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/history_manager/mod.rs#L842).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/history_manager/mod.rs:842`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Selects which commit the [`get_earliest_commit`](../operations/buoyant_kernel.history_manager.get_earliest_commit.md#op-c7b46a5888a0e61479251c50) query returns.

<a id="op-a3c94aac99585cdad3e98df1"></a>
## Published

`variant` · `buoyant_kernel::history_manager::HistoryCommitType::Published` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Published
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/history_manager/mod.rs#L845).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/history_manager/mod.rs:845`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

A filesystem or catalog commit present in the table's `_delta_log/`. Its presence does not
guarantee that the table can be reconstructed at that commit's version.

<a id="op-ed9a706cf7eb4e3fc81425ca"></a>
## Recreatable

`variant` · `buoyant_kernel::history_manager::HistoryCommitType::Recreatable` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Recreatable
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/history_manager/mod.rs#L848).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/history_manager/mod.rs:848`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

A commit whose version the table state can be fully reconstructed at and replayed forward
from to the latest version.

<a id="op-c00c6ee2cc72a2a68b9adbbd"></a>
## clone

`function` · `buoyant_kernel::history_manager::HistoryCommitType::clone` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> HistoryCommitType
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/history_manager/mod.rs#L841).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::history_manager::HistoryCommitType", "path": "HistoryCommitType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [841, 17], "end": [841, 22], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/history_manager/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/history_manager/mod.rs:841`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1fa664514da6089ee28f9ed6"></a>
## eq

`function` · `buoyant_kernel::history_manager::HistoryCommitType::eq` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eq(&self, other: &HistoryCommitType) -> bool
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/history_manager/mod.rs#L841).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::history_manager::HistoryCommitType", "path": "HistoryCommitType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [841, 30], "end": [841, 39], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/history_manager/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/history_manager/mod.rs:841`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-327695a23ee6108cf8298c71"></a>
## fmt

`function` · `buoyant_kernel::history_manager::HistoryCommitType::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/history_manager/mod.rs#L841).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::history_manager::HistoryCommitType", "path": "HistoryCommitType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [841, 10], "end": [841, 15], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/history_manager/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/history_manager/mod.rs:841`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.
