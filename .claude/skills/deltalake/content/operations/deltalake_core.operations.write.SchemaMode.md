# `deltalake_core::operations::write::SchemaMode`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.operations.write.SchemaMode.json).

<a id="op-d1bf43766000f07558d3ef98"></a>
## SchemaMode

`enum` · `deltalake_core::operations::write::SchemaMode` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
enum SchemaMode
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/write/mod.rs#L109).

Source: `crates/core/src/operations/write/mod.rs:109`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Specifies how to handle schema drifts

<a id="op-d4f1bf13b14bcce010416b4c"></a>
## Err

`assoc_type` · `deltalake_core::operations::write::SchemaMode::Err` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type Err = DeltaTableError
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/write/mod.rs#L117).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::write::SchemaMode", "path": "SchemaMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [116, 1], "end": [128, 2], "filename": "crates/core/src/operations/write/mod.rs"}, "trait": {"args": null, "id": "core::str::traits::FromStr", "path": "FromStr"}, "trait_path": "core::str::traits::FromStr"}`

Source: `crates/core/src/operations/write/mod.rs:117`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0b6759b82065ac3a654f8c7c"></a>
## Merge

`variant` · `deltalake_core::operations::write::SchemaMode::Merge` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Merge
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/write/mod.rs#L113).

Source: `crates/core/src/operations/write/mod.rs:113`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Append the new schema to the existing schema

<a id="op-b8723d5a5c4ce95b34a661db"></a>
## Overwrite

`variant` · `deltalake_core::operations::write::SchemaMode::Overwrite` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Overwrite
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/write/mod.rs#L111).

Source: `crates/core/src/operations/write/mod.rs:111`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Overwrite the schema with the new schema

<a id="op-45dcfdf2cb34fcbe9319f770"></a>
## clone

`function` · `deltalake_core::operations::write::SchemaMode::clone` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> SchemaMode
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/write/mod.rs#L108).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::write::SchemaMode", "path": "SchemaMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [108, 21], "end": [108, 26], "filename": "crates/core/src/operations/write/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `crates/core/src/operations/write/mod.rs:108`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-509e5252b0f237af3f0cfc2a"></a>
## eq

`function` · `deltalake_core::operations::write::SchemaMode::eq` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eq(&self, other: &SchemaMode) -> bool
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/write/mod.rs#L108).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::write::SchemaMode", "path": "SchemaMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [108, 10], "end": [108, 19], "filename": "crates/core/src/operations/write/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `crates/core/src/operations/write/mod.rs:108`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cc7d205af4e7211ee1f7081b"></a>
## from_str

`function` · `deltalake_core::operations::write::SchemaMode::from_str` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn from_str(s: &str) -> DeltaResult<Self>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/write/mod.rs#L119).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::write::SchemaMode", "path": "SchemaMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [116, 1], "end": [128, 2], "filename": "crates/core/src/operations/write/mod.rs"}, "trait": {"args": null, "id": "core::str::traits::FromStr", "path": "FromStr"}, "trait_path": "core::str::traits::FromStr"}`

Source: `crates/core/src/operations/write/mod.rs:119`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
