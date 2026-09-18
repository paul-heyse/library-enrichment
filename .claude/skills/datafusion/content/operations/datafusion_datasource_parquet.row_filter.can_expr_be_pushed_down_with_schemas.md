# `datafusion_datasource_parquet::row_filter::can_expr_be_pushed_down_with_schemas`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource_parquet.row_filter.can_expr_be_pushed_down_with_schemas.json).

<a id="op-9bd3b9e5ac548679cf9156a2"></a>
## can_expr_be_pushed_down_with_schemas

`function` · `datafusion_datasource_parquet::row_filter::can_expr_be_pushed_down_with_schemas` · datafusion-datasource-parquet 55.1.0

```rust
fn can_expr_be_pushed_down_with_schemas(expr: &std::sync::Arc<dyn PhysicalExpr>, file_schema: &arrow::datatypes::Schema) -> bool
```

Source: `src/row_filter.rs:349`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

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
