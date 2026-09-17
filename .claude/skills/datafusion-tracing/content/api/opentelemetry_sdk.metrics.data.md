# `opentelemetry_sdk::metrics::data`

Crate `opentelemetry_sdk` · 15 public items · structured records in [`model/opentelemetry_sdk.metrics.data.json`](../model/opentelemetry_sdk.metrics.data.json)

## AggregatedMetrics

`enum` · `opentelemetry_sdk::metrics::data::AggregatedMetrics`

```rust
enum AggregatedMetrics
```

**Variants**: `F64`, `U64`, `I64`

**Implements**: `core::convert::From`

**Derives**: Debug

**via `core::convert::From`**

```rust
fn from(value: MetricData<i64>) -> Self
fn from(value: MetricData<f64>) -> Self
fn from(value: MetricData<u64>) -> Self
```

Aggregated metrics data from an instrument

---

## MetricData

`enum` · `opentelemetry_sdk::metrics::data::MetricData`

```rust
enum MetricData<T>
```

**Variants**: `Gauge`, `Sum`, `Histogram`, `ExponentialHistogram`

**Implements**: `core::convert::From`

**Derives**: Debug

**via `core::convert::From`**

```rust
fn from(value: Histogram<T>) -> Self
fn from(value: Sum<T>) -> Self
fn from(value: Gauge<T>) -> Self
fn from(value: ExponentialHistogram<T>) -> Self
```

Metric data for all types

---

## Exemplar

`struct` · `opentelemetry_sdk::metrics::data::Exemplar`

```rust
struct Exemplar<T>
```

**Fields**: `value`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**Methods** (4)

```rust
fn filtered_attributes(&self) -> impl Iterator<Item = &KeyValue>
fn span_id(&self) -> &[u8; 8]
fn time(&self) -> SystemTime
fn trace_id(&self) -> &[u8; 16]
```

A measurement sampled from a time series providing a typical example.

---

## ExponentialBucket

`struct` · `opentelemetry_sdk::metrics::data::ExponentialBucket`

```rust
struct ExponentialBucket
```

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn counts(&self) -> impl Iterator<Item = u64> + '_
fn offset(&self) -> i32
```

A set of bucket counts, encoded in a contiguous array of counts.

---

## ExponentialHistogram

`struct` · `opentelemetry_sdk::metrics::data::ExponentialHistogram`

```rust
struct ExponentialHistogram<T>
```

**Derives**: Clone, Debug

**Methods** (4)

```rust
fn data_points(&self) -> impl Iterator<Item = &ExponentialHistogramDataPoint<T>>
fn start_time(&self) -> SystemTime
fn temporality(&self) -> Temporality
fn time(&self) -> SystemTime
```

The histogram of all measurements of values from an instrument.

---

## ExponentialHistogramDataPoint

`struct` · `opentelemetry_sdk::metrics::data::ExponentialHistogramDataPoint`

```rust
struct ExponentialHistogramDataPoint<T>
```

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**Methods** (11)

```rust
fn attributes(&self) -> impl Iterator<Item = &KeyValue>
fn count(&self) -> usize
fn exemplars(&self) -> impl Iterator<Item = &Exemplar<T>>
fn max(&self) -> Option<T>
fn min(&self) -> Option<T>
fn negative_bucket(&self) -> &ExponentialBucket
fn positive_bucket(&self) -> &ExponentialBucket
fn scale(&self) -> i8
fn sum(&self) -> T
fn zero_count(&self) -> u64
fn zero_threshold(&self) -> f64
```

A single exponential histogram data point in a time series.

---

## Gauge

`struct` · `opentelemetry_sdk::metrics::data::Gauge`

```rust
struct Gauge<T>
```

**Derives**: Clone, Debug

**Methods** (3)

```rust
fn data_points(&self) -> impl Iterator<Item = &GaugeDataPoint<T>>
fn start_time(&self) -> Option<SystemTime>
fn time(&self) -> SystemTime
```

A measurement of the current value of an instrument.

---

## GaugeDataPoint

`struct` · `opentelemetry_sdk::metrics::data::GaugeDataPoint`

```rust
struct GaugeDataPoint<T>
```

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn attributes(&self) -> impl Iterator<Item = &KeyValue>
fn exemplars(&self) -> impl Iterator<Item = &Exemplar<T>>
fn value(&self) -> T
```

DataPoint is a single data point in a time series.

---

## Histogram

`struct` · `opentelemetry_sdk::metrics::data::Histogram`

```rust
struct Histogram<T>
```

**Derives**: Clone, Debug

**Methods** (4)

```rust
fn data_points(&self) -> impl Iterator<Item = &HistogramDataPoint<T>>
fn start_time(&self) -> SystemTime
fn temporality(&self) -> Temporality
fn time(&self) -> SystemTime
```

Represents the histogram of all measurements of values from an instrument.

---

## HistogramDataPoint

`struct` · `opentelemetry_sdk::metrics::data::HistogramDataPoint`

```rust
struct HistogramDataPoint<T>
```

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**Methods** (8)

```rust
fn attributes(&self) -> impl Iterator<Item = &KeyValue>
fn bounds(&self) -> impl Iterator<Item = f64> + '_
fn bucket_counts(&self) -> impl Iterator<Item = u64> + '_
fn count(&self) -> u64
fn exemplars(&self) -> impl Iterator<Item = &Exemplar<T>>
fn max(&self) -> Option<T>
fn min(&self) -> Option<T>
fn sum(&self) -> T
```

A single histogram data point in a time series.

---

## Metric

`struct` · `opentelemetry_sdk::metrics::data::Metric`

```rust
struct Metric
```

**Derives**: Debug

**Methods** (4)

```rust
fn data(&self) -> &AggregatedMetrics
fn description(&self) -> &str
fn name(&self) -> &str
fn unit(&self) -> &str
```

A collection of one or more aggregated time series from an [Instrument].

[Instrument]: crate::metrics::Instrument

---

## ResourceMetrics

`struct` · `opentelemetry_sdk::metrics::data::ResourceMetrics`

```rust
struct ResourceMetrics
```

**Derives**: Debug, Default

**Methods** (2)

```rust
fn resource(&self) -> &Resource
fn scope_metrics(&self) -> impl Iterator<Item = &ScopeMetrics>
```

A collection of [ScopeMetrics] and the associated [Resource] that created them.

---

## ScopeMetrics

`struct` · `opentelemetry_sdk::metrics::data::ScopeMetrics`

```rust
struct ScopeMetrics
```

**Derives**: Debug, Default

**Methods** (2)

```rust
fn metrics(&self) -> impl Iterator<Item = &Metric>
fn scope(&self) -> &InstrumentationScope
```

A collection of metrics produced by a meter.

---

## Sum

`struct` · `opentelemetry_sdk::metrics::data::Sum`

```rust
struct Sum<T>
```

**Derives**: Clone, Debug

**Methods** (5)

```rust
fn data_points(&self) -> impl Iterator<Item = &SumDataPoint<T>>
fn is_monotonic(&self) -> bool
fn start_time(&self) -> SystemTime
fn temporality(&self) -> Temporality
fn time(&self) -> SystemTime
```

Represents the sum of all measurements of values from an instrument.

---

## SumDataPoint

`struct` · `opentelemetry_sdk::metrics::data::SumDataPoint`

```rust
struct SumDataPoint<T>
```

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn attributes(&self) -> impl Iterator<Item = &KeyValue>
fn exemplars(&self) -> impl Iterator<Item = &Exemplar<T>>
fn value(&self) -> T
```

DataPoint is a single data point in a time series.

---
