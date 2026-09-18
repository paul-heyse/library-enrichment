# `datafusion_physical_plan::operator_statistics::StatisticsResult`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.operator_statistics.StatisticsResult.json).

<a id="op-e76cdd29b31ac6aaf7341110"></a>
## StatisticsResult

`enum` · `datafusion_physical_plan::operator_statistics::StatisticsResult` · datafusion-physical-plan 55.1.0

```rust
enum StatisticsResult
```

Source: `src/operator_statistics/mod.rs:206`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Result of attempting to compute statistics with a [`StatisticsProvider`](../operations/datafusion_physical_plan.operator_statistics.StatisticsProvider.md#op-b3050eb3d6b996dd9e336fb5).

<a id="op-8692bf67418955a391809cf5"></a>
## Computed

`variant` · `datafusion_physical_plan::operator_statistics::StatisticsResult::Computed` · datafusion-physical-plan 55.1.0

```rust
Computed
```

Source: `src/operator_statistics/mod.rs:208`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Statistics were computed by this provider

<a id="op-653bea20ad3fe48803aea756"></a>
## Delegate

`variant` · `datafusion_physical_plan::operator_statistics::StatisticsResult::Delegate` · datafusion-physical-plan 55.1.0

```rust
Delegate
```

Source: `src/operator_statistics/mod.rs:210`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

This provider doesn't handle this operator; delegate to next in chain

<a id="op-04587b303bd421fc7765aa3e"></a>
## fmt

`function` · `datafusion_physical_plan::operator_statistics::StatisticsResult::fmt` · datafusion-physical-plan 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::operator_statistics::StatisticsResult", "path": "StatisticsResult"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [205, 10], "end": [205, 15], "filename": "src/operator_statistics/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/operator_statistics/mod.rs:205`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
