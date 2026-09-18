# `datafusion::physical_planner::DefaultPhysicalPlanner`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion.physical_planner.DefaultPhysicalPlanner.json).

<a id="op-c0521de1c5fa7d7c8cae7569"></a>
## DefaultPhysicalPlanner

`struct` · `datafusion::physical_planner::DefaultPhysicalPlanner` · datafusion 55.1.0

```rust
struct DefaultPhysicalPlanner
```

Source: `src/physical_planner.rs:149`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Default single node physical query planner that converts a
`LogicalPlan` to an `ExecutionPlan` suitable for execution.

This planner first flattens the `LogicalPlan` tree with a depth-first
traversal. It then builds the physical plan from the leaves to the root.
Up to [`planning_concurrency`] tasks execute concurrently.

[`planning_concurrency`]: crate::config::ExecutionOptions::planning_concurrency

<a id="op-cbe2808c819e58a2ccc8f2b6"></a>
## create_physical_expr

`function` · `datafusion::physical_planner::DefaultPhysicalPlanner::create_physical_expr` · datafusion 55.1.0

```rust
fn create_physical_expr(&self, expr: &Expr, input_dfschema: &DFSchema, session_state: &dyn Session, planning_ctx: &PhysicalPlanningContext) -> Result<Arc<dyn PhysicalExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::physical_planner::DefaultPhysicalPlanner", "path": "DefaultPhysicalPlanner"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [154, 1], "end": [194, 2], "filename": "src/physical_planner.rs"}, "trait": {"args": null, "id": "datafusion_session::planner::PhysicalPlanner", "path": "PhysicalPlanner"}, "trait_path": "datafusion_session::planner::PhysicalPlanner"}`

Source: `src/physical_planner.rs:180`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Create a physical expression from a logical expression
suitable for evaluation

`e`: the expression to convert

`input_dfschema`: the logical plan schema for evaluating `e`

<a id="op-6c38c79dc469f311bce53835"></a>
## create_physical_plan

`function` · `datafusion::physical_planner::DefaultPhysicalPlanner::create_physical_plan` · datafusion 55.1.0

```rust
async fn create_physical_plan(&self, logical_plan: &LogicalPlan, session_state: &dyn Session) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::physical_planner::DefaultPhysicalPlanner", "path": "DefaultPhysicalPlanner"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [154, 1], "end": [194, 2], "filename": "src/physical_planner.rs"}, "trait": {"args": null, "id": "datafusion_session::planner::PhysicalPlanner", "path": "PhysicalPlanner"}, "trait_path": "datafusion_session::planner::PhysicalPlanner"}`

Source: `src/physical_planner.rs:156`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Create a physical plan from a logical plan

<a id="op-a0813cd104709301e0a55e7e"></a>
## default

`function` · `datafusion::physical_planner::DefaultPhysicalPlanner::default` · datafusion 55.1.0

```rust
fn default() -> DefaultPhysicalPlanner
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::physical_planner::DefaultPhysicalPlanner", "path": "DefaultPhysicalPlanner"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [148, 10], "end": [148, 17], "filename": "src/physical_planner.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/physical_planner.rs:148`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-821cc4fac659927144ae8782"></a>
## optimize_physical_plan

`function` · `datafusion::physical_planner::DefaultPhysicalPlanner::optimize_physical_plan` · datafusion 55.1.0

```rust
fn optimize_physical_plan<F>(&self, plan: Arc<dyn ExecutionPlan>, session_state: &dyn Session, observer: F) -> Result<Arc<dyn ExecutionPlan>> where F: FnMut(&dyn ExecutionPlan, &dyn PhysicalOptimizerRule)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::physical_planner::DefaultPhysicalPlanner", "path": "DefaultPhysicalPlanner"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2609, 1], "end": [3128, 2], "filename": "src/physical_planner.rs"}, "trait": null, "trait_path": null}`

Source: `src/physical_planner.rs:2855`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Optimize a physical plan by applying each physical optimizer,
calling observer(plan, optimizer after each one)

<a id="op-3e0097b685037f44db2132dc"></a>
## with_extension_planners

`function` · `datafusion::physical_planner::DefaultPhysicalPlanner::with_extension_planners` · datafusion 55.1.0

```rust
fn with_extension_planners(extension_planners: Vec<Arc<dyn ExtensionPlanner + Send + Sync>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::physical_planner::DefaultPhysicalPlanner", "path": "DefaultPhysicalPlanner"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [253, 1], "end": [1927, 2], "filename": "src/physical_planner.rs"}, "trait": null, "trait_path": null}`

Source: `src/physical_planner.rs:259`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Create a physical planner that uses `extension_planners` to
plan user-defined logical nodes [`LogicalPlan::Extension`](../operations/datafusion_expr.logical_plan.plan.LogicalPlan.md#op-668ccb8cbe155ed07cf5a315)
or user-defined table sources in [`LogicalPlan::TableScan`](../operations/datafusion_expr.logical_plan.plan.LogicalPlan.md#op-3b721470f2fbd687699e58a8).
The planner uses the first [`ExtensionPlanner`](../operations/datafusion_session.planner.ExtensionPlanner.md#op-b97e1eb479e5c979b3058c3c) to return a non-`None`
plan.
