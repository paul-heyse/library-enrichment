# `datafusion_expr::logical_plan::ddl::DdlStatement`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.logical_plan.ddl.DdlStatement.json).

<a id="op-625609742e39679e4ed192e2"></a>
## DdlStatement

`enum` · `datafusion_expr::logical_plan::ddl::DdlStatement` · datafusion-expr 55.1.0

```rust
enum DdlStatement
```

Source: `src/logical_plan/ddl.rs:40`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Various types of DDL  (CREATE / DROP) catalog manipulation

<a id="op-7179f566d3c0f3534911b3bf"></a>
## CreateCatalog

`variant` · `datafusion_expr::logical_plan::ddl::DdlStatement::CreateCatalog` · datafusion-expr 55.1.0

```rust
CreateCatalog
```

Source: `src/logical_plan/ddl.rs:52`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Creates a new catalog (aka "Database").

<a id="op-531a073801933da0d6d5943f"></a>
## CreateCatalogSchema

`variant` · `datafusion_expr::logical_plan::ddl::DdlStatement::CreateCatalogSchema` · datafusion-expr 55.1.0

```rust
CreateCatalogSchema
```

Source: `src/logical_plan/ddl.rs:50`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Creates a new catalog schema.

<a id="op-61fc8eb0dc5845a04fcf39a1"></a>
## CreateExternalTable

`variant` · `datafusion_expr::logical_plan::ddl::DdlStatement::CreateExternalTable` · datafusion-expr 55.1.0

```rust
CreateExternalTable
```

Source: `src/logical_plan/ddl.rs:44`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Creates an external table. Boxed to keep `LogicalPlan` enum size down
— `CreateExternalTable` is ~312 bytes, dwarfing every other variant
in the plan tree and forcing the whole enum to that width.

<a id="op-0d03c4ccc6690b8a2361410e"></a>
## CreateFunction

`variant` · `datafusion_expr::logical_plan::ddl::DdlStatement::CreateFunction` · datafusion-expr 55.1.0

```rust
CreateFunction
```

Source: `src/logical_plan/ddl.rs:63`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Create function statement. Boxed for the same reason as
[`Self::CreateExternalTable`](../operations/datafusion_expr.logical_plan.ddl.DdlStatement.md#op-61fc8eb0dc5845a04fcf39a1) (~288 bytes).

<a id="op-6c333d35630a8c87734e0e08"></a>
## CreateIndex

`variant` · `datafusion_expr::logical_plan::ddl::DdlStatement::CreateIndex` · datafusion-expr 55.1.0

```rust
CreateIndex
```

Source: `src/logical_plan/ddl.rs:54`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Creates a new index.

<a id="op-53e35829ad493c833febf5c3"></a>
## CreateMemoryTable

`variant` · `datafusion_expr::logical_plan::ddl::DdlStatement::CreateMemoryTable` · datafusion-expr 55.1.0

```rust
CreateMemoryTable
```

Source: `src/logical_plan/ddl.rs:46`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Creates an in memory table.

<a id="op-06d284198e5066b2c9331ec2"></a>
## CreateView

`variant` · `datafusion_expr::logical_plan::ddl::DdlStatement::CreateView` · datafusion-expr 55.1.0

```rust
CreateView
```

Source: `src/logical_plan/ddl.rs:48`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Creates a new view.

<a id="op-a0029845008260c452c10e72"></a>
## DropCatalogSchema

`variant` · `datafusion_expr::logical_plan::ddl::DdlStatement::DropCatalogSchema` · datafusion-expr 55.1.0

```rust
DropCatalogSchema
```

Source: `src/logical_plan/ddl.rs:60`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Drops a catalog schema

<a id="op-b50f3cf30a424c21f2ab90fe"></a>
## DropFunction

`variant` · `datafusion_expr::logical_plan::ddl::DdlStatement::DropFunction` · datafusion-expr 55.1.0

```rust
DropFunction
```

Source: `src/logical_plan/ddl.rs:65`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Drop function statement

<a id="op-211b5b1872ff9ed2d3fa11b8"></a>
## DropTable

`variant` · `datafusion_expr::logical_plan::ddl::DdlStatement::DropTable` · datafusion-expr 55.1.0

```rust
DropTable
```

Source: `src/logical_plan/ddl.rs:56`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Drops a table.

<a id="op-49c0881ac90df012b95b2881"></a>
## DropView

`variant` · `datafusion_expr::logical_plan::ddl::DdlStatement::DropView` · datafusion-expr 55.1.0

```rust
DropView
```

Source: `src/logical_plan/ddl.rs:58`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Drops a view.

<a id="op-5c69201850f19e85f9152218"></a>
## clone

`function` · `datafusion_expr::logical_plan::ddl::DdlStatement::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> DdlStatement
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::ddl::DdlStatement", "path": "DdlStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [39, 17], "end": [39, 22], "filename": "src/logical_plan/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/logical_plan/ddl.rs:39`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-88cf99bc3c22bb1e9d07e32d"></a>
## display

`function` · `datafusion_expr::logical_plan::ddl::DdlStatement::display` · datafusion-expr 55.1.0

```rust
fn display(&self) -> impl Display + '_
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::ddl::DdlStatement", "path": "DdlStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [68, 1], "end": [205, 2], "filename": "src/logical_plan/ddl.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/ddl.rs:130`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Return a `format`able structure with the a human readable
description of this LogicalPlan node per node, not including
children.

See [crate::LogicalPlan::display](../operations/datafusion_expr.logical_plan.plan.LogicalPlan.md#op-a74af8650ebcf89cbff8609b) for an example

<a id="op-61ca716fef43ef477a41ea93"></a>
## eq

`function` · `datafusion_expr::logical_plan::ddl::DdlStatement::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &DdlStatement) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::ddl::DdlStatement", "path": "DdlStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [39, 24], "end": [39, 33], "filename": "src/logical_plan/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/logical_plan/ddl.rs:39`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2c526b6c9a27e142b1491492"></a>
## fmt

`function` · `datafusion_expr::logical_plan::ddl::DdlStatement::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::ddl::DdlStatement", "path": "DdlStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [39, 10], "end": [39, 15], "filename": "src/logical_plan/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/logical_plan/ddl.rs:39`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2e3de0076bd2f673230bf3dc"></a>
## hash

`function` · `datafusion_expr::logical_plan::ddl::DdlStatement::hash` · datafusion-expr 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::ddl::DdlStatement", "path": "DdlStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [39, 51], "end": [39, 55], "filename": "src/logical_plan/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/logical_plan/ddl.rs:39`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b7c5696f9f58422fd650afde"></a>
## inputs

`function` · `datafusion_expr::logical_plan::ddl::DdlStatement::inputs` · datafusion-expr 55.1.0

```rust
fn inputs(&self) -> Vec<&LogicalPlan>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::ddl::DdlStatement", "path": "DdlStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [68, 1], "end": [205, 2], "filename": "src/logical_plan/ddl.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/ddl.rs:107`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Return all inputs for this plan

<a id="op-dacbcd34b664012c6dee216a"></a>
## name

`function` · `datafusion_expr::logical_plan::ddl::DdlStatement::name` · datafusion-expr 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::ddl::DdlStatement", "path": "DdlStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [68, 1], "end": [205, 2], "filename": "src/logical_plan/ddl.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/ddl.rs:90`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Return a descriptive string describing the type of this
[`DdlStatement`](../operations/datafusion_expr.logical_plan.ddl.DdlStatement.md#op-625609742e39679e4ed192e2)

<a id="op-c342f96f603c89515e2c7011"></a>
## partial_cmp

`function` · `datafusion_expr::logical_plan::ddl::DdlStatement::partial_cmp` · datafusion-expr 55.1.0

```rust
fn partial_cmp(&self, other: &DdlStatement) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::ddl::DdlStatement", "path": "DdlStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [39, 39], "end": [39, 49], "filename": "src/logical_plan/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/logical_plan/ddl.rs:39`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-570b029ad7d1477ad58f14a5"></a>
## schema

`function` · `datafusion_expr::logical_plan::ddl::DdlStatement::schema` · datafusion-expr 55.1.0

```rust
fn schema(&self) -> &DFSchemaRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::ddl::DdlStatement", "path": "DdlStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [68, 1], "end": [205, 2], "filename": "src/logical_plan/ddl.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/ddl.rs:70`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Get a reference to the logical plan's schema
