# `datafusion_physical_expr::equivalence::convert_to_orderings`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.equivalence.convert_to_orderings.json).

<a id="op-f2844bdc61fd9bddb6176af2"></a>
## convert_to_orderings

`function` · `datafusion_physical_expr::equivalence::convert_to_orderings` · datafusion-physical-expr 55.1.0

```rust
fn convert_to_orderings<T: Borrow<std::sync::Arc<dyn PhysicalExpr>>>(args: &[Vec<(T, arrow::compute::SortOptions)>]) -> Vec<datafusion_physical_expr_common::sort_expr::LexOrdering>
```

Source: `src/equivalence/mod.rs:49`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
