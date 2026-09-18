# `datafusion_pruning::pruning_predicate::UnhandledPredicateHook`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_pruning.pruning_predicate.UnhandledPredicateHook.json).

<a id="op-f715433c379e53367c5df8b5"></a>
## UnhandledPredicateHook

`trait` · `datafusion_pruning::pruning_predicate::UnhandledPredicateHook` · datafusion-pruning 55.1.0

```rust
trait UnhandledPredicateHook
```

Source: `src/pruning_predicate.rs:539`. [Exact documentation build](https://docs.rs/crate/datafusion-pruning/55.1.0/json).

Rewrites predicates that [`PredicateRewriter`](../operations/datafusion_pruning.pruning_predicate.PredicateRewriter.md#op-580f45ac4a696442fc6d93be) can not handle, e.g. certain
complex expressions or predicates that reference columns that are not in the
schema.

<a id="op-8b0d4f130331adbc73f6076c"></a>
## handle

`function` · `datafusion_pruning::pruning_predicate::UnhandledPredicateHook::handle` · datafusion-pruning 55.1.0

```rust
fn handle(&self, expr: &Arc<dyn PhysicalExpr>) -> Arc<dyn PhysicalExpr>
```

Source: `src/pruning_predicate.rs:542`. [Exact documentation build](https://docs.rs/crate/datafusion-pruning/55.1.0/json).

Called when a predicate can not be rewritten in terms of statistics or
references a column that is not in the schema.
