# `datafusion_expr::udf_eq::UdfEq`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.udf_eq.UdfEq.json).

<a id="op-cb3082c5650a4d21b13b636a"></a>
## UdfEq

`struct` · `datafusion_expr::udf_eq::UdfEq` · datafusion-expr 55.1.0

```rust
struct UdfEq<Ptr: UdfPointer>
```

Source: `src/udf_eq.rs:31`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

A wrapper around a pointer to UDF that implements `Eq` and `Hash` delegating to
corresponding methods on the UDF trait.

If you want to just compare pointers for equality, use [`super::ptr_eq::PtrEq`](../operations/datafusion_expr.ptr_eq.PtrEq.md#op-bab65d64f183b190bc0cbeda).

<a id="op-9d8b049c7e11b22499d6f5e1"></a>
## Target

`assoc_type` · `datafusion_expr::udf_eq::UdfEq::Target` · datafusion-expr 55.1.0

```rust
Target
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "Ptr"}}], "constraints": []}}, "id": "datafusion_expr::udf_eq::UdfEq", "path": "UdfEq"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "Ptr"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "unresolved", "path": "UdfPointer"}}}], "generic_params": [], "type": {"generic": "Ptr"}}}]}, "is_negative": false, "span": {"begin": [69, 1], "end": [78, 2], "filename": "src/udf_eq.rs"}, "trait": {"args": null, "id": "core::ops::deref::Deref", "path": "Deref"}, "trait_path": "core::ops::deref::Deref"}`

Source: `src/udf_eq.rs:73`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9b32adf7653ee034ac34cc2c"></a>
## clone

`function` · `datafusion_expr::udf_eq::UdfEq::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> UdfEq<Ptr>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "Ptr"}}], "constraints": []}}, "id": "datafusion_expr::udf_eq::UdfEq", "path": "UdfEq"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::clone::Clone", "path": "$crate::clone::Clone"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "unresolved", "path": "UdfPointer"}}}], "default": null, "is_synthetic": false}}, "name": "Ptr"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [29, 10], "end": [29, 15], "filename": "src/udf_eq.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/udf_eq.rs:29`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-673393857f3aef794377ca8d"></a>
## deref

`function` · `datafusion_expr::udf_eq::UdfEq::deref` · datafusion-expr 55.1.0

```rust
fn deref(&self) -> &Self::Target
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "Ptr"}}], "constraints": []}}, "id": "datafusion_expr::udf_eq::UdfEq", "path": "UdfEq"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "Ptr"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "unresolved", "path": "UdfPointer"}}}], "generic_params": [], "type": {"generic": "Ptr"}}}]}, "is_negative": false, "span": {"begin": [69, 1], "end": [78, 2], "filename": "src/udf_eq.rs"}, "trait": {"args": null, "id": "core::ops::deref::Deref", "path": "Deref"}, "trait_path": "core::ops::deref::Deref"}`

Source: `src/udf_eq.rs:75`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1db7f8bd02b03d806a42b1eb"></a>
## eq

`function` · `datafusion_expr::udf_eq::UdfEq::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &Self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "Ptr"}}], "constraints": []}}, "id": "datafusion_expr::udf_eq::UdfEq", "path": "UdfEq"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "Ptr"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "unresolved", "path": "UdfPointer"}}}], "generic_params": [], "type": {"generic": "Ptr"}}}]}, "is_negative": false, "span": {"begin": [33, 1], "end": [40, 2], "filename": "src/udf_eq.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/udf_eq.rs:37`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-137ae0c626b8db106ccf134f"></a>
## fmt

`function` · `datafusion_expr::udf_eq::UdfEq::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "Ptr"}}], "constraints": []}}, "id": "datafusion_expr::udf_eq::UdfEq", "path": "UdfEq"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "Ptr"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "unresolved", "path": "UdfPointer"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}}}], "generic_params": [], "type": {"generic": "Ptr"}}}]}, "is_negative": false, "span": {"begin": [60, 1], "end": [67, 2], "filename": "src/udf_eq.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/udf_eq.rs:64`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-aef3077e651e9bed299975cf"></a>
## from

`function` · `datafusion_expr::udf_eq::UdfEq::from` · datafusion-expr 55.1.0

```rust
fn from(ptr: Ptr) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "Ptr"}}], "constraints": []}}, "id": "datafusion_expr::udf_eq::UdfEq", "path": "UdfEq"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "Ptr"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "unresolved", "path": "UdfPointer"}}}], "generic_params": [], "type": {"generic": "Ptr"}}}]}, "is_negative": false, "span": {"begin": [51, 1], "end": [58, 2], "filename": "src/udf_eq.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "Ptr"}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/udf_eq.rs:55`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4d31ade9fd855d89f396557a"></a>
## hash

`function` · `datafusion_expr::udf_eq::UdfEq::hash` · datafusion-expr 55.1.0

```rust
fn hash<H: Hasher>(&self, state: &mut H)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "Ptr"}}], "constraints": []}}, "id": "datafusion_expr::udf_eq::UdfEq", "path": "UdfEq"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "Ptr"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "unresolved", "path": "UdfPointer"}}}], "generic_params": [], "type": {"generic": "Ptr"}}}]}, "is_negative": false, "span": {"begin": [42, 1], "end": [49, 2], "filename": "src/udf_eq.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/udf_eq.rs:46`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
