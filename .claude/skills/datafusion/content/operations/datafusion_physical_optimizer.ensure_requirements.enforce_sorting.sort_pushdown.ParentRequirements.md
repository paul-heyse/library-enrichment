# `datafusion_physical_optimizer::ensure_requirements::enforce_sorting::sort_pushdown::ParentRequirements`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_optimizer.ensure_requirements.enforce_sorting.sort_pushdown.ParentRequirements.json).

<a id="op-02eea1693da3fa0a724da33f"></a>
## ParentRequirements

`struct` · `datafusion_physical_optimizer::ensure_requirements::enforce_sorting::sort_pushdown::ParentRequirements` · datafusion-physical-optimizer 55.1.0

```rust
struct ParentRequirements
```

Source: `src/ensure_requirements/enforce_sorting/sort_pushdown.rs:59`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

"Data class" used by sort pushdown (now driven from `EnsureRequirements`)
to push down [`SortExec`](../operations/datafusion_physical_plan.sorts.sort.SortExec.md#op-b7bc765f23cc8fb428c680af) in the plan. In some cases the total
computational cost is reduced by pushing down `SortExec`s through certain
executors. The object carries the parent required ordering, the (optional)
`fetch` value of the parent node, and the parent's distribution requirement
(used by the distribution-aware pushdown path) as its data.

<a id="op-8099569b1fa3bac21ecf05a4"></a>
## clone

`function` · `datafusion_physical_optimizer::ensure_requirements::enforce_sorting::sort_pushdown::ParentRequirements::clone` · datafusion-physical-optimizer 55.1.0

```rust
fn clone(&self) -> ParentRequirements
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::ensure_requirements::enforce_sorting::sort_pushdown::ParentRequirements", "path": "ParentRequirements"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [58, 10], "end": [58, 15], "filename": "src/ensure_requirements/enforce_sorting/sort_pushdown.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ensure_requirements/enforce_sorting/sort_pushdown.rs:58`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7ac06d1d532f638856a90c82"></a>
## default

`function` · `datafusion_physical_optimizer::ensure_requirements::enforce_sorting::sort_pushdown::ParentRequirements::default` · datafusion-physical-optimizer 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::ensure_requirements::enforce_sorting::sort_pushdown::ParentRequirements", "path": "ParentRequirements"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [68, 1], "end": [76, 2], "filename": "src/ensure_requirements/enforce_sorting/sort_pushdown.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/ensure_requirements/enforce_sorting/sort_pushdown.rs:69`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-688694c2db5155fc4c929f1c"></a>
## fmt

`function` · `datafusion_physical_optimizer::ensure_requirements::enforce_sorting::sort_pushdown::ParentRequirements::fmt` · datafusion-physical-optimizer 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_optimizer::ensure_requirements::enforce_sorting::sort_pushdown::ParentRequirements", "path": "ParentRequirements"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [58, 17], "end": [58, 22], "filename": "src/ensure_requirements/enforce_sorting/sort_pushdown.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ensure_requirements/enforce_sorting/sort_pushdown.rs:58`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
