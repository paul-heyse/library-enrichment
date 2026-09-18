# Schema and types

Delta's schema is the kernel's `StructType`, not Arrow's. `Schema` and `SchemaRef` in this library are aliases for it, and conversion to Arrow happens at the edges. Types round-trip imperfectly by design: Delta has no unsigned integers and its timestamps are normalised, so an Arrow schema that writes cleanly is not proof the Delta schema says what you meant.

## Entry points

| Type | Kind | Methods | Prose | Records |
|---|---|---:|---|---|
| `buoyant_kernel::schema::StructType` | struct | 38 | [prose](../api/buoyant_kernel.schema.md#structtype) | [records](../model/buoyant_kernel.schema.json) |
| `buoyant_kernel::schema::StructField` | struct | 29 | [prose](../api/buoyant_kernel.schema.md#structfield) | [records](../model/buoyant_kernel.schema.json) |
| `buoyant_kernel::schema::DataType` | enum | 19 | [prose](../api/buoyant_kernel.schema.md#datatype) | [records](../model/buoyant_kernel.schema.json) |
| `buoyant_kernel::schema::PrimitiveType` | enum | 10 | [prose](../api/buoyant_kernel.schema.md#primitivetype) | [records](../model/buoyant_kernel.schema.json) |
| `buoyant_kernel::expressions::scalars::Scalar` | enum | 42 | [prose](../api/buoyant_kernel.expressions.scalars.md#scalar) | [records](../model/buoyant_kernel.expressions.scalars.json) |

## Extension points

| Trait | Required | Provided | Implementors | Page |
|---|---:|---:|---:|---|
| `deltalake_core::kernel::schema::schema::StructTypeExt` | 2 | 0 | 1 | [StructTypeExt](../traits/StructTypeExt.md) |
| `buoyant_kernel::schema::ToSchema` | 1 | 0 | 12 | [ToSchema](../traits/ToSchema.md) |
| `deltalake_core::kernel::arrow::engine_ext::StructDataExt` | 3 | 0 | 1 | [StructDataExt](../traits/StructDataExt.md) |
| `deltalake_core::kernel::scalars::ScalarExt` | 4 | 0 | 1 | [ScalarExt](../traits/ScalarExt.md) |

## Runnable examples (4)

- [`corpus/examples/basic_operations.rs`](../corpus/examples/basic_operations.rs)
- [`corpus/examples/load_table.rs`](../corpus/examples/load_table.rs)
- [`corpus/examples/read_delta_table.rs`](../corpus/examples/read_delta_table.rs)
- [`corpus/examples/recordbatch-writer.rs`](../corpus/examples/recordbatch-writer.rs)

## Decision rules

- Declare a Delta schema when the domain requires independent schema control; batch-derived schemas are appropriate when their normalization and protocol consequences match the intended contract.
- Schema evolution is opt-in per write, through the write builder's schema mode -- not a table-level setting.

## Anti-patterns

- Assuming an Arrow schema and a Delta schema are the same thing because a round trip happened to work.
- Adding a column by rewriting the table when `add_columns` exists.

## Agent checklist

- Is the schema declared, or inherited from whatever Arrow inferred?
- Is a schema mode set on writes that may see new columns?
