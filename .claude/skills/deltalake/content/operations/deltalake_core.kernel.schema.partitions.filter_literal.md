# `deltalake_core::kernel::schema::partitions::filter_literal`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.kernel.schema.partitions.filter_literal.json).

<a id="op-d492ea9e8874a2363b695a0b"></a>
## filter_literal

`function` · `deltalake_core::kernel::schema::partitions::filter_literal` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn filter_literal<'a>(column: &'a str, op: &str, value: FilterValue<'a>) -> errors::DeltaResult<FilterLiteral<'a>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/schema/partitions.rs#L160).

Source: `crates/core/src/kernel/schema/partitions.rs:160`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Validate a raw `(column, op, value)` tuple into a [`FilterLiteral`](../operations/deltalake_core.kernel.schema.partitions.FilterLiteral.md#op-8c023059af8095f74348cd0a),
parsing the operator string.

This is the boundary where stringly-typed filters (e.g. from FFI) enter:
an unknown operator, an empty column name, or an operator/value shape
mismatch all yield the pinned `InvalidPartitionFilter` error.
