# `datafusion_physical_expr_common::metrics::baseline::RecordOutput`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr_common.metrics.baseline.RecordOutput.json).

<a id="op-3d47c46d82c1365d5c0c4dae"></a>
## RecordOutput

`trait` · `datafusion_physical_expr_common::metrics::baseline::RecordOutput` · datafusion-physical-expr-common 55.1.0

```rust
trait RecordOutput
```

Source: `src/metrics/baseline.rs:316`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Trait for things that produce output rows as a result of execution.

<a id="op-3141ff90c7587e3ca0578531"></a>
## record_output

`function` · `datafusion_physical_expr_common::metrics::baseline::RecordOutput::record_output` · datafusion-physical-expr-common 55.1.0

```rust
fn record_output(self, bm: &BaselineMetrics) -> Self
```

Source: `src/metrics/baseline.rs:321`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Record that some number of output rows have been produced

Meant to be composable so that instead of returning `batch`
the operator can return `batch.record_output(baseline_metrics)`
