# `datafusion_session::physical_optimizer::PhysicalOptimizerRule`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_session.physical_optimizer.PhysicalOptimizerRule.json).

<a id="op-266e99a7574020b03ac0e686"></a>
## PhysicalOptimizerRule

`trait` · `datafusion_session::physical_optimizer::PhysicalOptimizerRule` · datafusion-session 55.1.0

```rust
trait PhysicalOptimizerRule: Debug + std::any::Any
```

Source: `src/physical_optimizer.rs:52`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

`PhysicalOptimizerRule` transforms one [`ExecutionPlan`](../operations/datafusion_physical_plan.execution_plan.ExecutionPlan.md#op-ac09436cd73f869917923673) into another which
computes the same results, but in a potentially more efficient way.

Use [`SessionState::add_physical_optimizer_rule`] to register additional
`PhysicalOptimizerRule`s.

[`SessionState::add_physical_optimizer_rule`]: https://docs.rs/datafusion/latest/datafusion/execution/session_state/struct.SessionState.html#method.add_physical_optimizer_rule

<a id="op-ae368f6be07655b6377435f7"></a>
## name

`function` · `datafusion_session::physical_optimizer::PhysicalOptimizerRule::name` · datafusion-session 55.1.0

```rust
fn name(&self) -> &str
```

Source: `src/physical_optimizer.rs:77`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

A human readable name for this optimizer rule

<a id="op-1b3621bc25f7006610801b4e"></a>
## optimize

`function` · `datafusion_session::physical_optimizer::PhysicalOptimizerRule::optimize` · datafusion-session 55.1.0

```rust
fn optimize(&self, plan: Arc<dyn ExecutionPlan>, config: &ConfigOptions) -> Result<Arc<dyn ExecutionPlan>>
```

Source: `src/physical_optimizer.rs:57`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

Rewrite `plan` to an optimized form.

This is the primary optimization method. For rules that need access to
the statistics registry, override [`optimize_with_context`](Self::optimize_with_context) instead.

<a id="op-3d21f3b1d577eee3e74a5264"></a>
## optimize_with_context

`function` · `datafusion_session::physical_optimizer::PhysicalOptimizerRule::optimize_with_context` · datafusion-session 55.1.0

```rust
fn optimize_with_context(&self, plan: Arc<dyn ExecutionPlan>, context: &dyn PhysicalOptimizerContext) -> Result<Arc<dyn ExecutionPlan>>
```

Source: `src/physical_optimizer.rs:68`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

Rewrite `plan` with access to extended context (statistics registry, etc.).

Override this method if you need access to the statistics registry for
enhanced statistics lookup. The default implementation simply calls
[`optimize`](Self::optimize) with the config options from the context.

<a id="op-4b931427781cd598950c7996"></a>
## schema_check

`function` · `datafusion_session::physical_optimizer::PhysicalOptimizerRule::schema_check` · datafusion-session 55.1.0

```rust
fn schema_check(&self) -> bool
```

Source: `src/physical_optimizer.rs:83`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

A flag to indicate whether the physical planner should validate that the rule will not
change the schema of the plan after the rewriting.
Some of the optimization rules might change the nullable properties of the schema
and should disable the schema check.
