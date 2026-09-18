# `deltalake_core::kernel::schema::partitions::FilterLiteral`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.kernel.schema.partitions.FilterLiteral.json).

<a id="op-8c023059af8095f74348cd0a"></a>
## FilterLiteral

`type_alias` · `deltalake_core::kernel::schema::partitions::FilterLiteral` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type FilterLiteral<'a> = (&'a str, FilterOp, FilterValue<'a>)
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/schema/partitions.rs#L152).

Source: `crates/core/src/kernel/schema/partitions.rs:152`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

A `(column, op, value)` comparison, mirroring the tuple filters accepted by
the Python bindings.
