# `datafusion_expr::expr::AggregateFunction`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.expr.AggregateFunction.json).

<a id="op-2c30152da50e6f4ca7d2566e"></a>
## AggregateFunction

`struct` · `datafusion_expr::expr::AggregateFunction` · datafusion-expr 55.1.0

```rust
struct AggregateFunction
```

Source: `src/expr.rs:1114`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Aggregate function

See also  [`ExprFunctionExt`] to set these fields on `Expr`

[`ExprFunctionExt`]: crate::expr_fn::ExprFunctionExt

<a id="op-6365447261b4376127a07dfa"></a>
## clone

`function` · `datafusion_expr::expr::AggregateFunction::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> AggregateFunction
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::AggregateFunction", "path": "AggregateFunction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1113, 10], "end": [1113, 15], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/expr.rs:1113`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2acd273f8f6adf5811e0fb91"></a>
## eq

`function` · `datafusion_expr::expr::AggregateFunction::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &AggregateFunction) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::AggregateFunction", "path": "AggregateFunction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1113, 17], "end": [1113, 26], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/expr.rs:1113`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-56696c5a15193d7cf3705979"></a>
## fmt

`function` · `datafusion_expr::expr::AggregateFunction::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::AggregateFunction", "path": "AggregateFunction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1113, 50], "end": [1113, 55], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/expr.rs:1113`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1b3da27e601d41d61ad760f7"></a>
## func

`struct_field` · `datafusion_expr::expr::AggregateFunction::func` · datafusion-expr 55.1.0

```rust
func: std::sync::Arc<AggregateUDF>
```

Source: `src/expr.rs:1116`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Name of the function

<a id="op-b0d33f43bd9f94f01fd8ae4a"></a>
## hash

`function` · `datafusion_expr::expr::AggregateFunction::hash` · datafusion-expr 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::AggregateFunction", "path": "AggregateFunction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1113, 44], "end": [1113, 48], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/expr.rs:1113`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-790803f606e1705cd223a434"></a>
## new_udf

`function` · `datafusion_expr::expr::AggregateFunction::new_udf` · datafusion-expr 55.1.0

```rust
fn new_udf(func: Arc<AggregateUDF>, args: Vec<Expr>, distinct: bool, filter: Option<Box<Expr>>, order_by: Vec<Sort>, null_treatment: Option<NullTreatment>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::AggregateFunction", "path": "AggregateFunction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1132, 1], "end": [1153, 2], "filename": "src/expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/expr.rs:1134`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Create a new AggregateFunction expression with a user-defined function (UDF)

<a id="op-33cbf6b049e72a877c2f3cc8"></a>
## params

`struct_field` · `datafusion_expr::expr::AggregateFunction::params` · datafusion-expr 55.1.0

```rust
params: AggregateFunctionParams
```

Source: `src/expr.rs:1117`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dfb50fc5658c50af1abefdc2"></a>
## partial_cmp

`function` · `datafusion_expr::expr::AggregateFunction::partial_cmp` · datafusion-expr 55.1.0

```rust
fn partial_cmp(&self, other: &AggregateFunction) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::AggregateFunction", "path": "AggregateFunction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1113, 32], "end": [1113, 42], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/expr.rs:1113`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
