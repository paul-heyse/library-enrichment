# `deltalake_core::operations::drop_column_not_null`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.operations.drop_column_not_null.json).

<a id="op-0a8f3b6b2a5f79fdc2c42efb"></a>
## drop_column_not_null

`module` · `deltalake_core::operations::drop_column_not_null` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
mod drop_column_not_null
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/drop_column_not_null.rs#L1).

Source: `crates/core/src/operations/drop_column_not_null.rs:1`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Drop the `NOT NULL` constraint on a column, making it nullable.

This implements the equivalent of `ALTER TABLE <table> ALTER COLUMN <name> DROP NOT NULL`.
Only relaxing a column from non-nullable to nullable is supported. The reverse
(making a nullable column non-nullable) is intentionally not allowed here because it
requires validating existing data and/or a default value, which is out of scope.
