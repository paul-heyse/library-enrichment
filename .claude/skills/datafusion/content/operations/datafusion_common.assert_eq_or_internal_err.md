# `datafusion_common::assert_eq_or_internal_err`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.assert_eq_or_internal_err.json).

<a id="op-9244c67e2b2929694c32bc91"></a>
## assert_eq_or_internal_err

`macro` · `datafusion_common::assert_eq_or_internal_err` · datafusion-common 55.1.0

```rust
macro_rules! assert_eq_or_internal_err
```

Source: `src/error.rs:871`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Assert equality, returning `DataFusionError::Internal` on failure.

# Examples

```text
assert_eq_or_internal_err!(actual, expected);
assert_eq_or_internal_err!(left_expr, right_expr, "values must match");
assert_eq_or_internal_err!(lhs, rhs, "metadata: {}", extra);
```
