# `deltalake_core::kernel::models::actions::CommitInfo`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.kernel.models.actions.CommitInfo.json).

<a id="op-74aa5e4d95fc02a880bcd05e"></a>
## CommitInfo

`struct` · `deltalake_core::kernel::models::actions::CommitInfo` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct CommitInfo
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L1099).

Source: `crates/core/src/kernel/models/actions.rs:1099`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The commitInfo is a fairly flexible action within the delta specification, where arbitrary data can be stored.
However, the reference implementation as well as delta-rs store useful information that may for instance
allow us to be more permissive in commit conflict resolution.

<a id="op-0b79b685cc4893fb46e4bd97"></a>
## clone

`function` · `deltalake_core::kernel::models::actions::CommitInfo::clone` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> CommitInfo
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L1097).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::models::actions::CommitInfo", "path": "CommitInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1097, 41], "end": [1097, 46], "filename": "crates/core/src/kernel/models/actions.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `crates/core/src/kernel/models/actions.rs:1097`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7510ca3049baaa07d105894c"></a>
## default

`function` · `deltalake_core::kernel::models::actions::CommitInfo::default` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn default() -> CommitInfo
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L1097).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::models::actions::CommitInfo", "path": "CommitInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1097, 48], "end": [1097, 55], "filename": "crates/core/src/kernel/models/actions.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `crates/core/src/kernel/models/actions.rs:1097`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fde6f0f159215c24bae9400b"></a>
## deserialize

`function` · `deltalake_core::kernel::models::actions::CommitInfo::deserialize` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L1097).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::models::actions::CommitInfo", "path": "CommitInfo"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1097, 21], "end": [1097, 32], "filename": "crates/core/src/kernel/models/actions.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `crates/core/src/kernel/models/actions.rs:1097`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-201c2bdb89f832989faf9469"></a>
## engine_info

`struct_field` · `deltalake_core::kernel::models::actions::CommitInfo::engine_info` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
engine_info: Option<String>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L1138).

Source: `crates/core/src/kernel/models/actions.rs:1138`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Delta engine which created the commit.

<a id="op-36fe0251cfd7e965174aa26e"></a>
## eq

`function` · `deltalake_core::kernel::models::actions::CommitInfo::eq` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eq(&self, other: &CommitInfo) -> bool
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L1097).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::models::actions::CommitInfo", "path": "CommitInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1097, 57], "end": [1097, 66], "filename": "crates/core/src/kernel/models/actions.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `crates/core/src/kernel/models/actions.rs:1097`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-254ce98056fecc108beff138"></a>
## fmt

`function` · `deltalake_core::kernel::models::actions::CommitInfo::fmt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L1097).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::models::actions::CommitInfo", "path": "CommitInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1097, 34], "end": [1097, 39], "filename": "crates/core/src/kernel/models/actions.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/core/src/kernel/models/actions.rs:1097`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-416f67ca3458e5ca0bf8fef0"></a>
## in_commit_timestamp

`struct_field` · `deltalake_core::kernel::models::actions::CommitInfo::in_commit_timestamp` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
in_commit_timestamp: Option<i64>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L1106).

Source: `crates/core/src/kernel/models/actions.rs:1106`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Same as timestamp above, but cooler

<a id="op-ec7f0cf53a4c56f720d55f3e"></a>
## info

`struct_field` · `deltalake_core::kernel::models::actions::CommitInfo::info` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
info: std::collections::HashMap<String, serde_json::Value>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L1142).

Source: `crates/core/src/kernel/models/actions.rs:1142`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Additional provenance information for the commit

<a id="op-d90f57a5b79b04127cafbafc"></a>
## is_blind_append

`struct_field` · `deltalake_core::kernel::models::actions::CommitInfo::is_blind_append` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
is_blind_append: Option<bool>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L1134).

Source: `crates/core/src/kernel/models/actions.rs:1134`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

TODO

<a id="op-67078ca93cb2f5ce9764bd8c"></a>
## isolation_level

`struct_field` · `deltalake_core::kernel::models::actions::CommitInfo::isolation_level` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
isolation_level: Option<IsolationLevel>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L1130).

Source: `crates/core/src/kernel/models/actions.rs:1130`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The isolation level of the commit

<a id="op-9a64eb4d155b339679dc6ab6"></a>
## operation

`struct_field` · `deltalake_core::kernel::models::actions::CommitInfo::operation` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
operation: Option<String>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L1118).

Source: `crates/core/src/kernel/models/actions.rs:1118`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The operation performed during the

<a id="op-f741fe4a0da553f025be6647"></a>
## operation_parameters

`struct_field` · `deltalake_core::kernel::models::actions::CommitInfo::operation_parameters` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
operation_parameters: Option<std::collections::HashMap<String, serde_json::Value>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L1122).

Source: `crates/core/src/kernel/models/actions.rs:1122`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Parameters used for table operation

<a id="op-2eb0643b39c91275948ed883"></a>
## read_version

`struct_field` · `deltalake_core::kernel::models::actions::CommitInfo::read_version` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
read_version: Option<kernel::Version>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L1126).

Source: `crates/core/src/kernel/models/actions.rs:1126`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Version of the table when the operation was started

<a id="op-ec7a46d7d3814d250040835b"></a>
## serialize

`function` · `deltalake_core::kernel::models::actions::CommitInfo::serialize` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L1097).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::models::actions::CommitInfo", "path": "CommitInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1097, 10], "end": [1097, 19], "filename": "crates/core/src/kernel/models/actions.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `crates/core/src/kernel/models/actions.rs:1097`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cc0774d9fc961f3b91b522a7"></a>
## timestamp

`struct_field` · `deltalake_core::kernel::models::actions::CommitInfo::timestamp` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
timestamp: Option<i64>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L1102).

Source: `crates/core/src/kernel/models/actions.rs:1102`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Timestamp in millis when the commit was created

<a id="op-86f9f6fb010770e7fccffcdb"></a>
## user_id

`struct_field` · `deltalake_core::kernel::models::actions::CommitInfo::user_id` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
user_id: Option<String>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L1110).

Source: `crates/core/src/kernel/models/actions.rs:1110`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Id of the user invoking the commit

<a id="op-0029bb6b85a996cfd363c535"></a>
## user_metadata

`struct_field` · `deltalake_core::kernel::models::actions::CommitInfo::user_metadata` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
user_metadata: Option<String>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L1146).

Source: `crates/core/src/kernel/models/actions.rs:1146`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

User defined metadata

<a id="op-26c1d9ac57b59184e920637f"></a>
## user_name

`struct_field` · `deltalake_core::kernel::models::actions::CommitInfo::user_name` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
user_name: Option<String>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L1114).

Source: `crates/core/src/kernel/models/actions.rs:1114`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Name of the user invoking the commit
