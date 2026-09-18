# `datafusion_expr::logical_plan::ddl::CreateCatalogSchema`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.logical_plan.ddl.CreateCatalogSchema.json).

<a id="op-ad04b5ffc694451e3a980fa4"></a>
## CreateCatalogSchema

`struct` · `datafusion_expr::logical_plan::ddl::CreateCatalogSchema` · datafusion-expr 55.1.0

```rust
struct CreateCatalogSchema
```

Source: `src/logical_plan/ddl.rs:535`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Creates a schema.

<a id="op-40ce3f96e0c2f5d84ded57b9"></a>
## clone

`function` · `datafusion_expr::logical_plan::ddl::CreateCatalogSchema::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> CreateCatalogSchema
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::ddl::CreateCatalogSchema", "path": "CreateCatalogSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [534, 17], "end": [534, 22], "filename": "src/logical_plan/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/logical_plan/ddl.rs:534`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f52ba50cb0e1eeabe52c1672"></a>
## eq

`function` · `datafusion_expr::logical_plan::ddl::CreateCatalogSchema::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &CreateCatalogSchema) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::ddl::CreateCatalogSchema", "path": "CreateCatalogSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [534, 24], "end": [534, 33], "filename": "src/logical_plan/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/logical_plan/ddl.rs:534`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0cc4ce429a7b84b7b960f417"></a>
## fmt

`function` · `datafusion_expr::logical_plan::ddl::CreateCatalogSchema::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::ddl::CreateCatalogSchema", "path": "CreateCatalogSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [534, 10], "end": [534, 15], "filename": "src/logical_plan/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/logical_plan/ddl.rs:534`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a2684fb622c357ce148ac6ce"></a>
## hash

`function` · `datafusion_expr::logical_plan::ddl::CreateCatalogSchema::hash` · datafusion-expr 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::ddl::CreateCatalogSchema", "path": "CreateCatalogSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [534, 39], "end": [534, 43], "filename": "src/logical_plan/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/logical_plan/ddl.rs:534`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e653b4301d11c19a2f9f74a2"></a>
## if_not_exists

`struct_field` · `datafusion_expr::logical_plan::ddl::CreateCatalogSchema::if_not_exists` · datafusion-expr 55.1.0

```rust
if_not_exists: bool
```

Source: `src/logical_plan/ddl.rs:539`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Do nothing (except issuing a notice) if a schema with the same name already exists

<a id="op-29b66d10079ed7807c599a4d"></a>
## partial_cmp

`function` · `datafusion_expr::logical_plan::ddl::CreateCatalogSchema::partial_cmp` · datafusion-expr 55.1.0

```rust
fn partial_cmp(&self, other: &Self) -> Option<Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::ddl::CreateCatalogSchema", "path": "CreateCatalogSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [545, 1], "end": [554, 2], "filename": "src/logical_plan/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/logical_plan/ddl.rs:546`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-005ec1e75582d34eb4346924"></a>
## schema

`struct_field` · `datafusion_expr::logical_plan::ddl::CreateCatalogSchema::schema` · datafusion-expr 55.1.0

```rust
schema: datafusion_common::DFSchemaRef
```

Source: `src/logical_plan/ddl.rs:541`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Empty schema

<a id="op-542d09bea925d846a6b64ab1"></a>
## schema_name

`struct_field` · `datafusion_expr::logical_plan::ddl::CreateCatalogSchema::schema_name` · datafusion-expr 55.1.0

```rust
schema_name: String
```

Source: `src/logical_plan/ddl.rs:537`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The table schema
