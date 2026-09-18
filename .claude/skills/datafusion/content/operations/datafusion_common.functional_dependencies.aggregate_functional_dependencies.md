# `datafusion_common::functional_dependencies::aggregate_functional_dependencies`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.functional_dependencies.aggregate_functional_dependencies.json).

<a id="op-78dce3f7655c3168f446ad7c"></a>
## aggregate_functional_dependencies

`function` · `datafusion_common::functional_dependencies::aggregate_functional_dependencies` · datafusion-common 55.1.0

```rust
fn aggregate_functional_dependencies(aggr_input_schema: &DFSchema, group_by_expr_names: &[String], aggr_schema: &DFSchema) -> FunctionalDependencies
```

Source: `src/functional_dependencies.rs:420`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Calculates functional dependencies for aggregate output, when there is a GROUP BY expression.
