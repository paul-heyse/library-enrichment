# `datafusion_datasource_parquet::row_filter`

Crate `datafusion-datasource-parquet` · 2 public items · structured records in [`model/datafusion_datasource_parquet.row_filter.json`](../model/datafusion_datasource_parquet.row_filter.json)

## build_row_filter

`function` · `datafusion_datasource_parquet::row_filter::build_row_filter`

Also reachable as `datafusion::datasource::physical_plan::parquet::build_row_filter`, `datafusion_datasource_parquet::build_row_filter`

```rust
fn build_row_filter(expr: &std::sync::Arc<dyn PhysicalExpr>, file_schema: &arrow::datatypes::SchemaRef, metadata: &parquet::file::metadata::ParquetMetaData, reorder_predicates: bool, file_metrics: &super::ParquetFileMetrics) -> datafusion_common::Result<Option<parquet::arrow::arrow_reader::RowFilter>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_datasource_parquet.row_filter.build_row_filter.md).


Build a [`RowFilter`] from the given predicate expression if possible.

# Arguments
* `expr` - The filter predicate, already adapted to reference columns in `file_schema`
* `file_schema` - The Arrow schema of the parquet file (the result of converting
  the parquet schema to Arrow, potentially with type coercions applied)
* `metadata` - Parquet file metadata used for cost estimation
* `reorder_predicates` - If true, reorder predicates to minimize I/O
* `file_metrics` - Metrics for tracking filter performance

# Returns
* `Ok(Some(row_filter))` if the expression can be used as a RowFilter
* `Ok(None)` if the expression cannot be used as a RowFilter
* `Err(e)` if an error occurs while building the filter

Note: The returned `RowFilter` may not contain all conjuncts from the original
expression. Conjuncts that cannot be evaluated as an `ArrowPredicate` are ignored.

For example, if the expression is `a = 1 AND b = 2 AND c = 3` and `b = 2`
cannot be evaluated for some reason, the returned `RowFilter` will contain
only `a = 1` and `c = 3`.

---

## can_expr_be_pushed_down_with_schemas

`function` · `datafusion_datasource_parquet::row_filter::can_expr_be_pushed_down_with_schemas`

Also reachable as `datafusion::datasource::physical_plan::parquet::can_expr_be_pushed_down_with_schemas`, `datafusion_datasource_parquet::can_expr_be_pushed_down_with_schemas`

```rust
fn can_expr_be_pushed_down_with_schemas(expr: &std::sync::Arc<dyn PhysicalExpr>, file_schema: &arrow::datatypes::Schema) -> bool
```

[Full member, field, variant and typed contracts](../operations/datafusion_datasource_parquet.row_filter.can_expr_be_pushed_down_with_schemas.md).


Checks if a predicate expression can be pushed down to the parquet decoder.

Returns `true` if all columns referenced by the expression:
- Exist in the provided schema
- Are primitive types OR list columns with supported predicates
  (e.g., `array_has`, `array_has_all`, `array_has_any`, IS NULL, IS NOT NULL)
- Are struct columns accessed via `get_field` where the leaf type is primitive
- Direct references to whole struct columns will prevent pushdown

# Arguments
* `expr` - The filter expression to check
* `file_schema` - The Arrow schema of the parquet file (or table schema when
  the file schema is not yet available during planning)

# Examples

Primitive column filters can be pushed down:
```ignore
use datafusion_expr::{col, Expr};
use datafusion_common::ScalarValue;
use arrow::datatypes::{DataType, Field, Schema};
use std::sync::Arc;

let schema = Arc::new(Schema::new(vec![
    Field::new("age", DataType::Int32, false),
]));

// Primitive filter: can be pushed down
let expr = col("age").gt(Expr::Literal(ScalarValue::Int32(Some(30)), None));
let expr = logical2physical(&expr, &schema);
assert!(can_expr_be_pushed_down_with_schemas(&expr, &schema));
```

Struct column filters cannot be pushed down:
```ignore
use arrow::datatypes::Fields;

let schema = Arc::new(Schema::new(vec![
    Field::new("person", DataType::Struct(
        Fields::from(vec![Field::new("name", DataType::Utf8, true)])
    ), true),
]));

// Struct filter: cannot be pushed down
let expr = col("person").is_not_null();
let expr = logical2physical(&expr, &schema);
assert!(!can_expr_be_pushed_down_with_schemas(&expr, &schema));
```

List column filters with supported predicates can be pushed down:
```ignore
use datafusion_functions_nested::expr_fn::{array_has_all, make_array};

let schema = Arc::new(Schema::new(vec![
    Field::new("tags", DataType::List(
        Arc::new(Field::new("item", DataType::Utf8, true))
    ), true),
]));

// Array filter with supported predicate: can be pushed down
let expr = array_has_all(col("tags"), make_array(vec![
    Expr::Literal(ScalarValue::Utf8(Some("rust".to_string())), None)
]));
let expr = logical2physical(&expr, &schema);
assert!(can_expr_be_pushed_down_with_schemas(&expr, &schema));
```

---
