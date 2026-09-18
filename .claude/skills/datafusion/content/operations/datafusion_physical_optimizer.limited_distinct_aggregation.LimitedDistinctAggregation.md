# `datafusion_physical_optimizer::limited_distinct_aggregation::LimitedDistinctAggregation`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_optimizer.limited_distinct_aggregation.LimitedDistinctAggregation.json).

<a id="op-693c47d710872e0b585668f2"></a>
## LimitedDistinctAggregation

`struct` · `datafusion_physical_optimizer::limited_distinct_aggregation::LimitedDistinctAggregation` · datafusion-physical-optimizer 55.1.0

```rust
struct LimitedDistinctAggregation
```

Source: `src/limited_distinct_aggregation.rs:39`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

An optimizer rule that passes a `limit` hint into grouped aggregations which don't require all
rows in the group to be processed for correctness. Example queries fitting this description are:
- `SELECT distinct l_orderkey FROM lineitem LIMIT 10;`
- `SELECT l_orderkey FROM lineitem GROUP BY l_orderkey LIMIT 10;`

<a id="op-7b19c7ddc57026e51f89c6c3"></a>
## default

`function` · `datafusion_physical_optimizer::limited_distinct_aggregation::LimitedDistinctAggregation::default` · datafusion-physical-optimizer 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::limited_distinct_aggregation::LimitedDistinctAggregation", "path": "LimitedDistinctAggregation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [141, 1], "end": [145, 2], "filename": "src/limited_distinct_aggregation.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/limited_distinct_aggregation.rs:142`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ab4eebde4099ec1d7ebd7132"></a>
## fmt

`function` · `datafusion_physical_optimizer::limited_distinct_aggregation::LimitedDistinctAggregation::fmt` · datafusion-physical-optimizer 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::limited_distinct_aggregation::LimitedDistinctAggregation", "path": "LimitedDistinctAggregation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [38, 10], "end": [38, 15], "filename": "src/limited_distinct_aggregation.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/limited_distinct_aggregation.rs:38`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4fbeffa0c5c3ad1aa2b57bd1"></a>
## name

`function` · `datafusion_physical_optimizer::limited_distinct_aggregation::LimitedDistinctAggregation::name` · datafusion-physical-optimizer 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::limited_distinct_aggregation::LimitedDistinctAggregation", "path": "LimitedDistinctAggregation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [147, 1], "end": [178, 2], "filename": "src/limited_distinct_aggregation.rs"}, "trait": {"args": null, "id": "datafusion_session::physical_optimizer::PhysicalOptimizerRule", "path": "PhysicalOptimizerRule"}, "trait_path": "datafusion_session::physical_optimizer::PhysicalOptimizerRule"}`

Source: `src/limited_distinct_aggregation.rs:171`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2da0144dd0e2e2707f030e74"></a>
## new

`function` · `datafusion_physical_optimizer::limited_distinct_aggregation::LimitedDistinctAggregation::new` · datafusion-physical-optimizer 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::limited_distinct_aggregation::LimitedDistinctAggregation", "path": "LimitedDistinctAggregation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [41, 1], "end": [139, 2], "filename": "src/limited_distinct_aggregation.rs"}, "trait": null, "trait_path": null}`

Source: `src/limited_distinct_aggregation.rs:43`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

Create a new `LimitedDistinctAggregation`

<a id="op-4e4e9af62383a9a9a460bb70"></a>
## optimize

`function` · `datafusion_physical_optimizer::limited_distinct_aggregation::LimitedDistinctAggregation::optimize` · datafusion-physical-optimizer 55.1.0

```rust
fn optimize(&self, plan: Arc<dyn ExecutionPlan>, config: &ConfigOptions) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::limited_distinct_aggregation::LimitedDistinctAggregation", "path": "LimitedDistinctAggregation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [147, 1], "end": [178, 2], "filename": "src/limited_distinct_aggregation.rs"}, "trait": {"args": null, "id": "datafusion_session::physical_optimizer::PhysicalOptimizerRule", "path": "PhysicalOptimizerRule"}, "trait_path": "datafusion_session::physical_optimizer::PhysicalOptimizerRule"}`

Source: `src/limited_distinct_aggregation.rs:148`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-97469447728736ef53fd2aa4"></a>
## schema_check

`function` · `datafusion_physical_optimizer::limited_distinct_aggregation::LimitedDistinctAggregation::schema_check` · datafusion-physical-optimizer 55.1.0

```rust
fn schema_check(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::limited_distinct_aggregation::LimitedDistinctAggregation", "path": "LimitedDistinctAggregation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [147, 1], "end": [178, 2], "filename": "src/limited_distinct_aggregation.rs"}, "trait": {"args": null, "id": "datafusion_session::physical_optimizer::PhysicalOptimizerRule", "path": "PhysicalOptimizerRule"}, "trait_path": "datafusion_session::physical_optimizer::PhysicalOptimizerRule"}`

Source: `src/limited_distinct_aggregation.rs:175`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
