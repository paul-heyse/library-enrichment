# `datafusion_physical_plan::operator_statistics::StatisticsRegistry`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.operator_statistics.StatisticsRegistry.json).

<a id="op-058fa5423a9fbd82dda192ea"></a>
## StatisticsRegistry

`struct` · `datafusion_physical_plan::operator_statistics::StatisticsRegistry` · datafusion-physical-plan 55.1.0

```rust
struct StatisticsRegistry
```

Source: `src/operator_statistics/mod.rs:282`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Registry that chains [`StatisticsProvider`](../operations/datafusion_physical_plan.operator_statistics.StatisticsProvider.md#op-b3050eb3d6b996dd9e336fb5) implementations.

The registry is a stateless provider chain: it holds no mutable state
and is cheaply `Clone`able / `Send` / `Sync`.

<a id="op-c486b04b6be8c2c138a78138"></a>
## clone

`function` · `datafusion_physical_plan::operator_statistics::StatisticsRegistry::clone` · datafusion-physical-plan 55.1.0

```rust
fn clone(&self) -> StatisticsRegistry
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::operator_statistics::StatisticsRegistry", "path": "StatisticsRegistry"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [281, 10], "end": [281, 15], "filename": "src/operator_statistics/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/operator_statistics/mod.rs:281`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bbf9cc3b14d694b23398d7a8"></a>
## compute

`function` · `datafusion_physical_plan::operator_statistics::StatisticsRegistry::compute` · datafusion-physical-plan 55.1.0

```rust
fn compute(&self, plan: &dyn ExecutionPlan) -> Result<ExtendedStatistics>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::operator_statistics::StatisticsRegistry", "path": "StatisticsRegistry"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [298, 1], "end": [396, 2], "filename": "src/operator_statistics/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/operator_statistics/mod.rs:359`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Compute extended statistics for a plan through the provider chain.

Performs a bottom-up tree walk: child statistics are computed recursively
and passed to providers, mirroring how `partition_statistics` composes
operators. Once [#20184](https://github.com/apache/datafusion/issues/20184)
lands, the registry can feed enriched base stats directly into
`partition_statistics(child_stats)`, removing the need for a separate walk.

If no providers are registered, falls back to the plan's built-in
`partition_statistics(None)` with no overhead.

<a id="op-fd4a648f7de5089a0cc1f70c"></a>
## compute_base

`function` · `datafusion_physical_plan::operator_statistics::StatisticsRegistry::compute_base` · datafusion-physical-plan 55.1.0

```rust
fn compute_base(&self, plan: &dyn ExecutionPlan) -> Result<Statistics>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::operator_statistics::StatisticsRegistry", "path": "StatisticsRegistry"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [298, 1], "end": [396, 2], "filename": "src/operator_statistics/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/operator_statistics/mod.rs:393`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Compute statistics and return only the base Statistics (no extensions).

Convenience method for callers that don't need extensions.

<a id="op-fd89b7b1b6a383808b937464"></a>
## default

`function` · `datafusion_physical_plan::operator_statistics::StatisticsRegistry::default` · datafusion-physical-plan 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::operator_statistics::StatisticsRegistry", "path": "StatisticsRegistry"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [292, 1], "end": [296, 2], "filename": "src/operator_statistics/mod.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/operator_statistics/mod.rs:293`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-45b4c6a30765147be2c4ba93"></a>
## default_with_builtin_providers

`function` · `datafusion_physical_plan::operator_statistics::StatisticsRegistry::default_with_builtin_providers` · datafusion-physical-plan 55.1.0

```rust
fn default_with_builtin_providers() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::operator_statistics::StatisticsRegistry", "path": "StatisticsRegistry"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [298, 1], "end": [396, 2], "filename": "src/operator_statistics/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/operator_statistics/mod.rs:326`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Create a registry pre-loaded with the standard built-in providers.

Provider order (first match wins):
1. [`FilterStatisticsProvider`](../operations/datafusion_physical_plan.operator_statistics.FilterStatisticsProvider.md#op-59066bd11e2065860e644852)
2. [`ProjectionStatisticsProvider`](../operations/datafusion_physical_plan.operator_statistics.ProjectionStatisticsProvider.md#op-c2b09bcaeb037f1e11a4b411)
3. [`PassthroughStatisticsProvider`](../operations/datafusion_physical_plan.operator_statistics.PassthroughStatisticsProvider.md#op-644756eff17c7a9b7fd6c3a5)
4. [`AggregateStatisticsProvider`](../operations/datafusion_physical_plan.operator_statistics.AggregateStatisticsProvider.md#op-d28aa37ea36e0bf183ef455b)
5. [`JoinStatisticsProvider`](../operations/datafusion_physical_plan.operator_statistics.JoinStatisticsProvider.md#op-192cd9d26ef10e0d08f3f4ef)
6. [`LimitStatisticsProvider`](../operations/datafusion_physical_plan.operator_statistics.LimitStatisticsProvider.md#op-146faf3a043b4d34b5169fa8)
7. [`UnionStatisticsProvider`](../operations/datafusion_physical_plan.operator_statistics.UnionStatisticsProvider.md#op-57284633242dfde6dceb8223)
8. [`DefaultStatisticsProvider`](../operations/datafusion_physical_plan.operator_statistics.DefaultStatisticsProvider.md#op-5664b61a0d6c36c14eab5141)

<a id="op-a87fd94596a3ec42dcf8685a"></a>
## fmt

`function` · `datafusion_physical_plan::operator_statistics::StatisticsRegistry::fmt` · datafusion-physical-plan 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::operator_statistics::StatisticsRegistry", "path": "StatisticsRegistry"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [286, 1], "end": [290, 2], "filename": "src/operator_statistics/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/operator_statistics/mod.rs:287`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-97da4098e896c8c74c436b41"></a>
## new

`function` · `datafusion_physical_plan::operator_statistics::StatisticsRegistry::new` · datafusion-physical-plan 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::operator_statistics::StatisticsRegistry", "path": "StatisticsRegistry"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [298, 1], "end": [396, 2], "filename": "src/operator_statistics/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/operator_statistics/mod.rs:304`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Create a new empty registry.

With no providers, `compute()` falls back to each plan node's
built-in `partition_statistics()`. Register providers to enhance
statistics (e.g., inject NDV, use histograms).

<a id="op-56582994a332ad837f9c5012"></a>
## providers

`function` · `datafusion_physical_plan::operator_statistics::StatisticsRegistry::providers` · datafusion-physical-plan 55.1.0

```rust
fn providers(&self) -> &[Arc<dyn StatisticsProvider>]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::operator_statistics::StatisticsRegistry", "path": "StatisticsRegistry"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [298, 1], "end": [396, 2], "filename": "src/operator_statistics/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/operator_statistics/mod.rs:345`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Returns the current provider chain.

<a id="op-0f43d4ae35ea6818e25cfcfc"></a>
## register

`function` · `datafusion_physical_plan::operator_statistics::StatisticsRegistry::register` · datafusion-physical-plan 55.1.0

```rust
fn register(&mut self, provider: Arc<dyn StatisticsProvider>)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::operator_statistics::StatisticsRegistry", "path": "StatisticsRegistry"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [298, 1], "end": [396, 2], "filename": "src/operator_statistics/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/operator_statistics/mod.rs:340`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Register a provider at the front of the chain (higher priority).

<a id="op-e4b0cd93be6d619713e63699"></a>
## with_providers

`function` · `datafusion_physical_plan::operator_statistics::StatisticsRegistry::with_providers` · datafusion-physical-plan 55.1.0

```rust
fn with_providers(providers: Vec<Arc<dyn StatisticsProvider>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::operator_statistics::StatisticsRegistry", "path": "StatisticsRegistry"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [298, 1], "end": [396, 2], "filename": "src/operator_statistics/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/operator_statistics/mod.rs:311`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Create a registry with the given provider chain.
