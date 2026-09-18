# `deltalake_core::operations::update_table_metadata::TableMetadataUpdate`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.operations.update_table_metadata.TableMetadataUpdate.json).

<a id="op-c72dd61e75a5f083adcfe0ca"></a>
## TableMetadataUpdate

`struct` · `deltalake_core::operations::update_table_metadata::TableMetadataUpdate` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct TableMetadataUpdate
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/update_table_metadata.rs#L24).

Source: `crates/core/src/operations/update_table_metadata.rs:24`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

A validated set of metadata fields to update on a Delta table.

At least one field must be provided; lengths are validated to stay within Delta's limits.

<a id="op-2444e325593277288390d9ab"></a>
## Args

`assoc_type` · `deltalake_core::operations::update_table_metadata::TableMetadataUpdate::Args` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type Args = ()
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/update_table_metadata.rs#L16).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::update_table_metadata::TableMetadataUpdate", "path": "TableMetadataUpdate"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'v_a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [16, 62], "end": [16, 70], "filename": "crates/core/src/operations/update_table_metadata.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'v_a"}], "constraints": []}}, "id": "validator::traits::ValidateArgs", "path": "ValidateArgs"}, "trait_path": "validator::traits::ValidateArgs"}`

Source: `crates/core/src/operations/update_table_metadata.rs:16`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a179090ef7ddc87c5f3593ee"></a>
## clone

`function` · `deltalake_core::operations::update_table_metadata::TableMetadataUpdate::clone` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> TableMetadataUpdate
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/update_table_metadata.rs#L16).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::update_table_metadata::TableMetadataUpdate", "path": "TableMetadataUpdate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [16, 17], "end": [16, 22], "filename": "crates/core/src/operations/update_table_metadata.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `crates/core/src/operations/update_table_metadata.rs:16`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-85b2e316aceca560ba573e9a"></a>
## description

`struct_field` · `deltalake_core::operations::update_table_metadata::TableMetadataUpdate::description` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
description: Option<String>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/update_table_metadata.rs#L38).

Source: `crates/core/src/operations/update_table_metadata.rs:38`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

New table description. When set, must be at most 4000 characters.

<a id="op-1038ffa28fd74ed0b7021b18"></a>
## deserialize

`function` · `deltalake_core::operations::update_table_metadata::TableMetadataUpdate::deserialize` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/update_table_metadata.rs#L16).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::update_table_metadata::TableMetadataUpdate", "path": "TableMetadataUpdate"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [16, 42], "end": [16, 60], "filename": "crates/core/src/operations/update_table_metadata.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `crates/core/src/operations/update_table_metadata.rs:16`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ac16c41974c47540f1253008"></a>
## fmt

`function` · `deltalake_core::operations::update_table_metadata::TableMetadataUpdate::fmt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/update_table_metadata.rs#L16).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::update_table_metadata::TableMetadataUpdate", "path": "TableMetadataUpdate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [16, 10], "end": [16, 15], "filename": "crates/core/src/operations/update_table_metadata.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/core/src/operations/update_table_metadata.rs:16`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-11c15fd6b70f9957b42268ab"></a>
## name

`struct_field` · `deltalake_core::operations::update_table_metadata::TableMetadataUpdate::name` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
name: Option<String>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/update_table_metadata.rs#L31).

Source: `crates/core/src/operations/update_table_metadata.rs:31`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

New table name. When set, must be 1-255 characters.

<a id="op-9e2af1d209283bcd0d0314e3"></a>
## serialize

`function` · `deltalake_core::operations::update_table_metadata::TableMetadataUpdate::serialize` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/update_table_metadata.rs#L16).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::update_table_metadata::TableMetadataUpdate", "path": "TableMetadataUpdate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [16, 24], "end": [16, 40], "filename": "crates/core/src/operations/update_table_metadata.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `crates/core/src/operations/update_table_metadata.rs:16`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e1a499cb8229dae3e46a0182"></a>
## validate

`function` · `deltalake_core::operations::update_table_metadata::TableMetadataUpdate::validate` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn validate(&self) -> ::std::result::Result<(), ::validator::ValidationErrors>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/update_table_metadata.rs#L16).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::update_table_metadata::TableMetadataUpdate", "path": "TableMetadataUpdate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [16, 62], "end": [16, 70], "filename": "crates/core/src/operations/update_table_metadata.rs"}, "trait": {"args": null, "id": "validator::traits::Validate", "path": "Validate"}, "trait_path": "validator::traits::Validate"}`

Source: `crates/core/src/operations/update_table_metadata.rs:16`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-03506f626433024afbc54f21"></a>
## validate_with_args

`function` · `deltalake_core::operations::update_table_metadata::TableMetadataUpdate::validate_with_args` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn validate_with_args(&self, args: Self::Args) -> ::std::result::Result<(), ::validator::ValidationErrors>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/update_table_metadata.rs#L16).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::update_table_metadata::TableMetadataUpdate", "path": "TableMetadataUpdate"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'v_a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [16, 62], "end": [16, 70], "filename": "crates/core/src/operations/update_table_metadata.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'v_a"}], "constraints": []}}, "id": "validator::traits::ValidateArgs", "path": "ValidateArgs"}, "trait_path": "validator::traits::ValidateArgs"}`

Source: `crates/core/src/operations/update_table_metadata.rs:16`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
