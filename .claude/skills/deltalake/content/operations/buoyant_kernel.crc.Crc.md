# `buoyant_kernel::crc::Crc`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.crc.Crc.json).

<a id="op-eb0003c051215379df8b89fa"></a>
## Crc

`struct` · `buoyant_kernel::crc::Crc` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct Crc
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/crc/mod.rs#L65).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/crc/mod.rs:65`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Parsed content of a CRC (version checksum) file.

A `Crc` is either (a) loaded from disk (deserialized from a `.crc` JSON file via
the private `CrcRaw` intermediate) or (b) computed in memory (built incrementally via
`Crc::apply`).

A CRC file must:
1. Be named `{version}.crc` with version zero-padded to 20 digits: `00000000000000000001.crc`
2. Be stored directly in the _delta_log directory alongside Delta log files
3. Contain exactly one JSON object with the schema mirrored by `CrcRaw`.

This struct and its fields are marked `pub`, but the `crc` module is only re-exported as `pub`
when the `internal-api` feature is enabled (otherwise `pub(crate)`). See `kernel/src/lib.rs`.

<a id="op-f19cadf534d48034e3fec0c1"></a>
## clone

`function` · `buoyant_kernel::crc::Crc::clone` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> Crc
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/crc/mod.rs#L64).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::crc::Crc", "path": "Crc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [64, 17], "end": [64, 22], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/crc/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/crc/mod.rs:64`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e277104bd0ade069b32a261d"></a>
## default

`function` · `buoyant_kernel::crc::Crc::default` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn default() -> Crc
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/crc/mod.rs#L64).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::crc::Crc", "path": "Crc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [64, 24], "end": [64, 31], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/crc/mod.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/crc/mod.rs:64`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8404fb61abd39f51056756ca"></a>
## domain_metadata_state

`struct_field` · `buoyant_kernel::crc::Crc::domain_metadata_state` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
domain_metadata_state: DomainMetadataState
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/crc/mod.rs#L92).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/crc/mod.rs:92`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Active (non-removed) [`DomainMetadata`](../operations/buoyant_kernel.actions.DomainMetadata.md#op-d3f1b17fff86b91870a0d719) actions at this version, as a typed
[`DomainMetadataState`](../operations/buoyant_kernel.crc.state.DomainMetadataState.md#op-1bdc582d72a8cd8896b7ca99). Tombstones (`removed=true`) are never stored. `Complete(map)`
is authoritative for misses; `Partial(map)` carries known-correct entries but requires
log replay for misses. Only the `Complete` variant is persisted to the CRC file.

TODO: when the table protocol does not enable the `domainMetadata` feature, no DM
      action can exist, so `Partial(_)` is semantically equivalent to
      `Complete(empty)` and both serde paths could collapse the distinction.

<a id="op-f38ba573941180d0b01a10fa"></a>
## eq

`function` · `buoyant_kernel::crc::Crc::eq` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eq(&self, other: &Crc) -> bool
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/crc/mod.rs#L64).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::crc::Crc", "path": "Crc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [64, 33], "end": [64, 42], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/crc/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/crc/mod.rs:64`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ed80f67a536ed122c43e1f3e"></a>
## file_stats

`function` · `buoyant_kernel::crc::Crc::file_stats` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn file_stats(&self) -> Option<&FileStats>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/crc/mod.rs#L113).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::crc::Crc", "path": "Crc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [107, 1], "end": [123, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/crc/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/crc/mod.rs:113`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Returns absolute file-level statistics only if `file_stats_state` is `Complete`.

Returns `None` when file stats cannot be trusted -- for example, when the CRC was
built from incremental replay that encountered a non-incremental operation or a
missing file size.

<a id="op-65edeec8ae72fe9645473d00"></a>
## fmt

`function` · `buoyant_kernel::crc::Crc::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/crc/mod.rs#L64).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::crc::Crc", "path": "Crc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [64, 10], "end": [64, 15], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/crc/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/crc/mod.rs:64`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6419653a73fa0e90bf29212b"></a>
## in_commit_timestamp_opt

`struct_field` · `buoyant_kernel::crc::Crc::in_commit_timestamp_opt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
in_commit_timestamp_opt: Option<i64>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/crc/mod.rs#L78).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/crc/mod.rs:78`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The in-commit timestamp of this version. Present iff In-Commit Timestamps are enabled.

<a id="op-411f0481bd9f60e67bcfa6ee"></a>
## metadata

`struct_field` · `buoyant_kernel::crc::Crc::metadata` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
metadata: actions::Metadata
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/crc/mod.rs#L70).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/crc/mod.rs:70`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The table [`Metadata`](../operations/buoyant_kernel.actions.Metadata.md#op-1843fecb3566e0e5ad63d7aa) at this version.

<a id="op-863c0c59bdaff7b1a153fc7f"></a>
## protocol

`struct_field` · `buoyant_kernel::crc::Crc::protocol` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
protocol: actions::Protocol
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/crc/mod.rs#L72).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/crc/mod.rs:72`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The table [`Protocol`](../operations/buoyant_kernel.actions.Protocol.md#op-0b0c1a8cbad46c3db514befe) at this version.

<a id="op-fbcf5c1af6501afcb0248add"></a>
## serialize

`function` · `buoyant_kernel::crc::Crc::serialize` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/crc/mod.rs#L128).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::crc::Crc", "path": "Crc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [127, 1], "end": [133, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/crc/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/crc/mod.rs:128`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e2f5bb8eb2689a3adf8912b1"></a>
## set_transaction_state

`struct_field` · `buoyant_kernel::crc::Crc::set_transaction_state` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
set_transaction_state: SetTransactionState
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/crc/mod.rs#L83).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/crc/mod.rs:83`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Active [`SetTransaction`](../operations/buoyant_kernel.actions.SetTransaction.md#op-63ccdda4a88886d7f887feaa) actions at this version, as a typed [`SetTransactionState`](../operations/buoyant_kernel.crc.state.SetTransactionState.md#op-813a79731eedededb5c6ef13).
`Complete(map)` is authoritative for misses; `Partial(map)` carries known-correct entries
but requires log replay for misses. Only the `Complete` variant is persisted to the CRC
file.

<a id="op-07e98786103d359f91b03e2e"></a>
## version

`struct_field` · `buoyant_kernel::crc::Crc::version` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
version: Version
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/crc/mod.rs#L68).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/crc/mod.rs:68`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The table version this CRC describes.

<a id="op-dbc662455b2bc9f20abca9da"></a>
## all_files

`struct_field` · `buoyant_kernel::crc::Crc::all_files` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
all_files: Option<Vec<actions::Add>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/crc/mod.rs#L98).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/crc/mod.rs:98`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

All live [`Add`](../operations/buoyant_kernel.actions.Add.md#op-84c3d1000106eb776de85180) file actions at this version.

<a id="op-ee542ba3bf38d086e664262a"></a>
## deleted_record_counts_histogram_opt

`struct_field` · `buoyant_kernel::crc::Crc::deleted_record_counts_histogram_opt` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
deleted_record_counts_histogram_opt: Option<DeletedRecordCountsHistogram>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/crc/mod.rs#L104).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/crc/mod.rs:104`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Distribution of deleted record counts across files.

<a id="op-6d5675268db52f6508eb93cd"></a>
## file_stats_state

`struct_field` · `buoyant_kernel::crc::Crc::file_stats_state` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
file_stats_state: FileStatsState
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/crc/mod.rs#L74).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/crc/mod.rs:74`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

File-level statistics as a typed state. See [`FileStatsState`](../operations/buoyant_kernel.crc.state.FileStatsState.md#op-fa49cf56a10f2e93f21b3b8a).

<a id="op-c6d4c8df7851e517f2f76a46"></a>
## num_deleted_records_opt

`struct_field` · `buoyant_kernel::crc::Crc::num_deleted_records_opt` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
num_deleted_records_opt: Option<i64>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/crc/mod.rs#L100).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/crc/mod.rs:100`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Number of records deleted through Deletion Vectors in this table version.

<a id="op-2446f132f27964c4cc8fb5fa"></a>
## num_deletion_vectors_opt

`struct_field` · `buoyant_kernel::crc::Crc::num_deletion_vectors_opt` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
num_deletion_vectors_opt: Option<i64>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/crc/mod.rs#L102).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/crc/mod.rs:102`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Number of Deletion Vectors active in this table version.

<a id="op-c670ce9c129761948fd5d224"></a>
## txn_id

`struct_field` · `buoyant_kernel::crc::Crc::txn_id` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
txn_id: Option<String>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/crc/mod.rs#L96).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/crc/mod.rs:96`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

A unique identifier for the transaction that produced this commit.
