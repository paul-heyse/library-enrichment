# `datafusion_pruning::pruning_predicate::PredicateRewriter`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_pruning.pruning_predicate.PredicateRewriter.json).

<a id="op-580f45ac4a696442fc6d93be"></a>
## PredicateRewriter

`struct` · `datafusion_pruning::pruning_predicate::PredicateRewriter` · datafusion-pruning 55.1.0

```rust
struct PredicateRewriter
```

Source: `src/pruning_predicate.rs:1471`. [Exact documentation build](https://docs.rs/crate/datafusion-pruning/55.1.0/json).

Rewrite a predicate expression in terms of statistics (min/max/null_counts)
for use as a [`PruningPredicate`](../operations/datafusion_pruning.pruning_predicate.PruningPredicate.md#op-ee1b051e6137f40583414068).

<a id="op-46e2601cacae549283091ea0"></a>
## default

`function` · `datafusion_pruning::pruning_predicate::PredicateRewriter::default` · datafusion-pruning 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_pruning::pruning_predicate::PredicateRewriter", "path": "PredicateRewriter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1476, 1], "end": [1483, 2], "filename": "src/pruning_predicate.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/pruning_predicate.rs:1477`. [Exact documentation build](https://docs.rs/crate/datafusion-pruning/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6a108fc0b940a8e9603cac22"></a>
## new

`function` · `datafusion_pruning::pruning_predicate::PredicateRewriter::new` · datafusion-pruning 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_pruning::pruning_predicate::PredicateRewriter", "path": "PredicateRewriter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1485, 1], "end": [1537, 2], "filename": "src/pruning_predicate.rs"}, "trait": null, "trait_path": null}`

Source: `src/pruning_predicate.rs:1487`. [Exact documentation build](https://docs.rs/crate/datafusion-pruning/55.1.0/json).

Create a new `PredicateRewriter`

<a id="op-cf766b39dc465ca4a57f92c7"></a>
## rewrite_predicate_to_statistics_predicate

`function` · `datafusion_pruning::pruning_predicate::PredicateRewriter::rewrite_predicate_to_statistics_predicate` · datafusion-pruning 55.1.0

```rust
fn rewrite_predicate_to_statistics_predicate(&self, expr: &Arc<dyn PhysicalExpr>, schema: &Schema) -> Arc<dyn PhysicalExpr>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_pruning::pruning_predicate::PredicateRewriter", "path": "PredicateRewriter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1485, 1], "end": [1537, 2], "filename": "src/pruning_predicate.rs"}, "trait": null, "trait_path": null}`

Source: `src/pruning_predicate.rs:1523`. [Exact documentation build](https://docs.rs/crate/datafusion-pruning/55.1.0/json).

Translate logical filter expression into pruning predicate
expression that will evaluate to FALSE if it can be determined no
rows between the min/max values could pass the predicates.

Any predicates that can not be translated will be passed to `unhandled_hook`.

Returns the pruning predicate as an [`PhysicalExpr`](../operations/datafusion_physical_expr_common.physical_expr.PhysicalExpr.md#op-fe8284c43330456b0d4e6af7)

Notice: `IN (...)` lists longer than `max_in_list_size` (default
[`MAX_IN_LIST_SIZE`](../operations/datafusion_pruning.pruning_predicate.MAX_IN_LIST_SIZE.md#op-6ffaf54ce6196eab4ef02c82)) fall back to calling `unhandled_hook`.

<a id="op-b022a1e85f97d9c752a69e3f"></a>
## with_max_in_list_size

`function` · `datafusion_pruning::pruning_predicate::PredicateRewriter::with_max_in_list_size` · datafusion-pruning 55.1.0

```rust
fn with_max_in_list_size(self, max_in_list_size: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_pruning::pruning_predicate::PredicateRewriter", "path": "PredicateRewriter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1485, 1], "end": [1537, 2], "filename": "src/pruning_predicate.rs"}, "trait": null, "trait_path": null}`

Source: `src/pruning_predicate.rs:1508`. [Exact documentation build](https://docs.rs/crate/datafusion-pruning/55.1.0/json).

Set the maximum size of an `IN (...)` list that will be rewritten into a
chain of per-value statistics checks. Lists longer than this fall back
to the unhandled-predicate hook (typically "keep the container"),
effectively skipping container-level pruning for large IN lists.

The default (see [`MAX_IN_LIST_SIZE`](../operations/datafusion_pruning.pruning_predicate.MAX_IN_LIST_SIZE.md#op-6ffaf54ce6196eab4ef02c82)) preserves the
historical behaviour. Callers wiring config through can override via
`datafusion.execution.max_in_list_size`.

<a id="op-cd5a8bbc8f156d0c8feff3c8"></a>
## with_unhandled_hook

`function` · `datafusion_pruning::pruning_predicate::PredicateRewriter::with_unhandled_hook` · datafusion-pruning 55.1.0

```rust
fn with_unhandled_hook(self, unhandled_hook: Arc<dyn UnhandledPredicateHook>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_pruning::pruning_predicate::PredicateRewriter", "path": "PredicateRewriter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1485, 1], "end": [1537, 2], "filename": "src/pruning_predicate.rs"}, "trait": null, "trait_path": null}`

Source: `src/pruning_predicate.rs:1492`. [Exact documentation build](https://docs.rs/crate/datafusion-pruning/55.1.0/json).

Set the unhandled hook to be used when a predicate can not be rewritten
