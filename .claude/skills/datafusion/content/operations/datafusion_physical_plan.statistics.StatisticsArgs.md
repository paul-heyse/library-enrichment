# `datafusion_physical_plan::statistics::StatisticsArgs`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.statistics.StatisticsArgs.json).

<a id="op-33712b0db80fc4a46659cb66"></a>
## StatisticsArgs

`struct` · `datafusion_physical_plan::statistics::StatisticsArgs` · datafusion-physical-plan 55.1.0

```rust
struct StatisticsArgs
```

Source: `src/statistics.rs:74`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Arguments passed to [`ExecutionPlan::statistics_from_inputs`](../operations/datafusion_physical_plan.execution_plan.ExecutionPlan.md#op-7dad80ba3b610abc3c532d8a) carrying
external information that operators can use when computing their
statistics.

<a id="op-d5799f9d5b99b8aa3ef85f6a"></a>
## clone

`function` · `datafusion_physical_plan::statistics::StatisticsArgs::clone` · datafusion-physical-plan 55.1.0

```rust
fn clone(&self) -> StatisticsArgs
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::statistics::StatisticsArgs", "path": "StatisticsArgs"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [73, 26], "end": [73, 31], "filename": "src/statistics.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/statistics.rs:73`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8b588c2a52ba727fc70375d5"></a>
## default

`function` · `datafusion_physical_plan::statistics::StatisticsArgs::default` · datafusion-physical-plan 55.1.0

```rust
fn default() -> StatisticsArgs
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::statistics::StatisticsArgs", "path": "StatisticsArgs"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [73, 17], "end": [73, 24], "filename": "src/statistics.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/statistics.rs:73`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2a2b3a59f507e5c47b473ee9"></a>
## fmt

`function` · `datafusion_physical_plan::statistics::StatisticsArgs::fmt` · datafusion-physical-plan 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::statistics::StatisticsArgs", "path": "StatisticsArgs"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [73, 10], "end": [73, 15], "filename": "src/statistics.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/statistics.rs:73`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dd0acc52ea276e6b14c9d335"></a>
## new

`function` · `datafusion_physical_plan::statistics::StatisticsArgs::new` · datafusion-physical-plan 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::statistics::StatisticsArgs", "path": "StatisticsArgs"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [78, 1], "end": [106, 2], "filename": "src/statistics.rs"}, "trait": null, "trait_path": null}`

Source: `src/statistics.rs:83`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Creates new statistics arguments.

By default the partition is set to `None` (statistics should be computed
for the entire plan).

<a id="op-1128b30b5588553c960b66e0"></a>
## partition

`function` · `datafusion_physical_plan::statistics::StatisticsArgs::partition` · datafusion-physical-plan 55.1.0

```rust
fn partition(&self) -> Option<usize>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::statistics::StatisticsArgs", "path": "StatisticsArgs"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [78, 1], "end": [106, 2], "filename": "src/statistics.rs"}, "trait": null, "trait_path": null}`

Source: `src/statistics.rs:103`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Return the partition to compute statistics

<a id="op-e9b3122e5c4d0cea74994ea7"></a>
## set_partition

`function` · `datafusion_physical_plan::statistics::StatisticsArgs::set_partition` · datafusion-physical-plan 55.1.0

```rust
fn set_partition(&mut self, partition: Option<usize>)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::statistics::StatisticsArgs", "path": "StatisticsArgs"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [78, 1], "end": [106, 2], "filename": "src/statistics.rs"}, "trait": null, "trait_path": null}`

Source: `src/statistics.rs:92`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Set the partition to compute statistics

* `None` means statistics should be computed for the entire plan.
* `Some(idx)` means statistics should be computed for the specified
  partition index.

<a id="op-7a3da57c967a77dc569530ed"></a>
## with_partition

`function` · `datafusion_physical_plan::statistics::StatisticsArgs::with_partition` · datafusion-physical-plan 55.1.0

```rust
fn with_partition(self, partition: Option<usize>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::statistics::StatisticsArgs", "path": "StatisticsArgs"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [78, 1], "end": [106, 2], "filename": "src/statistics.rs"}, "trait": null, "trait_path": null}`

Source: `src/statistics.rs:97`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Builder Style API for [`Self::set_partition`](../operations/datafusion_physical_plan.statistics.StatisticsArgs.md#op-e9b3122e5c4d0cea74994ea7)
