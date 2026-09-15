# `datafusion_datasource_parquet::schema_coercion`

Crate `datafusion-datasource-parquet` · 5 public items · structured records in [`model/datafusion_datasource_parquet.schema_coercion.json`](../model/datafusion_datasource_parquet.schema_coercion.json)

## apply_file_schema_type_coercions

`function` · `datafusion_datasource_parquet::schema_coercion::apply_file_schema_type_coercions`

Also reachable as `datafusion::datasource::file_format::parquet::apply_file_schema_type_coercions`, `datafusion::datasource::physical_plan::parquet::apply_file_schema_type_coercions`, `datafusion_datasource_parquet::apply_file_schema_type_coercions`, `datafusion_datasource_parquet::file_format::apply_file_schema_type_coercions`

```rust
fn apply_file_schema_type_coercions(table_schema: &arrow::datatypes::Schema, file_schema: &arrow::datatypes::Schema) -> Option<arrow::datatypes::Schema>
```

Apply necessary schema type coercions to make file schema match table schema.

This function performs two main types of transformations in a single pass:
1. Binary types to string types conversion - Converts binary data types to their
   corresponding string types when the table schema expects string data
2. Regular to view types conversion - Converts standard string/binary types to
   view types when the table schema uses view types

# Arguments
* `table_schema` - The table schema containing the desired types
* `file_schema` - The file schema to be transformed

# Returns
* `Some(Schema)` - If any transformations were applied, returns the transformed schema
* `None` - If no transformations were needed

---

## coerce_int96_to_resolution

`function` · `datafusion_datasource_parquet::schema_coercion::coerce_int96_to_resolution`

> **Deprecated** — since 53.2.0: use `Int96Coercer` instead

Also reachable as `datafusion::datasource::file_format::parquet::coerce_int96_to_resolution`, `datafusion::datasource::physical_plan::parquet::coerce_int96_to_resolution`, `datafusion_datasource_parquet::coerce_int96_to_resolution`, `datafusion_datasource_parquet::file_format::coerce_int96_to_resolution`

```rust
fn coerce_int96_to_resolution(parquet_schema: &parquet::schema::types::SchemaDescriptor, file_schema: &arrow::datatypes::Schema, time_unit: &arrow::datatypes::TimeUnit) -> Option<arrow::datatypes::Schema>
```

Coerces the file schema's Timestamps to the provided TimeUnit if the
Parquet schema contains INT96.

Deprecated wrapper around [`Int96Coercer`]; use the builder directly
instead — it also supports attaching a timezone via
[`Int96Coercer::with_timezone`].

---

## transform_binary_to_string

`function` · `datafusion_datasource_parquet::schema_coercion::transform_binary_to_string`

Also reachable as `datafusion::datasource::file_format::parquet::transform_binary_to_string`, `datafusion::datasource::physical_plan::parquet::transform_binary_to_string`, `datafusion_datasource_parquet::file_format::transform_binary_to_string`, `datafusion_datasource_parquet::transform_binary_to_string`

```rust
fn transform_binary_to_string(schema: &arrow::datatypes::Schema) -> arrow::datatypes::Schema
```

Transform a schema so that any binary types are strings

---

## transform_schema_to_view

`function` · `datafusion_datasource_parquet::schema_coercion::transform_schema_to_view`

Also reachable as `datafusion::datasource::file_format::parquet::transform_schema_to_view`, `datafusion::datasource::physical_plan::parquet::transform_schema_to_view`, `datafusion_datasource_parquet::file_format::transform_schema_to_view`, `datafusion_datasource_parquet::transform_schema_to_view`

```rust
fn transform_schema_to_view(schema: &arrow::datatypes::Schema) -> arrow::datatypes::Schema
```

Transform a schema to use view types for Utf8 and Binary

See [`ParquetFormat::force_view_types`](crate::file_format::ParquetFormat::force_view_types) for details

---

## Int96Coercer

`struct` · `datafusion_datasource_parquet::schema_coercion::Int96Coercer`

Also reachable as `datafusion::datasource::file_format::parquet::Int96Coercer`, `datafusion::datasource::physical_plan::parquet::Int96Coercer`, `datafusion_datasource_parquet::Int96Coercer`, `datafusion_datasource_parquet::file_format::Int96Coercer`

```rust
struct Int96Coercer<'a>
```

**Methods** (3)

```rust
fn coerce(self) -> Option<Schema>
fn new(parquet_schema: &'a SchemaDescriptor, file_schema: &'a Schema, time_unit: &'a TimeUnit) -> Self
fn with_timezone(self, timezone: Option<Arc<str>>) -> Self
```

Builder for coercing INT96-originated Timestamp columns in `file_schema`
to a specific [`TimeUnit`], optionally attaching a timezone.

INT96 is the legacy Parquet representation that systems like Spark use for
timestamps. Arrow surfaces it as `Timestamp(Nanosecond, None)`, but the
underlying values are written as UTC-adjusted instants. Use this builder
to:

- Coerce INT96-derived columns to a smaller [`TimeUnit`] (e.g. microseconds)
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

---
