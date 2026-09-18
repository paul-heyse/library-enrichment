# `deltalake_core::delta_datafusion::table_provider::next::MissingSelectedFilePolicy`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.delta_datafusion.table_provider.next.MissingSelectedFilePolicy.json).

<a id="op-29df79f11ef845c5b7c06089"></a>
## MissingSelectedFilePolicy

`enum` · `deltalake_core::delta_datafusion::table_provider::next::MissingSelectedFilePolicy` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
enum MissingSelectedFilePolicy
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider/next/mod.rs#L68).

Source: `crates/core/src/delta_datafusion/table_provider/next/mod.rs:68`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Policy for selected files that are not active in a scan snapshot.

<a id="op-0c29ef37527b04af9b07dc91"></a>
## Error

`variant` · `deltalake_core::delta_datafusion::table_provider::next::MissingSelectedFilePolicy::Error` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Error
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider/next/mod.rs#L71).

Source: `crates/core/src/delta_datafusion/table_provider/next/mod.rs:71`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Return an error when a selected file is not active in the scan snapshot.

<a id="op-32653216dfc55154bf74f90f"></a>
## Ignore

`variant` · `deltalake_core::delta_datafusion::table_provider::next::MissingSelectedFilePolicy::Ignore` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Ignore
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider/next/mod.rs#L79).

Source: `crates/core/src/delta_datafusion/table_provider/next/mod.rs:79`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Skip selected files that are not active in the scan snapshot.

Malformed paths, paths outside the table root, protocol failures, object store failures,
and query planning failures still return errors.

Useful for maintenance rewrites where removed files may be skipped. Use
[`MissingSelectedFilePolicy::Error`](../operations/deltalake_core.delta_datafusion.table_provider.next.MissingSelectedFilePolicy.md#op-0c29ef37527b04af9b07dc91) when every selected file must be present.

<a id="op-f8a93adaf5ec3dc89d468405"></a>
## clone

`function` · `deltalake_core::delta_datafusion::table_provider::next::MissingSelectedFilePolicy::clone` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> MissingSelectedFilePolicy
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider/next/mod.rs#L67).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::table_provider::next::MissingSelectedFilePolicy", "path": "MissingSelectedFilePolicy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [67, 10], "end": [67, 15], "filename": "crates/core/src/delta_datafusion/table_provider/next/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `crates/core/src/delta_datafusion/table_provider/next/mod.rs:67`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-88ae6eb530fc109fc03589d5"></a>
## default

`function` · `deltalake_core::delta_datafusion::table_provider::next::MissingSelectedFilePolicy::default` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn default() -> MissingSelectedFilePolicy
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider/next/mod.rs#L67).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::table_provider::next::MissingSelectedFilePolicy", "path": "MissingSelectedFilePolicy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [67, 30], "end": [67, 37], "filename": "crates/core/src/delta_datafusion/table_provider/next/mod.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `crates/core/src/delta_datafusion/table_provider/next/mod.rs:67`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e40b038dcfc467949e19c464"></a>
## deserialize

`function` · `deltalake_core::delta_datafusion::table_provider::next::MissingSelectedFilePolicy::deserialize` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider/next/mod.rs#L67).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::table_provider::next::MissingSelectedFilePolicy", "path": "MissingSelectedFilePolicy"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [67, 50], "end": [67, 61], "filename": "crates/core/src/delta_datafusion/table_provider/next/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `crates/core/src/delta_datafusion/table_provider/next/mod.rs:67`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-13aee374f314f122481e2071"></a>
## eq

`function` · `deltalake_core::delta_datafusion::table_provider::next::MissingSelectedFilePolicy::eq` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eq(&self, other: &MissingSelectedFilePolicy) -> bool
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider/next/mod.rs#L67).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::table_provider::next::MissingSelectedFilePolicy", "path": "MissingSelectedFilePolicy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [67, 63], "end": [67, 72], "filename": "crates/core/src/delta_datafusion/table_provider/next/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `crates/core/src/delta_datafusion/table_provider/next/mod.rs:67`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-67a2e0b298723edab1566c42"></a>
## fmt

`function` · `deltalake_core::delta_datafusion::table_provider::next::MissingSelectedFilePolicy::fmt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider/next/mod.rs#L67).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::table_provider::next::MissingSelectedFilePolicy", "path": "MissingSelectedFilePolicy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [67, 23], "end": [67, 28], "filename": "crates/core/src/delta_datafusion/table_provider/next/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/core/src/delta_datafusion/table_provider/next/mod.rs:67`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7b409ed804d4e8e66e05e1ae"></a>
## serialize

`function` · `deltalake_core::delta_datafusion::table_provider::next::MissingSelectedFilePolicy::serialize` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider/next/mod.rs#L67).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::table_provider::next::MissingSelectedFilePolicy", "path": "MissingSelectedFilePolicy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [67, 39], "end": [67, 48], "filename": "crates/core/src/delta_datafusion/table_provider/next/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `crates/core/src/delta_datafusion/table_provider/next/mod.rs:67`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
