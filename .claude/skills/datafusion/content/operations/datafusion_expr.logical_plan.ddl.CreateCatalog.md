# `datafusion_expr::logical_plan::ddl::CreateCatalog`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.logical_plan.ddl.CreateCatalog.json).

<a id="op-f7d41f69d145385db498443f"></a>
## CreateCatalog

`struct` · `datafusion_expr::logical_plan::ddl::CreateCatalog` · datafusion-expr 55.1.0

```rust
struct CreateCatalog
```

Source: `src/logical_plan/ddl.rs:512`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Creates a catalog (aka "Database").

<a id="op-1a45467282cf775daafe0ae8"></a>
## catalog_name

`struct_field` · `datafusion_expr::logical_plan::ddl::CreateCatalog::catalog_name` · datafusion-expr 55.1.0

```rust
catalog_name: String
```

Source: `src/logical_plan/ddl.rs:514`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The catalog name

<a id="op-f8158ac2adc0b24b036fabd3"></a>
## clone

`function` · `datafusion_expr::logical_plan::ddl::CreateCatalog::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> CreateCatalog
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::ddl::CreateCatalog", "path": "CreateCatalog"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [511, 17], "end": [511, 22], "filename": "src/logical_plan/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/logical_plan/ddl.rs:511`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a18d33246a93341b2cd8633e"></a>
## eq

`function` · `datafusion_expr::logical_plan::ddl::CreateCatalog::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &CreateCatalog) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::ddl::CreateCatalog", "path": "CreateCatalog"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [511, 24], "end": [511, 33], "filename": "src/logical_plan/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/logical_plan/ddl.rs:511`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5d6a1951c2ab069e47732e4a"></a>
## fmt

`function` · `datafusion_expr::logical_plan::ddl::CreateCatalog::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::ddl::CreateCatalog", "path": "CreateCatalog"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [511, 10], "end": [511, 15], "filename": "src/logical_plan/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/logical_plan/ddl.rs:511`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-254ec74097a545864effa1bc"></a>
## hash

`function` · `datafusion_expr::logical_plan::ddl::CreateCatalog::hash` · datafusion-expr 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::ddl::CreateCatalog", "path": "CreateCatalog"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [511, 39], "end": [511, 43], "filename": "src/logical_plan/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/logical_plan/ddl.rs:511`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fddc55189cd03665b173acab"></a>
## if_not_exists

`struct_field` · `datafusion_expr::logical_plan::ddl::CreateCatalog::if_not_exists` · datafusion-expr 55.1.0

```rust
if_not_exists: bool
```

Source: `src/logical_plan/ddl.rs:516`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Do nothing (except issuing a notice) if a schema with the same name already exists

<a id="op-2a4d0729062cbd71e68be99c"></a>
## partial_cmp

`function` · `datafusion_expr::logical_plan::ddl::CreateCatalog::partial_cmp` · datafusion-expr 55.1.0

```rust
fn partial_cmp(&self, other: &Self) -> Option<Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::ddl::CreateCatalog", "path": "CreateCatalog"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [522, 1], "end": [531, 2], "filename": "src/logical_plan/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/logical_plan/ddl.rs:523`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-86c7f7c7e76c23a83efcc38d"></a>
## schema

`struct_field` · `datafusion_expr::logical_plan::ddl::CreateCatalog::schema` · datafusion-expr 55.1.0

```rust
schema: datafusion_common::DFSchemaRef
```

Source: `src/logical_plan/ddl.rs:518`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Empty schema
