# `buoyant_kernel::engine::arrow_utils::json_arrow_schema`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.engine.arrow_utils.json_arrow_schema.json).

<a id="op-770dbb3917873ecccddc894c"></a>
## json_arrow_schema

`function` · `buoyant_kernel::engine::arrow_utils::json_arrow_schema` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn json_arrow_schema(schema: &schema::StructType) -> DeltaResult<arrow::datatypes::Schema>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine/arrow_utils/mod.rs#L1488).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine/arrow_utils/mod.rs:1488`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Builds an Arrow [`ArrowSchema`] from `schema` containing only the "real" JSON columns,
omitting any fields annotated with [`MetadataColumnSpec`](../operations/buoyant_kernel.schema.MetadataColumnSpec.md#op-2710452c41a584a4cd5350bb).

Pass the returned schema to Arrow's JSON reader; then call [`build_json_reorder_indices`](../operations/buoyant_kernel.engine.arrow_utils.build_json_reorder_indices.md#op-a21507790603b8522b73a394)
once on the same schema and apply `reorder_struct_array` to each resulting batch to
insert the synthesized metadata columns at their correct positions.

Unresolved upstream links (retained, not inferred): ``ArrowSchema``.
