# `deltalake_core::kernel::schema::partitions::dnf_to_kernel_predicate`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.kernel.schema.partitions.dnf_to_kernel_predicate.json).

<a id="op-04c544e698f5aa798d21e379"></a>
## dnf_to_kernel_predicate

`function` · `deltalake_core::kernel::schema::partitions::dnf_to_kernel_predicate` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn dnf_to_kernel_predicate(dnf: &[Vec<FilterLiteral<'_>>], table_schema: &delta_kernel::schema::StructType) -> errors::DeltaResult<delta_kernel::expressions::Predicate>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/schema/partitions.rs#L299).

Source: `crates/core/src/kernel/schema/partitions.rs:299`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Translate filters in disjunctive normal form -- an OR across conjunctions
(AND groups) of `(column, op, value)` literals -- into a kernel [`Predicate`](../operations/buoyant_kernel.expressions.Predicate.md#op-0c0ee21b4ebfcd851e64ceb4).
