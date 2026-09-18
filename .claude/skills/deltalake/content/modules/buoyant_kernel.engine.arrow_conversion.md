# `buoyant_kernel::engine::arrow_conversion`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.engine.arrow_conversion.json).

<a id="op-9467d97cb06987457b8bba35"></a>
## arrow_conversion

`module` · `buoyant_kernel::engine::arrow_conversion` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
mod arrow_conversion
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine/arrow_conversion/mod.rs#L1).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine/arrow_conversion/mod.rs:1`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Conversions between kernel schema types and arrow schema types.

Two directions, used at the engine <-> kernel boundary:

- **arrow -> kernel** ([`TryFromArrow`](../operations/buoyant_kernel.engine.arrow_conversion.TryFromArrow.md#op-5cc3880c91de985c46801d4a) / [`TryIntoKernel`](../operations/buoyant_kernel.engine.arrow_conversion.TryIntoKernel.md#op-6e10c11c45601e477d85eb7b)): an example usage is converting a
  checkpoint Parquet file's Arrow footer schema into a kernel `StructType` during log replay.

- **kernel -> arrow** ([`TryFromKernel`](../operations/buoyant_kernel.engine.arrow_conversion.TryFromKernel.md#op-1e593785ffff46db05df0313) / [`TryIntoArrow`](../operations/buoyant_kernel.engine.arrow_conversion.TryIntoArrow.md#op-95d72eadcf91c464c844af81)): an example usage is materializing
  a kernel `StructType` as an `ArrowSchema` to build a `RecordBatch` on the write path.

These conversions can be used on both logical and physical schemas. E.g. when writing a parquet
file, the default engine converts the physical kernel schema to an Arrow schema and writes the
data. When transforming physical data to logical data via an
[`Expression`][crate::expressions::Expression](../operations/buoyant_kernel.expressions.Expression.md#op-b6e8a7405239f2ae57fc5e51),
[`DefaultExpressionEvaluator`][crate::engine::arrow_expression::DefaultExpressionEvaluator](../operations/buoyant_kernel.engine.arrow_expression.DefaultExpressionEvaluator.md#op-ad5991447301e816ad3823e7)
converts the logical schema to an Arrow schema as the output schema.
