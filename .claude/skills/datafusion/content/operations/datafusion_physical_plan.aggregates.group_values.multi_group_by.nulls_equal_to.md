# `datafusion_physical_plan::aggregates::group_values::multi_group_by::nulls_equal_to`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.aggregates.group_values.multi_group_by.nulls_equal_to.json).

<a id="op-2d0d2d4bbde5a4a296d8613b"></a>
## nulls_equal_to

`function` · `datafusion_physical_plan::aggregates::group_values::multi_group_by::nulls_equal_to` · datafusion-physical-plan 55.1.0

```rust
fn nulls_equal_to(lhs_null: bool, rhs_null: bool) -> Option<bool>
```

Source: `src/aggregates/group_values/multi_group_by/mod.rs:123`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Determines if the nullability of the existing and new input array can be used
to short-circuit the comparison of the two values.

Returns `Some(result)` if the result of the comparison can be determined
from the nullness of the two values, and `None` if the comparison must be
done on the values themselves.
