# `datafusion_expr::logical_plan::ddl::DropCatalogSchema`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.logical_plan.ddl.DropCatalogSchema.json).

<a id="op-2f2a4e573702233721e1f85a"></a>
## DropCatalogSchema

`struct` · `datafusion_expr::logical_plan::ddl::DropCatalogSchema` · datafusion-expr 55.1.0

```rust
struct DropCatalogSchema
```

Source: `src/logical_plan/ddl.rs:604`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Drops a schema

<a id="op-b46bfded3683d3732ac9dda9"></a>
## cascade

`struct_field` · `datafusion_expr::logical_plan::ddl::DropCatalogSchema::cascade` · datafusion-expr 55.1.0

```rust
cascade: bool
```

Source: `src/logical_plan/ddl.rs:610`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Whether drop should cascade

<a id="op-c81a05085ca3406aaa783b96"></a>
## clone

`function` · `datafusion_expr::logical_plan::ddl::DropCatalogSchema::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> DropCatalogSchema
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::ddl::DropCatalogSchema", "path": "DropCatalogSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [603, 17], "end": [603, 22], "filename": "src/logical_plan/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/logical_plan/ddl.rs:603`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8c7359fca7687643291615dd"></a>
## eq

`function` · `datafusion_expr::logical_plan::ddl::DropCatalogSchema::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &DropCatalogSchema) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::ddl::DropCatalogSchema", "path": "DropCatalogSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [603, 24], "end": [603, 33], "filename": "src/logical_plan/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/logical_plan/ddl.rs:603`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a5e0fbf439b3c18fd07f8829"></a>
## fmt

`function` · `datafusion_expr::logical_plan::ddl::DropCatalogSchema::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::ddl::DropCatalogSchema", "path": "DropCatalogSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [603, 10], "end": [603, 15], "filename": "src/logical_plan/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/logical_plan/ddl.rs:603`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ea5dd2ce4b1171470777ced2"></a>
## hash

`function` · `datafusion_expr::logical_plan::ddl::DropCatalogSchema::hash` · datafusion-expr 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::ddl::DropCatalogSchema", "path": "DropCatalogSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [603, 39], "end": [603, 43], "filename": "src/logical_plan/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/logical_plan/ddl.rs:603`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a62f8cbd7b194f5ad3c9d142"></a>
## if_exists

`struct_field` · `datafusion_expr::logical_plan::ddl::DropCatalogSchema::if_exists` · datafusion-expr 55.1.0

```rust
if_exists: bool
```

Source: `src/logical_plan/ddl.rs:608`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

If the schema exists

<a id="op-8a66491157326aa91f757c96"></a>
## name

`struct_field` · `datafusion_expr::logical_plan::ddl::DropCatalogSchema::name` · datafusion-expr 55.1.0

```rust
name: datafusion_common::SchemaReference
```

Source: `src/logical_plan/ddl.rs:606`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The schema name

<a id="op-f81b4852d899918db4fb0df0"></a>
## partial_cmp

`function` · `datafusion_expr::logical_plan::ddl::DropCatalogSchema::partial_cmp` · datafusion-expr 55.1.0

```rust
fn partial_cmp(&self, other: &Self) -> Option<Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::ddl::DropCatalogSchema", "path": "DropCatalogSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [616, 1], "end": [628, 2], "filename": "src/logical_plan/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/logical_plan/ddl.rs:617`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9b21824597d032b1a575139e"></a>
## schema

`struct_field` · `datafusion_expr::logical_plan::ddl::DropCatalogSchema::schema` · datafusion-expr 55.1.0

```rust
schema: datafusion_common::DFSchemaRef
```

Source: `src/logical_plan/ddl.rs:612`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Dummy schema
