# `datafusion_pruning::pruning_predicate::build_pruning_predicate`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_pruning.pruning_predicate.build_pruning_predicate.json).

<a id="op-30b7193a615c9f9c0f9f7b95"></a>
## build_pruning_predicate

`function` · `datafusion_pruning::pruning_predicate::build_pruning_predicate` · datafusion-pruning 55.1.0

```rust
fn build_pruning_predicate(predicate: std::sync::Arc<dyn PhysicalExpr>, file_schema: &arrow::datatypes::SchemaRef, predicate_creation_errors: &datafusion_physical_plan::metrics::Count) -> Option<std::sync::Arc<PruningPredicate>>
```

Source: `src/pruning_predicate.rs:388`. [Exact documentation build](https://docs.rs/crate/datafusion-pruning/55.1.0/json).

Build a pruning predicate from an optional predicate expression.
If the predicate is None or the predicate cannot be converted to a pruning
predicate, return None.
If there is an error creating the pruning predicate it is recorded by incrementing
the `predicate_creation_errors` counter.
