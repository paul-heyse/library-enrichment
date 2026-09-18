# `datafusion_physical_plan::joins::utils::check_join_is_valid`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.joins.utils.check_join_is_valid.json).

<a id="op-942c85e39dad5e3aaed41ea8"></a>
## check_join_is_valid

`function` · `datafusion_physical_plan::joins::utils::check_join_is_valid` · datafusion-physical-plan 55.1.0

```rust
fn check_join_is_valid(left: &arrow::datatypes::Schema, right: &arrow::datatypes::Schema, on: JoinOnRef<'_>) -> datafusion_common::Result<()>
```

Source: `src/joins/utils.rs:87`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Checks whether the schemas "left" and "right" and columns "on" represent a valid join.
They are valid whenever their columns' intersection equals the set `on`
