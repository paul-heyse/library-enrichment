# `parquet::arrow::arrow_reader::metrics::ArrowReaderMetrics`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.arrow.arrow_reader.metrics.ArrowReaderMetrics.json).

<a id="op-c2424d6c9242ce925e8a9bf7"></a>
## ArrowReaderMetrics

`enum` · `parquet::arrow::arrow_reader::metrics::ArrowReaderMetrics` · parquet 59.3.0

```rust
enum ArrowReaderMetrics
```

Source: `src/arrow/arrow_reader/metrics.rs:31`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

This enum represents the state of Arrow reader metrics collection.

The inner metrics are stored in an `Arc<ArrowReaderMetricsInner>`
so cloning the `ArrowReaderMetrics` enum will not clone the inner metrics.

To access metrics, create an `ArrowReaderMetrics` via [`ArrowReaderMetrics::enabled()`](../operations/parquet.arrow.arrow_reader.metrics.ArrowReaderMetrics.md#op-70aced14453bed4ce0e4948a)
and configure the `ArrowReaderBuilder` with a clone.

<a id="op-d5dba25bb20e32a0d106483f"></a>
## Disabled

`variant` · `parquet::arrow::arrow_reader::metrics::ArrowReaderMetrics::Disabled` · parquet 59.3.0

```rust
Disabled
```

Source: `src/arrow/arrow_reader/metrics.rs:33`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Metrics are not collected (default)

<a id="op-82155d7f8292baa33085fc3a"></a>
## Enabled

`variant` · `parquet::arrow::arrow_reader::metrics::ArrowReaderMetrics::Enabled` · parquet 59.3.0

```rust
Enabled
```

Source: `src/arrow/arrow_reader/metrics.rs:37`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Metrics are collected and stored in an `Arc`.

Create this via [`ArrowReaderMetrics::enabled()`](../operations/parquet.arrow.arrow_reader.metrics.ArrowReaderMetrics.md#op-70aced14453bed4ce0e4948a).

<a id="op-15e080d63474a637838ee691"></a>
## clone

`function` · `parquet::arrow::arrow_reader::metrics::ArrowReaderMetrics::clone` · parquet 59.3.0

```rust
fn clone(&self) -> ArrowReaderMetrics
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_reader::metrics::ArrowReaderMetrics", "path": "ArrowReaderMetrics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [30, 17], "end": [30, 22], "filename": "src/arrow/arrow_reader/metrics.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/arrow/arrow_reader/metrics.rs:30`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-85fcb0659df38d993d0a0177"></a>
## disabled

`function` · `parquet::arrow::arrow_reader::metrics::ArrowReaderMetrics::disabled` · parquet 59.3.0

```rust
fn disabled() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_reader::metrics::ArrowReaderMetrics", "path": "ArrowReaderMetrics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [40, 1], "end": [113, 2], "filename": "src/arrow/arrow_reader/metrics.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_reader/metrics.rs:42`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Creates a new instance of [`ArrowReaderMetrics::Disabled`](../operations/parquet.arrow.arrow_reader.metrics.ArrowReaderMetrics.md#op-d5dba25bb20e32a0d106483f)

<a id="op-70aced14453bed4ce0e4948a"></a>
## enabled

`function` · `parquet::arrow::arrow_reader::metrics::ArrowReaderMetrics::enabled` · parquet 59.3.0

```rust
fn enabled() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_reader::metrics::ArrowReaderMetrics", "path": "ArrowReaderMetrics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [40, 1], "end": [113, 2], "filename": "src/arrow/arrow_reader/metrics.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_reader/metrics.rs:47`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Creates a new instance of [`ArrowReaderMetrics::Enabled`](../operations/parquet.arrow.arrow_reader.metrics.ArrowReaderMetrics.md#op-82155d7f8292baa33085fc3a)

<a id="op-54461b1f6246b2a5a72eedc3"></a>
## fmt

`function` · `parquet::arrow::arrow_reader::metrics::ArrowReaderMetrics::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_reader::metrics::ArrowReaderMetrics", "path": "ArrowReaderMetrics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [30, 10], "end": [30, 15], "filename": "src/arrow/arrow_reader/metrics.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/arrow/arrow_reader/metrics.rs:30`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f68f3b8f401d29d0332a59a5"></a>
## records_read_from_cache

`function` · `parquet::arrow::arrow_reader::metrics::ArrowReaderMetrics::records_read_from_cache` · parquet 59.3.0

```rust
fn records_read_from_cache(&self) -> Option<usize>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_reader::metrics::ArrowReaderMetrics", "path": "ArrowReaderMetrics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [40, 1], "end": [113, 2], "filename": "src/arrow/arrow_reader/metrics.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_reader/metrics.rs:82`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Predicate Cache: number of records read from the cache

This is the total number of records read from the cache actually
decoding). It measures the amount of work that was avoided with caching.

It returns the number of records read across all columns, so if you read
2 columns each with 100 records from the cache, this will return 200.

Returns None if metrics are disabled.

<a id="op-b5301107e614b0f8b94880d7"></a>
## records_read_from_inner

`function` · `parquet::arrow::arrow_reader::metrics::ArrowReaderMetrics::records_read_from_inner` · parquet 59.3.0

```rust
fn records_read_from_inner(&self) -> Option<usize>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::arrow_reader::metrics::ArrowReaderMetrics", "path": "ArrowReaderMetrics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [40, 1], "end": [113, 2], "filename": "src/arrow/arrow_reader/metrics.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_reader/metrics.rs:62`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Predicate Cache: number of records read directly from the inner reader

This is the total number of records read from the inner reader (that is
actually decoding). It measures the amount of work that could not be
avoided with caching.

It returns the number of records read across all columns, so if you read
2 columns each with 100 records, this will return 200.


Returns None if metrics are disabled.
