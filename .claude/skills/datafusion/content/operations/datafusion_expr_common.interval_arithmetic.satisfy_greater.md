# `datafusion_expr_common::interval_arithmetic::satisfy_greater`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr_common.interval_arithmetic.satisfy_greater.json).

<a id="op-23f602cb7e1a8a012cbe4ce8"></a>
## satisfy_greater

`function` · `datafusion_expr_common::interval_arithmetic::satisfy_greater` · datafusion-expr-common 55.1.0

```rust
fn satisfy_greater(left: &Interval, right: &Interval, strict: bool) -> datafusion_common::Result<Option<(Interval, Interval)>>
```

Source: `src/interval_arithmetic.rs:1397`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

This function updates the given intervals by enforcing (i.e. propagating)
the inequality `left > right` (or the `left >= right` inequality, if `strict`
is `true`).

Returns a `Result` wrapping an `Option` containing the tuple of resulting
intervals. If the comparison is infeasible, returns `None`.

Example usage:
```
use datafusion_common::DataFusionError;
use datafusion_expr_common::interval_arithmetic::{satisfy_greater, Interval};

let left = Interval::make(Some(-1000.0_f32), Some(1000.0_f32))?;
let right = Interval::make(Some(500.0_f32), Some(2000.0_f32))?;
let strict = false;
assert_eq!(
    satisfy_greater(&left, &right, strict)?,
    Some((
        Interval::make(Some(500.0_f32), Some(1000.0_f32))?,
        Interval::make(Some(500.0_f32), Some(1000.0_f32))?
    ))
);
Ok::<(), DataFusionError>(())
```

NOTE: This function only works with intervals of the same data type.
      Attempting to compare intervals of different data types will lead
      to an error.
