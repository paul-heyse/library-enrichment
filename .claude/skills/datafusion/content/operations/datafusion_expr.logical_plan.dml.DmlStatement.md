# `datafusion_expr::logical_plan::dml::DmlStatement`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.logical_plan.dml.DmlStatement.json).

<a id="op-2ac6db89e384424dd62ef114"></a>
## DmlStatement

`struct` · `datafusion_expr::logical_plan::dml::DmlStatement` · datafusion-expr 55.1.0

```rust
struct DmlStatement
```

Source: `src/logical_plan/dml.rs:140`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Modifies the content of a database

This operator is used to perform DML operations such as INSERT, DELETE,
UPDATE, and CTAS (CREATE TABLE AS SELECT).

* `INSERT` - Appends new rows to the existing table. Calls
  [`TableProvider::insert_into`]

* `DELETE` - Removes rows from the table. Calls [`TableProvider::delete_from`]

* `UPDATE` - Modifies existing rows in the table. Calls [`TableProvider::update`]

* `CREATE TABLE AS SELECT` - Creates a new table and populates it with data
  from a query. This is similar to the `INSERT` operation, but it creates a new
  table instead of modifying an existing one.

Note that the structure is adapted from substrait WriteRel)

[`TableProvider`]: https://docs.rs/datafusion/latest/datafusion/datasource/trait.TableProvider.html
[`TableProvider::insert_into`]: https://docs.rs/datafusion/latest/datafusion/datasource/trait.TableProvider.html#method.insert_into
[`TableProvider::delete_from`]: https://docs.rs/datafusion/latest/datafusion/datasource/trait.TableProvider.html#method.delete_from
[`TableProvider::update`]: https://docs.rs/datafusion/latest/datafusion/datasource/trait.TableProvider.html#method.update

<a id="op-27f568bb1672aadc3bc277ab"></a>
## clone

`function` · `datafusion_expr::logical_plan::dml::DmlStatement::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> DmlStatement
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::dml::DmlStatement", "path": "DmlStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [139, 10], "end": [139, 15], "filename": "src/logical_plan/dml.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/logical_plan/dml.rs:139`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-71b2967cfd261c7d8b016c21"></a>
## eq

`function` · `datafusion_expr::logical_plan::dml::DmlStatement::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &Self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::dml::DmlStatement", "path": "DmlStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [163, 1], "end": [171, 2], "filename": "src/logical_plan/dml.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/logical_plan/dml.rs:164`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-40aab5baa3a9bf8379e518ea"></a>
## fmt

`function` · `datafusion_expr::logical_plan::dml::DmlStatement::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::dml::DmlStatement", "path": "DmlStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [173, 1], "end": [184, 2], "filename": "src/logical_plan/dml.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/logical_plan/dml.rs:174`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-07a6b2824375352a197a1585"></a>
## hash

`function` · `datafusion_expr::logical_plan::dml::DmlStatement::hash` · datafusion-expr 55.1.0

```rust
fn hash<H: Hasher>(&self, state: &mut H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::dml::DmlStatement", "path": "DmlStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [153, 1], "end": [161, 2], "filename": "src/logical_plan/dml.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/logical_plan/dml.rs:154`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fea12b33b37c78bb9479e916"></a>
## input

`struct_field` · `datafusion_expr::logical_plan::dml::DmlStatement::input` · datafusion-expr 55.1.0

```rust
input: std::sync::Arc<LogicalPlan>
```

Source: `src/logical_plan/dml.rs:148`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The relation that determines the tuples to add/remove/modify the schema must match with table_schema

<a id="op-2b6633581ee814a972cbbc3c"></a>
## name

`function` · `datafusion_expr::logical_plan::dml::DmlStatement::name` · datafusion-expr 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::dml::DmlStatement", "path": "DmlStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [186, 1], "end": [209, 2], "filename": "src/logical_plan/dml.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/dml.rs:206`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Return a descriptive name of this [`DmlStatement`](../operations/datafusion_expr.logical_plan.dml.DmlStatement.md#op-2ac6db89e384424dd62ef114)

<a id="op-192e4ad4eaa666e53d518ff5"></a>
## new

`function` · `datafusion_expr::logical_plan::dml::DmlStatement::new` · datafusion-expr 55.1.0

```rust
fn new(table_name: TableReference, target: Arc<dyn TableSource>, op: WriteOp, input: Arc<LogicalPlan>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::dml::DmlStatement", "path": "DmlStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [186, 1], "end": [209, 2], "filename": "src/logical_plan/dml.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/dml.rs:188`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Creates a new DML statement with the output schema set to a single `count` column.

<a id="op-2635dcdcc21277452a71b2d7"></a>
## op

`struct_field` · `datafusion_expr::logical_plan::dml::DmlStatement::op` · datafusion-expr 55.1.0

```rust
op: WriteOp
```

Source: `src/logical_plan/dml.rs:146`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The type of operation to perform

<a id="op-bfe31508f4af55f4093d1ba4"></a>
## output_schema

`struct_field` · `datafusion_expr::logical_plan::dml::DmlStatement::output_schema` · datafusion-expr 55.1.0

```rust
output_schema: datafusion_common::DFSchemaRef
```

Source: `src/logical_plan/dml.rs:150`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The schema of the output relation

<a id="op-a4888884306889fa968cf1f5"></a>
## partial_cmp

`function` · `datafusion_expr::logical_plan::dml::DmlStatement::partial_cmp` · datafusion-expr 55.1.0

```rust
fn partial_cmp(&self, other: &Self) -> Option<Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::dml::DmlStatement", "path": "DmlStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [213, 1], "end": [225, 2], "filename": "src/logical_plan/dml.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/logical_plan/dml.rs:214`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-95392c0ab0c803c7480ec4f5"></a>
## table_name

`struct_field` · `datafusion_expr::logical_plan::dml::DmlStatement::table_name` · datafusion-expr 55.1.0

```rust
table_name: datafusion_common::TableReference
```

Source: `src/logical_plan/dml.rs:142`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The table name

<a id="op-c3af5046a459f4b206bbc4a1"></a>
## target

`struct_field` · `datafusion_expr::logical_plan::dml::DmlStatement::target` · datafusion-expr 55.1.0

```rust
target: std::sync::Arc<dyn TableSource>
```

Source: `src/logical_plan/dml.rs:144`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

this is target table to insert into
