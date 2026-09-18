# `datafusion_physical_expr::analysis::analyze`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.analysis.analyze.json).

<a id="op-7ad524040e41f75a5ee46f0b"></a>
## analyze

`function` · `datafusion_physical_expr::analysis::analyze` · datafusion-physical-expr 55.1.0

```rust
fn analyze(expr: &std::sync::Arc<dyn PhysicalExpr>, context: AnalysisContext, schema: &arrow::datatypes::Schema) -> datafusion_common::Result<AnalysisContext>
```

Source: `src/analysis.rs:164`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Attempts to refine column boundaries and compute a selectivity value.

The function accepts boundaries of the input columns in the `context` parameter.
It then tries to tighten these boundaries based on the provided `expr`.
The resulting selectivity value is calculated by comparing the initial and final boundaries.
The computation assumes that the data within the column is uniformly distributed and not sorted.

# Arguments

* `context` - The context holding input column boundaries.
* `expr` - The expression used to shrink the column boundaries.

# Returns

* `AnalysisContext` constructed by pruned boundaries and a selectivity value.
