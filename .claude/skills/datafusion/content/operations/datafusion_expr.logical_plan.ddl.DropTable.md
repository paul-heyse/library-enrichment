# `datafusion_expr::logical_plan::ddl::DropTable`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.logical_plan.ddl.DropTable.json).

<a id="op-ef2f28e3924e957540b49301"></a>
## DropTable

`struct` · `datafusion_expr::logical_plan::ddl::DropTable` · datafusion-expr 55.1.0

```rust
struct DropTable
```

Source: `src/logical_plan/ddl.rs:558`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Drops a table.

<a id="op-1d80e6a5097a553e2fee85d9"></a>
## clone

`function` · `datafusion_expr::logical_plan::ddl::DropTable::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> DropTable
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::ddl::DropTable", "path": "DropTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [557, 17], "end": [557, 22], "filename": "src/logical_plan/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/logical_plan/ddl.rs:557`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-885f6950147b64f0494f1a0e"></a>
## eq

`function` · `datafusion_expr::logical_plan::ddl::DropTable::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &DropTable) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::ddl::DropTable", "path": "DropTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [557, 24], "end": [557, 33], "filename": "src/logical_plan/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/logical_plan/ddl.rs:557`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-676b55102465114b52079d8c"></a>
## fmt

`function` · `datafusion_expr::logical_plan::ddl::DropTable::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::ddl::DropTable", "path": "DropTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [557, 10], "end": [557, 15], "filename": "src/logical_plan/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/logical_plan/ddl.rs:557`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0c5f146d24b498bc770c6d57"></a>
## hash

`function` · `datafusion_expr::logical_plan::ddl::DropTable::hash` · datafusion-expr 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::ddl::DropTable", "path": "DropTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [557, 39], "end": [557, 43], "filename": "src/logical_plan/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/logical_plan/ddl.rs:557`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c7119e44a62be38a0da646da"></a>
## if_exists

`struct_field` · `datafusion_expr::logical_plan::ddl::DropTable::if_exists` · datafusion-expr 55.1.0

```rust
if_exists: bool
```

Source: `src/logical_plan/ddl.rs:562`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

If the table exists

<a id="op-88568bdf9db17ec27f075610"></a>
## name

`struct_field` · `datafusion_expr::logical_plan::ddl::DropTable::name` · datafusion-expr 55.1.0

```rust
name: datafusion_common::TableReference
```

Source: `src/logical_plan/ddl.rs:560`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The table name

<a id="op-6c4e82cbfac43697258d037e"></a>
## partial_cmp

`function` · `datafusion_expr::logical_plan::ddl::DropTable::partial_cmp` · datafusion-expr 55.1.0

```rust
fn partial_cmp(&self, other: &Self) -> Option<Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::ddl::DropTable", "path": "DropTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [568, 1], "end": [577, 2], "filename": "src/logical_plan/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/logical_plan/ddl.rs:569`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3174ce29c211c8481d198f0d"></a>
## schema

`struct_field` · `datafusion_expr::logical_plan::ddl::DropTable::schema` · datafusion-expr 55.1.0

```rust
schema: datafusion_common::DFSchemaRef
```

Source: `src/logical_plan/ddl.rs:564`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Dummy schema
