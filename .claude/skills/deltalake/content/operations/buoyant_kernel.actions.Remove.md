# `buoyant_kernel::actions::Remove`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.actions.Remove.json).

<a id="op-c773aa336d006787bca7934e"></a>
## Remove

`struct` · `buoyant_kernel::actions::Remove` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct Remove
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/mod.rs#L821).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/mod.rs:821`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cda779f40ae0c33c0c1ed3d4"></a>
## clone

`function` · `buoyant_kernel::actions::Remove::clone` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> Remove
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/mod.rs#L818).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::actions::Remove", "path": "Remove"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [818, 17], "end": [818, 22], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/mod.rs:818`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9dc072ace1d318a203a0ef92"></a>
## eq

`function` · `buoyant_kernel::actions::Remove::eq` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eq(&self, other: &Remove) -> bool
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/mod.rs#L818).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::actions::Remove", "path": "Remove"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [818, 24], "end": [818, 33], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/mod.rs:818`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-994ef3e99bd6968d08c71499"></a>
## fmt

`function` · `buoyant_kernel::actions::Remove::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/mod.rs#L818).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::actions::Remove", "path": "Remove"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [818, 10], "end": [818, 15], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/mod.rs:818`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c4f84d9c587a25c4dd50cea5"></a>
## stats

`struct_field` · `buoyant_kernel::actions::Remove::stats` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
stats: Option<String>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/mod.rs#L861).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/mod.rs:861`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Contains [statistics] (e.g., count, min/max values for columns) about the data in this
logical file encoded as a JSON string.

[statistics]: https://github.com/delta-io/delta/blob/master/PROTOCOL.md#Per-file-Statistics

<a id="op-ba8706253a627218b985943f"></a>
## to_schema

`function` · `buoyant_kernel::actions::Remove::to_schema` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn to_schema() -> delta_kernel::schema::StructType
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/mod.rs#L818).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::actions::Remove", "path": "Remove"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [818, 39], "end": [818, 47], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/mod.rs"}, "trait": {"args": null, "id": "buoyant_kernel::schema::ToSchema", "path": "ToSchema"}, "trait_path": "buoyant_kernel::schema::ToSchema"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/mod.rs:818`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e558079b4c1aa63e7257bd50"></a>
## base_row_id

`struct_field` · `buoyant_kernel::actions::Remove::base_row_id` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
base_row_id: Option<i64>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/mod.rs#L876).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/mod.rs:876`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Default generated Row ID of the first row in the file. The default generated Row IDs
of the other rows in the file can be reconstructed by adding the physical index of the
row within the file to the base Row ID

<a id="op-e50003833f689dfda44d67e2"></a>
## data_change

`struct_field` · `buoyant_kernel::actions::Remove::data_change` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
data_change: bool
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/mod.rs#L835).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/mod.rs:835`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

When `false` the logical file must already be present in the table or the records
in the added file must be contained in one or more remove actions in the same version.

<a id="op-2724fa9e1f0bcba47476a282"></a>
## default_row_commit_version

`struct_field` · `buoyant_kernel::actions::Remove::default_row_commit_version` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
default_row_commit_version: Option<i64>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/mod.rs#L880).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/mod.rs:880`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

First commit version in which an add action with the same path was committed to the table.

<a id="op-6236f80fa07b17cfc309fa02"></a>
## deletion_timestamp

`struct_field` · `buoyant_kernel::actions::Remove::deletion_timestamp` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
deletion_timestamp: Option<i64>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/mod.rs#L831).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/mod.rs:831`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The time this logical file was created, as milliseconds since the epoch.

<a id="op-7af1f1c11ea74150872a5a99"></a>
## deletion_vector

`struct_field` · `buoyant_kernel::actions::Remove::deletion_vector` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
deletion_vector: Option<self::deletion_vector::DeletionVectorDescriptor>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/mod.rs#L870).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/mod.rs:870`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Information about deletion vector (DV) associated with this add action

<a id="op-c592e38bd9dd5af75183ab91"></a>
## extended_file_metadata

`struct_field` · `buoyant_kernel::actions::Remove::extended_file_metadata` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
extended_file_metadata: Option<bool>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/mod.rs#L839).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/mod.rs:839`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

When true the fields `partition_values`, `size`, and `tags` are present

<a id="op-eb583889bcc87f57ebd95f95"></a>
## partition_values

`struct_field` · `buoyant_kernel::actions::Remove::partition_values` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
partition_values: Option<std::collections::HashMap<String, String>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/mod.rs#L850).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/mod.rs:850`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

A map from partition column to value for this logical file. This map can contain null in
the values meaning a partition is null. We drop those values from this map, due to the
`allow_null_container_values` annotation allowing them and because [`materialize`] drops
null values. This means an engine can assume that if a partition is found in
[`Metadata::partition_columns`](../operations/buoyant_kernel.actions.Metadata.md#op-de78cf3a06826a729aeb3b7f) but not in this map, its value is null.

[`materialize`]: crate::engine_data::EngineMap::materialize

<a id="op-d9b73500a0cd54a98be8afaf"></a>
## path

`struct_field` · `buoyant_kernel::actions::Remove::path` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
path: String
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/mod.rs#L827).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/mod.rs:827`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

A relative path to a data file from the root of the table or an absolute path to a file
that should be added to the table. The path is a URI as specified by
[RFC 2396 URI Generic Syntax], which needs to be decoded to get the data file path.

[RFC 2396 URI Generic Syntax]: https://www.ietf.org/rfc/rfc2396.txt

<a id="op-66e68d06c469077fd8527392"></a>
## size

`struct_field` · `buoyant_kernel::actions::Remove::size` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
size: Option<i64>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/mod.rs#L854).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/mod.rs:854`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The size of this data file in bytes

<a id="op-e38de1f5ba295bcbcbe8c87f"></a>
## tags

`struct_field` · `buoyant_kernel::actions::Remove::tags` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
tags: Option<std::collections::HashMap<String, String>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/mod.rs#L866).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/mod.rs:866`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Map containing metadata about this logical file. Values can be null.
