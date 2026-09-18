# `buoyant_kernel::committer::commit_types::CommitType`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.committer.commit_types.CommitType.json).

<a id="op-b0a0a995360f8760580e70ea"></a>
## CommitType

`enum` · `buoyant_kernel::committer::commit_types::CommitType` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
enum CommitType
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/committer/commit_types.rs#L16).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/committer/commit_types.rs:16`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The type of commit operation being performed. This communicates to the committer whether this
is a table creation or a write to an existing table, and whether the table is catalog-managed.

<a id="op-efba135798100be71b8bb4bf"></a>
## CatalogManagedCreate

`variant` · `buoyant_kernel::committer::commit_types::CommitType::CatalogManagedCreate` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
CatalogManagedCreate
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/committer/commit_types.rs#L20).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/committer/commit_types.rs:20`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Creating a new catalog-managed table.

<a id="op-72bc42370415a955ef043828"></a>
## CatalogManagedWrite

`variant` · `buoyant_kernel::committer::commit_types::CommitType::CatalogManagedWrite` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
CatalogManagedWrite
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/committer/commit_types.rs#L24).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/committer/commit_types.rs:24`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Writing to an existing catalog-managed table.

<a id="op-99994aa2fb310a00f8303a64"></a>
## DowngradeToPathBased

`variant` · `buoyant_kernel::committer::commit_types::CommitType::DowngradeToPathBased` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
DowngradeToPathBased
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/committer/commit_types.rs#L31).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/committer/commit_types.rs:31`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Downgrading an existing catalog-managed table to path-based. Not currently supported.

<a id="op-4db9235d93d6263ecbca181c"></a>
## PathBasedCreate

`variant` · `buoyant_kernel::committer::commit_types::CommitType::PathBasedCreate` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
PathBasedCreate
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/committer/commit_types.rs#L18).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/committer/commit_types.rs:18`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Creating a new table via filesystem (no catalog involvement).

<a id="op-d6b8b7bef24a3b347ece647f"></a>
## PathBasedWrite

`variant` · `buoyant_kernel::committer::commit_types::CommitType::PathBasedWrite` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
PathBasedWrite
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/committer/commit_types.rs#L22).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/committer/commit_types.rs:22`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Writing to an existing path-based table.

<a id="op-dd9bc5f0e092a7e78ed9a932"></a>
## UpgradeToCatalogManaged

`variant` · `buoyant_kernel::committer::commit_types::CommitType::UpgradeToCatalogManaged` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
UpgradeToCatalogManaged
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/committer/commit_types.rs#L28).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/committer/commit_types.rs:28`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Upgrading an existing path-based table to catalog-managed. Not currently supported.

<a id="op-92c1004186182fa580f343df"></a>
## clone

`function` · `buoyant_kernel::committer::commit_types::CommitType::clone` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> CommitType
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/committer/commit_types.rs#L15).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::committer::commit_types::CommitType", "path": "CommitType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [15, 17], "end": [15, 22], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/committer/commit_types.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/committer/commit_types.rs:15`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3c3bb11e78df588bc5f13001"></a>
## eq

`function` · `buoyant_kernel::committer::commit_types::CommitType::eq` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eq(&self, other: &CommitType) -> bool
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/committer/commit_types.rs#L15).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::committer::commit_types::CommitType", "path": "CommitType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [15, 30], "end": [15, 39], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/committer/commit_types.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/committer/commit_types.rs:15`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-282e752b04d344d50e95d011"></a>
## fmt

`function` · `buoyant_kernel::committer::commit_types::CommitType::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/committer/commit_types.rs#L15).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::committer::commit_types::CommitType", "path": "CommitType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [15, 10], "end": [15, 15], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/committer/commit_types.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/committer/commit_types.rs:15`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fded876d1482a076b9450c75"></a>
## is_create

`function` · `buoyant_kernel::committer::commit_types::CommitType::is_create` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn is_create(&self) -> bool
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/committer/commit_types.rs#L36).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::committer::commit_types::CommitType", "path": "CommitType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [34, 1], "end": [51, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/committer/commit_types.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/committer/commit_types.rs:36`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Returns `true` if this is a create-table commit (version 0).

<a id="op-edcd0563c157c33cdb0d3982"></a>
## requires_catalog_committer

`function` · `buoyant_kernel::committer::commit_types::CommitType::requires_catalog_committer` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn requires_catalog_committer(&self) -> bool
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/committer/commit_types.rs#L42).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::committer::commit_types::CommitType", "path": "CommitType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [34, 1], "end": [51, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/committer/commit_types.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/committer/commit_types.rs:42`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Returns `true` if this commit includes a catalog-managed operation,
including upgrade/downgrade.
