# `datafusion_expr::expr::HigherOrderFunction`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.expr.HigherOrderFunction.json).

<a id="op-116039d54aad245a60edcd39"></a>
## HigherOrderFunction

`struct` · `datafusion_expr::expr::HigherOrderFunction` · datafusion-expr 55.1.0

```rust
struct HigherOrderFunction
```

Source: `src/expr.rs:438`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Invoke a [`HigherOrderUDF`](../operations/datafusion_expr.higher_order_function.HigherOrderUDF.md#op-67b8632773d35bc58334e2bb) with a set of arguments

<a id="op-00e77d9fe3aab22b344b261b"></a>
## args

`struct_field` · `datafusion_expr::expr::HigherOrderFunction::args` · datafusion-expr 55.1.0

```rust
args: Vec<Expr>
```

Source: `src/expr.rs:442`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

List of expressions to feed to the functions as arguments

<a id="op-0bdc6b6c040e21c920a996b4"></a>
## clone

`function` · `datafusion_expr::expr::HigherOrderFunction::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> HigherOrderFunction
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::HigherOrderFunction", "path": "HigherOrderFunction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [437, 10], "end": [437, 15], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/expr.rs:437`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-eaf38830e47c5a4474709bcb"></a>
## eq

`function` · `datafusion_expr::expr::HigherOrderFunction::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &Self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::HigherOrderFunction", "path": "HigherOrderFunction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [497, 1], "end": [501, 2], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/expr.rs:498`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-619e0547543184d3180680a7"></a>
## fmt

`function` · `datafusion_expr::expr::HigherOrderFunction::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::HigherOrderFunction", "path": "HigherOrderFunction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [437, 33], "end": [437, 38], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/expr.rs:437`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-386e61fcf77ddbb3e4c336d8"></a>
## func

`struct_field` · `datafusion_expr::expr::HigherOrderFunction::func` · datafusion-expr 55.1.0

```rust
func: std::sync::Arc<higher_order_function::HigherOrderUDF>
```

Source: `src/expr.rs:440`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The function

<a id="op-b5ab16b15056e358bc9fa6f0"></a>
## hash

`function` · `datafusion_expr::expr::HigherOrderFunction::hash` · datafusion-expr 55.1.0

```rust
fn hash<H: Hasher>(&self, state: &mut H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::HigherOrderFunction", "path": "HigherOrderFunction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [490, 1], "end": [495, 2], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/expr.rs:491`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-46e2ce5cdc505e08e406b318"></a>
## lambda_parameters

`function` · `datafusion_expr::expr::HigherOrderFunction::lambda_parameters` · datafusion-expr 55.1.0

```rust
fn lambda_parameters(&self, schema: &dyn ExprSchema) -> Result<Vec<Vec<FieldRef>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::HigherOrderFunction", "path": "HigherOrderFunction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [445, 1], "end": [488, 2], "filename": "src/expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/expr.rs:462`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Invokes the inner function [`crate::HigherOrderUDFImpl::lambda_parameters`](../operations/datafusion_expr.higher_order_function.HigherOrderUDFImpl.md#op-8a89b1f3117f6cf763ede484)
using the arguments of this invocation. This expression lambda
variables must be already resolved either by coming from the
default sql planner or by calling [Expr::resolve_lambda_variables](../operations/datafusion_expr.expr.Expr.md#op-a84a19ab1b8cbaf85e0d80f0)
or [LogicalPlan::resolve_lambda_variables]

[LogicalPlan::resolve_lambda_variables]: crate::LogicalPlan::resolve_lambda_variables

<a id="op-5614e321164bd0cd714523dc"></a>
## name

`function` · `datafusion_expr::expr::HigherOrderFunction::name` · datafusion-expr 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::HigherOrderFunction", "path": "HigherOrderFunction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [445, 1], "end": [488, 2], "filename": "src/expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/expr.rs:451`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a9f1eb692e33dd98f412c257"></a>
## new

`function` · `datafusion_expr::expr::HigherOrderFunction::new` · datafusion-expr 55.1.0

```rust
fn new(func: Arc<HigherOrderUDF>, args: Vec<Expr>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::HigherOrderFunction", "path": "HigherOrderFunction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [445, 1], "end": [488, 2], "filename": "src/expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/expr.rs:447`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Create a new `HigherOrderFunction` from a [`HigherOrderUDF`](../operations/datafusion_expr.higher_order_function.HigherOrderUDF.md#op-67b8632773d35bc58334e2bb)

<a id="op-f45473b91a7a3f641f201d83"></a>
## partial_cmp

`function` · `datafusion_expr::expr::HigherOrderFunction::partial_cmp` · datafusion-expr 55.1.0

```rust
fn partial_cmp(&self, other: &HigherOrderFunction) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::HigherOrderFunction", "path": "HigherOrderFunction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [437, 21], "end": [437, 31], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/expr.rs:437`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
