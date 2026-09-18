# `datafusion_physical_expr_adapter::schema_rewriter::BatchAdapterFactory`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr_adapter.schema_rewriter.BatchAdapterFactory.json).

<a id="op-739e1733de121a3801ee33b4"></a>
## BatchAdapterFactory

`struct` · `datafusion_physical_expr_adapter::schema_rewriter::BatchAdapterFactory` · datafusion-physical-expr-adapter 55.1.0

```rust
struct BatchAdapterFactory
```

Source: `src/schema_rewriter.rs:715`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-adapter/55.1.0/json).

Factory for creating [`BatchAdapter`](../operations/datafusion_physical_expr_adapter.schema_rewriter.BatchAdapter.md#op-edcc6a5fbe6af1e296777540) instances to adapt record batches
to a target schema.

This binds a target schema and allows creating adapters for different source schemas.
It handles:
- **Column reordering**: Columns are reordered to match the target schema
- **Type casting**: Automatic type conversion (e.g., Int32 to Int64)
- **Missing columns**: Nullable columns missing from source are filled with nulls
- **Struct field adaptation**: Nested struct fields are recursively adapted

## Examples

```rust
use arrow::array::{Int32Array, Int64Array, StringArray, RecordBatch};
use arrow::datatypes::{DataType, Field, Schema};
use datafusion_physical_expr_adapter::BatchAdapterFactory;
use std::sync::Arc;

// Target schema has different column order and types
let target_schema = Arc::new(Schema::new(vec![
    Field::new("name", DataType::Utf8, true),
    Field::new("id", DataType::Int64, false),    // Int64 in target
    Field::new("score", DataType::Float64, true), // Missing from source
]));

// Source schema has different column order and Int32 for id
let source_schema = Arc::new(Schema::new(vec![
    Field::new("id", DataType::Int32, false),    // Int32 in source
    Field::new("name", DataType::Utf8, true),
    // Note: 'score' column is missing from source
]));

// Create factory with target schema
let factory = BatchAdapterFactory::new(Arc::clone(&target_schema));

// Create adapter for this specific source schema
let adapter = factory.make_adapter(&source_schema).unwrap();

// Create a source batch
let source_batch = RecordBatch::try_new(
    source_schema,
    vec![
        Arc::new(Int32Array::from(vec![1, 2, 3])),
        Arc::new(StringArray::from(vec!["Alice", "Bob", "Carol"])),
    ],
).unwrap();

// Adapt the batch to match target schema
let adapted = adapter.adapt_batch(&source_batch).unwrap();

assert_eq!(adapted.num_columns(), 3);
assert_eq!(adapted.column(0).data_type(), &DataType::Utf8);   // name
assert_eq!(adapted.column(1).data_type(), &DataType::Int64);  // id (cast from Int32)
assert_eq!(adapted.column(2).data_type(), &DataType::Float64); // score (filled with nulls)
```

<a id="op-6dfb10c255300119fe86f2d8"></a>
## fmt

`function` · `datafusion_physical_expr_adapter::schema_rewriter::BatchAdapterFactory::fmt` · datafusion-physical-expr-adapter 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_adapter::schema_rewriter::BatchAdapterFactory", "path": "BatchAdapterFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [714, 10], "end": [714, 15], "filename": "src/schema_rewriter.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/schema_rewriter.rs:714`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-adapter/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-509edc04b4b49f63258d7a61"></a>
## make_adapter

`function` · `datafusion_physical_expr_adapter::schema_rewriter::BatchAdapterFactory::make_adapter` · datafusion-physical-expr-adapter 55.1.0

```rust
fn make_adapter(&self, source_schema: &SchemaRef) -> Result<BatchAdapter>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_adapter::schema_rewriter::BatchAdapterFactory", "path": "BatchAdapterFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [720, 1], "end": [768, 2], "filename": "src/schema_rewriter.rs"}, "trait": null, "trait_path": null}`

Source: `src/schema_rewriter.rs:750`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-adapter/55.1.0/json).

Create a new [`BatchAdapter`](../operations/datafusion_physical_expr_adapter.schema_rewriter.BatchAdapter.md#op-edcc6a5fbe6af1e296777540) for the given source schema.

Batches fed into this [`BatchAdapter`](../operations/datafusion_physical_expr_adapter.schema_rewriter.BatchAdapter.md#op-edcc6a5fbe6af1e296777540) *must* conform to the source schema,
no validation is performed at runtime to minimize overheads.

<a id="op-0e3cab036563351924556ea6"></a>
## new

`function` · `datafusion_physical_expr_adapter::schema_rewriter::BatchAdapterFactory::new` · datafusion-physical-expr-adapter 55.1.0

```rust
fn new(target_schema: SchemaRef) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_adapter::schema_rewriter::BatchAdapterFactory", "path": "BatchAdapterFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [720, 1], "end": [768, 2], "filename": "src/schema_rewriter.rs"}, "trait": null, "trait_path": null}`

Source: `src/schema_rewriter.rs:722`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-adapter/55.1.0/json).

Create a new [`BatchAdapterFactory`](../operations/datafusion_physical_expr_adapter.schema_rewriter.BatchAdapterFactory.md#op-739e1733de121a3801ee33b4) with the given target schema.

<a id="op-3d7381e364a77cbe9eb4f912"></a>
## with_adapter_factory

`function` · `datafusion_physical_expr_adapter::schema_rewriter::BatchAdapterFactory::with_adapter_factory` · datafusion-physical-expr-adapter 55.1.0

```rust
fn with_adapter_factory(self, factory: Arc<dyn PhysicalExprAdapterFactory>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr_adapter::schema_rewriter::BatchAdapterFactory", "path": "BatchAdapterFactory"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [720, 1], "end": [768, 2], "filename": "src/schema_rewriter.rs"}, "trait": null, "trait_path": null}`

Source: `src/schema_rewriter.rs:736`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-adapter/55.1.0/json).

Set a custom [`PhysicalExprAdapterFactory`](../operations/datafusion_physical_expr_adapter.schema_rewriter.PhysicalExprAdapterFactory.md#op-66df2b6b668622dcee375500) to use when adapting expressions.

Use this to customize behavior when adapting batches, e.g. to fill in missing values
with defaults instead of nulls.

See [`PhysicalExprAdapter`](../operations/datafusion_physical_expr_adapter.schema_rewriter.PhysicalExprAdapter.md#op-3d7f05fc0e38d49de65f327a) for more details.
