# `datafusion_common::utils::normalize_float_zero_scalar`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.utils.normalize_float_zero_scalar.json).

<a id="op-e3dac7f8a26ea02f5fe829a7"></a>
## normalize_float_zero_scalar

`function` · `datafusion_common::utils::normalize_float_zero_scalar` · datafusion-common 55.1.0

```rust
fn normalize_float_zero_scalar(scalar: ScalarValue) -> ScalarValue
```

Source: `src/utils/mod.rs:1456`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Replace `-0.0` with `+0.0` in `Float16`, `Float32`, or `Float64` scalar
values. Other variants are returned unchanged. See [`normalize_float_zero`](../operations/datafusion_common.utils.normalize_float_zero.md#op-dac60b1ac805aa63126dd00f)
for context.
