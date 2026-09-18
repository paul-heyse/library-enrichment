# `deltalake_core::kernel::transaction::TransactionError::SerializeLogJson`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.kernel.transaction.TransactionError.SerializeLogJson.json).

<a id="op-80374189910e447632505664"></a>
## json_err

`struct_field` · `deltalake_core::kernel::transaction::TransactionError::SerializeLogJson::json_err` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
json_err: serde_json::error::Error
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/mod.rs#L181).

Source: `crates/core/src/kernel/transaction/mod.rs:181`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Commit log record JSON serialization error.
