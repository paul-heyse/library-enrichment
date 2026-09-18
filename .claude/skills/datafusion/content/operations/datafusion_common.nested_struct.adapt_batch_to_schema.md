# `datafusion_common::nested_struct::adapt_batch_to_schema`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.nested_struct.adapt_batch_to_schema.json).

<a id="op-8c223cbccc61fc9a82da1f43"></a>
## adapt_batch_to_schema

`function` · `datafusion_common::nested_struct::adapt_batch_to_schema` · datafusion-common 55.1.0

```rust
fn adapt_batch_to_schema(batch: arrow::array::RecordBatch, target_schema: &arrow::datatypes::SchemaRef) -> error::Result<arrow::array::RecordBatch>
```

Source: `src/nested_struct.rs:1838`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Adapts a `RecordBatch` to conform to `target_schema`, verifying that each target field
type contains the incoming column data type (as verified by [`arrow::datatypes::DataType::contains`])
and transforms the metadata/types of differing columns to match `target_schema`
without copying primitive buffer data.

If `batch` has an incompatible column count or incompatible column data types,
an error is returned.

Unresolved upstream links (retained, not inferred): ``arrow::datatypes::DataType::contains``.
