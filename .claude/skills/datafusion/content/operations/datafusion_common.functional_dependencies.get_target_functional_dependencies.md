# `datafusion_common::functional_dependencies::get_target_functional_dependencies`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.functional_dependencies.get_target_functional_dependencies.json).

<a id="op-c8361623aaa3dc6e851cb7bf"></a>
## get_target_functional_dependencies

`function` · `datafusion_common::functional_dependencies::get_target_functional_dependencies` · datafusion-common 55.1.0

```rust
fn get_target_functional_dependencies(schema: &DFSchema, group_by_expr_names: &[String]) -> Option<Vec<usize>>
```

Source: `src/functional_dependencies.rs:515`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Returns target indices, for the determinant keys that are inside
group by expressions.
