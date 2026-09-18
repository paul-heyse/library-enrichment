# `datafusion_expr::logical_plan::ddl::DropView`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.logical_plan.ddl.DropView.json).

<a id="op-6718cec8ee5ce7c1dff3f0c5"></a>
## DropView

`struct` · `datafusion_expr::logical_plan::ddl::DropView` · datafusion-expr 55.1.0

```rust
struct DropView
```

Source: `src/logical_plan/ddl.rs:581`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Drops a view.

<a id="op-0ccbdfc5bfe96f56219513c7"></a>
## clone

`function` · `datafusion_expr::logical_plan::ddl::DropView::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> DropView
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::ddl::DropView", "path": "DropView"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [580, 17], "end": [580, 22], "filename": "src/logical_plan/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/logical_plan/ddl.rs:580`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d81c1e785b1ef90a2037c176"></a>
## eq

`function` · `datafusion_expr::logical_plan::ddl::DropView::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &DropView) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::ddl::DropView", "path": "DropView"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [580, 24], "end": [580, 33], "filename": "src/logical_plan/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/logical_plan/ddl.rs:580`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b2b4fa79f54adf17c004724b"></a>
## fmt

`function` · `datafusion_expr::logical_plan::ddl::DropView::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::ddl::DropView", "path": "DropView"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [580, 10], "end": [580, 15], "filename": "src/logical_plan/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/logical_plan/ddl.rs:580`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cfec684900bdb42f5a2275ae"></a>
## hash

`function` · `datafusion_expr::logical_plan::ddl::DropView::hash` · datafusion-expr 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::ddl::DropView", "path": "DropView"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [580, 39], "end": [580, 43], "filename": "src/logical_plan/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/logical_plan/ddl.rs:580`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b3fc94a97c5fce64cc1a2365"></a>
## if_exists

`struct_field` · `datafusion_expr::logical_plan::ddl::DropView::if_exists` · datafusion-expr 55.1.0

```rust
if_exists: bool
```

Source: `src/logical_plan/ddl.rs:585`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

If the view exists

<a id="op-006b5a0cb97fdad71252bb5f"></a>
## name

`struct_field` · `datafusion_expr::logical_plan::ddl::DropView::name` · datafusion-expr 55.1.0

```rust
name: datafusion_common::TableReference
```

Source: `src/logical_plan/ddl.rs:583`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The view name

<a id="op-96cb4279387e796bd538b6f7"></a>
## partial_cmp

`function` · `datafusion_expr::logical_plan::ddl::DropView::partial_cmp` · datafusion-expr 55.1.0

```rust
fn partial_cmp(&self, other: &Self) -> Option<Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::ddl::DropView", "path": "DropView"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [591, 1], "end": [600, 2], "filename": "src/logical_plan/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/logical_plan/ddl.rs:592`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-268bf60b6b9f59bbe60fde9d"></a>
## schema

`struct_field` · `datafusion_expr::logical_plan::ddl::DropView::schema` · datafusion-expr 55.1.0

```rust
schema: datafusion_common::DFSchemaRef
```

Source: `src/logical_plan/ddl.rs:587`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Dummy schema
