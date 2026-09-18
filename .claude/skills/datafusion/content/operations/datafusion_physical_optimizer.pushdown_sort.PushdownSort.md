# `datafusion_physical_optimizer::pushdown_sort::PushdownSort`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_optimizer.pushdown_sort.PushdownSort.json).

<a id="op-a8651a9fdd39b43e6ee353c8"></a>
## PushdownSort

`struct` · `datafusion_physical_optimizer::pushdown_sort::PushdownSort` · datafusion-physical-optimizer 55.1.0

```rust
struct PushdownSort
```

Source: `src/pushdown_sort.rs:75`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

A PhysicalOptimizerRule that attempts to push down sort requirements to data sources.

See module-level documentation for details.

<a id="op-a4482f567e61984c57cb9aae"></a>
## clone

`function` · `datafusion_physical_optimizer::pushdown_sort::PushdownSort::clone` · datafusion-physical-optimizer 55.1.0

```rust
fn clone(&self) -> PushdownSort
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::pushdown_sort::PushdownSort", "path": "PushdownSort"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [74, 17], "end": [74, 22], "filename": "src/pushdown_sort.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/pushdown_sort.rs:74`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-958e30bcdbd2c374d244350f"></a>
## default

`function` · `datafusion_physical_optimizer::pushdown_sort::PushdownSort::default` · datafusion-physical-optimizer 55.1.0

```rust
fn default() -> PushdownSort
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::pushdown_sort::PushdownSort", "path": "PushdownSort"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [74, 24], "end": [74, 31], "filename": "src/pushdown_sort.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/pushdown_sort.rs:74`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d577a96ed6f168b574cf99e9"></a>
## fmt

`function` · `datafusion_physical_optimizer::pushdown_sort::PushdownSort::fmt` · datafusion-physical-optimizer 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::pushdown_sort::PushdownSort", "path": "PushdownSort"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [74, 10], "end": [74, 15], "filename": "src/pushdown_sort.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/pushdown_sort.rs:74`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f101e9de0c3e8bef4b659be2"></a>
## name

`function` · `datafusion_physical_optimizer::pushdown_sort::PushdownSort::name` · datafusion-physical-optimizer 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::pushdown_sort::PushdownSort", "path": "PushdownSort"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [83, 1], "end": [231, 2], "filename": "src/pushdown_sort.rs"}, "trait": {"args": null, "id": "datafusion_session::physical_optimizer::PhysicalOptimizerRule", "path": "PhysicalOptimizerRule"}, "trait_path": "datafusion_session::physical_optimizer::PhysicalOptimizerRule"}`

Source: `src/pushdown_sort.rs:224`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-005d01958e54b62d4fb1850b"></a>
## new

`function` · `datafusion_physical_optimizer::pushdown_sort::PushdownSort::new` · datafusion-physical-optimizer 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::pushdown_sort::PushdownSort", "path": "PushdownSort"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [77, 1], "end": [81, 2], "filename": "src/pushdown_sort.rs"}, "trait": null, "trait_path": null}`

Source: `src/pushdown_sort.rs:78`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1a0055938f81e1307970c004"></a>
## optimize

`function` · `datafusion_physical_optimizer::pushdown_sort::PushdownSort::optimize` · datafusion-physical-optimizer 55.1.0

```rust
fn optimize(&self, plan: Arc<dyn ExecutionPlan>, config: &ConfigOptions) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::pushdown_sort::PushdownSort", "path": "PushdownSort"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [83, 1], "end": [231, 2], "filename": "src/pushdown_sort.rs"}, "trait": {"args": null, "id": "datafusion_session::physical_optimizer::PhysicalOptimizerRule", "path": "PhysicalOptimizerRule"}, "trait_path": "datafusion_session::physical_optimizer::PhysicalOptimizerRule"}`

Source: `src/pushdown_sort.rs:84`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-306e9a1002343d027250be94"></a>
## schema_check

`function` · `datafusion_physical_optimizer::pushdown_sort::PushdownSort::schema_check` · datafusion-physical-optimizer 55.1.0

```rust
fn schema_check(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::pushdown_sort::PushdownSort", "path": "PushdownSort"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [83, 1], "end": [231, 2], "filename": "src/pushdown_sort.rs"}, "trait": {"args": null, "id": "datafusion_session::physical_optimizer::PhysicalOptimizerRule", "path": "PhysicalOptimizerRule"}, "trait_path": "datafusion_session::physical_optimizer::PhysicalOptimizerRule"}`

Source: `src/pushdown_sort.rs:228`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
