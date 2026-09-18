# `datafusion_common::types::field::LogicalFields`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.types.field.LogicalFields.json).

<a id="op-08e0f0d2fb6acf7b41f584e9"></a>
## LogicalFields

`struct` · `datafusion_common::types::field::LogicalFields` · datafusion-common 55.1.0

```rust
struct LogicalFields
```

Source: `src/types/field.rs:63`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

A cheaply cloneable, owned collection of [`LogicalFieldRef`](../operations/datafusion_common.types.field.LogicalFieldRef.md#op-cfdcac04adfc685c41911798).

<a id="op-02e59e2ef1dd7460ea8a176f"></a>
## Target

`assoc_type` · `datafusion_common::types::field::LogicalFields::Target` · datafusion-common 55.1.0

```rust
Target
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::types::field::LogicalFields", "path": "LogicalFields"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [65, 1], "end": [71, 2], "filename": "src/types/field.rs"}, "trait": {"args": null, "id": "core::ops::deref::Deref", "path": "Deref"}, "trait_path": "core::ops::deref::Deref"}`

Source: `src/types/field.rs:66`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ef0f062a473affd608336065"></a>
## clone

`function` · `datafusion_common::types::field::LogicalFields::clone` · datafusion-common 55.1.0

```rust
fn clone(&self) -> LogicalFields
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::types::field::LogicalFields", "path": "LogicalFields"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [62, 17], "end": [62, 22], "filename": "src/types/field.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/types/field.rs:62`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-10a57c640da00868f636dfec"></a>
## cmp

`function` · `datafusion_common::types::field::LogicalFields::cmp` · datafusion-common 55.1.0

```rust
fn cmp(&self, other: &LogicalFields) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::types::field::LogicalFields", "path": "LogicalFields"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [62, 57], "end": [62, 60], "filename": "src/types/field.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/types/field.rs:62`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ea1c36546d57019f20782faf"></a>
## deref

`function` · `datafusion_common::types::field::LogicalFields::deref` · datafusion-common 55.1.0

```rust
fn deref(&self) -> &Self::Target
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::types::field::LogicalFields", "path": "LogicalFields"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [65, 1], "end": [71, 2], "filename": "src/types/field.rs"}, "trait": {"args": null, "id": "core::ops::deref::Deref", "path": "Deref"}, "trait_path": "core::ops::deref::Deref"}`

Source: `src/types/field.rs:68`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-32c60f6d6f2080d675a893c5"></a>
## eq

`function` · `datafusion_common::types::field::LogicalFields::eq` · datafusion-common 55.1.0

```rust
fn eq(&self, other: &LogicalFields) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::types::field::LogicalFields", "path": "LogicalFields"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [62, 24], "end": [62, 33], "filename": "src/types/field.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/types/field.rs:62`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2188aec8cf85eda13d40764f"></a>
## fmt

`function` · `datafusion_common::types::field::LogicalFields::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::types::field::LogicalFields", "path": "LogicalFields"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [62, 10], "end": [62, 15], "filename": "src/types/field.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/types/field.rs:62`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-57a53a82238c0e7a3e4dbd14"></a>
## from

`function` · `datafusion_common::types::field::LogicalFields::from` · datafusion-common 55.1.0

```rust
fn from(value: &Fields) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::types::field::LogicalFields", "path": "LogicalFields"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [73, 1], "end": [80, 2], "filename": "src/types/field.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": null, "id": "arrow_schema::fields::Fields", "path": "Fields"}}}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/types/field.rs:74`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d849f3b8b0d4fe71aacf877e"></a>
## from_iter

`function` · `datafusion_common::types::field::LogicalFields::from_iter` · datafusion-common 55.1.0

```rust
fn from_iter<T: IntoIterator<Item = LogicalFieldRef>>(iter: T) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::types::field::LogicalFields", "path": "LogicalFields"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [82, 1], "end": [86, 2], "filename": "src/types/field.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "datafusion_common::types::field::LogicalField", "path": "LogicalField"}}}], "constraints": []}}, "id": "alloc::sync::Arc", "path": "Arc"}}}], "constraints": []}}, "id": "core::iter::traits::collect::FromIterator", "path": "FromIterator"}, "trait_path": "core::iter::traits::collect::FromIterator"}`

Source: `src/types/field.rs:83`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a9fc8fef12f30bfd78b5c880"></a>
## hash

`function` · `datafusion_common::types::field::LogicalFields::hash` · datafusion-common 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::types::field::LogicalFields", "path": "LogicalFields"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [62, 39], "end": [62, 43], "filename": "src/types/field.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/types/field.rs:62`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e28295e340e71a1aaadd42a9"></a>
## partial_cmp

`function` · `datafusion_common::types::field::LogicalFields::partial_cmp` · datafusion-common 55.1.0

```rust
fn partial_cmp(&self, other: &LogicalFields) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::types::field::LogicalFields", "path": "LogicalFields"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [62, 45], "end": [62, 55], "filename": "src/types/field.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/types/field.rs:62`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
