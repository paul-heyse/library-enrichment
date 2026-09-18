# `datafusion_expr::utils::only_or_err`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.utils.only_or_err.json).

<a id="op-367840e61960223cf9334c8a"></a>
## only_or_err

`function` · `datafusion_expr::utils::only_or_err` · datafusion-expr 55.1.0

```rust
fn only_or_err<T>(slice: &[T]) -> datafusion_common::Result<&T>
```

Source: `src/utils.rs:1391`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns the first (and only) element in a slice, or an error

# Arguments

* `slice` - The slice to extract from

# Return value

The first element, or an error
