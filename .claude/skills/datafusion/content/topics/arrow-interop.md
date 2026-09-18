# Arrow interop

Arrow arrays, fields and RecordBatch carry the columnar values and schema. DFSchema adds relational qualifiers. Arrow kernels can operate directly on batches/arrays; DataFusion expressions expose work to planning. Extension metadata survives only through operations that preserve the relevant fields.

## Entry points

| Type | Kind | Methods | Prose | Records |
|---|---|---:|---|---|
| `datafusion_common::dfschema::DFSchema` | struct | 57 | [prose](../api/datafusion_common.dfschema.md#dfschema) | [records](../model/datafusion_common.dfschema.json) |
| `datafusion_common::scalar::ScalarValue` | enum | 104 | [prose](../api/datafusion_common.scalar.md#scalarvalue) | [records](../model/datafusion_common.scalar.json) |
| `arrow_array::record_batch::RecordBatch` | struct | 28 | [prose](../api/arrow_array.record_batch.md#recordbatch) | [records](../model/arrow_array.record_batch.json) |
| `arrow_schema::schema::Schema` | struct | 25 | [prose](../api/arrow_schema.schema.md#schema) | [records](../model/arrow_schema.schema.json) |
| `arrow_schema::field::Field` | struct | 48 | [prose](../api/arrow_schema.field.md#field) | [records](../model/arrow_schema.field.json) |
| `arrow_row::RowConverter` | struct | 11 | [prose](../api/arrow_row.md#rowconverter) | [records](../model/arrow_row.json) |
| `arrow_cast::cast::CastOptions` | struct | 5 | [prose](../api/arrow_cast.cast.md#castoptions) | [records](../model/arrow_cast.cast.json) |
| `arrow_select::filter::FilterBuilder` | struct | 5 | [prose](../api/arrow_select.filter.md#filterbuilder) | [records](../model/arrow_select.filter.json) |

## Settings (1)

Full table with Rust setters in [`../catalogs/config-options.md`](../catalogs/config-options.md).

| Setting | Default |
|---|---|
| `datafusion.execution.batch_size` | 8192 |

## Runnable examples (2)

- [`corpus/examples/extension_types/main.rs`](../corpus/examples/extension_types/main.rs)
- [`corpus/examples/extension_types/temperature.rs`](../corpus/examples/extension_types/temperature.rs)

## Upstream guides

- [`corpus/guides/user-guide/arrow-introduction.md`](../corpus/guides/user-guide/arrow-introduction.md)
- [`corpus/guides/user-guide/sql/data_types.md`](../corpus/guides/user-guide/sql/data_types.md)
- [`corpus/guides/user-guide/sql/struct_coercion.md`](../corpus/guides/user-guide/sql/struct_coercion.md)

## Decision rules

- Inside a UDF, reach for an Arrow compute kernel before writing a loop; search `content/index/symbols.tsv` for `arrow_` crates.
- Extension type metadata lives on the `Field`, so it propagates only where field metadata is preserved — check that the operators in the plan do.

## Anti-patterns

- Converting to row objects before checking available columnar kernels and their null/type contracts.
- Assuming field metadata survives every operator; DataFusion 55 differs between logical and physical UNION on this point.

## Agent checklist

- Is the array accessed through a typed accessor rather than a downcast-and-hope?
- Does the null handling match Arrow's semantics rather than the host language's?
