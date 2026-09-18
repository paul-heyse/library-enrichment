# `datafusion_datasource::morsel::MorselPlan`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource.morsel.MorselPlan.json).

<a id="op-5accb7e9b2b1602fe2da379e"></a>
## MorselPlan

`struct` · `datafusion_datasource::morsel::MorselPlan` · datafusion-datasource 55.1.0

```rust
struct MorselPlan
```

Source: `src/morsel/mod.rs:122`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Return result of [`MorselPlanner::plan`](../operations/datafusion_datasource.morsel.MorselPlanner.md#op-14db0580d35832f565a1984f).

# Logical Ordering

For plans where the output order of rows is maintained, the output order of
a [`MorselPlanner`](../operations/datafusion_datasource.morsel.MorselPlanner.md#op-75e3192888d61e52d84762e9) is logically defined as follows:
1. All morsels that are directly produced
2. Recursively, all morsels produced by the returned `planners`

<a id="op-967e18fb6007a71268548c69"></a>
## default

`function` · `datafusion_datasource::morsel::MorselPlan::default` · datafusion-datasource 55.1.0

```rust
fn default() -> MorselPlan
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::morsel::MorselPlan", "path": "MorselPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [121, 10], "end": [121, 17], "filename": "src/morsel/mod.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/morsel/mod.rs:121`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8c46084a9c65594526960bb7"></a>
## has_io_future

`function` · `datafusion_datasource::morsel::MorselPlan::has_io_future` · datafusion-datasource 55.1.0

```rust
fn has_io_future(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::morsel::MorselPlan", "path": "MorselPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [135, 1], "end": [189, 2], "filename": "src/morsel/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/morsel/mod.rs:186`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Returns `true` if this plan contains an I/O future.

<a id="op-42b215bcb5fbe5cf7ab8cb39"></a>
## new

`function` · `datafusion_datasource::morsel::MorselPlan::new` · datafusion-datasource 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::morsel::MorselPlan", "path": "MorselPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [135, 1], "end": [189, 2], "filename": "src/morsel/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/morsel/mod.rs:137`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Create an empty morsel plan.

<a id="op-cd0d49a7a71fa217f4bfe8a4"></a>
## set_pending_planner

`function` · `datafusion_datasource::morsel::MorselPlan::set_pending_planner` · datafusion-datasource 55.1.0

```rust
fn set_pending_planner<F>(&mut self, io_future: F) where F: Future<Output = Result<Box<dyn MorselPlanner>>> + Send + 'static
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::morsel::MorselPlan", "path": "MorselPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [135, 1], "end": [189, 2], "filename": "src/morsel/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/morsel/mod.rs:163`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Set the pending planner  for an I/O phase.

<a id="op-f8af7db75b6f88d6db85f54e"></a>
## take_morsels

`function` · `datafusion_datasource::morsel::MorselPlan::take_morsels` · datafusion-datasource 55.1.0

```rust
fn take_morsels(&mut self) -> Vec<Box<dyn Morsel>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::morsel::MorselPlan", "path": "MorselPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [135, 1], "end": [189, 2], "filename": "src/morsel/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/morsel/mod.rs:171`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Take the ready morsels.

<a id="op-f1235b5401dc50ca9a6d0fa8"></a>
## take_pending_planner

`function` · `datafusion_datasource::morsel::MorselPlan::take_pending_planner` · datafusion-datasource 55.1.0

```rust
fn take_pending_planner(&mut self) -> Option<PendingMorselPlanner>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::morsel::MorselPlan", "path": "MorselPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [135, 1], "end": [189, 2], "filename": "src/morsel/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/morsel/mod.rs:181`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Take the pending I/O future, if any.

<a id="op-1a4f7f0b5bcb3b85fe940a49"></a>
## take_ready_planners

`function` · `datafusion_datasource::morsel::MorselPlan::take_ready_planners` · datafusion-datasource 55.1.0

```rust
fn take_ready_planners(&mut self) -> Vec<Box<dyn MorselPlanner>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::morsel::MorselPlan", "path": "MorselPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [135, 1], "end": [189, 2], "filename": "src/morsel/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/morsel/mod.rs:176`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Take the ready child planners.

<a id="op-a2fd55987756f28237bed4b6"></a>
## with_morsels

`function` · `datafusion_datasource::morsel::MorselPlan::with_morsels` · datafusion-datasource 55.1.0

```rust
fn with_morsels(self, morsels: Vec<Box<dyn Morsel>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::morsel::MorselPlan", "path": "MorselPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [135, 1], "end": [189, 2], "filename": "src/morsel/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/morsel/mod.rs:142`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Set the ready morsels.

<a id="op-8ebbaa1d07bbe282c53bae7c"></a>
## with_pending_planner

`function` · `datafusion_datasource::morsel::MorselPlan::with_pending_planner` · datafusion-datasource 55.1.0

```rust
fn with_pending_planner<F>(self, io_future: F) -> Self where F: Future<Output = Result<Box<dyn MorselPlanner>>> + Send + 'static
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::morsel::MorselPlan", "path": "MorselPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [135, 1], "end": [189, 2], "filename": "src/morsel/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/morsel/mod.rs:154`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Set the pending planner for an I/O phase.

<a id="op-cd1b2aa49523428685c4074e"></a>
## with_planners

`function` · `datafusion_datasource::morsel::MorselPlan::with_planners` · datafusion-datasource 55.1.0

```rust
fn with_planners(self, planners: Vec<Box<dyn MorselPlanner>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::morsel::MorselPlan", "path": "MorselPlan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [135, 1], "end": [189, 2], "filename": "src/morsel/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/morsel/mod.rs:148`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Set the ready child planners.
