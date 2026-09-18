# `datafusion_physical_optimizer::aggregate_statistics::AggregateStatistics`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_optimizer.aggregate_statistics.AggregateStatistics.json).

<a id="op-94a1d02b274536828fb0f273"></a>
## AggregateStatistics

`struct` · `datafusion_physical_optimizer::aggregate_statistics::AggregateStatistics` · datafusion-physical-optimizer 55.1.0

```rust
struct AggregateStatistics
```

Source: `src/aggregate_statistics.rs:39`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

Optimizer that uses available statistics for aggregate functions

<a id="op-813a09b65e6952abe4254b6b"></a>
## default

`function` · `datafusion_physical_optimizer::aggregate_statistics::AggregateStatistics::default` · datafusion-physical-optimizer 55.1.0

```rust
fn default() -> AggregateStatistics
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::aggregate_statistics::AggregateStatistics", "path": "AggregateStatistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [38, 10], "end": [38, 17], "filename": "src/aggregate_statistics.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/aggregate_statistics.rs:38`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2ec5e72f34f86ca1b9a605f4"></a>
## fmt

`function` · `datafusion_physical_optimizer::aggregate_statistics::AggregateStatistics::fmt` · datafusion-physical-optimizer 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::aggregate_statistics::AggregateStatistics", "path": "AggregateStatistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [38, 19], "end": [38, 24], "filename": "src/aggregate_statistics.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/aggregate_statistics.rs:38`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ca0021e421e390160927d1ed"></a>
## name

`function` · `datafusion_physical_optimizer::aggregate_statistics::AggregateStatistics::name` · datafusion-physical-optimizer 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::aggregate_statistics::AggregateStatistics", "path": "AggregateStatistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 1], "end": [113, 2], "filename": "src/aggregate_statistics.rs"}, "trait": {"args": null, "id": "datafusion_session::physical_optimizer::PhysicalOptimizerRule", "path": "PhysicalOptimizerRule"}, "trait_path": "datafusion_session::physical_optimizer::PhysicalOptimizerRule"}`

Source: `src/aggregate_statistics.rs:105`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-aa31020e4d2f9bcf56da5ef1"></a>
## new

`function` · `datafusion_physical_optimizer::aggregate_statistics::AggregateStatistics::new` · datafusion-physical-optimizer 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::aggregate_statistics::AggregateStatistics", "path": "AggregateStatistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [41, 1], "end": [46, 2], "filename": "src/aggregate_statistics.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregate_statistics.rs:43`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-97a718bad88030c21bf8c99c"></a>
## optimize

`function` · `datafusion_physical_optimizer::aggregate_statistics::AggregateStatistics::optimize` · datafusion-physical-optimizer 55.1.0

```rust
fn optimize(&self, plan: Arc<dyn ExecutionPlan>, config: &ConfigOptions) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::aggregate_statistics::AggregateStatistics", "path": "AggregateStatistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 1], "end": [113, 2], "filename": "src/aggregate_statistics.rs"}, "trait": {"args": null, "id": "datafusion_session::physical_optimizer::PhysicalOptimizerRule", "path": "PhysicalOptimizerRule"}, "trait_path": "datafusion_session::physical_optimizer::PhysicalOptimizerRule"}`

Source: `src/aggregate_statistics.rs:49`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-03c5c75f9678f1c6304cdb0b"></a>
## schema_check

`function` · `datafusion_physical_optimizer::aggregate_statistics::AggregateStatistics::schema_check` · datafusion-physical-optimizer 55.1.0

```rust
fn schema_check(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::aggregate_statistics::AggregateStatistics", "path": "AggregateStatistics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 1], "end": [113, 2], "filename": "src/aggregate_statistics.rs"}, "trait": {"args": null, "id": "datafusion_session::physical_optimizer::PhysicalOptimizerRule", "path": "PhysicalOptimizerRule"}, "trait_path": "datafusion_session::physical_optimizer::PhysicalOptimizerRule"}`

Source: `src/aggregate_statistics.rs:110`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

This rule will change the nullable properties of the schema, disable the schema check.
