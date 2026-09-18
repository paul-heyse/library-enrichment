# `datafusion_datasource_parquet::schema_coercion::Int96Coercer`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource_parquet.schema_coercion.Int96Coercer.json).

<a id="op-deb7f4a75c0180b58e0bdc25"></a>
## Int96Coercer

`struct` · `datafusion_datasource_parquet::schema_coercion::Int96Coercer` · datafusion-datasource-parquet 55.1.0

```rust
struct Int96Coercer<'a>
```

Source: `src/schema_coercion.rs:181`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Builder for coercing INT96-originated Timestamp columns in `file_schema`
to a specific [`TimeUnit`](../operations/arrow_schema.datatype.TimeUnit.md#op-4e6bd6e27e392e178d8a5b2e), optionally attaching a timezone.

INT96 is the legacy Parquet representation that systems like Spark use for
timestamps. Arrow surfaces it as `Timestamp(Nanosecond, None)`, but the
underlying values are written as UTC-adjusted instants. Use this builder
to:

- Coerce INT96-derived columns to a smaller [`TimeUnit`](../operations/arrow_schema.datatype.TimeUnit.md#op-4e6bd6e27e392e178d8a5b2e) (e.g. microseconds)
  to extend the representable date range.
- Optionally attach a timezone so the resulting Arrow type carries the
  timezone-aware semantic (`Timestamp(unit, Some(tz))`). Without a
  timezone, INT96-derived columns become `Timestamp(unit, None)` — the
  historical default.

Returns `None` if `file_schema` contains no INT96-derived columns.

# Example

```ignore
use std::sync::Arc;
use arrow::datatypes::TimeUnit;
use datafusion_datasource_parquet::Int96Coercer;

let coerced = Int96Coercer::new(parquet_schema, file_schema, &TimeUnit::Microsecond)
    .with_timezone(Some(Arc::from("UTC")))
    .coerce();
```

<a id="op-245ddbc64f41f2d107f6e0a1"></a>
## coerce

`function` · `datafusion_datasource_parquet::schema_coercion::Int96Coercer::coerce` · datafusion-datasource-parquet 55.1.0

```rust
fn coerce(self) -> Option<Schema>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_datasource_parquet::schema_coercion::Int96Coercer", "path": "Int96Coercer"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [188, 1], "end": [231, 2], "filename": "src/schema_coercion.rs"}, "trait": null, "trait_path": null}`

Source: `src/schema_coercion.rs:217`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Run the coercion, returning the rewritten schema or `None` if
`file_schema` contains no INT96-derived columns.

<a id="op-b20c7aa6d196483d8187e33f"></a>
## new

`function` · `datafusion_datasource_parquet::schema_coercion::Int96Coercer::new` · datafusion-datasource-parquet 55.1.0

```rust
fn new(parquet_schema: &'a SchemaDescriptor, file_schema: &'a Schema, time_unit: &'a TimeUnit) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_datasource_parquet::schema_coercion::Int96Coercer", "path": "Int96Coercer"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [188, 1], "end": [231, 2], "filename": "src/schema_coercion.rs"}, "trait": null, "trait_path": null}`

Source: `src/schema_coercion.rs:191`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Create a new builder. INT96-derived columns will coerce to
`Timestamp(time_unit, None)` unless [`Self::with_timezone`](../operations/datafusion_datasource_parquet.schema_coercion.Int96Coercer.md#op-5cef0cf54d16790f0155946b) is set.

<a id="op-5cef0cf54d16790f0155946b"></a>
## with_timezone

`function` · `datafusion_datasource_parquet::schema_coercion::Int96Coercer::with_timezone` · datafusion-datasource-parquet 55.1.0

```rust
fn with_timezone(self, timezone: Option<Arc<str>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_datasource_parquet::schema_coercion::Int96Coercer", "path": "Int96Coercer"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [188, 1], "end": [231, 2], "filename": "src/schema_coercion.rs"}, "trait": null, "trait_path": null}`

Source: `src/schema_coercion.rs:210`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

Attach a timezone to INT96-derived columns. When `Some`, INT96-derived
columns coerce to `Timestamp(time_unit, Some(timezone))` instead of
the default `Timestamp(time_unit, None)`. Spark and other systems
write INT96 as UTC-adjusted instants, so callers that need the
resulting Arrow type to be timezone-aware should pass
`Some(Arc::from("UTC"))`.
