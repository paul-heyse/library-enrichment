# `datafusion_expr::ptr_eq::PtrEq`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.ptr_eq.PtrEq.json).

<a id="op-bab65d64f183b190bc0cbeda"></a>
## PtrEq

`struct` · `datafusion_expr::ptr_eq::PtrEq` · datafusion-expr 55.1.0

```rust
struct PtrEq<Ptr: PointerType>
```

Source: `src/ptr_eq.rs:43`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

A wrapper around a pointer that implements `Eq` and `Hash` comparing
the underlying pointer address.

If you have pointers to a `dyn UDF impl` consider using [`super::udf_eq::UdfEq`](../operations/datafusion_expr.udf_eq.UdfEq.md#op-cb3082c5650a4d21b13b636a).

<a id="op-32737a5f3a8064e74a67d3ac"></a>
## Target

`assoc_type` · `datafusion_expr::ptr_eq::PtrEq::Target` · datafusion-expr 55.1.0

```rust
Target
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "Ptr"}}], "constraints": []}}, "id": "datafusion_expr::ptr_eq::PtrEq", "path": "PtrEq"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "Ptr"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "unresolved", "path": "PointerType"}}}], "generic_params": [], "type": {"generic": "Ptr"}}}]}, "is_negative": false, "span": {"begin": [91, 1], "end": [100, 2], "filename": "src/ptr_eq.rs"}, "trait": {"args": null, "id": "core::ops::deref::Deref", "path": "Deref"}, "trait_path": "core::ops::deref::Deref"}`

Source: `src/ptr_eq.rs:95`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-77b2e62abb06fcf5fb93bee1"></a>
## clone

`function` · `datafusion_expr::ptr_eq::PtrEq::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> PtrEq<Ptr>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "Ptr"}}], "constraints": []}}, "id": "datafusion_expr::ptr_eq::PtrEq", "path": "PtrEq"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::clone::Clone", "path": "$crate::clone::Clone"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "unresolved", "path": "PointerType"}}}], "default": null, "is_synthetic": false}}, "name": "Ptr"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [41, 10], "end": [41, 15], "filename": "src/ptr_eq.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ptr_eq.rs:41`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c855319fd0e7cbe5eaf8aaf1"></a>
## deref

`function` · `datafusion_expr::ptr_eq::PtrEq::deref` · datafusion-expr 55.1.0

```rust
fn deref(&self) -> &Self::Target
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "Ptr"}}], "constraints": []}}, "id": "datafusion_expr::ptr_eq::PtrEq", "path": "PtrEq"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "Ptr"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "unresolved", "path": "PointerType"}}}], "generic_params": [], "type": {"generic": "Ptr"}}}]}, "is_negative": false, "span": {"begin": [91, 1], "end": [100, 2], "filename": "src/ptr_eq.rs"}, "trait": {"args": null, "id": "core::ops::deref::Deref", "path": "Deref"}, "trait_path": "core::ops::deref::Deref"}`

Source: `src/ptr_eq.rs:97`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a269c8592ad1c3f05dda6ab9"></a>
## eq

`function` · `datafusion_expr::ptr_eq::PtrEq::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &Self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "alloc::sync::Arc", "path": "std::sync::Arc"}}}], "constraints": []}}, "id": "datafusion_expr::ptr_eq::PtrEq", "path": "PtrEq"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "maybe", "trait": {"args": null, "id": "core::marker::Sized", "path": "Sized"}}}], "generic_params": [], "type": {"generic": "T"}}}]}, "is_negative": false, "span": {"begin": [45, 1], "end": [52, 2], "filename": "src/ptr_eq.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ptr_eq.rs:49`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c5b220099dc9a5bf60e80a12"></a>
## fmt

`function` · `datafusion_expr::ptr_eq::PtrEq::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "Ptr"}}], "constraints": []}}, "id": "datafusion_expr::ptr_eq::PtrEq", "path": "PtrEq"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "Ptr"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "unresolved", "path": "PointerType"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}}}], "generic_params": [], "type": {"generic": "Ptr"}}}]}, "is_negative": false, "span": {"begin": [82, 1], "end": [89, 2], "filename": "src/ptr_eq.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ptr_eq.rs:86`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-45849f4c7373866e149bb2c7"></a>
## from

`function` · `datafusion_expr::ptr_eq::PtrEq::from` · datafusion-expr 55.1.0

```rust
fn from(ptr: Ptr) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "Ptr"}}], "constraints": []}}, "id": "datafusion_expr::ptr_eq::PtrEq", "path": "PtrEq"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "Ptr"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "unresolved", "path": "PointerType"}}}], "generic_params": [], "type": {"generic": "Ptr"}}}]}, "is_negative": false, "span": {"begin": [64, 1], "end": [71, 2], "filename": "src/ptr_eq.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "Ptr"}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/ptr_eq.rs:68`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-91beffd99ba205949449ef90"></a>
## hash

`function` · `datafusion_expr::ptr_eq::PtrEq::hash` · datafusion-expr 55.1.0

```rust
fn hash<H: Hasher>(&self, state: &mut H)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "alloc::sync::Arc", "path": "std::sync::Arc"}}}], "constraints": []}}, "id": "datafusion_expr::ptr_eq::PtrEq", "path": "PtrEq"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "maybe", "trait": {"args": null, "id": "core::marker::Sized", "path": "Sized"}}}], "generic_params": [], "type": {"generic": "T"}}}]}, "is_negative": false, "span": {"begin": [55, 1], "end": [62, 2], "filename": "src/ptr_eq.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ptr_eq.rs:59`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
