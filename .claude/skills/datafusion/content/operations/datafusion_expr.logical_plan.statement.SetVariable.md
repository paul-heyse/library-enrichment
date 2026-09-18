# `datafusion_expr::logical_plan::statement::SetVariable`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.logical_plan.statement.SetVariable.json).

<a id="op-e0a2458e9b8942ee6871680e"></a>
## SetVariable

`struct` · `datafusion_expr::logical_plan::statement::SetVariable` · datafusion-expr 55.1.0

```rust
struct SetVariable
```

Source: `src/logical_plan/statement.rs:193`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Set a Variable's value -- value in
[`ConfigOptions`](datafusion_common::config::ConfigOptions)

<a id="op-0df663b4c754dc3426181363"></a>
## clone

`function` · `datafusion_expr::logical_plan::statement::SetVariable::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> SetVariable
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::statement::SetVariable", "path": "SetVariable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [192, 17], "end": [192, 22], "filename": "src/logical_plan/statement.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/logical_plan/statement.rs:192`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3f2dc2d6b6fc0bca6dd3e43a"></a>
## eq

`function` · `datafusion_expr::logical_plan::statement::SetVariable::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &SetVariable) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::statement::SetVariable", "path": "SetVariable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [192, 24], "end": [192, 33], "filename": "src/logical_plan/statement.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/logical_plan/statement.rs:192`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0cbe42fe645e6de4664a0a82"></a>
## fmt

`function` · `datafusion_expr::logical_plan::statement::SetVariable::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::statement::SetVariable", "path": "SetVariable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [192, 10], "end": [192, 15], "filename": "src/logical_plan/statement.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/logical_plan/statement.rs:192`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9ac62d88afe16f961e2ffbf1"></a>
## hash

`function` · `datafusion_expr::logical_plan::statement::SetVariable::hash` · datafusion-expr 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::statement::SetVariable", "path": "SetVariable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [192, 51], "end": [192, 55], "filename": "src/logical_plan/statement.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/logical_plan/statement.rs:192`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c1541b12bc7573704b0656a0"></a>
## partial_cmp

`function` · `datafusion_expr::logical_plan::statement::SetVariable::partial_cmp` · datafusion-expr 55.1.0

```rust
fn partial_cmp(&self, other: &SetVariable) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::statement::SetVariable", "path": "SetVariable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [192, 35], "end": [192, 45], "filename": "src/logical_plan/statement.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/logical_plan/statement.rs:192`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-514465f33f353b725fc06769"></a>
## value

`struct_field` · `datafusion_expr::logical_plan::statement::SetVariable::value` · datafusion-expr 55.1.0

```rust
value: String
```

Source: `src/logical_plan/statement.rs:197`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The value to set

<a id="op-d7b3bd7be8826daef7869c3e"></a>
## variable

`struct_field` · `datafusion_expr::logical_plan::statement::SetVariable::variable` · datafusion-expr 55.1.0

```rust
variable: String
```

Source: `src/logical_plan/statement.rs:195`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The variable name
