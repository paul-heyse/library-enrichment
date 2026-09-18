# `datafusion_common::types::field::LogicalField`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.types.field.LogicalField.json).

<a id="op-d0134fb6bb2dac36d69f0ca9"></a>
## LogicalField

`struct` · `datafusion_common::types::field::LogicalField` · datafusion-common 55.1.0

```rust
struct LogicalField
```

Source: `src/types/field.rs:26`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

A record of a logical type, its name and its nullability.

<a id="op-dd810b9493221b79acb446c6"></a>
## clone

`function` · `datafusion_common::types::field::LogicalField::clone` · datafusion-common 55.1.0

```rust
fn clone(&self) -> LogicalField
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::types::field::LogicalField", "path": "LogicalField"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [25, 17], "end": [25, 22], "filename": "src/types/field.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/types/field.rs:25`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bc7f9795c8c6dc65ea9e3f42"></a>
## cmp

`function` · `datafusion_common::types::field::LogicalField::cmp` · datafusion-common 55.1.0

```rust
fn cmp(&self, other: &LogicalField) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::types::field::LogicalField", "path": "LogicalField"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [25, 40], "end": [25, 43], "filename": "src/types/field.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/types/field.rs:25`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-46c4fc4c5f93af8ebdc5a548"></a>
## eq

`function` · `datafusion_common::types::field::LogicalField::eq` · datafusion-common 55.1.0

```rust
fn eq(&self, other: &Self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::types::field::LogicalField", "path": "LogicalField"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [32, 1], "end": [38, 2], "filename": "src/types/field.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/types/field.rs:33`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-90c860e5be30b37a285bff5e"></a>
## fmt

`function` · `datafusion_common::types::field::LogicalField::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::types::field::LogicalField", "path": "LogicalField"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [25, 10], "end": [25, 15], "filename": "src/types/field.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/types/field.rs:25`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d417d04d62703f743d11f9cd"></a>
## from

`function` · `datafusion_common::types::field::LogicalField::from` · datafusion-common 55.1.0

```rust
fn from(value: &Field) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::types::field::LogicalField", "path": "LogicalField"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 1], "end": [56, 2], "filename": "src/types/field.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": null, "id": "arrow_schema::field::Field", "path": "Field"}}}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/types/field.rs:49`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6772413ff0801283ab90a9d8"></a>
## hash

`function` · `datafusion_common::types::field::LogicalField::hash` · datafusion-common 55.1.0

```rust
fn hash<H: Hasher>(&self, state: &mut H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::types::field::LogicalField", "path": "LogicalField"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [40, 1], "end": [46, 2], "filename": "src/types/field.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/types/field.rs:41`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1685ff6b8db2ca29491f4f84"></a>
## logical_type

`struct_field` · `datafusion_common::types::field::LogicalField::logical_type` · datafusion-common 55.1.0

```rust
logical_type: super::LogicalTypeRef
```

Source: `src/types/field.rs:28`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-eaba9a00f0054f82e9ca3d2a"></a>
## name

`struct_field` · `datafusion_common::types::field::LogicalField::name` · datafusion-common 55.1.0

```rust
name: String
```

Source: `src/types/field.rs:27`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ad3c09fff3950695845d02c6"></a>
## nullable

`struct_field` · `datafusion_common::types::field::LogicalField::nullable` · datafusion-common 55.1.0

```rust
nullable: bool
```

Source: `src/types/field.rs:29`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-36cbe8a45a531cfe81f8cd1c"></a>
## partial_cmp

`function` · `datafusion_common::types::field::LogicalField::partial_cmp` · datafusion-common 55.1.0

```rust
fn partial_cmp(&self, other: &LogicalField) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::types::field::LogicalField", "path": "LogicalField"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [25, 28], "end": [25, 38], "filename": "src/types/field.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/types/field.rs:25`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
