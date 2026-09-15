# Arrow interop

DataFusion's data model is Arrow's, so every signature is saturated with Arrow types. `DFSchema` wraps `Schema` with table qualifiers. The Arrow compute kernels are a large surface of their own and are often the right answer inside a UDF rather than writing element-wise code. Extension types carry semantic meaning in field metadata that survives through a plan.

## Entry points

| Type | Kind | Methods | Prose | Records |
|---|---|---:|---|---|
| `datafusion_common::dfschema::DFSchema` | struct | 57 | [prose](../api/datafusion_common.dfschema.md#dfschema) | [records](../model/datafusion_common.dfschema.json) |
| `datafusion_common::scalar::ScalarValue` | enum | 104 | [prose](../api/datafusion_common.scalar.md#scalarvalue) | [records](../model/datafusion_common.scalar.json) |

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

- Converting an Arrow array to a Vec to operate on it.
- Assuming field metadata survives every operator; DataFusion 55 differs between logical and physical UNION on this point.

## Agent checklist

- Is the array accessed through a typed accessor rather than a downcast-and-hope?
- Does the null handling match Arrow's semantics rather than the host language's?
