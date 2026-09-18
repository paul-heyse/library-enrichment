# `datafusion_pruning::pruning_predicate::PruningPredicateBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_pruning.pruning_predicate.PruningPredicateBuilder.json).

<a id="op-ea310a6e9152a39747b3a0e2"></a>
## PruningPredicateBuilder

`struct` · `datafusion_pruning::pruning_predicate::PruningPredicateBuilder` · datafusion-pruning 55.1.0

```rust
struct PruningPredicateBuilder<'a>
```

Source: `src/pruning_predicate.rs:413`. [Exact documentation build](https://docs.rs/crate/datafusion-pruning/55.1.0/json).

Builder for a [`PruningPredicate`](../operations/datafusion_pruning.pruning_predicate.PruningPredicate.md#op-ee1b051e6137f40583414068). Groups optional configuration —
`IN (...)` rewrite cap, error counter — so future additions do not
churn the top-level API.

The two entry points are:
 - [`Self::build`](../operations/datafusion_pruning.pruning_predicate.PruningPredicateBuilder.md#op-cad1bd85d70959fc8f0ff50b): convenience for scan sites that already track a
   `predicate_creation_errors` counter. Returns `Some(Arc<..>)` when the
   resulting predicate can actually prune, `None` when it is trivially
   true or when construction failed (in which case the error counter is
   incremented if one was supplied).
 - [`Self::try_build`](../operations/datafusion_pruning.pruning_predicate.PruningPredicateBuilder.md#op-b70a1607a0717b043f013e56): returns a raw `Result<PruningPredicate>` for
   callers that want to surface errors themselves.


<a id="op-cad1bd85d70959fc8f0ff50b"></a>
## build

`function` · `datafusion_pruning::pruning_predicate::PruningPredicateBuilder::build` · datafusion-pruning 55.1.0

```rust
fn build(self, predicate: Arc<dyn PhysicalExpr>) -> Option<Arc<PruningPredicate>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_pruning::pruning_predicate::PruningPredicateBuilder", "path": "PruningPredicateBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [419, 1], "end": [534, 2], "filename": "src/pruning_predicate.rs"}, "trait": null, "trait_path": null}`

Source: `src/pruning_predicate.rs:459`. [Exact documentation build](https://docs.rs/crate/datafusion-pruning/55.1.0/json).

Build a [`PruningPredicate`](../operations/datafusion_pruning.pruning_predicate.PruningPredicate.md#op-ee1b051e6137f40583414068) wrapped in `Some(Arc<..>)` when it can
prune, `None` when it is trivially true or when construction fails.
If [`Self::with_error_counter`](../operations/datafusion_pruning.pruning_predicate.PruningPredicateBuilder.md#op-a752ced6027e8ab41c25fd79) was set, construction failures are
recorded there.

<a id="op-8b74f1e9d8d8b2c7426c567a"></a>
## default

`function` · `datafusion_pruning::pruning_predicate::PruningPredicateBuilder::default` · datafusion-pruning 55.1.0

```rust
fn default() -> PruningPredicateBuilder<'a>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_pruning::pruning_predicate::PruningPredicateBuilder", "path": "PruningPredicateBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [412, 10], "end": [412, 17], "filename": "src/pruning_predicate.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/pruning_predicate.rs:412`. [Exact documentation build](https://docs.rs/crate/datafusion-pruning/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e029d65777377b1ae8c3396e"></a>
## new

`function` · `datafusion_pruning::pruning_predicate::PruningPredicateBuilder::new` · datafusion-pruning 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_pruning::pruning_predicate::PruningPredicateBuilder", "path": "PruningPredicateBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [419, 1], "end": [534, 2], "filename": "src/pruning_predicate.rs"}, "trait": null, "trait_path": null}`

Source: `src/pruning_predicate.rs:421`. [Exact documentation build](https://docs.rs/crate/datafusion-pruning/55.1.0/json).

Create a new builder with the default pruning predicate configuration.

<a id="op-b70a1607a0717b043f013e56"></a>
## try_build

`function` · `datafusion_pruning::pruning_predicate::PruningPredicateBuilder::try_build` · datafusion-pruning 55.1.0

```rust
fn try_build(self, predicate: Arc<dyn PhysicalExpr>) -> Result<PruningPredicate>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_pruning::pruning_predicate::PruningPredicateBuilder", "path": "PruningPredicateBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [419, 1], "end": [534, 2], "filename": "src/pruning_predicate.rs"}, "trait": null, "trait_path": null}`

Source: `src/pruning_predicate.rs:483`. [Exact documentation build](https://docs.rs/crate/datafusion-pruning/55.1.0/json).

Build a [`PruningPredicate`](../operations/datafusion_pruning.pruning_predicate.PruningPredicate.md#op-ee1b051e6137f40583414068), returning the construction error
directly. Callers that want the always-true predicate elided or
errors folded into a counter should use [`Self::build`](../operations/datafusion_pruning.pruning_predicate.PruningPredicateBuilder.md#op-cad1bd85d70959fc8f0ff50b) instead.

<a id="op-a752ced6027e8ab41c25fd79"></a>
## with_error_counter

`function` · `datafusion_pruning::pruning_predicate::PruningPredicateBuilder::with_error_counter` · datafusion-pruning 55.1.0

```rust
fn with_error_counter(self, error_counter: &'a Count) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_pruning::pruning_predicate::PruningPredicateBuilder", "path": "PruningPredicateBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [419, 1], "end": [534, 2], "filename": "src/pruning_predicate.rs"}, "trait": null, "trait_path": null}`

Source: `src/pruning_predicate.rs:439`. [Exact documentation build](https://docs.rs/crate/datafusion-pruning/55.1.0/json).

Metric counter incremented once per predicate that fails to build.
Only consulted by [`Self::build`](../operations/datafusion_pruning.pruning_predicate.PruningPredicateBuilder.md#op-cad1bd85d70959fc8f0ff50b); [`Self::try_build`](../operations/datafusion_pruning.pruning_predicate.PruningPredicateBuilder.md#op-b70a1607a0717b043f013e56) surfaces the
error directly.

<a id="op-04a9f80fa708afa3a8d4a4d4"></a>
## with_file_schema

`function` · `datafusion_pruning::pruning_predicate::PruningPredicateBuilder::with_file_schema` · datafusion-pruning 55.1.0

```rust
fn with_file_schema(self, file_schema: SchemaRef) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_pruning::pruning_predicate::PruningPredicateBuilder", "path": "PruningPredicateBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [419, 1], "end": [534, 2], "filename": "src/pruning_predicate.rs"}, "trait": null, "trait_path": null}`

Source: `src/pruning_predicate.rs:431`. [Exact documentation build](https://docs.rs/crate/datafusion-pruning/55.1.0/json).

Set the schema of the container that will be pruned (typically the
parquet file schema).

<a id="op-0956a1b942bbe739e93821cf"></a>
## with_max_in_list_size

`function` · `datafusion_pruning::pruning_predicate::PruningPredicateBuilder::with_max_in_list_size` · datafusion-pruning 55.1.0

```rust
fn with_max_in_list_size(self, max_in_list_size: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_pruning::pruning_predicate::PruningPredicateBuilder", "path": "PruningPredicateBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [419, 1], "end": [534, 2], "filename": "src/pruning_predicate.rs"}, "trait": null, "trait_path": null}`

Source: `src/pruning_predicate.rs:450`. [Exact documentation build](https://docs.rs/crate/datafusion-pruning/55.1.0/json).

Cap on the size of `IN (...)` lists that will be rewritten into per-
value min/max statistics checks. Lists longer than this fall back to
the unhandled-predicate hook (typically "keep the container").

Query engines typically pass
`datafusion.execution.parquet.max_in_list_size` here.
