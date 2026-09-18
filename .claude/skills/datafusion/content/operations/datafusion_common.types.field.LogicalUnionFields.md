# `datafusion_common::types::field::LogicalUnionFields`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.types.field.LogicalUnionFields.json).

<a id="op-44ad4e9b8492e7fcb38b5d92"></a>
## LogicalUnionFields

`struct` · `datafusion_common::types::field::LogicalUnionFields` · datafusion-common 55.1.0

```rust
struct LogicalUnionFields
```

Source: `src/types/field.rs:91`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

A cheaply cloneable, owned collection of [`LogicalFieldRef`](../operations/datafusion_common.types.field.LogicalFieldRef.md#op-cfdcac04adfc685c41911798) and their
corresponding type ids.

<a id="op-dac3bd7677257f3ddae2e761"></a>
## Target

`assoc_type` · `datafusion_common::types::field::LogicalUnionFields::Target` · datafusion-common 55.1.0

```rust
Target
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::types::field::LogicalUnionFields", "path": "LogicalUnionFields"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [93, 1], "end": [99, 2], "filename": "src/types/field.rs"}, "trait": {"args": null, "id": "core::ops::deref::Deref", "path": "Deref"}, "trait_path": "core::ops::deref::Deref"}`

Source: `src/types/field.rs:94`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e7d2f0dfb2d3cb1e36ebae24"></a>
## clone

`function` · `datafusion_common::types::field::LogicalUnionFields::clone` · datafusion-common 55.1.0

```rust
fn clone(&self) -> LogicalUnionFields
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::types::field::LogicalUnionFields", "path": "LogicalUnionFields"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [90, 17], "end": [90, 22], "filename": "src/types/field.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/types/field.rs:90`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ecf6c7fa8b4d5847042a4936"></a>
## cmp

`function` · `datafusion_common::types::field::LogicalUnionFields::cmp` · datafusion-common 55.1.0

```rust
fn cmp(&self, other: &LogicalUnionFields) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::types::field::LogicalUnionFields", "path": "LogicalUnionFields"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [90, 57], "end": [90, 60], "filename": "src/types/field.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/types/field.rs:90`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b42f8228b6e0083c26a69e9c"></a>
## deref

`function` · `datafusion_common::types::field::LogicalUnionFields::deref` · datafusion-common 55.1.0

```rust
fn deref(&self) -> &Self::Target
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::types::field::LogicalUnionFields", "path": "LogicalUnionFields"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [93, 1], "end": [99, 2], "filename": "src/types/field.rs"}, "trait": {"args": null, "id": "core::ops::deref::Deref", "path": "Deref"}, "trait_path": "core::ops::deref::Deref"}`

Source: `src/types/field.rs:96`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cab5aa74aa8a625cd31a7e05"></a>
## eq

`function` · `datafusion_common::types::field::LogicalUnionFields::eq` · datafusion-common 55.1.0

```rust
fn eq(&self, other: &LogicalUnionFields) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::types::field::LogicalUnionFields", "path": "LogicalUnionFields"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [90, 24], "end": [90, 33], "filename": "src/types/field.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/types/field.rs:90`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-922e83358ea3fc9457ff32c1"></a>
## fmt

`function` · `datafusion_common::types::field::LogicalUnionFields::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::types::field::LogicalUnionFields", "path": "LogicalUnionFields"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [90, 10], "end": [90, 15], "filename": "src/types/field.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/types/field.rs:90`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f2c49ae45358fd9d2b462481"></a>
## from

`function` · `datafusion_common::types::field::LogicalUnionFields::from` · datafusion-common 55.1.0

```rust
fn from(value: &UnionFields) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::types::field::LogicalUnionFields", "path": "LogicalUnionFields"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [101, 1], "end": [108, 2], "filename": "src/types/field.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": null, "id": "arrow_schema::fields::UnionFields", "path": "UnionFields"}}}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/types/field.rs:102`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8fbd5f39848f277c7f017d4b"></a>
## from_iter

`function` · `datafusion_common::types::field::LogicalUnionFields::from_iter` · datafusion-common 55.1.0

```rust
fn from_iter<T: IntoIterator<Item = (i8, LogicalFieldRef)>>(iter: T) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::types::field::LogicalUnionFields", "path": "LogicalUnionFields"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [110, 1], "end": [114, 2], "filename": "src/types/field.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"tuple": [{"primitive": "i8"}, {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "datafusion_common::types::field::LogicalField", "path": "LogicalField"}}}], "constraints": []}}, "id": "alloc::sync::Arc", "path": "Arc"}}]}}], "constraints": []}}, "id": "core::iter::traits::collect::FromIterator", "path": "FromIterator"}, "trait_path": "core::iter::traits::collect::FromIterator"}`

Source: `src/types/field.rs:111`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7ae3797dde83fef2fdaf0db9"></a>
## hash

`function` · `datafusion_common::types::field::LogicalUnionFields::hash` · datafusion-common 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::types::field::LogicalUnionFields", "path": "LogicalUnionFields"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [90, 39], "end": [90, 43], "filename": "src/types/field.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/types/field.rs:90`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0a92b8f74c53553c7a9a7c54"></a>
## partial_cmp

`function` · `datafusion_common::types::field::LogicalUnionFields::partial_cmp` · datafusion-common 55.1.0

```rust
fn partial_cmp(&self, other: &LogicalUnionFields) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::types::field::LogicalUnionFields", "path": "LogicalUnionFields"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [90, 45], "end": [90, 55], "filename": "src/types/field.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/types/field.rs:90`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
