# `datafusion_physical_expr_adapter::schema_rewriter::PhysicalExprAdapter`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr_adapter.schema_rewriter.PhysicalExprAdapter.json).

<a id="op-3d7f05fc0e38d49de65f327a"></a>
## PhysicalExprAdapter

`trait` · `datafusion_physical_expr_adapter::schema_rewriter::PhysicalExprAdapter` · datafusion-physical-expr-adapter 55.1.0

```rust
trait PhysicalExprAdapter: Send + Sync + std::fmt::Debug
```

Source: `src/schema_rewriter.rs:152`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-adapter/55.1.0/json).

Trait for adapting [`PhysicalExpr`](../operations/datafusion_physical_expr_common.physical_expr.PhysicalExpr.md#op-fe8284c43330456b0d4e6af7) expressions to match a target schema.

This is used in file scans to rewrite expressions so that they can be
evaluated against the physical schema of the file being scanned. It allows
for handling differences between logical and physical schemas, such as type
mismatches or missing columns common in [Schema evolution] scenarios.

[Schema evolution]: https://www.dremio.com/wiki/schema-evolution/

## Default Implementations

The default implementation [`DefaultPhysicalExprAdapter`](../operations/datafusion_physical_expr_adapter.schema_rewriter.DefaultPhysicalExprAdapter.md#op-0786dd5cec45ace7db749372)  handles common
cases.

## Custom Implementations

You can create a custom implementation of this trait to handle specific rewriting logic.
For example, to fill in missing columns with default values instead of nulls:

```rust
use datafusion_physical_expr_adapter::{PhysicalExprAdapter, PhysicalExprAdapterFactory};
use arrow::datatypes::{Schema, Field, DataType, FieldRef, SchemaRef};
use datafusion_physical_expr_common::physical_expr::PhysicalExpr;
use datafusion_common::{Result, ScalarValue, tree_node::{Transformed, TransformedResult, TreeNode}};
use datafusion_physical_expr::expressions::{self, Column};
use std::sync::Arc;

#[derive(Debug)]
pub struct CustomPhysicalExprAdapter {
    logical_file_schema: SchemaRef,
    physical_file_schema: SchemaRef,
}

impl PhysicalExprAdapter for CustomPhysicalExprAdapter {
    fn rewrite(&self, expr: Arc<dyn PhysicalExpr>) -> Result<Arc<dyn PhysicalExpr>> {
        expr.transform(|expr| {
            if let Some(column) = expr.downcast_ref::<Column>() {
                // Check if the column exists in the physical schema
                if self.physical_file_schema.index_of(column.name()).is_err() {
                    // If the column is missing, fill it with a default value instead of null
                    // The default value could be stored in the table schema's column metadata for example.
                    let default_value = ScalarValue::Int32(Some(0));
                    return Ok(Transformed::yes(expressions::lit(default_value)));
                }
            }
            // If the column exists, return it as is
            Ok(Transformed::no(expr))
        }).data()
    }
}

#[derive(Debug)]
pub struct CustomPhysicalExprAdapterFactory;

impl PhysicalExprAdapterFactory for CustomPhysicalExprAdapterFactory {
    fn create(
        &self,
        logical_file_schema: SchemaRef,
        physical_file_schema: SchemaRef,
    ) -> Result<Arc<dyn PhysicalExprAdapter>> {
        Ok(Arc::new(CustomPhysicalExprAdapter {
            logical_file_schema,
            physical_file_schema,
        }))
    }
}
```

<a id="op-828e0b034441d54fdaa48a89"></a>
## rewrite

`function` · `datafusion_physical_expr_adapter::schema_rewriter::PhysicalExprAdapter::rewrite` · datafusion-physical-expr-adapter 55.1.0

```rust
fn rewrite(&self, expr: Arc<dyn PhysicalExpr>) -> Result<Arc<dyn PhysicalExpr>>
```

Source: `src/schema_rewriter.rs:169`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-adapter/55.1.0/json).

Rewrite a physical expression to match the target schema.

This method should return a transformed expression that matches the target schema.

Arguments:
- `expr`: The physical expression to rewrite.
- `logical_file_schema`: The logical schema of the table being queried, excluding any partition columns.
- `physical_file_schema`: The physical schema of the file being scanned.
- `partition_values`: Optional partition values to use for rewriting partition column references.
  These are handled as if they were columns appended onto the logical file schema.

Returns:
- `Arc<dyn PhysicalExpr>`: The rewritten physical expression that can be evaluated against the physical schema.

See Also:
- [`replace_columns_with_literals`](../operations/datafusion_physical_expr_adapter.schema_rewriter.replace_columns_with_literals.md#op-c25b062649f7c614e6bac900): for replacing partition column references with their literal values.
