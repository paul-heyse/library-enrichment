# `buoyant_kernel::actions::Cdc`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.actions.Cdc.json).

<a id="op-c19865a19b05e68cf9a56c83"></a>
## Cdc

`struct` · `buoyant_kernel::actions::Cdc` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct Cdc
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/mod.rs#L886).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/mod.rs:886`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ed3561aac753641ed586d669"></a>
## clone

`function` · `buoyant_kernel::actions::Cdc::clone` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> Cdc
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/mod.rs#L883).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::actions::Cdc", "path": "Cdc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [883, 17], "end": [883, 22], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/mod.rs:883`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-14382322ebee9fbc2b405b3f"></a>
## data_change

`struct_field` · `buoyant_kernel::actions::Cdc::data_change` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
data_change: bool
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/mod.rs#L912).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/mod.rs:912`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

When `false` the logical file must already be present in the table or the records
in the added file must be contained in one or more remove actions in the same version.

Should always be set to false for `cdc` actions because they *do not* change the underlying
data of the table

<a id="op-4ff2e25d621a74a6d03ab17f"></a>
## eq

`function` · `buoyant_kernel::actions::Cdc::eq` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eq(&self, other: &Cdc) -> bool
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/mod.rs#L883).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::actions::Cdc", "path": "Cdc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [883, 24], "end": [883, 33], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/mod.rs:883`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-701cbe62ad41e5b28c9923a0"></a>
## fmt

`function` · `buoyant_kernel::actions::Cdc::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/mod.rs#L883).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::actions::Cdc", "path": "Cdc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [883, 10], "end": [883, 15], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/mod.rs:883`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5be7f059ec3b0af1638f5b65"></a>
## partition_values

`struct_field` · `buoyant_kernel::actions::Cdc::partition_values` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
partition_values: std::collections::HashMap<String, String>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/mod.rs#L902).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/mod.rs:902`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

A map from partition column to value for this logical file. This map can contain null in
the values meaning a partition is null. We drop those values from this map, due to the
`allow_null_container_values` annotation allowing them and because [`materialize`] drops
null values. This means an engine can assume that if a partition is found in
[`Metadata::partition_columns`](../operations/buoyant_kernel.actions.Metadata.md#op-829098d8bb37f949b16402ad) but not in this map, its value is null.

[`materialize`]: crate::engine_data::MapItem::materialize

<a id="op-ebc0dca554e9c9a137bc9c7c"></a>
## path

`struct_field` · `buoyant_kernel::actions::Cdc::path` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
path: String
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/mod.rs#L892).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/mod.rs:892`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

A relative path to a change data file from the root of the table or an absolute path to a
change data file that should be added to the table. The path is a URI as specified by
[RFC 2396 URI Generic Syntax], which needs to be decoded to get the file path.

[RFC 2396 URI Generic Syntax]: https://www.ietf.org/rfc/rfc2396.txt

<a id="op-98597e74ea16ba5830489d39"></a>
## size

`struct_field` · `buoyant_kernel::actions::Cdc::size` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
size: i64
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/mod.rs#L905).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/mod.rs:905`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The size of this cdc file in bytes

<a id="op-70c8b28e5b7bade5bf256333"></a>
## tags

`struct_field` · `buoyant_kernel::actions::Cdc::tags` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
tags: Option<std::collections::HashMap<String, String>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/mod.rs#L916).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/mod.rs:916`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Map containing metadata about this logical file. Values can be null.

<a id="op-ec3d0510234e748b9c02df87"></a>
## to_schema

`function` · `buoyant_kernel::actions::Cdc::to_schema` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn to_schema() -> delta_kernel::schema::StructType
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/mod.rs#L883).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::actions::Cdc", "path": "Cdc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [883, 39], "end": [883, 47], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/mod.rs"}, "trait": {"args": null, "id": "buoyant_kernel::schema::ToSchema", "path": "ToSchema"}, "trait_path": "buoyant_kernel::schema::ToSchema"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/mod.rs:883`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.
