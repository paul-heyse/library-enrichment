# `datafusion_physical_optimizer::topk_aggregation::TopKAggregation`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_optimizer.topk_aggregation.TopKAggregation.json).

<a id="op-b9fc584defad1b33356a6558"></a>
## TopKAggregation

`struct` · `datafusion_physical_optimizer::topk_aggregation::TopKAggregation` · datafusion-physical-optimizer 55.1.0

```rust
struct TopKAggregation
```

Source: `src/topk_aggregation.rs:37`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

An optimizer rule that passes a `limit` hint to aggregations if the whole result is not needed

<a id="op-991241b2deb1769773f90dd8"></a>
## default

`function` · `datafusion_physical_optimizer::topk_aggregation::TopKAggregation::default` · datafusion-physical-optimizer 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::topk_aggregation::TopKAggregation", "path": "TopKAggregation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [175, 1], "end": [179, 2], "filename": "src/topk_aggregation.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/topk_aggregation.rs:176`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-23a34230b92bca36db34df6f"></a>
## fmt

`function` · `datafusion_physical_optimizer::topk_aggregation::TopKAggregation::fmt` · datafusion-physical-optimizer 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::topk_aggregation::TopKAggregation", "path": "TopKAggregation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [36, 10], "end": [36, 15], "filename": "src/topk_aggregation.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/topk_aggregation.rs:36`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e9478a7ca2c26290e5638492"></a>
## name

`function` · `datafusion_physical_optimizer::topk_aggregation::TopKAggregation::name` · datafusion-physical-optimizer 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::topk_aggregation::TopKAggregation", "path": "TopKAggregation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [181, 1], "end": [208, 2], "filename": "src/topk_aggregation.rs"}, "trait": {"args": null, "id": "datafusion_session::physical_optimizer::PhysicalOptimizerRule", "path": "PhysicalOptimizerRule"}, "trait_path": "datafusion_session::physical_optimizer::PhysicalOptimizerRule"}`

Source: `src/topk_aggregation.rs:201`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bd1fe3961bc30bbf7a3760fd"></a>
## new

`function` · `datafusion_physical_optimizer::topk_aggregation::TopKAggregation::new` · datafusion-physical-optimizer 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::topk_aggregation::TopKAggregation", "path": "TopKAggregation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [39, 1], "end": [173, 2], "filename": "src/topk_aggregation.rs"}, "trait": null, "trait_path": null}`

Source: `src/topk_aggregation.rs:41`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

Create a new `LimitAggregation`

<a id="op-07075b200e510678caa7e613"></a>
## optimize

`function` · `datafusion_physical_optimizer::topk_aggregation::TopKAggregation::optimize` · datafusion-physical-optimizer 55.1.0

```rust
fn optimize(&self, plan: Arc<dyn ExecutionPlan>, config: &ConfigOptions) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::topk_aggregation::TopKAggregation", "path": "TopKAggregation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [181, 1], "end": [208, 2], "filename": "src/topk_aggregation.rs"}, "trait": {"args": null, "id": "datafusion_session::physical_optimizer::PhysicalOptimizerRule", "path": "PhysicalOptimizerRule"}, "trait_path": "datafusion_session::physical_optimizer::PhysicalOptimizerRule"}`

Source: `src/topk_aggregation.rs:182`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-453c7edad06e7ce0ec9f7f9d"></a>
## schema_check

`function` · `datafusion_physical_optimizer::topk_aggregation::TopKAggregation::schema_check` · datafusion-physical-optimizer 55.1.0

```rust
fn schema_check(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::topk_aggregation::TopKAggregation", "path": "TopKAggregation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [181, 1], "end": [208, 2], "filename": "src/topk_aggregation.rs"}, "trait": {"args": null, "id": "datafusion_session::physical_optimizer::PhysicalOptimizerRule", "path": "PhysicalOptimizerRule"}, "trait_path": "datafusion_session::physical_optimizer::PhysicalOptimizerRule"}`

Source: `src/topk_aggregation.rs:205`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
