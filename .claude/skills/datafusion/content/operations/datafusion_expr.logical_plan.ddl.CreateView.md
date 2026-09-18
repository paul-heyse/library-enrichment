# `datafusion_expr::logical_plan::ddl::CreateView`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.logical_plan.ddl.CreateView.json).

<a id="op-d833e39613a64550d6659f08"></a>
## CreateView

`struct` · `datafusion_expr::logical_plan::ddl::CreateView` · datafusion-expr 55.1.0

```rust
struct CreateView
```

Source: `src/logical_plan/ddl.rs:497`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Creates a view.

<a id="op-7376197f7fc8ade5c0cfa90b"></a>
## clone

`function` · `datafusion_expr::logical_plan::ddl::CreateView::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> CreateView
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::ddl::CreateView", "path": "CreateView"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [496, 17], "end": [496, 22], "filename": "src/logical_plan/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/logical_plan/ddl.rs:496`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-417f59dd1c99a685526dc3db"></a>
## definition

`struct_field` · `datafusion_expr::logical_plan::ddl::CreateView::definition` · datafusion-expr 55.1.0

```rust
definition: Option<String>
```

Source: `src/logical_plan/ddl.rs:505`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

SQL used to create the view, if available

<a id="op-a6dd7b2137dcd35c7e0a40aa"></a>
## eq

`function` · `datafusion_expr::logical_plan::ddl::CreateView::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &CreateView) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::ddl::CreateView", "path": "CreateView"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [496, 24], "end": [496, 33], "filename": "src/logical_plan/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/logical_plan/ddl.rs:496`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ac69894cdfc64c34822e2182"></a>
## fmt

`function` · `datafusion_expr::logical_plan::ddl::CreateView::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::ddl::CreateView", "path": "CreateView"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [496, 10], "end": [496, 15], "filename": "src/logical_plan/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/logical_plan/ddl.rs:496`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7272cb8b30a889c218b1616a"></a>
## hash

`function` · `datafusion_expr::logical_plan::ddl::CreateView::hash` · datafusion-expr 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::ddl::CreateView", "path": "CreateView"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [496, 51], "end": [496, 55], "filename": "src/logical_plan/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/logical_plan/ddl.rs:496`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ebd1dc23466c7c72348a7bdf"></a>
## input

`struct_field` · `datafusion_expr::logical_plan::ddl::CreateView::input` · datafusion-expr 55.1.0

```rust
input: std::sync::Arc<LogicalPlan>
```

Source: `src/logical_plan/ddl.rs:501`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The logical plan

<a id="op-8636c3fe25abd9792a57cc44"></a>
## name

`struct_field` · `datafusion_expr::logical_plan::ddl::CreateView::name` · datafusion-expr 55.1.0

```rust
name: datafusion_common::TableReference
```

Source: `src/logical_plan/ddl.rs:499`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The table name

<a id="op-08e8247ab1a8cb592613e248"></a>
## or_replace

`struct_field` · `datafusion_expr::logical_plan::ddl::CreateView::or_replace` · datafusion-expr 55.1.0

```rust
or_replace: bool
```

Source: `src/logical_plan/ddl.rs:503`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Option to not error if table already exists

<a id="op-1f4e5805d3847dbe29a45758"></a>
## partial_cmp

`function` · `datafusion_expr::logical_plan::ddl::CreateView::partial_cmp` · datafusion-expr 55.1.0

```rust
fn partial_cmp(&self, other: &CreateView) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::ddl::CreateView", "path": "CreateView"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [496, 35], "end": [496, 45], "filename": "src/logical_plan/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/logical_plan/ddl.rs:496`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9c8eb57bde4c5749203c10b8"></a>
## temporary

`struct_field` · `datafusion_expr::logical_plan::ddl::CreateView::temporary` · datafusion-expr 55.1.0

```rust
temporary: bool
```

Source: `src/logical_plan/ddl.rs:507`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Whether the view is ephemeral
