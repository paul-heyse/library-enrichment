# `buoyant_kernel::actions::Add`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.actions.Add.json).

<a id="op-84c3d1000106eb776de85180"></a>
## Add

`struct` · `buoyant_kernel::actions::Add` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct Add
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/mod.rs#L747).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/mod.rs:747`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e7fd2b4e4b9abe0a4f07aaf6"></a>
## base_row_id

`struct_field` · `buoyant_kernel::actions::Add::base_row_id` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
base_row_id: Option<i64>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/mod.rs#L799).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/mod.rs:799`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Default generated Row ID of the first row in the file. The default generated Row IDs
of the other rows in the file can be reconstructed by adding the physical index of the
row within the file to the base Row ID.

<a id="op-ce0edf9b70eeac9f13d88c0b"></a>
## clone

`function` · `buoyant_kernel::actions::Add::clone` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> Add
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/mod.rs#L740).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::actions::Add", "path": "Add"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [740, 17], "end": [740, 22], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/mod.rs:740`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ae715982b9561e11a15e974f"></a>
## clustering_provider

`struct_field` · `buoyant_kernel::actions::Add::clustering_provider` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
clustering_provider: Option<String>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/mod.rs#L807).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/mod.rs:807`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The name of the clustering implementation

<a id="op-1610efd89a81ad901cea0eed"></a>
## default_row_commit_version

`struct_field` · `buoyant_kernel::actions::Add::default_row_commit_version` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
default_row_commit_version: Option<i64>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/mod.rs#L803).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/mod.rs:803`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

First commit version in which an add action with the same path was committed to the table.

<a id="op-8c47ae1de6d50782f67462a9"></a>
## deletion_vector

`struct_field` · `buoyant_kernel::actions::Add::deletion_vector` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
deletion_vector: Option<self::deletion_vector::DeletionVectorDescriptor>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/mod.rs#L793).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/mod.rs:793`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Information about deletion vector (DV) associated with this add action

<a id="op-96f37fce14aff688a9dfb4f8"></a>
## dv_unique_id

`function` · `buoyant_kernel::actions::Add::dv_unique_id` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn dv_unique_id(&self) -> Option<String>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/mod.rs#L813).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::actions::Add", "path": "Add"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [810, 1], "end": [816, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/mod.rs:813`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-95f01a6df448cde3957bd7a5"></a>
## eq

`function` · `buoyant_kernel::actions::Add::eq` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eq(&self, other: &Add) -> bool
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/mod.rs#L740).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::actions::Add", "path": "Add"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [740, 24], "end": [740, 33], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/mod.rs:740`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ae9a41319a9171d1c7f2b920"></a>
## fmt

`function` · `buoyant_kernel::actions::Add::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/mod.rs#L740).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::actions::Add", "path": "Add"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [740, 10], "end": [740, 15], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/mod.rs:740`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d4df2a39cef80daad79a31c6"></a>
## stats

`struct_field` · `buoyant_kernel::actions::Add::stats` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
stats: Option<String>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/mod.rs#L780).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/mod.rs:780`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Contains [statistics] (e.g., count, min/max values for columns) about the data in this
logical file encoded as a JSON string.

[statistics]: https://github.com/delta-io/delta/blob/master/PROTOCOL.md#Per-file-Statistics

<a id="op-dbccc1cf4f34b722aa92fd0b"></a>
## tags

`struct_field` · `buoyant_kernel::actions::Add::tags` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
tags: Option<std::collections::HashMap<String, Option<String>>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/mod.rs#L789).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/mod.rs:789`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Map containing metadata about this logical file.
Note: map values can be null.
We don't use `#[allow_null_container_values]` here because [`MapItem::materialize`]
drops null values when that attribute is present.

[`MapItem::materialize`]: crate::engine_data::MapItem::materialize

<a id="op-3b6762039d86ca9d422c5b17"></a>
## to_schema

`function` · `buoyant_kernel::actions::Add::to_schema` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn to_schema() -> delta_kernel::schema::StructType
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/mod.rs#L740).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::actions::Add", "path": "Add"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [740, 39], "end": [740, 47], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/mod.rs"}, "trait": {"args": null, "id": "buoyant_kernel::schema::ToSchema", "path": "ToSchema"}, "trait_path": "buoyant_kernel::schema::ToSchema"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/mod.rs:740`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e21abc3fe4aeb9c2d0b0a4f4"></a>
## data_change

`struct_field` · `buoyant_kernel::actions::Add::data_change` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
data_change: bool
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/mod.rs#L773).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/mod.rs:773`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

When `false` the logical file must already be present in the table or the records
in the added file must be contained in one or more remove actions in the same version.

<a id="op-ae15c016a72d29369e8da1a9"></a>
## modification_time

`struct_field` · `buoyant_kernel::actions::Add::modification_time` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
modification_time: i64
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/mod.rs#L769).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/mod.rs:769`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The time this logical file was created, as milliseconds since the epoch.

<a id="op-010eabf334d50e2155443ce5"></a>
## partition_values

`struct_field` · `buoyant_kernel::actions::Add::partition_values` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
partition_values: std::collections::HashMap<String, String>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/mod.rs#L763).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/mod.rs:763`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

A map from partition column to value for this logical file. This map can contain null in
the values meaning a partition is null. We drop those values from this map, due to the
`allow_null_container_values` annotation allowing them and because [`materialize`] drops
null values. This means an engine can assume that if a partition is found in
[`Metadata::partition_columns`](../operations/buoyant_kernel.actions.Metadata.md#op-de78cf3a06826a729aeb3b7f) but not in this map, its value is null.

[`materialize`]: crate::engine_data::MapItem::materialize

Unresolved upstream links (retained, not inferred): `crate::engine_data::MapItem::materialize`.

<a id="op-b6dce176543422ba171573ed"></a>
## path

`struct_field` · `buoyant_kernel::actions::Add::path` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
path: String
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/mod.rs#L753).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/mod.rs:753`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

A relative path to a data file from the root of the table or an absolute path to a file
that should be added to the table. The path is a URI as specified by
[RFC 2396 URI Generic Syntax], which needs to be decoded to get the data file path.

[RFC 2396 URI Generic Syntax]: https://www.ietf.org/rfc/rfc2396.txt

<a id="op-52863e6d06df88769e1877c1"></a>
## size

`struct_field` · `buoyant_kernel::actions::Add::size` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
size: i64
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/mod.rs#L766).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/mod.rs:766`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The size of this data file in bytes
