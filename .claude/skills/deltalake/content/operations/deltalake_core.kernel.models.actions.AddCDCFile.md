# `deltalake_core::kernel::models::actions::AddCDCFile`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.kernel.models.actions.AddCDCFile.json).

<a id="op-95692480a8073956bdd6eb8f"></a>
## AddCDCFile

`struct` · `deltalake_core::kernel::models::actions::AddCDCFile` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct AddCDCFile
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L1038).

Source: `crates/core/src/kernel/models/actions.rs:1038`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Delta AddCDCFile action that describes a parquet CDC data file.

<a id="op-e7e4abd6c2d4d7bdd276fa1b"></a>
## clone

`function` · `deltalake_core::kernel::models::actions::AddCDCFile::clone` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> AddCDCFile
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L1036).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::models::actions::AddCDCFile", "path": "AddCDCFile"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1036, 34], "end": [1036, 39], "filename": "crates/core/src/kernel/models/actions.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `crates/core/src/kernel/models/actions.rs:1036`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-076de74cb813164b25e17d4d"></a>
## data_change

`struct_field` · `deltalake_core::kernel::models::actions::AddCDCFile::data_change` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
data_change: bool
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L1051).

Source: `crates/core/src/kernel/models/actions.rs:1051`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Should always be set to false because they do not change the underlying data of the table

<a id="op-b99e0e141972bdaebabd2ce1"></a>
## default

`function` · `deltalake_core::kernel::models::actions::AddCDCFile::default` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn default() -> AddCDCFile
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L1036).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::models::actions::AddCDCFile", "path": "AddCDCFile"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1036, 48], "end": [1036, 55], "filename": "crates/core/src/kernel/models/actions.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `crates/core/src/kernel/models/actions.rs:1036`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2161fa91f782a5a2c525c3de"></a>
## deletion_vector

`function` · `deltalake_core::kernel::models::actions::AddCDCFile::deletion_vector` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn deletion_vector(&self) -> Option<DeletionVectorDescriptor>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/cdf/mod.rs#L128).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::models::actions::AddCDCFile", "path": "crate::kernel::AddCDCFile"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [115, 1], "end": [131, 2], "filename": "crates/core/src/delta_datafusion/cdf/mod.rs"}, "trait": {"args": null, "id": "deltalake_core::delta_datafusion::cdf::FileAction", "path": "FileAction"}, "trait_path": "deltalake_core::delta_datafusion::cdf::FileAction"}`

Source: `crates/core/src/delta_datafusion/cdf/mod.rs:128`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9137f79bee1ef15f180ab445"></a>
## deserialize

`function` · `deltalake_core::kernel::models::actions::AddCDCFile::deserialize` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L1036).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::models::actions::AddCDCFile", "path": "AddCDCFile"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1036, 21], "end": [1036, 32], "filename": "crates/core/src/kernel/models/actions.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `crates/core/src/kernel/models/actions.rs:1036`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-00b2bf7a36587d0b24952830"></a>
## eq

`function` · `deltalake_core::kernel::models::actions::AddCDCFile::eq` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eq(&self, other: &AddCDCFile) -> bool
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L1036).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::models::actions::AddCDCFile", "path": "AddCDCFile"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1036, 57], "end": [1036, 66], "filename": "crates/core/src/kernel/models/actions.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `crates/core/src/kernel/models/actions.rs:1036`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f85167d847da37b1c94fbd92"></a>
## fmt

`function` · `deltalake_core::kernel::models::actions::AddCDCFile::fmt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L1036).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::models::actions::AddCDCFile", "path": "AddCDCFile"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1036, 41], "end": [1036, 46], "filename": "crates/core/src/kernel/models/actions.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/core/src/kernel/models/actions.rs:1036`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2dda7301d2f3196d8cdac903"></a>
## partition_values

`function` · `deltalake_core::kernel::models::actions::AddCDCFile::partition_values` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn partition_values(&self) -> DeltaResult<&HashMap<String, Option<String>>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/cdf/mod.rs#L116).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::models::actions::AddCDCFile", "path": "crate::kernel::AddCDCFile"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [115, 1], "end": [131, 2], "filename": "crates/core/src/delta_datafusion/cdf/mod.rs"}, "trait": {"args": null, "id": "deltalake_core::delta_datafusion::cdf::FileAction", "path": "FileAction"}, "trait_path": "deltalake_core::delta_datafusion::cdf::FileAction"}`

Source: `crates/core/src/delta_datafusion/cdf/mod.rs:116`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4233ae16910dcb72080aeb4e"></a>
## partition_values

`struct_field` · `deltalake_core::kernel::models::actions::AddCDCFile::partition_values` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
partition_values: std::collections::HashMap<String, Option<String>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L1048).

Source: `crates/core/src/kernel/models/actions.rs:1048`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

A map from partition column to value for this file

<a id="op-a6441dc159f05a3b2bad39a4"></a>
## path

`function` · `deltalake_core::kernel::models::actions::AddCDCFile::path` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn path(&self) -> String
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/cdf/mod.rs#L120).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::models::actions::AddCDCFile", "path": "crate::kernel::AddCDCFile"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [115, 1], "end": [131, 2], "filename": "crates/core/src/delta_datafusion/cdf/mod.rs"}, "trait": {"args": null, "id": "deltalake_core::delta_datafusion::cdf::FileAction", "path": "FileAction"}, "trait_path": "deltalake_core::delta_datafusion::cdf::FileAction"}`

Source: `crates/core/src/delta_datafusion/cdf/mod.rs:120`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e1c122da5b54644754d7f669"></a>
## path

`struct_field` · `deltalake_core::kernel::models::actions::AddCDCFile::path` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
path: String
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L1042).

Source: `crates/core/src/kernel/models/actions.rs:1042`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

A relative path, from the root of the table, or an
absolute path to a CDC file

<a id="op-03fe7fc188d0f3b7f35b0535"></a>
## serialize

`function` · `deltalake_core::kernel::models::actions::AddCDCFile::serialize` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L1036).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::models::actions::AddCDCFile", "path": "AddCDCFile"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1036, 10], "end": [1036, 19], "filename": "crates/core/src/kernel/models/actions.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `crates/core/src/kernel/models/actions.rs:1036`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-25ea884669f5b542d05b3b15"></a>
## size

`function` · `deltalake_core::kernel::models::actions::AddCDCFile::size` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn size(&self) -> DeltaResult<usize>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/cdf/mod.rs#L124).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::models::actions::AddCDCFile", "path": "crate::kernel::AddCDCFile"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [115, 1], "end": [131, 2], "filename": "crates/core/src/delta_datafusion/cdf/mod.rs"}, "trait": {"args": null, "id": "deltalake_core::delta_datafusion::cdf::FileAction", "path": "FileAction"}, "trait_path": "deltalake_core::delta_datafusion::cdf::FileAction"}`

Source: `crates/core/src/delta_datafusion/cdf/mod.rs:124`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-45bc2f0d1bb0ab7806a8bd30"></a>
## size

`struct_field` · `deltalake_core::kernel::models::actions::AddCDCFile::size` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
size: i64
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L1045).

Source: `crates/core/src/kernel/models/actions.rs:1045`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The size of this file in bytes

<a id="op-ace415d131684ced123f82ad"></a>
## tags

`struct_field` · `deltalake_core::kernel::models::actions::AddCDCFile::tags` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
tags: Option<std::collections::HashMap<String, Option<String>>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L1055).

Source: `crates/core/src/kernel/models/actions.rs:1055`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Map containing metadata about this file
