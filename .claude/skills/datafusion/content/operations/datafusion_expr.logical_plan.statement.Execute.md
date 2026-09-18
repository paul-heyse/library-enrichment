# `datafusion_expr::logical_plan::statement::Execute`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.logical_plan.statement.Execute.json).

<a id="op-781fd3f471c35989113aa99e"></a>
## Execute

`struct` · `datafusion_expr::logical_plan::statement::Execute` · datafusion-expr 55.1.0

```rust
struct Execute
```

Source: `src/logical_plan/statement.rs:220`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Execute a prepared statement.

<a id="op-0b56633d5dccb68b5186ed45"></a>
## clone

`function` · `datafusion_expr::logical_plan::statement::Execute::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> Execute
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::statement::Execute", "path": "Execute"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [219, 17], "end": [219, 22], "filename": "src/logical_plan/statement.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/logical_plan/statement.rs:219`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-947938a8e0a856e6da92e549"></a>
## eq

`function` · `datafusion_expr::logical_plan::statement::Execute::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &Execute) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::statement::Execute", "path": "Execute"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [219, 24], "end": [219, 33], "filename": "src/logical_plan/statement.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/logical_plan/statement.rs:219`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f21acdee0bc56cbcf0c8529a"></a>
## fmt

`function` · `datafusion_expr::logical_plan::statement::Execute::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::statement::Execute", "path": "Execute"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [219, 10], "end": [219, 15], "filename": "src/logical_plan/statement.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/logical_plan/statement.rs:219`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c63a1a2a2868f637759ef8d9"></a>
## hash

`function` · `datafusion_expr::logical_plan::statement::Execute::hash` · datafusion-expr 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::statement::Execute", "path": "Execute"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [219, 51], "end": [219, 55], "filename": "src/logical_plan/statement.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/logical_plan/statement.rs:219`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c63a4e6ca02eb4ba990b45a1"></a>
## name

`struct_field` · `datafusion_expr::logical_plan::statement::Execute::name` · datafusion-expr 55.1.0

```rust
name: String
```

Source: `src/logical_plan/statement.rs:222`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The name of the prepared statement to execute

<a id="op-44ccb91371b8fc185adabed4"></a>
## parameters

`struct_field` · `datafusion_expr::logical_plan::statement::Execute::parameters` · datafusion-expr 55.1.0

```rust
parameters: Vec<Expr>
```

Source: `src/logical_plan/statement.rs:224`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The execute parameters

<a id="op-0dfdd950e1eb750629a86088"></a>
## partial_cmp

`function` · `datafusion_expr::logical_plan::statement::Execute::partial_cmp` · datafusion-expr 55.1.0

```rust
fn partial_cmp(&self, other: &Execute) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::statement::Execute", "path": "Execute"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [219, 35], "end": [219, 45], "filename": "src/logical_plan/statement.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/logical_plan/statement.rs:219`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
