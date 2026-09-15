# `datafusion_functions::math::monotonicity`

Crate `datafusion-functions` · 44 public items · structured records in [`model/datafusion_functions.math.monotonicity.json`](../model/datafusion_functions.math.monotonicity.json)

## acos_order

`function` · `datafusion_functions::math::monotonicity::acos_order`

```rust
fn acos_order(input: &[datafusion_expr::sort_properties::ExprProperties]) -> datafusion_common::Result<datafusion_expr::sort_properties::SortProperties>
```

Non-increasing on the interval \[−1, 1\], undefined otherwise.

---

## acosh_order

`function` · `datafusion_functions::math::monotonicity::acosh_order`

```rust
fn acosh_order(input: &[datafusion_expr::sort_properties::ExprProperties]) -> datafusion_common::Result<datafusion_expr::sort_properties::SortProperties>
```

Non-decreasing for x ≥ 1, undefined otherwise.

---

## asin_order

`function` · `datafusion_functions::math::monotonicity::asin_order`

```rust
fn asin_order(input: &[datafusion_expr::sort_properties::ExprProperties]) -> datafusion_common::Result<datafusion_expr::sort_properties::SortProperties>
```

Non-decreasing on the interval \[−1, 1\], undefined otherwise.

---

## asinh_order

`function` · `datafusion_functions::math::monotonicity::asinh_order`

```rust
fn asinh_order(input: &[datafusion_expr::sort_properties::ExprProperties]) -> datafusion_common::Result<datafusion_expr::sort_properties::SortProperties>
```

Non-decreasing for all real numbers.

---

## atan2_order

`function` · `datafusion_functions::math::monotonicity::atan2_order`

```rust
fn atan2_order(_input: &[datafusion_expr::sort_properties::ExprProperties]) -> datafusion_common::Result<datafusion_expr::sort_properties::SortProperties>
```

Order depends on the quadrant.

---

## atan_order

`function` · `datafusion_functions::math::monotonicity::atan_order`

```rust
fn atan_order(input: &[datafusion_expr::sort_properties::ExprProperties]) -> datafusion_common::Result<datafusion_expr::sort_properties::SortProperties>
```

Non-decreasing for all real numbers.

---

## atanh_order

`function` · `datafusion_functions::math::monotonicity::atanh_order`

```rust
fn atanh_order(input: &[datafusion_expr::sort_properties::ExprProperties]) -> datafusion_common::Result<datafusion_expr::sort_properties::SortProperties>
```

Non-decreasing on the interval \[−1, 1\], undefined otherwise.

---

## cbrt_order

`function` · `datafusion_functions::math::monotonicity::cbrt_order`

```rust
fn cbrt_order(input: &[datafusion_expr::sort_properties::ExprProperties]) -> datafusion_common::Result<datafusion_expr::sort_properties::SortProperties>
```

Non-decreasing for all real numbers.

---

## ceil_order

`function` · `datafusion_functions::math::monotonicity::ceil_order`

```rust
fn ceil_order(input: &[datafusion_expr::sort_properties::ExprProperties]) -> datafusion_common::Result<datafusion_expr::sort_properties::SortProperties>
```

Non-decreasing for all real numbers.

---

## cos_order

`function` · `datafusion_functions::math::monotonicity::cos_order`

```rust
fn cos_order(_input: &[datafusion_expr::sort_properties::ExprProperties]) -> datafusion_common::Result<datafusion_expr::sort_properties::SortProperties>
```

Non-increasing on \[0, π\] and then non-decreasing on \[π, 2π\].
This pattern repeats periodically with a period of 2π.

---

## cosh_order

`function` · `datafusion_functions::math::monotonicity::cosh_order`

```rust
fn cosh_order(input: &[datafusion_expr::sort_properties::ExprProperties]) -> datafusion_common::Result<datafusion_expr::sort_properties::SortProperties>
```

Non-decreasing for x ≥ 0 and symmetrically non-increasing for x ≤ 0.

---

## degrees_order

`function` · `datafusion_functions::math::monotonicity::degrees_order`

```rust
fn degrees_order(input: &[datafusion_expr::sort_properties::ExprProperties]) -> datafusion_common::Result<datafusion_expr::sort_properties::SortProperties>
```

Non-decreasing function that converts radians to degrees.

---

## exp_order

`function` · `datafusion_functions::math::monotonicity::exp_order`

```rust
fn exp_order(input: &[datafusion_expr::sort_properties::ExprProperties]) -> datafusion_common::Result<datafusion_expr::sort_properties::SortProperties>
```

Non-decreasing for all real numbers.

---

## floor_order

`function` · `datafusion_functions::math::monotonicity::floor_order`

```rust
fn floor_order(input: &[datafusion_expr::sort_properties::ExprProperties]) -> datafusion_common::Result<datafusion_expr::sort_properties::SortProperties>
```

Non-decreasing for all real numbers.

---

## get_acos_doc

`function` · `datafusion_functions::math::monotonicity::get_acos_doc`

```rust
fn get_acos_doc() -> &'static datafusion_expr::Documentation
```

---

## get_acosh_doc

`function` · `datafusion_functions::math::monotonicity::get_acosh_doc`

```rust
fn get_acosh_doc() -> &'static datafusion_expr::Documentation
```

---

## get_asin_doc

`function` · `datafusion_functions::math::monotonicity::get_asin_doc`

```rust
fn get_asin_doc() -> &'static datafusion_expr::Documentation
```

---

## get_asinh_doc

`function` · `datafusion_functions::math::monotonicity::get_asinh_doc`

```rust
fn get_asinh_doc() -> &'static datafusion_expr::Documentation
```

---

## get_atan2_doc

`function` · `datafusion_functions::math::monotonicity::get_atan2_doc`

```rust
fn get_atan2_doc() -> &'static datafusion_expr::Documentation
```

---

## get_atan_doc

`function` · `datafusion_functions::math::monotonicity::get_atan_doc`

```rust
fn get_atan_doc() -> &'static datafusion_expr::Documentation
```

---

## get_atanh_doc

`function` · `datafusion_functions::math::monotonicity::get_atanh_doc`

```rust
fn get_atanh_doc() -> &'static datafusion_expr::Documentation
```

---

## get_cbrt_doc

`function` · `datafusion_functions::math::monotonicity::get_cbrt_doc`

```rust
fn get_cbrt_doc() -> &'static datafusion_expr::Documentation
```

---

## get_cos_doc

`function` · `datafusion_functions::math::monotonicity::get_cos_doc`

```rust
fn get_cos_doc() -> &'static datafusion_expr::Documentation
```

---

## get_cosh_doc

`function` · `datafusion_functions::math::monotonicity::get_cosh_doc`

```rust
fn get_cosh_doc() -> &'static datafusion_expr::Documentation
```

---

## get_degrees_doc

`function` · `datafusion_functions::math::monotonicity::get_degrees_doc`

```rust
fn get_degrees_doc() -> &'static datafusion_expr::Documentation
```

---

## get_exp_doc

`function` · `datafusion_functions::math::monotonicity::get_exp_doc`

```rust
fn get_exp_doc() -> &'static datafusion_expr::Documentation
```

---

## get_ln_doc

`function` · `datafusion_functions::math::monotonicity::get_ln_doc`

```rust
fn get_ln_doc() -> &'static datafusion_expr::Documentation
```

---

## get_log10_doc

`function` · `datafusion_functions::math::monotonicity::get_log10_doc`

```rust
fn get_log10_doc() -> &'static datafusion_expr::Documentation
```

---

## get_log2_doc

`function` · `datafusion_functions::math::monotonicity::get_log2_doc`

```rust
fn get_log2_doc() -> &'static datafusion_expr::Documentation
```

---

## get_radians_doc

`function` · `datafusion_functions::math::monotonicity::get_radians_doc`

```rust
fn get_radians_doc() -> &'static datafusion_expr::Documentation
```

---

## get_sin_doc

`function` · `datafusion_functions::math::monotonicity::get_sin_doc`

```rust
fn get_sin_doc() -> &'static datafusion_expr::Documentation
```

---

## get_sinh_doc

`function` · `datafusion_functions::math::monotonicity::get_sinh_doc`

```rust
fn get_sinh_doc() -> &'static datafusion_expr::Documentation
```

---

## get_sqrt_doc

`function` · `datafusion_functions::math::monotonicity::get_sqrt_doc`

```rust
fn get_sqrt_doc() -> &'static datafusion_expr::Documentation
```

---

## get_tan_doc

`function` · `datafusion_functions::math::monotonicity::get_tan_doc`

```rust
fn get_tan_doc() -> &'static datafusion_expr::Documentation
```

---

## get_tanh_doc

`function` · `datafusion_functions::math::monotonicity::get_tanh_doc`

```rust
fn get_tanh_doc() -> &'static datafusion_expr::Documentation
```

---

## ln_order

`function` · `datafusion_functions::math::monotonicity::ln_order`

```rust
fn ln_order(input: &[datafusion_expr::sort_properties::ExprProperties]) -> datafusion_common::Result<datafusion_expr::sort_properties::SortProperties>
```

Non-decreasing for x ≥ 0, undefined otherwise.

---

## log10_order

`function` · `datafusion_functions::math::monotonicity::log10_order`

```rust
fn log10_order(input: &[datafusion_expr::sort_properties::ExprProperties]) -> datafusion_common::Result<datafusion_expr::sort_properties::SortProperties>
```

Non-decreasing for x ≥ 0, undefined otherwise.

---

## log2_order

`function` · `datafusion_functions::math::monotonicity::log2_order`

```rust
fn log2_order(input: &[datafusion_expr::sort_properties::ExprProperties]) -> datafusion_common::Result<datafusion_expr::sort_properties::SortProperties>
```

Non-decreasing for x ≥ 0, undefined otherwise.

---

## radians_order

`function` · `datafusion_functions::math::monotonicity::radians_order`

```rust
fn radians_order(input: &[datafusion_expr::sort_properties::ExprProperties]) -> datafusion_common::Result<datafusion_expr::sort_properties::SortProperties>
```

Non-decreasing for all real numbers x.

---

## sin_order

`function` · `datafusion_functions::math::monotonicity::sin_order`

```rust
fn sin_order(_input: &[datafusion_expr::sort_properties::ExprProperties]) -> datafusion_common::Result<datafusion_expr::sort_properties::SortProperties>
```

Non-decreasing on \[0, π\] and then non-increasing on \[π, 2π\].
This pattern repeats periodically with a period of 2π.

---

## sinh_order

`function` · `datafusion_functions::math::monotonicity::sinh_order`

```rust
fn sinh_order(input: &[datafusion_expr::sort_properties::ExprProperties]) -> datafusion_common::Result<datafusion_expr::sort_properties::SortProperties>
```

Non-decreasing for all real numbers.

---

## sqrt_order

`function` · `datafusion_functions::math::monotonicity::sqrt_order`

```rust
fn sqrt_order(input: &[datafusion_expr::sort_properties::ExprProperties]) -> datafusion_common::Result<datafusion_expr::sort_properties::SortProperties>
```

Non-decreasing for x ≥ 0, undefined otherwise.

---

## tan_order

`function` · `datafusion_functions::math::monotonicity::tan_order`

```rust
fn tan_order(_input: &[datafusion_expr::sort_properties::ExprProperties]) -> datafusion_common::Result<datafusion_expr::sort_properties::SortProperties>
```

Non-decreasing between vertical asymptotes at x = k * π ± π / 2 for any
integer k.

---

## tanh_order

`function` · `datafusion_functions::math::monotonicity::tanh_order`

```rust
fn tanh_order(input: &[datafusion_expr::sort_properties::ExprProperties]) -> datafusion_common::Result<datafusion_expr::sort_properties::SortProperties>
```

Non-decreasing for all real numbers.

---
