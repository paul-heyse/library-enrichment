# `datafusion_physical_expr_common::metrics::value::MetricValue::Custom`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr_common.metrics.value.MetricValue.Custom.json).

<a id="op-ac2e7132a22714e12ac62b5d"></a>
## name

`struct_field` · `datafusion_physical_expr_common::metrics::value::MetricValue::Custom::name` · datafusion-physical-expr-common 55.1.0

```rust
name: std::borrow::Cow<'static, str>
```

Source: `src/metrics/value.rs:705`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

The provided name of this metric

<a id="op-a6e571c1447091ed71a04a28"></a>
## value

`struct_field` · `datafusion_physical_expr_common::metrics::value::MetricValue::Custom::value` · datafusion-physical-expr-common 55.1.0

```rust
value: std::sync::Arc<dyn CustomMetricValue>
```

Source: `src/metrics/value.rs:707`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

A custom implementation of the metric value.
