# `deltalake_core::kernel::models::actions::Add`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.kernel.models.actions.Add.json).

<a id="op-165aeba4e3bd6a2b7f853b9a"></a>
## Add

`struct` · `deltalake_core::kernel::models::actions::Add` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct Add
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L938).

Source: `crates/core/src/kernel/models/actions.rs:938`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Defines an add action

<a id="op-c129f5bfdce6b2bc51c9a010"></a>
## base_row_id

`struct_field` · `deltalake_core::kernel::models::actions::Add::base_row_id` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
base_row_id: Option<i64>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L975).

Source: `crates/core/src/kernel/models/actions.rs:975`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Default generated Row ID of the first row in the file. The default generated Row IDs
of the other rows in the file can be reconstructed by adding the physical index of the
row within the file to the base Row ID

<a id="op-3b1e23c4300b8e22a9e7c49a"></a>
## clone

`function` · `deltalake_core::kernel::models::actions::Add::clone` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> Add
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L935).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::models::actions::Add", "path": "Add"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [935, 41], "end": [935, 46], "filename": "crates/core/src/kernel/models/actions.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `crates/core/src/kernel/models/actions.rs:935`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-39a0e035efeca4627d8e1462"></a>
## clustering_provider

`struct_field` · `deltalake_core::kernel::models::actions::Add::clustering_provider` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
clustering_provider: Option<String>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L981).

Source: `crates/core/src/kernel/models/actions.rs:981`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The name of the clustering implementation

<a id="op-4228b07f6f88484f5ef0802b"></a>
## data_change

`struct_field` · `deltalake_core::kernel::models::actions::Add::data_change` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
data_change: bool
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L958).

Source: `crates/core/src/kernel/models/actions.rs:958`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

When `false` the logical file must already be present in the table or the records
in the added file must be contained in one or more remove actions in the same version.

<a id="op-fe272ddd2151d5b2a047df90"></a>
## default

`function` · `deltalake_core::kernel::models::actions::Add::default` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn default() -> Add
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L935).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::models::actions::Add", "path": "Add"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [935, 48], "end": [935, 55], "filename": "crates/core/src/kernel/models/actions.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `crates/core/src/kernel/models/actions.rs:935`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-108166f9f47ccb43027fc2ac"></a>
## default_row_commit_version

`struct_field` · `deltalake_core::kernel::models::actions::Add::default_row_commit_version` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
default_row_commit_version: Option<i64>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L978).

Source: `crates/core/src/kernel/models/actions.rs:978`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

First commit version in which an add action with the same path was committed to the table.

<a id="op-bb63473a5975d626c3ab0443"></a>
## deletion_vector

`function` · `deltalake_core::kernel::models::actions::Add::deletion_vector` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn deletion_vector(&self) -> Option<DeletionVectorDescriptor>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/cdf/mod.rs#L106).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::models::actions::Add", "path": "crate::kernel::Add"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [93, 1], "end": [113, 2], "filename": "crates/core/src/delta_datafusion/cdf/mod.rs"}, "trait": {"args": null, "id": "deltalake_core::delta_datafusion::cdf::FileAction", "path": "FileAction"}, "trait_path": "deltalake_core::delta_datafusion::cdf::FileAction"}`

Source: `crates/core/src/delta_datafusion/cdf/mod.rs:106`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e2ef0d89cd309443252b9a23"></a>
## deletion_vector

`struct_field` · `deltalake_core::kernel::models::actions::Add::deletion_vector` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
deletion_vector: Option<DeletionVectorDescriptor>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L970).

Source: `crates/core/src/kernel/models/actions.rs:970`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Information about deletion vector (DV) associated with this add action

<a id="op-165c40e8c08d83c322a258e3"></a>
## deserialize

`function` · `deltalake_core::kernel::models::actions::Add::deserialize` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L935).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::models::actions::Add", "path": "Add"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [935, 21], "end": [935, 32], "filename": "crates/core/src/kernel/models/actions.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `crates/core/src/kernel/models/actions.rs:935`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3b4d6f723683e6b3667d0b81"></a>
## eq

`function` · `deltalake_core::kernel::models::actions::Add::eq` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eq(&self, other: &Self) -> bool
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L158).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::models::actions::Add", "path": "crate::kernel::Add"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [157, 1], "end": [168, 2], "filename": "crates/core/src/protocol/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `crates/core/src/protocol/mod.rs:158`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e44990241908d1a0403fc98b"></a>
## fmt

`function` · `deltalake_core::kernel::models::actions::Add::fmt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L935).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::models::actions::Add", "path": "Add"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [935, 34], "end": [935, 39], "filename": "crates/core/src/kernel/models/actions.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/core/src/kernel/models/actions.rs:935`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c934123a2b45c5e7f8efe258"></a>
## get_stats

`function` · `deltalake_core::kernel::models::actions::Add::get_stats` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn get_stats(&self) -> Result<Option<Stats>, serde_json::error::Error>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L174).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::models::actions::Add", "path": "crate::kernel::Add"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [172, 1], "end": [186, 2], "filename": "crates/core/src/protocol/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/protocol/mod.rs:174`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Get whatever stats are available. Uses (parquet struct) parsed_stats if present falling back to json stats.

<a id="op-96f2fa2f883701c9357a6e37"></a>
## has_deletion_vector

`function` · `deltalake_core::kernel::models::actions::Add::has_deletion_vector` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn has_deletion_vector(&self) -> bool
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/cdf/mod.rs#L110).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::models::actions::Add", "path": "crate::kernel::Add"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [93, 1], "end": [113, 2], "filename": "crates/core/src/delta_datafusion/cdf/mod.rs"}, "trait": {"args": null, "id": "deltalake_core::delta_datafusion::cdf::FileAction", "path": "FileAction"}, "trait_path": "deltalake_core::delta_datafusion::cdf::FileAction"}`

Source: `crates/core/src/delta_datafusion/cdf/mod.rs:110`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8d9101aa4d64102f76778e3b"></a>
## hash

`function` · `deltalake_core::kernel::models::actions::Add::hash` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn hash<H: Hasher>(&self, state: &mut H)
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L152).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::models::actions::Add", "path": "crate::kernel::Add"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [151, 1], "end": [155, 2], "filename": "crates/core/src/protocol/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `crates/core/src/protocol/mod.rs:152`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4dad9c449ca44362289bb998"></a>
## modification_time

`struct_field` · `deltalake_core::kernel::models::actions::Add::modification_time` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
modification_time: i64
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L954).

Source: `crates/core/src/kernel/models/actions.rs:954`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The time this logical file was created, as milliseconds since the epoch.

<a id="op-4182795171b128918be4bea2"></a>
## partition_values

`struct_field` · `deltalake_core::kernel::models::actions::Add::partition_values` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
partition_values: std::collections::HashMap<String, Option<String>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L948).

Source: `crates/core/src/kernel/models/actions.rs:948`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

A map from partition column to value for this logical file.

<a id="op-98bcc865beeab1593c6ee720"></a>
## partition_values

`function` · `deltalake_core::kernel::models::actions::Add::partition_values` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn partition_values(&self) -> DeltaResult<&HashMap<String, Option<String>>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/cdf/mod.rs#L94).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::models::actions::Add", "path": "crate::kernel::Add"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [93, 1], "end": [113, 2], "filename": "crates/core/src/delta_datafusion/cdf/mod.rs"}, "trait": {"args": null, "id": "deltalake_core::delta_datafusion::cdf::FileAction", "path": "FileAction"}, "trait_path": "deltalake_core::delta_datafusion::cdf::FileAction"}`

Source: `crates/core/src/delta_datafusion/cdf/mod.rs:94`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-33619f09822c46fad48ede6a"></a>
## path

`struct_field` · `deltalake_core::kernel::models::actions::Add::path` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
path: String
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L945).

Source: `crates/core/src/kernel/models/actions.rs:945`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

A relative path to a data file from the root of the table or an absolute path to a file
that should be added to the table. The path is a URI as specified by
[RFC 2396 URI Generic Syntax], which needs to be decoded to get the data file path.

[RFC 2396 URI Generic Syntax]: https://www.ietf.org/rfc/rfc2396.txt

<a id="op-ee23f2f32ceec5a74e3750e5"></a>
## path

`function` · `deltalake_core::kernel::models::actions::Add::path` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn path(&self) -> String
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/cdf/mod.rs#L98).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::models::actions::Add", "path": "crate::kernel::Add"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [93, 1], "end": [113, 2], "filename": "crates/core/src/delta_datafusion/cdf/mod.rs"}, "trait": {"args": null, "id": "deltalake_core::delta_datafusion::cdf::FileAction", "path": "FileAction"}, "trait_path": "deltalake_core::delta_datafusion::cdf::FileAction"}`

Source: `crates/core/src/delta_datafusion/cdf/mod.rs:98`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-93b85a60ec7824d702436820"></a>
## serialize

`function` · `deltalake_core::kernel::models::actions::Add::serialize` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L935).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::models::actions::Add", "path": "Add"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [935, 10], "end": [935, 19], "filename": "crates/core/src/kernel/models/actions.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `crates/core/src/kernel/models/actions.rs:935`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3a30b3b362a8a3cf39729adf"></a>
## size

`function` · `deltalake_core::kernel::models::actions::Add::size` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn size(&self) -> DeltaResult<usize>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/cdf/mod.rs#L102).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::models::actions::Add", "path": "crate::kernel::Add"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [93, 1], "end": [113, 2], "filename": "crates/core/src/delta_datafusion/cdf/mod.rs"}, "trait": {"args": null, "id": "deltalake_core::delta_datafusion::cdf::FileAction", "path": "FileAction"}, "trait_path": "deltalake_core::delta_datafusion::cdf::FileAction"}`

Source: `crates/core/src/delta_datafusion/cdf/mod.rs:102`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9947e02a26d5b0a15196d953"></a>
## size

`struct_field` · `deltalake_core::kernel::models::actions::Add::size` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
size: i64
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L951).

Source: `crates/core/src/kernel/models/actions.rs:951`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The size of this data file in bytes

<a id="op-c55e740200925c984b5996f8"></a>
## stats

`struct_field` · `deltalake_core::kernel::models::actions::Add::stats` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
stats: Option<String>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L963).

Source: `crates/core/src/kernel/models/actions.rs:963`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Contains [statistics] (e.g., count, min/max values for columns) about the data in this logical file.

[statistics]: https://github.com/delta-io/delta/blob/master/PROTOCOL.md#Per-file-Statistics

<a id="op-3ad5531a00a4b901ec4f4538"></a>
## tags

`struct_field` · `deltalake_core::kernel::models::actions::Add::tags` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
tags: Option<std::collections::HashMap<String, Option<String>>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L966).

Source: `crates/core/src/kernel/models/actions.rs:966`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Map containing metadata about this logical file.
