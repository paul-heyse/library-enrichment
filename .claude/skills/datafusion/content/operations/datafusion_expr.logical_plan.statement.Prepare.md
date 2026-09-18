# `datafusion_expr::logical_plan::statement::Prepare`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.logical_plan.statement.Prepare.json).

<a id="op-6e444d0f4fa105dfc8b9126e"></a>
## Prepare

`struct` · `datafusion_expr::logical_plan::statement::Prepare` · datafusion-expr 55.1.0

```rust
struct Prepare
```

Source: `src/logical_plan/statement.rs:209`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Prepare a statement but do not execute it. Prepare statements can have 0 or more
`Expr::Placeholder` expressions that are filled in during execution

<a id="op-4bd835d10a7daabd48a9f24c"></a>
## clone

`function` · `datafusion_expr::logical_plan::statement::Prepare::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> Prepare
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::statement::Prepare", "path": "Prepare"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [208, 17], "end": [208, 22], "filename": "src/logical_plan/statement.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/logical_plan/statement.rs:208`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-42c68331b31a51d4eaef25cb"></a>
## eq

`function` · `datafusion_expr::logical_plan::statement::Prepare::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &Prepare) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::statement::Prepare", "path": "Prepare"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [208, 24], "end": [208, 33], "filename": "src/logical_plan/statement.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/logical_plan/statement.rs:208`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-43143d72956f4277411a8c44"></a>
## fields

`struct_field` · `datafusion_expr::logical_plan::statement::Prepare::fields` · datafusion-expr 55.1.0

```rust
fields: Vec<arrow::datatypes::FieldRef>
```

Source: `src/logical_plan/statement.rs:213`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Data types of the parameters ([`Expr::Placeholder`](../operations/datafusion_expr.expr.Expr.md#op-439f205dd6f1cd4001b2dc15))

<a id="op-9450c6a07c37f23f9c7718fc"></a>
## fmt

`function` · `datafusion_expr::logical_plan::statement::Prepare::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::statement::Prepare", "path": "Prepare"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [208, 10], "end": [208, 15], "filename": "src/logical_plan/statement.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/logical_plan/statement.rs:208`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-435633a759b408a01171d017"></a>
## hash

`function` · `datafusion_expr::logical_plan::statement::Prepare::hash` · datafusion-expr 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::statement::Prepare", "path": "Prepare"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [208, 51], "end": [208, 55], "filename": "src/logical_plan/statement.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/logical_plan/statement.rs:208`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ff9849ae591df269a9ef3ff0"></a>
## input

`struct_field` · `datafusion_expr::logical_plan::statement::Prepare::input` · datafusion-expr 55.1.0

```rust
input: std::sync::Arc<LogicalPlan>
```

Source: `src/logical_plan/statement.rs:215`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The logical plan of the statements

<a id="op-260c1528f6fd75c3ef1ace5d"></a>
## name

`struct_field` · `datafusion_expr::logical_plan::statement::Prepare::name` · datafusion-expr 55.1.0

```rust
name: String
```

Source: `src/logical_plan/statement.rs:211`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The name of the statement

<a id="op-b2774a710d5763593581914f"></a>
## partial_cmp

`function` · `datafusion_expr::logical_plan::statement::Prepare::partial_cmp` · datafusion-expr 55.1.0

```rust
fn partial_cmp(&self, other: &Prepare) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::statement::Prepare", "path": "Prepare"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [208, 39], "end": [208, 49], "filename": "src/logical_plan/statement.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/logical_plan/statement.rs:208`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
