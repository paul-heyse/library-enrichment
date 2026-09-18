# `deltalake_core::kernel::schema::partitions::literal_to_kernel_predicate`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.kernel.schema.partitions.literal_to_kernel_predicate.json).

<a id="op-792f784aca2cd2e4b1aea3d9"></a>
## literal_to_kernel_predicate

`function` · `deltalake_core::kernel::schema::partitions::literal_to_kernel_predicate` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn literal_to_kernel_predicate(literal: &FilterLiteral<'_>, table_schema: &delta_kernel::schema::StructType) -> errors::DeltaResult<delta_kernel::expressions::Predicate>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/schema/partitions.rs#L202).

Source: `crates/core/src/kernel/schema/partitions.rs:202`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Translate a single filter literal into a kernel [`Predicate`](../operations/buoyant_kernel.expressions.Predicate.md#op-0c0ee21b4ebfcd851e64ceb4).

The raw value is parsed against the schema type of `column`. A null scalar
under `=` / `!=` becomes an IS [NOT] NULL check: in SQL NULL compares equal
to nothing, itself included, but these filters have always allowed equality
against the null partition value.
