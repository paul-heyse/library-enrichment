# `datafusion_physical_expr::simplifier::unwrap_cast`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.simplifier.unwrap_cast.json).

<a id="op-69923dc20eb5169cc193c60d"></a>
## unwrap_cast

`module` · `datafusion_physical_expr::simplifier::unwrap_cast` · datafusion-physical-expr 55.1.0

```rust
mod unwrap_cast
```

Source: `src/simplifier/unwrap_cast.rs:18`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Unwrap casts in binary comparisons for physical expressions

This module provides optimization for physical expressions similar to the logical
optimizer's unwrap_cast module. It attempts to remove casts from comparisons to
literals by applying the casts to the literals if possible.

The optimization improves performance by:
1. Reducing runtime cast operations on column data
2. Enabling better predicate pushdown opportunities
3. Optimizing filter expressions in physical plans

# Example

Physical expression: `cast(column as INT64) > INT64(10)`
Optimized to: `column > INT32(10)` (assuming column is INT32)
