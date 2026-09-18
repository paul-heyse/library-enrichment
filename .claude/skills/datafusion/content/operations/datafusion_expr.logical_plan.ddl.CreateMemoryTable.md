# `datafusion_expr::logical_plan::ddl::CreateMemoryTable`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.logical_plan.ddl.CreateMemoryTable.json).

<a id="op-5139f0898b3555528b7abcf7"></a>
## CreateMemoryTable

`struct` · `datafusion_expr::logical_plan::ddl::CreateMemoryTable` · datafusion-expr 55.1.0

```rust
struct CreateMemoryTable
```

Source: `src/logical_plan/ddl.rs:478`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Creates an in memory table.

<a id="op-8a81e31b91b33db34e771da1"></a>
## clone

`function` · `datafusion_expr::logical_plan::ddl::CreateMemoryTable::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> CreateMemoryTable
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::ddl::CreateMemoryTable", "path": "CreateMemoryTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [477, 17], "end": [477, 22], "filename": "src/logical_plan/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/logical_plan/ddl.rs:477`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bf68967aa26e9ef9539a7110"></a>
## column_defaults

`struct_field` · `datafusion_expr::logical_plan::ddl::CreateMemoryTable::column_defaults` · datafusion-expr 55.1.0

```rust
column_defaults: Vec<(String, Expr)>
```

Source: `src/logical_plan/ddl.rs:490`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Default values for columns

<a id="op-fe13b9cdfce36ab87ba5e7fc"></a>
## constraints

`struct_field` · `datafusion_expr::logical_plan::ddl::CreateMemoryTable::constraints` · datafusion-expr 55.1.0

```rust
constraints: datafusion_common::Constraints
```

Source: `src/logical_plan/ddl.rs:482`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The list of constraints in the schema, such as primary key, unique, etc.

<a id="op-0057b8d037cac6003b2203ba"></a>
## eq

`function` · `datafusion_expr::logical_plan::ddl::CreateMemoryTable::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &CreateMemoryTable) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::ddl::CreateMemoryTable", "path": "CreateMemoryTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [477, 24], "end": [477, 33], "filename": "src/logical_plan/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/logical_plan/ddl.rs:477`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1486772fb7dcbf3c5c0e24bd"></a>
## fmt

`function` · `datafusion_expr::logical_plan::ddl::CreateMemoryTable::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::ddl::CreateMemoryTable", "path": "CreateMemoryTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [477, 10], "end": [477, 15], "filename": "src/logical_plan/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/logical_plan/ddl.rs:477`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-53d3c782d37e8255bed6d253"></a>
## hash

`function` · `datafusion_expr::logical_plan::ddl::CreateMemoryTable::hash` · datafusion-expr 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::ddl::CreateMemoryTable", "path": "CreateMemoryTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [477, 51], "end": [477, 55], "filename": "src/logical_plan/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/logical_plan/ddl.rs:477`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f323ca996b55204a7b16da0c"></a>
## if_not_exists

`struct_field` · `datafusion_expr::logical_plan::ddl::CreateMemoryTable::if_not_exists` · datafusion-expr 55.1.0

```rust
if_not_exists: bool
```

Source: `src/logical_plan/ddl.rs:486`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Option to not error if table already exists

<a id="op-f24d8713f37cf2da0619b955"></a>
## input

`struct_field` · `datafusion_expr::logical_plan::ddl::CreateMemoryTable::input` · datafusion-expr 55.1.0

```rust
input: std::sync::Arc<LogicalPlan>
```

Source: `src/logical_plan/ddl.rs:484`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The logical plan

<a id="op-37ccb35ce091a8eb8d975662"></a>
## name

`struct_field` · `datafusion_expr::logical_plan::ddl::CreateMemoryTable::name` · datafusion-expr 55.1.0

```rust
name: datafusion_common::TableReference
```

Source: `src/logical_plan/ddl.rs:480`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The table name

<a id="op-493e138431334faf30a1012c"></a>
## or_replace

`struct_field` · `datafusion_expr::logical_plan::ddl::CreateMemoryTable::or_replace` · datafusion-expr 55.1.0

```rust
or_replace: bool
```

Source: `src/logical_plan/ddl.rs:488`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Option to replace table content if table already exists

<a id="op-db9724e0e5081d94e7316396"></a>
## partial_cmp

`function` · `datafusion_expr::logical_plan::ddl::CreateMemoryTable::partial_cmp` · datafusion-expr 55.1.0

```rust
fn partial_cmp(&self, other: &CreateMemoryTable) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::ddl::CreateMemoryTable", "path": "CreateMemoryTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [477, 39], "end": [477, 49], "filename": "src/logical_plan/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/logical_plan/ddl.rs:477`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fcf8565849aff0079e4747fc"></a>
## temporary

`struct_field` · `datafusion_expr::logical_plan::ddl::CreateMemoryTable::temporary` · datafusion-expr 55.1.0

```rust
temporary: bool
```

Source: `src/logical_plan/ddl.rs:492`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Whether the table is `TableType::Temporary`
