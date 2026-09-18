# `datafusion_common::utils::normalize_float_zero`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.utils.normalize_float_zero.json).

<a id="op-dac60b1ac805aa63126dd00f"></a>
## normalize_float_zero

`function` · `datafusion_common::utils::normalize_float_zero` · datafusion-common 55.1.0

```rust
fn normalize_float_zero(array: &arrow::array::ArrayRef) -> arrow::array::ArrayRef
```

Source: `src/utils/mod.rs:1396`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Replace `-0.0` with `+0.0` in any `Float16`, `Float32`, or `Float64` array.
For non-float arrays returns the input unchanged. NaN payloads are
preserved.

Arrow's comparison kernels (`arrow::compute::kernels::cmp::eq` etc.) and
row-encoding (`arrow::row::RowConverter`) use IEEE 754 totalOrder
semantics, which treats `-0.0` and `+0.0` as distinct. SQL semantics
(PostgreSQL / IEEE 754 equality) require them to compare equal, so
callers normalize before invoking those kernels.

The common case - no `-0.0` present - is allocation-free: a single
read-only scan of the underlying buffer (auto-vectorizable to an
OR-reduction) decides whether to fall through to the rewriting path.
Only arrays that actually contain `-0.0` pay for a new buffer.
