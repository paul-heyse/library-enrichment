# `datafusion_physical_expr_common::metrics::custom::CustomMetricValue`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr_common.metrics.custom.CustomMetricValue.json).

<a id="op-704fc39805dfc975c900f015"></a>
## CustomMetricValue

`trait` · `datafusion_physical_expr_common::metrics::custom::CustomMetricValue` · datafusion-physical-expr-common 55.1.0

```rust
trait CustomMetricValue: Display + Debug + Send + Sync
```

Source: `src/metrics/custom.rs:90`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

A trait for implementing custom metric values.

This trait enables defining application- or operator-specific metric types
that can be aggregated and displayed alongside standard metrics. These
custom metrics integrate with [`MetricValue::Custom`] and support
aggregation logic, introspection, and optional numeric representation.

# Requirements
Implementations of `CustomMetricValue` must satisfy the following:

1. [`Self::aggregate`](../operations/datafusion_physical_expr_common.metrics.custom.CustomMetricValue.md#op-d81922d97f52b4918fd98f37): Defines how two metric values are combined
2. [`Self::new_empty`](../operations/datafusion_physical_expr_common.metrics.custom.CustomMetricValue.md#op-74c90cd32e5b78e04f0ac92d): Returns a new, zero-value instance for accumulation
3. [`Self::as_any`](../operations/datafusion_physical_expr_common.metrics.custom.CustomMetricValue.md#op-e4186d0720a30490a520a4e5): Enables dynamic downcasting for type-specific operations
4. [`Self::as_usize`](../operations/datafusion_physical_expr_common.metrics.custom.CustomMetricValue.md#op-2a06a94486168b5a61196453): Optionally maps the value to a `usize` (for sorting, display, etc.)
5. [`Self::is_eq`](../operations/datafusion_physical_expr_common.metrics.custom.CustomMetricValue.md#op-832287bd5cf6f5862d9bb943): Implements comparison between two values, this isn't reusing the std
   PartialEq trait because this trait is used dynamically in the context of
   [`MetricValue::Custom`]

# Examples
```
# use std::sync::Arc;
# use std::fmt::{Debug, Display};
# use std::any::Any;
# use std::sync::atomic::{AtomicUsize, Ordering};

# use datafusion_physical_expr_common::metrics::CustomMetricValue;

#[derive(Debug, Default)]
struct MyCounter {
    count: AtomicUsize,
}

impl Display for MyCounter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "count: {}", self.count.load(Ordering::Relaxed))
    }
}

impl CustomMetricValue for MyCounter {
    fn new_empty(&self) -> Arc<dyn CustomMetricValue> {
        Arc::new(Self::default())
    }

    fn aggregate(&self, other: Arc<dyn CustomMetricValue>) {
        let other = other.as_any().downcast_ref::<Self>().unwrap();
        self.count
            .fetch_add(other.count.load(Ordering::Relaxed), Ordering::Relaxed);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_usize(&self) -> usize {
        self.count.load(Ordering::Relaxed)
    }

    fn is_eq(&self, other: &Arc<dyn CustomMetricValue>) -> bool {
        let Some(other) = other.as_any().downcast_ref::<Self>() else {
            return false;
        };

        self.count.load(Ordering::Relaxed) == other.count.load(Ordering::Relaxed)
    }
}
```

[`MetricValue::Custom`]: super::MetricValue::Custom

<a id="op-d81922d97f52b4918fd98f37"></a>
## aggregate

`function` · `datafusion_physical_expr_common::metrics::custom::CustomMetricValue::aggregate` · datafusion-physical-expr-common 55.1.0

```rust
fn aggregate(&self, other: Arc<dyn CustomMetricValue + 'static>)
```

Source: `src/metrics/custom.rs:99`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Merges another metric value into this one.

The type of `other` could be of a different custom type as long as it's aggregatable into self.

<a id="op-e4186d0720a30490a520a4e5"></a>
## as_any

`function` · `datafusion_physical_expr_common::metrics::custom::CustomMetricValue::as_any` · datafusion-physical-expr-common 55.1.0

```rust
fn as_any(&self) -> &dyn Any
```

Source: `src/metrics/custom.rs:102`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Returns this value as a [`Any`] to support dynamic downcasting.

Unresolved upstream links (retained, not inferred): ``Any``.

<a id="op-2a06a94486168b5a61196453"></a>
## as_usize

`function` · `datafusion_physical_expr_common::metrics::custom::CustomMetricValue::as_usize` · datafusion-physical-expr-common 55.1.0

```rust
fn as_usize(&self) -> usize
```

Source: `src/metrics/custom.rs:108`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Optionally returns a numeric representation of the value, if meaningful.
Otherwise will default to zero.

This is used for sorting and summarizing metrics.

<a id="op-832287bd5cf6f5862d9bb943"></a>
## is_eq

`function` · `datafusion_physical_expr_common::metrics::custom::CustomMetricValue::is_eq` · datafusion-physical-expr-common 55.1.0

```rust
fn is_eq(&self, other: &Arc<dyn CustomMetricValue>) -> bool
```

Source: `src/metrics/custom.rs:113`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Compares this value with another custom value.

<a id="op-74c90cd32e5b78e04f0ac92d"></a>
## new_empty

`function` · `datafusion_physical_expr_common::metrics::custom::CustomMetricValue::new_empty` · datafusion-physical-expr-common 55.1.0

```rust
fn new_empty(&self) -> Arc<dyn CustomMetricValue>
```

Source: `src/metrics/custom.rs:94`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Returns a new, zero-initialized version of this metric value.

This value is used during metric aggregation to accumulate results.
