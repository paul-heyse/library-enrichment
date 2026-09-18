# `datafusion_physical_optimizer::update_aggr_exprs::OptimizeAggregateOrder`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_optimizer.update_aggr_exprs.OptimizeAggregateOrder.json).

<a id="op-2969b17b832bb9faf37ae2af"></a>
## OptimizeAggregateOrder

`struct` · `datafusion_physical_optimizer::update_aggr_exprs::OptimizeAggregateOrder` · datafusion-physical-optimizer 55.1.0

```rust
struct OptimizeAggregateOrder
```

Source: `src/update_aggr_exprs.rs:51`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

This optimizer rule checks ordering requirements of aggregate expressions.

There are 3 kinds of aggregators in terms of ordering requirements:
- `AggregateOrderSensitivity::Insensitive`, meaning that ordering is not
  important.
- `AggregateOrderSensitivity::HardRequirement`, meaning that the aggregator
  requires a specific ordering.
- `AggregateOrderSensitivity::Beneficial`, meaning that the aggregator can
  handle unordered input, but can run more efficiently if its input conforms
  to a specific ordering.

This rule analyzes aggregate expressions of type `Beneficial` to see whether
their input ordering requirements are satisfied. If this is the case, the
aggregators are modified to run in a more efficient mode.

<a id="op-6d62ff16c4feb27a84fc4134"></a>
## default

`function` · `datafusion_physical_optimizer::update_aggr_exprs::OptimizeAggregateOrder::default` · datafusion-physical-optimizer 55.1.0

```rust
fn default() -> OptimizeAggregateOrder
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::update_aggr_exprs::OptimizeAggregateOrder", "path": "OptimizeAggregateOrder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [50, 10], "end": [50, 17], "filename": "src/update_aggr_exprs.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/update_aggr_exprs.rs:50`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d4a64e89b9c4587484f5a0ff"></a>
## fmt

`function` · `datafusion_physical_optimizer::update_aggr_exprs::OptimizeAggregateOrder::fmt` · datafusion-physical-optimizer 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::update_aggr_exprs::OptimizeAggregateOrder", "path": "OptimizeAggregateOrder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [50, 19], "end": [50, 24], "filename": "src/update_aggr_exprs.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/update_aggr_exprs.rs:50`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9f04417e3cd2b825ee120dc0"></a>
## name

`function` · `datafusion_physical_optimizer::update_aggr_exprs::OptimizeAggregateOrder::name` · datafusion-physical-optimizer 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::update_aggr_exprs::OptimizeAggregateOrder", "path": "OptimizeAggregateOrder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [60, 1], "end": [130, 2], "filename": "src/update_aggr_exprs.rs"}, "trait": {"args": null, "id": "datafusion_session::physical_optimizer::PhysicalOptimizerRule", "path": "PhysicalOptimizerRule"}, "trait_path": "datafusion_session::physical_optimizer::PhysicalOptimizerRule"}`

Source: `src/update_aggr_exprs.rs:123`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ead770b6ebbcf48bfd4d8119"></a>
## new

`function` · `datafusion_physical_optimizer::update_aggr_exprs::OptimizeAggregateOrder::new` · datafusion-physical-optimizer 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::update_aggr_exprs::OptimizeAggregateOrder", "path": "OptimizeAggregateOrder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [53, 1], "end": [58, 2], "filename": "src/update_aggr_exprs.rs"}, "trait": null, "trait_path": null}`

Source: `src/update_aggr_exprs.rs:55`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2004c3de7a41fa211f6a0cff"></a>
## optimize

`function` · `datafusion_physical_optimizer::update_aggr_exprs::OptimizeAggregateOrder::optimize` · datafusion-physical-optimizer 55.1.0

```rust
fn optimize(&self, plan: Arc<dyn ExecutionPlan>, _config: &ConfigOptions) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::update_aggr_exprs::OptimizeAggregateOrder", "path": "OptimizeAggregateOrder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [60, 1], "end": [130, 2], "filename": "src/update_aggr_exprs.rs"}, "trait": {"args": null, "id": "datafusion_session::physical_optimizer::PhysicalOptimizerRule", "path": "PhysicalOptimizerRule"}, "trait_path": "datafusion_session::physical_optimizer::PhysicalOptimizerRule"}`

Source: `src/update_aggr_exprs.rs:75`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

Applies the `OptimizeAggregateOrder` rule to the provided execution plan.

This function traverses the execution plan tree, identifies `AggregateExec` nodes,
and optimizes their aggregate expressions based on existing input orderings.
If optimizations are applied, it returns a modified execution plan.

# Arguments

* `plan` - The root of the execution plan to optimize.
* `_config` - Configuration options (currently unused).

# Returns

A `Result` containing the potentially optimized execution plan or an error.

<a id="op-a9577977a4f128617d7f96b4"></a>
## schema_check

`function` · `datafusion_physical_optimizer::update_aggr_exprs::OptimizeAggregateOrder::schema_check` · datafusion-physical-optimizer 55.1.0

```rust
fn schema_check(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::update_aggr_exprs::OptimizeAggregateOrder", "path": "OptimizeAggregateOrder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [60, 1], "end": [130, 2], "filename": "src/update_aggr_exprs.rs"}, "trait": {"args": null, "id": "datafusion_session::physical_optimizer::PhysicalOptimizerRule", "path": "PhysicalOptimizerRule"}, "trait_path": "datafusion_session::physical_optimizer::PhysicalOptimizerRule"}`

Source: `src/update_aggr_exprs.rs:127`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
