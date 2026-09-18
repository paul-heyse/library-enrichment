# `buoyant_kernel::actions::CommitInfo`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.actions.CommitInfo.json).

<a id="op-cf1aedf15af3e50c25d55c86"></a>
## CommitInfo

`struct` · `buoyant_kernel::actions::CommitInfo` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct CommitInfo
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/mod.rs#L691).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/mod.rs:691`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-86f93fd641a2bd05bbb9b08c"></a>
## clone

`function` · `buoyant_kernel::actions::CommitInfo::clone` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> CommitInfo
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/mod.rs#L688).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::actions::CommitInfo", "path": "CommitInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [688, 17], "end": [688, 22], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/mod.rs:688`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-42d8054f94ffc9832ca98ced"></a>
## eq

`function` · `buoyant_kernel::actions::CommitInfo::eq` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eq(&self, other: &CommitInfo) -> bool
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/mod.rs#L688).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::actions::CommitInfo", "path": "CommitInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [688, 24], "end": [688, 33], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/mod.rs:688`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-aca7510ada4ffb43167fb275"></a>
## fmt

`function` · `buoyant_kernel::actions::CommitInfo::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/mod.rs#L688).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::actions::CommitInfo", "path": "CommitInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [688, 10], "end": [688, 15], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/mod.rs:688`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6c8ef1b74643cd6991dd71b8"></a>
## into_engine_data

`function` · `buoyant_kernel::actions::CommitInfo::into_engine_data` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn into_engine_data(self, schema: delta_kernel::schema::SchemaRef, engine: &dyn delta_kernel::Engine) -> delta_kernel::DeltaResult<Box<dyn delta_kernel::EngineData>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/mod.rs#L688).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::actions::CommitInfo", "path": "CommitInfo"}}, "generics": {"params": [], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::scalars::Scalar", "path": "delta_kernel::expressions::Scalar"}}}], "constraints": []}}, "id": "core::convert::TryInto", "path": "TryInto"}}}], "generic_params": [], "type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "i64"}}], "constraints": []}}, "id": "core::option::Option", "path": "Option"}}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::scalars::Scalar", "path": "delta_kernel::expressions::Scalar"}}}], "constraints": []}}, "id": "core::convert::TryInto", "path": "TryInto"}}}], "generic_params": [], "type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "alloc::string::String", "path": "String"}}}], "constraints": []}}, "id": "core::option::Option", "path": "Option"}}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::scalars::Scalar", "path": "delta_kernel::expressions::Scalar"}}}], "constraints": []}}, "id": "core::convert::TryInto", "path": "TryInto"}}}], "generic_params": [], "type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "alloc::string::String", "path": "String"}}}, {"type": {"resolved_path": {"args": null, "id": "alloc::string::String", "path": "String"}}}], "constraints": []}}, "id": "std::collections::hash::map::HashMap", "path": "std::collections::HashMap"}}}], "constraints": []}}, "id": "core::option::Option", "path": "Option"}}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::scalars::Scalar", "path": "delta_kernel::expressions::Scalar"}}}], "constraints": []}}, "id": "core::convert::TryInto", "path": "TryInto"}}}], "generic_params": [], "type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "bool"}}], "constraints": []}}, "id": "core::option::Option", "path": "Option"}}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"qualified_path": {"args": null, "name": "Error", "self_type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "i64"}}], "constraints": []}}, "id": "core::option::Option", "path": "Option"}}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::scalars::Scalar", "path": "delta_kernel::expressions::Scalar"}}}], "constraints": []}}, "id": "core::convert::TryInto", "path": "TryInto"}}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"qualified_path": {"args": null, "name": "Error", "self_type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "alloc::string::String", "path": "String"}}}], "constraints": []}}, "id": "core::option::Option", "path": "Option"}}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::scalars::Scalar", "path": "delta_kernel::expressions::Scalar"}}}], "constraints": []}}, "id": "core::convert::TryInto", "path": "TryInto"}}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"qualified_path": {"args": null, "name": "Error", "self_type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "alloc::string::String", "path": "String"}}}, {"type": {"resolved_path": {"args": null, "id": "alloc::string::String", "path": "String"}}}], "constraints": []}}, "id": "std::collections::hash::map::HashMap", "path": "std::collections::HashMap"}}}], "constraints": []}}, "id": "core::option::Option", "path": "Option"}}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::scalars::Scalar", "path": "delta_kernel::expressions::Scalar"}}}], "constraints": []}}, "id": "core::convert::TryInto", "path": "TryInto"}}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"qualified_path": {"args": null, "name": "Error", "self_type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "bool"}}], "constraints": []}}, "id": "core::option::Option", "path": "Option"}}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::scalars::Scalar", "path": "delta_kernel::expressions::Scalar"}}}], "constraints": []}}, "id": "core::convert::TryInto", "path": "TryInto"}}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}}}], "generic_params": [], "type": {"resolved_path": {"args": null, "id": "buoyant_kernel::error::Error", "path": "delta_kernel::Error"}}}}]}, "is_negative": false, "span": {"begin": [688, 49], "end": [688, 63], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/mod.rs"}, "trait": {"args": null, "id": "buoyant_kernel::IntoEngineData", "path": "IntoEngineData"}, "trait_path": "buoyant_kernel::IntoEngineData"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/mod.rs:688`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-42d9f34441efbb29e690a9c8"></a>
## to_schema

`function` · `buoyant_kernel::actions::CommitInfo::to_schema` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn to_schema() -> delta_kernel::schema::StructType
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/mod.rs#L688).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::actions::CommitInfo", "path": "CommitInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [688, 39], "end": [688, 47], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/mod.rs"}, "trait": {"args": null, "id": "buoyant_kernel::schema::ToSchema", "path": "ToSchema"}, "trait_path": "buoyant_kernel::schema::ToSchema"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/mod.rs:688`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b622a1b4b56d29336104790c"></a>
## engine_info

`struct_field` · `buoyant_kernel::actions::CommitInfo::engine_info` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
engine_info: Option<String>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/mod.rs#L714).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/mod.rs:714`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

A place for the engine to store additional metadata associated with this commit

<a id="op-9d72ed792ca7366fffcb6d40"></a>
## in_commit_timestamp

`struct_field` · `buoyant_kernel::actions::CommitInfo::in_commit_timestamp` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
in_commit_timestamp: Option<i64>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/mod.rs#L700).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/mod.rs:700`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The time this logical file was created, as milliseconds since the epoch. Unlike
`timestamp`, this field is guaranteed to be monotonically increase with each commit.
Note: If in-commit timestamps are enabled, both the following must be true:
- The `inCommitTimestamp` field must always be present in CommitInfo.
- The CommitInfo action must always be the first one in a commit.

<a id="op-0b7e40a9f1bed542c6704b5f"></a>
## is_blind_append

`struct_field` · `buoyant_kernel::actions::CommitInfo::is_blind_append` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
is_blind_append: Option<bool>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/mod.rs#L712).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/mod.rs:712`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Whether this commit is a blind append.

<a id="op-169a3ab532bb5ba32718d3a0"></a>
## kernel_version

`struct_field` · `buoyant_kernel::actions::CommitInfo::kernel_version` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
kernel_version: Option<String>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/mod.rs#L710).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/mod.rs:710`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The version of the delta_kernel crate used to write this commit. The kernel will always
write this field, but it is optional since many tables will not have this field (i.e. any
tables not written by kernel).

<a id="op-be95dc66b967cbaa61882faf"></a>
## operation

`struct_field` · `buoyant_kernel::actions::CommitInfo::operation` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
operation: Option<String>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/mod.rs#L703).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/mod.rs:703`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

An arbitrary string that identifies the operation associated with this commit. This is
specified by the engine. Read: optional, write: required (that is, kernel alwarys writes).

<a id="op-451fec2208cd32fbf487c45e"></a>
## operation_parameters

`struct_field` · `buoyant_kernel::actions::CommitInfo::operation_parameters` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
operation_parameters: Option<std::collections::HashMap<String, String>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/mod.rs#L706).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/mod.rs:706`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Map of arbitrary string key-value pairs that provide additional information about the
operation. This is specified by the engine. For now this is always empty on write.

<a id="op-0352ef9f335c0f09568c824d"></a>
## timestamp

`struct_field` · `buoyant_kernel::actions::CommitInfo::timestamp` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
timestamp: Option<i64>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/mod.rs#L694).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/mod.rs:694`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The time this logical file was created, as milliseconds since the epoch.
Read: optional, write: required (that is, kernel always writes).

<a id="op-a67f1cd239e0cf8f3c7ad87f"></a>
## txn_id

`struct_field` · `buoyant_kernel::actions::CommitInfo::txn_id` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
txn_id: Option<String>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/mod.rs#L716).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/mod.rs:716`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

A unique transaction identifier for this commit.
