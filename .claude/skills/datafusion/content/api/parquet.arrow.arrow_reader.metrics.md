# `parquet::arrow::arrow_reader::metrics`

Crate `parquet` · 2 public items · structured records in [`model/parquet.arrow.arrow_reader.metrics.json`](../model/parquet.arrow.arrow_reader.metrics.json)

## ArrowReaderMetrics

`enum` · `parquet::arrow::arrow_reader::metrics::ArrowReaderMetrics`

```rust
enum ArrowReaderMetrics
```

**Variants**: `Disabled`, `Enabled`

**Derives**: Clone, Debug

**Methods** (4)

```rust
fn disabled() -> Self
fn enabled() -> Self
fn records_read_from_cache(&self) -> Option<usize>
fn records_read_from_inner(&self) -> Option<usize>
```

[Full member, field, variant and typed contracts](../operations/parquet.arrow.arrow_reader.metrics.ArrowReaderMetrics.md).


This enum represents the state of Arrow reader metrics collection.

The inner metrics are stored in an `Arc<ArrowReaderMetricsInner>`
so cloning the `ArrowReaderMetrics` enum will not clone the inner metrics.

To access metrics, create an `ArrowReaderMetrics` via [`ArrowReaderMetrics::enabled()`]
and configure the `ArrowReaderBuilder` with a clone.

---

## ArrowReaderMetricsInner

`struct` · `parquet::arrow::arrow_reader::metrics::ArrowReaderMetricsInner`

```rust
struct ArrowReaderMetricsInner
```

**Derives**: Debug

[Full member, field, variant and typed contracts](../operations/parquet.arrow.arrow_reader.metrics.ArrowReaderMetricsInner.md).


Holds the actual metrics for the Arrow reader.

Please see [`ArrowReaderMetrics`] for the public interface.

---
