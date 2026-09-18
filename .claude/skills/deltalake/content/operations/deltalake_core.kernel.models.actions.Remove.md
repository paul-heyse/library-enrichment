# `deltalake_core::kernel::models::actions::Remove`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.kernel.models.actions.Remove.json).

<a id="op-96ee3f7ab9ca28cce68478e7"></a>
## Remove

`struct` · `deltalake_core::kernel::models::actions::Remove` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct Remove
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L987).

Source: `crates/core/src/kernel/models/actions.rs:987`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Represents a tombstone (deleted file) in the Delta log.

<a id="op-031ce668363d6be2cddfd333"></a>
## base_row_id

`struct_field` · `deltalake_core::kernel::models::actions::Remove::base_row_id` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
base_row_id: Option<i64>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L1028).

Source: `crates/core/src/kernel/models/actions.rs:1028`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Default generated Row ID of the first row in the file. The default generated Row IDs
of the other rows in the file can be reconstructed by adding the physical index of the
row within the file to the base Row ID

<a id="op-5426f030493396e84e9d1d04"></a>
## borrow

`function` · `deltalake_core::kernel::models::actions::Remove::borrow` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn borrow(&self) -> &str
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L197).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::models::actions::Remove", "path": "crate::kernel::Remove"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [196, 1], "end": [200, 2], "filename": "crates/core/src/protocol/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "str"}}], "constraints": []}}, "id": "core::borrow::Borrow", "path": "Borrow"}, "trait_path": "core::borrow::Borrow"}`

Source: `crates/core/src/protocol/mod.rs:197`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f399be982362371bf3413f0e"></a>
## clone

`function` · `deltalake_core::kernel::models::actions::Remove::clone` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> Remove
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L985).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::models::actions::Remove", "path": "Remove"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [985, 41], "end": [985, 46], "filename": "crates/core/src/kernel/models/actions.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `crates/core/src/kernel/models/actions.rs:985`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-057a9d5362c20aee7c34ef48"></a>
## data_change

`struct_field` · `deltalake_core::kernel::models::actions::Remove::data_change` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
data_change: bool
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L998).

Source: `crates/core/src/kernel/models/actions.rs:998`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

When `false` the records in the removed file must be contained
in one or more add file actions in the same version

<a id="op-7a5a6eb9f4f795be361cd8c6"></a>
## default

`function` · `deltalake_core::kernel::models::actions::Remove::default` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn default() -> Remove
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L985).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::models::actions::Remove", "path": "Remove"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [985, 52], "end": [985, 59], "filename": "crates/core/src/kernel/models/actions.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `crates/core/src/kernel/models/actions.rs:985`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-354237684d8ad08899238ce1"></a>
## default_row_commit_version

`struct_field` · `deltalake_core::kernel::models::actions::Remove::default_row_commit_version` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
default_row_commit_version: Option<i64>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L1032).

Source: `crates/core/src/kernel/models/actions.rs:1032`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

First commit version in which an add action with the same path was committed to the table.

<a id="op-11516dcd01fabaf5bd472d57"></a>
## deletion_timestamp

`struct_field` · `deltalake_core::kernel::models::actions::Remove::deletion_timestamp` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
deletion_timestamp: Option<i64>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L1002).

Source: `crates/core/src/kernel/models/actions.rs:1002`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The time the deletion occurred, represented as milliseconds since the epoch

<a id="op-cd963848eab2e4edf126ac76"></a>
## deletion_vector

`function` · `deltalake_core::kernel::models::actions::Remove::deletion_vector` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn deletion_vector(&self) -> Option<DeletionVectorDescriptor>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/cdf/mod.rs#L166).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::models::actions::Remove", "path": "crate::kernel::Remove"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [133, 1], "end": [173, 2], "filename": "crates/core/src/delta_datafusion/cdf/mod.rs"}, "trait": {"args": null, "id": "deltalake_core::delta_datafusion::cdf::FileAction", "path": "FileAction"}, "trait_path": "deltalake_core::delta_datafusion::cdf::FileAction"}`

Source: `crates/core/src/delta_datafusion/cdf/mod.rs:166`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f281f7352ad138bcdd97b32a"></a>
## deletion_vector

`struct_field` · `deltalake_core::kernel::models::actions::Remove::deletion_vector` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
deletion_vector: Option<DeletionVectorDescriptor>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L1022).

Source: `crates/core/src/kernel/models/actions.rs:1022`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Information about deletion vector (DV) associated with this remove action

<a id="op-5a427b862b724f90fc83dca0"></a>
## deserialize

`function` · `deltalake_core::kernel::models::actions::Remove::deserialize` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L985).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::models::actions::Remove", "path": "Remove"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [985, 21], "end": [985, 32], "filename": "crates/core/src/kernel/models/actions.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `crates/core/src/kernel/models/actions.rs:985`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dfa97d56778505622cf4ad16"></a>
## eq

`function` · `deltalake_core::kernel::models::actions::Remove::eq` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eq(&self, other: &Self) -> bool
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L203).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::models::actions::Remove", "path": "crate::kernel::Remove"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [202, 1], "end": [213, 2], "filename": "crates/core/src/protocol/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `crates/core/src/protocol/mod.rs:203`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d437d7d35b42ff754a0c3260"></a>
## extended_file_metadata

`struct_field` · `deltalake_core::kernel::models::actions::Remove::extended_file_metadata` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
extended_file_metadata: Option<bool>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L1006).

Source: `crates/core/src/kernel/models/actions.rs:1006`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

When true the fields `partition_values`, `size`, and `tags` are present

<a id="op-50746fc0910b69e2544e729f"></a>
## fmt

`function` · `deltalake_core::kernel::models::actions::Remove::fmt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L985).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::models::actions::Remove", "path": "Remove"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [985, 34], "end": [985, 39], "filename": "crates/core/src/kernel/models/actions.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/core/src/kernel/models/actions.rs:985`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e33c52b25714f21bdd0a3a72"></a>
## has_deletion_vector

`function` · `deltalake_core::kernel::models::actions::Remove::has_deletion_vector` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn has_deletion_vector(&self) -> bool
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/cdf/mod.rs#L170).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::models::actions::Remove", "path": "crate::kernel::Remove"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [133, 1], "end": [173, 2], "filename": "crates/core/src/delta_datafusion/cdf/mod.rs"}, "trait": {"args": null, "id": "deltalake_core::delta_datafusion::cdf::FileAction", "path": "FileAction"}, "trait_path": "deltalake_core::delta_datafusion::cdf::FileAction"}`

Source: `crates/core/src/delta_datafusion/cdf/mod.rs:170`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-728bcd1ab5d913d6168e02cc"></a>
## hash

`function` · `deltalake_core::kernel::models::actions::Remove::hash` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn hash<H: Hasher>(&self, state: &mut H)
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L189).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::models::actions::Remove", "path": "crate::kernel::Remove"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [188, 1], "end": [192, 2], "filename": "crates/core/src/protocol/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `crates/core/src/protocol/mod.rs:189`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-600deb8952c5c03c89fa64e7"></a>
## partition_values

`function` · `deltalake_core::kernel::models::actions::Remove::partition_values` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn partition_values(&self) -> DeltaResult<&HashMap<String, Option<String>>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/cdf/mod.rs#L134).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::models::actions::Remove", "path": "crate::kernel::Remove"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [133, 1], "end": [173, 2], "filename": "crates/core/src/delta_datafusion/cdf/mod.rs"}, "trait": {"args": null, "id": "deltalake_core::delta_datafusion::cdf::FileAction", "path": "FileAction"}, "trait_path": "deltalake_core::delta_datafusion::cdf::FileAction"}`

Source: `crates/core/src/delta_datafusion/cdf/mod.rs:134`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8667e2af275d12d4df374494"></a>
## partition_values

`struct_field` · `deltalake_core::kernel::models::actions::Remove::partition_values` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
partition_values: Option<std::collections::HashMap<String, Option<String>>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L1010).

Source: `crates/core/src/kernel/models/actions.rs:1010`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

A map from partition column to value for this logical file.

<a id="op-1b4e80343b83ad7d9cabf516"></a>
## path

`struct_field` · `deltalake_core::kernel::models::actions::Remove::path` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
path: String
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L994).

Source: `crates/core/src/kernel/models/actions.rs:994`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

A relative path to a data file from the root of the table or an absolute path to a file
that should be removed from the table. The path is a URI as specified by
[RFC 2396 URI Generic Syntax], which needs to be decoded to get the data file path.

[RFC 2396 URI Generic Syntax]: https://www.ietf.org/rfc/rfc2396.txt

<a id="op-8c01a68e10300e86466dd7cf"></a>
## path

`function` · `deltalake_core::kernel::models::actions::Remove::path` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn path(&self) -> String
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/cdf/mod.rs#L148).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::models::actions::Remove", "path": "crate::kernel::Remove"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [133, 1], "end": [173, 2], "filename": "crates/core/src/delta_datafusion/cdf/mod.rs"}, "trait": {"args": null, "id": "deltalake_core::delta_datafusion::cdf::FileAction", "path": "FileAction"}, "trait_path": "deltalake_core::delta_datafusion::cdf::FileAction"}`

Source: `crates/core/src/delta_datafusion/cdf/mod.rs:148`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-29258761ec35c37d68cc78c0"></a>
## serialize

`function` · `deltalake_core::kernel::models::actions::Remove::serialize` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L985).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::models::actions::Remove", "path": "Remove"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [985, 10], "end": [985, 19], "filename": "crates/core/src/kernel/models/actions.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `crates/core/src/kernel/models/actions.rs:985`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1fce10a70f3e4e007b1c2f7b"></a>
## size

`struct_field` · `deltalake_core::kernel::models::actions::Remove::size` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
size: Option<i64>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L1014).

Source: `crates/core/src/kernel/models/actions.rs:1014`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The size of this data file in bytes

<a id="op-ef1e86a3f2d2d225228946f0"></a>
## size

`function` · `deltalake_core::kernel::models::actions::Remove::size` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn size(&self) -> DeltaResult<usize>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/cdf/mod.rs#L152).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::models::actions::Remove", "path": "crate::kernel::Remove"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [133, 1], "end": [173, 2], "filename": "crates/core/src/delta_datafusion/cdf/mod.rs"}, "trait": {"args": null, "id": "deltalake_core::delta_datafusion::cdf::FileAction", "path": "FileAction"}, "trait_path": "deltalake_core::delta_datafusion::cdf::FileAction"}`

Source: `crates/core/src/delta_datafusion/cdf/mod.rs:152`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b97a317e25870555d96b67e5"></a>
## tags

`struct_field` · `deltalake_core::kernel::models::actions::Remove::tags` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
tags: Option<std::collections::HashMap<String, Option<String>>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L1018).

Source: `crates/core/src/kernel/models/actions.rs:1018`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Map containing metadata about this logical file.
