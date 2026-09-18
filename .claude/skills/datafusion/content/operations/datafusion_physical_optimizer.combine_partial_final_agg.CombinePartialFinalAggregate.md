# `datafusion_physical_optimizer::combine_partial_final_agg::CombinePartialFinalAggregate`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_optimizer.combine_partial_final_agg.CombinePartialFinalAggregate.json).

<a id="op-ce52cae6d51a59bcdc842ece"></a>
## CombinePartialFinalAggregate

`struct` · `datafusion_physical_optimizer::combine_partial_final_agg::CombinePartialFinalAggregate` · datafusion-physical-optimizer 55.1.0

```rust
struct CombinePartialFinalAggregate
```

Source: `src/combine_partial_final_agg.rs:41`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

CombinePartialFinalAggregate optimizer rule combines the adjacent Partial and Final AggregateExecs
into a Single AggregateExec if their grouping exprs and aggregate exprs equal.

This rule should be applied after the `EnsureRequirements` rule (which
handles both distribution and sorting enforcement).

<a id="op-b3a6101468eee2303f476286"></a>
## default

`function` · `datafusion_physical_optimizer::combine_partial_final_agg::CombinePartialFinalAggregate::default` · datafusion-physical-optimizer 55.1.0

```rust
fn default() -> CombinePartialFinalAggregate
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::combine_partial_final_agg::CombinePartialFinalAggregate", "path": "CombinePartialFinalAggregate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [40, 10], "end": [40, 17], "filename": "src/combine_partial_final_agg.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/combine_partial_final_agg.rs:40`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8c06da1f82a18a974d4e1e91"></a>
## fmt

`function` · `datafusion_physical_optimizer::combine_partial_final_agg::CombinePartialFinalAggregate::fmt` · datafusion-physical-optimizer 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::combine_partial_final_agg::CombinePartialFinalAggregate", "path": "CombinePartialFinalAggregate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [40, 19], "end": [40, 24], "filename": "src/combine_partial_final_agg.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/combine_partial_final_agg.rs:40`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7caa75bfdda60ad26055dd3c"></a>
## name

`function` · `datafusion_physical_optimizer::combine_partial_final_agg::CombinePartialFinalAggregate::name` · datafusion-physical-optimizer 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::combine_partial_final_agg::CombinePartialFinalAggregate", "path": "CombinePartialFinalAggregate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [50, 1], "end": [125, 2], "filename": "src/combine_partial_final_agg.rs"}, "trait": {"args": null, "id": "datafusion_session::physical_optimizer::PhysicalOptimizerRule", "path": "PhysicalOptimizerRule"}, "trait_path": "datafusion_session::physical_optimizer::PhysicalOptimizerRule"}`

Source: `src/combine_partial_final_agg.rs:118`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c203c0be4f38bef95c7638bc"></a>
## new

`function` · `datafusion_physical_optimizer::combine_partial_final_agg::CombinePartialFinalAggregate::new` · datafusion-physical-optimizer 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::combine_partial_final_agg::CombinePartialFinalAggregate", "path": "CombinePartialFinalAggregate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [43, 1], "end": [48, 2], "filename": "src/combine_partial_final_agg.rs"}, "trait": null, "trait_path": null}`

Source: `src/combine_partial_final_agg.rs:45`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-471d1704c2ebaad451cc86b5"></a>
## optimize

`function` · `datafusion_physical_optimizer::combine_partial_final_agg::CombinePartialFinalAggregate::optimize` · datafusion-physical-optimizer 55.1.0

```rust
fn optimize(&self, plan: Arc<dyn ExecutionPlan>, _config: &ConfigOptions) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::combine_partial_final_agg::CombinePartialFinalAggregate", "path": "CombinePartialFinalAggregate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [50, 1], "end": [125, 2], "filename": "src/combine_partial_final_agg.rs"}, "trait": {"args": null, "id": "datafusion_session::physical_optimizer::PhysicalOptimizerRule", "path": "PhysicalOptimizerRule"}, "trait_path": "datafusion_session::physical_optimizer::PhysicalOptimizerRule"}`

Source: `src/combine_partial_final_agg.rs:51`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d698b5085783babd762253a4"></a>
## schema_check

`function` · `datafusion_physical_optimizer::combine_partial_final_agg::CombinePartialFinalAggregate::schema_check` · datafusion-physical-optimizer 55.1.0

```rust
fn schema_check(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::combine_partial_final_agg::CombinePartialFinalAggregate", "path": "CombinePartialFinalAggregate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [50, 1], "end": [125, 2], "filename": "src/combine_partial_final_agg.rs"}, "trait": {"args": null, "id": "datafusion_session::physical_optimizer::PhysicalOptimizerRule", "path": "PhysicalOptimizerRule"}, "trait_path": "datafusion_session::physical_optimizer::PhysicalOptimizerRule"}`

Source: `src/combine_partial_final_agg.rs:122`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
