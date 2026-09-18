# `datafusion_optimizer::optimizer::OptimizerRule`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_optimizer.optimizer.OptimizerRule.json).

<a id="op-16265e807b19887d2b41f187"></a>
## OptimizerRule

`trait` · `datafusion_optimizer::optimizer::OptimizerRule` · datafusion-optimizer 55.1.0

```rust
trait OptimizerRule: Debug
```

Source: `src/optimizer.rs:83`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

Transforms one [`LogicalPlan`](../operations/datafusion_expr.logical_plan.plan.LogicalPlan.md#op-2f2092c4f87ff1cc0b33c3da) into another which computes the same results,
but in a potentially more efficient way.

See notes on [`Self::rewrite`](../operations/datafusion_optimizer.optimizer.OptimizerRule.md#op-772b75ea550d90b0f29b1cb0) for details on how to implement an `OptimizerRule`.

To change the semantics of a `LogicalPlan`, see [`AnalyzerRule`].

Use [`SessionState::add_optimizer_rule`] to register additional
`OptimizerRule`s.

[`AnalyzerRule`]: crate::analyzer::AnalyzerRule
[`SessionState::add_optimizer_rule`]: https://docs.rs/datafusion/latest/datafusion/execution/session_state/struct.SessionState.html#method.add_optimizer_rule

<a id="op-e0c3f3c77d3245a632163ab1"></a>
## apply_order

`function` · `datafusion_optimizer::optimizer::OptimizerRule::apply_order` · datafusion-optimizer 55.1.0

```rust
fn apply_order(&self) -> Option<ApplyOrder>
```

Source: `src/optimizer.rs:91`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

How should the rule be applied by the optimizer? See comments on
[`ApplyOrder`](../operations/datafusion_optimizer.optimizer.ApplyOrder.md#op-f004ab45507daf3f6c265a54) for details.

If returns `None`, the default, the rule must handle recursion itself

<a id="op-033b5831c1adad092428dbad"></a>
## name

`function` · `datafusion_optimizer::optimizer::OptimizerRule::name` · datafusion-optimizer 55.1.0

```rust
fn name(&self) -> &str
```

Source: `src/optimizer.rs:85`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

A human readable name for this optimizer rule

<a id="op-772b75ea550d90b0f29b1cb0"></a>
## rewrite

`function` · `datafusion_optimizer::optimizer::OptimizerRule::rewrite` · datafusion-optimizer 55.1.0

```rust
fn rewrite(&self, _plan: LogicalPlan, _config: &dyn OptimizerConfig) -> Result<Transformed<LogicalPlan>, DataFusionError>
```

Source: `src/optimizer.rs:135`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

Try to rewrite `plan` to an optimized form, returning [`Transformed::yes`]
if the plan was rewritten and [`Transformed::no`] if it was not.

# Notes for implementations:

## Return the same plan if no changes were made

If there are no suitable transformations for the input plan,
the optimizer should simply return it unmodified.

The optimizer will call `rewrite` several times until a fixed point is
reached, so it is important that `rewrite` return [`Transformed::no`] if
the output is the same.

## Matching on functions

The rule should avoid function-specific transformations, and instead use
methods on [`ScalarUDFImpl`] and [`AggregateUDFImpl`]. Specifically, the
rule should not check function names as functions can be overridden, and
may not have the same semantics as the functions provided with
DataFusion.

For example, if a rule rewrites a function based on the check
`func.name() == "sum"`, it may rewrite the plan incorrectly if the
registered `sum` function has different semantics (for example, the
`sum` function from the `datafusion-spark` crate).

There are still several cases that rely on function name checking in
the rules included with DataFusion. Please see [#18643] for more details
and to help remove these cases.

[`ScalarUDFImpl`]: datafusion_expr::ScalarUDFImpl
[`AggregateUDFImpl`]: datafusion_expr::ScalarUDFImpl
[#18643]: https://github.com/apache/datafusion/issues/18643

Unresolved upstream links (retained, not inferred): ``Transformed::no``, ``Transformed::yes``.

<a id="op-dfd17cb346a8c50304e9301e"></a>
## supports_rewrite

`function` · `datafusion_optimizer::optimizer::OptimizerRule::supports_rewrite` · datafusion-optimizer 55.1.0

```rust
fn supports_rewrite(&self) -> bool
```

Source: `src/optimizer.rs:97`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

Does this rule support rewriting owned plans (rather than by reference)?
