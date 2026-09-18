# `datafusion_physical_expr_common::metrics::elapsed_compute::ElapsedComputeFutureExt`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr_common.metrics.elapsed_compute.ElapsedComputeFutureExt.json).

<a id="op-87438364a5591158994bcfd3"></a>
## ElapsedComputeFutureExt

`trait` · `datafusion_physical_expr_common::metrics::elapsed_compute::ElapsedComputeFutureExt` · datafusion-physical-expr-common 55.1.0

```rust
trait ElapsedComputeFutureExt: Future + Sized
```

Source: `src/metrics/elapsed_compute.rs:86`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Extension trait that wraps any [`Future`] with [`ElapsedComputeFuture`](../operations/datafusion_physical_expr_common.metrics.elapsed_compute.ElapsedComputeFuture.md#op-35519ba29206359f0bacd223).

Unresolved upstream links (retained, not inferred): ``Future``.

<a id="op-a03fd9fa7b70a299c179ec8d"></a>
## with_elapsed_compute

`function` · `datafusion_physical_expr_common::metrics::elapsed_compute::ElapsedComputeFutureExt::with_elapsed_compute` · datafusion-physical-expr-common 55.1.0

```rust
fn with_elapsed_compute(self, elapsed_compute: Time) -> ElapsedComputeFuture<Self>
```

Source: `src/metrics/elapsed_compute.rs:90`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Wraps this future so that the time spent inside each [`Future::poll`]
call is accumulated into `elapsed_compute`. See [`ElapsedComputeFuture`](../operations/datafusion_physical_expr_common.metrics.elapsed_compute.ElapsedComputeFuture.md#op-35519ba29206359f0bacd223)
for a full description of what is and is not measured.

Unresolved upstream links (retained, not inferred): ``Future::poll``.
