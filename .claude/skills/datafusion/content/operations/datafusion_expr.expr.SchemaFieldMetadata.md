# `datafusion_expr::expr::SchemaFieldMetadata`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.expr.SchemaFieldMetadata.json).

<a id="op-ad4f614c35e846ecf189a9ad"></a>
## SchemaFieldMetadata

`type_alias` · `datafusion_expr::expr::SchemaFieldMetadata` · datafusion-expr 55.1.0

```rust
type SchemaFieldMetadata = std::collections::HashMap<String, String>
```

Source: `src/expr.rs:625`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The metadata used in [`Field::metadata`].

This represents the metadata associated with an Arrow [`Field`](../operations/arrow_schema.field.Field.md#op-66eb7ef45bcc129b0a0189cf). The metadata consists of key-value pairs.

# Common Use Cases

Field metadata is commonly used to store:
- Default values for columns when data is missing
- Column descriptions or documentation
- Data lineage information
- Custom application-specific annotations
- Encoding hints or display formatting preferences

# Example: Storing Default Values

A practical example of using field metadata is storing default values for columns
that may be missing in the physical data but present in the logical schema.
See the [default_column_values.rs] example implementation.

[default_column_values.rs]: https://github.com/apache/datafusion/blob/main/datafusion-examples/examples/custom_data_source/default_column_values.rs

Unresolved upstream links (retained, not inferred): ``Field::metadata``.
