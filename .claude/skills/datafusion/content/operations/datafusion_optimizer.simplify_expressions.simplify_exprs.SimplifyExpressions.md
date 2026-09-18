# `datafusion_optimizer::simplify_expressions::simplify_exprs::SimplifyExpressions`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_optimizer.simplify_expressions.simplify_exprs.SimplifyExpressions.json).

<a id="op-94f80e58d12ab3105b2c3053"></a>
## SimplifyExpressions

`struct` · `datafusion_optimizer::simplify_expressions::simplify_exprs::SimplifyExpressions` · datafusion-optimizer 55.1.0

```rust
struct SimplifyExpressions
```

Source: `src/simplify_expressions/simplify_exprs.rs:51`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

Optimizer Pass that simplifies [`LogicalPlan`](../operations/datafusion_expr.logical_plan.plan.LogicalPlan.md#op-2f2092c4f87ff1cc0b33c3da)s by rewriting
[`Expr`]`s evaluating constants and applying algebraic
simplifications

# Introduction
It uses boolean algebra laws to simplify or reduce the number of terms in expressions.

# Example:
`Filter: b > 2 AND b > 2`
is optimized to
`Filter: b > 2`

[`Expr`]: datafusion_expr::Expr

<a id="op-3ed7fd6dbd0307a5fbc85ef9"></a>
## apply_order

`function` · `datafusion_optimizer::simplify_expressions::simplify_exprs::SimplifyExpressions::apply_order` · datafusion-optimizer 55.1.0

```rust
fn apply_order(&self) -> Option<ApplyOrder>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::simplify_expressions::simplify_exprs::SimplifyExpressions", "path": "SimplifyExpressions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [53, 1], "end": [73, 2], "filename": "src/simplify_expressions/simplify_exprs.rs"}, "trait": {"args": null, "id": "datafusion_optimizer::optimizer::OptimizerRule", "path": "OptimizerRule"}, "trait_path": "datafusion_optimizer::optimizer::OptimizerRule"}`

Source: `src/simplify_expressions/simplify_exprs.rs:58`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-18cc4bad88868e731a7b3326"></a>
## default

`function` · `datafusion_optimizer::simplify_expressions::simplify_exprs::SimplifyExpressions::default` · datafusion-optimizer 55.1.0

```rust
fn default() -> SimplifyExpressions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::simplify_expressions::simplify_exprs::SimplifyExpressions", "path": "SimplifyExpressions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [50, 10], "end": [50, 17], "filename": "src/simplify_expressions/simplify_exprs.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/simplify_expressions/simplify_exprs.rs:50`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c34cb2f6aa4b38e2c61a7499"></a>
## fmt

`function` · `datafusion_optimizer::simplify_expressions::simplify_exprs::SimplifyExpressions::fmt` · datafusion-optimizer 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::simplify_expressions::simplify_exprs::SimplifyExpressions", "path": "SimplifyExpressions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [50, 19], "end": [50, 24], "filename": "src/simplify_expressions/simplify_exprs.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/simplify_expressions/simplify_exprs.rs:50`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-01b7d441fe1843797cb48494"></a>
## name

`function` · `datafusion_optimizer::simplify_expressions::simplify_exprs::SimplifyExpressions::name` · datafusion-optimizer 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::simplify_expressions::simplify_exprs::SimplifyExpressions", "path": "SimplifyExpressions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [53, 1], "end": [73, 2], "filename": "src/simplify_expressions/simplify_exprs.rs"}, "trait": {"args": null, "id": "datafusion_optimizer::optimizer::OptimizerRule", "path": "OptimizerRule"}, "trait_path": "datafusion_optimizer::optimizer::OptimizerRule"}`

Source: `src/simplify_expressions/simplify_exprs.rs:54`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6c56e5fb7053b25407d37cae"></a>
## new

`function` · `datafusion_optimizer::simplify_expressions::simplify_exprs::SimplifyExpressions::new` · datafusion-optimizer 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::simplify_expressions::simplify_exprs::SimplifyExpressions", "path": "SimplifyExpressions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [161, 1], "end": [166, 2], "filename": "src/simplify_expressions/simplify_exprs.rs"}, "trait": null, "trait_path": null}`

Source: `src/simplify_expressions/simplify_exprs.rs:163`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-34a4ce859384bf3b2144da37"></a>
## rewrite

`function` · `datafusion_optimizer::simplify_expressions::simplify_exprs::SimplifyExpressions::rewrite` · datafusion-optimizer 55.1.0

```rust
fn rewrite(&self, plan: LogicalPlan, config: &dyn OptimizerConfig) -> Result<Transformed<LogicalPlan>, DataFusionError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::simplify_expressions::simplify_exprs::SimplifyExpressions", "path": "SimplifyExpressions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [53, 1], "end": [73, 2], "filename": "src/simplify_expressions/simplify_exprs.rs"}, "trait": {"args": null, "id": "datafusion_optimizer::optimizer::OptimizerRule", "path": "OptimizerRule"}, "trait_path": "datafusion_optimizer::optimizer::OptimizerRule"}`

Source: `src/simplify_expressions/simplify_exprs.rs:66`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-26de307094fb5e4914315eed"></a>
## supports_rewrite

`function` · `datafusion_optimizer::simplify_expressions::simplify_exprs::SimplifyExpressions::supports_rewrite` · datafusion-optimizer 55.1.0

```rust
fn supports_rewrite(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::simplify_expressions::simplify_exprs::SimplifyExpressions", "path": "SimplifyExpressions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [53, 1], "end": [73, 2], "filename": "src/simplify_expressions/simplify_exprs.rs"}, "trait": {"args": null, "id": "datafusion_optimizer::optimizer::OptimizerRule", "path": "OptimizerRule"}, "trait_path": "datafusion_optimizer::optimizer::OptimizerRule"}`

Source: `src/simplify_expressions/simplify_exprs.rs:62`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
