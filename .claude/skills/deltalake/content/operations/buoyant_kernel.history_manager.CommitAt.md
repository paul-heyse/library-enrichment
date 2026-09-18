# `buoyant_kernel::history_manager::CommitAt`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.history_manager.CommitAt.json).

<a id="op-86a3435c27e3b39467f81178"></a>
## CommitAt

`struct` · `buoyant_kernel::history_manager::CommitAt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct CommitAt
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/history_manager/mod.rs#L52).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/history_manager/mod.rs:52`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

A commit located by a timestamp query: the commit [`Version`](../operations/buoyant_kernel.Version.md#op-7434cc865af7c4038dc36837) paired with its timestamp.

<a id="op-b753630782af16ac89314572"></a>
## clone

`function` · `buoyant_kernel::history_manager::CommitAt::clone` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> CommitAt
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/history_manager/mod.rs#L51).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::history_manager::CommitAt", "path": "CommitAt"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [51, 17], "end": [51, 22], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/history_manager/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/history_manager/mod.rs:51`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0d6512a7436c054d8ecbf64c"></a>
## eq

`function` · `buoyant_kernel::history_manager::CommitAt::eq` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eq(&self, other: &CommitAt) -> bool
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/history_manager/mod.rs#L51).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::history_manager::CommitAt", "path": "CommitAt"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [51, 30], "end": [51, 39], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/history_manager/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/history_manager/mod.rs:51`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-746cf92de7d89d9274ac3229"></a>
## fmt

`function` · `buoyant_kernel::history_manager::CommitAt::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/history_manager/mod.rs#L51).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::history_manager::CommitAt", "path": "CommitAt"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [51, 10], "end": [51, 15], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/history_manager/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/history_manager/mod.rs:51`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f1f7e28cb88ed63243129f60"></a>
## new

`function` · `buoyant_kernel::history_manager::CommitAt::new` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn new(version: Version, timestamp: Timestamp) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/history_manager/mod.rs#L64).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::history_manager::CommitAt", "path": "CommitAt"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [62, 1], "end": [67, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/history_manager/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/history_manager/mod.rs:64`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Creates a [`CommitAt`](../operations/buoyant_kernel.history_manager.CommitAt.md#op-86a3435c27e3b39467f81178) pairing a commit `version` with its `timestamp`.

<a id="op-790caf551fc89fc3b1e61297"></a>
## timestamp

`struct_field` · `buoyant_kernel::history_manager::CommitAt::timestamp` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
timestamp: Timestamp
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/history_manager/mod.rs#L59).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/history_manager/mod.rs:59`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Timestamp (milliseconds since the Unix epoch) associated with this commit.

This is the commit's in-commit timestamp (ICT) when ICT is enabled, otherwise the
commit's file modification time.

<a id="op-f04fa2419863d6ee043d641f"></a>
## version

`struct_field` · `buoyant_kernel::history_manager::CommitAt::version` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
version: Version
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/history_manager/mod.rs#L54).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/history_manager/mod.rs:54`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The commit version.
