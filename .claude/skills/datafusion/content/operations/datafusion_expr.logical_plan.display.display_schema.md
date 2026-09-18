# `datafusion_expr::logical_plan::display::display_schema`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.logical_plan.display.display_schema.json).

<a id="op-3a90a0ac54fc9884c2b49977"></a>
## display_schema

`function` · `datafusion_expr::logical_plan::display::display_schema` · datafusion-expr 55.1.0

```rust
fn display_schema(schema: &arrow::datatypes::Schema) -> impl fmt::Display + '_
```

Source: `src/logical_plan/display.rs:109`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Print the schema in a compact representation to `buf`

For example: `foo:Utf8` if `foo` can not be null, and
`foo:Utf8;N` if `foo` is nullable.

```
use arrow::datatypes::{DataType, Field, Schema};
# use datafusion_expr::logical_plan::display_schema;
let schema = Schema::new(vec![
    Field::new("id", DataType::Int32, false),
    Field::new("first_name", DataType::Utf8, true),
]);

assert_eq!(
    "[id:Int32, first_name:Utf8;N]",
    format!("{}", display_schema(&schema))
);
```
