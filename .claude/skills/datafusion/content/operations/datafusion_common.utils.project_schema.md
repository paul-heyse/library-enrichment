# `datafusion_common::utils::project_schema`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.utils.project_schema.json).

<a id="op-df2cdac6e460d5adda1fc8fc"></a>
## project_schema

`function` · `datafusion_common::utils::project_schema` · datafusion-common 55.1.0

```rust
fn project_schema(schema: &arrow::datatypes::SchemaRef, projection: Option<&impl AsRef<[usize]>>) -> Result<arrow::datatypes::SchemaRef>
```

Source: `src/utils/mod.rs:83`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Applies an optional projection to a [`SchemaRef`](../operations/arrow_schema.schema.SchemaRef.md#op-e48a1bb89d62307cfab4093a), returning the
projected schema

Example:
```
use arrow::datatypes::{DataType, Field, Schema, SchemaRef};
use datafusion_common::project_schema;

// Schema with columns 'a', 'b', and 'c'
let schema = SchemaRef::new(Schema::new(vec![
    Field::new("a", DataType::Int32, true),
    Field::new("b", DataType::Int64, true),
    Field::new("c", DataType::Utf8, true),
]));

// Pick columns 'c' and 'b'
let projection = Some(vec![2, 1]);
let projected_schema = project_schema(&schema, projection.as_ref()).unwrap();

let expected_schema = SchemaRef::new(Schema::new(vec![
    Field::new("c", DataType::Utf8, true),
    Field::new("b", DataType::Int64, true),
]));

assert_eq!(projected_schema, expected_schema);
```
