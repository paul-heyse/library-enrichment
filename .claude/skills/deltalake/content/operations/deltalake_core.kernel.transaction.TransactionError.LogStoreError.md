# `deltalake_core::kernel::transaction::TransactionError::LogStoreError`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.kernel.transaction.TransactionError.LogStoreError.json).

<a id="op-9d6cbb70b3ed66c8baa0e53c"></a>
## msg

`struct_field` · `deltalake_core::kernel::transaction::TransactionError::LogStoreError::msg` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
msg: String
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/mod.rs#L219).

Source: `crates/core/src/kernel/transaction/mod.rs:219`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Detailed message for the commit failure.

<a id="op-55aed30a971d3e8952db78d9"></a>
## source

`struct_field` · `deltalake_core::kernel::transaction::TransactionError::LogStoreError::source` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
source: Box<dyn std::error::Error + Send + Sync + 'static>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/mod.rs#L221).

Source: `crates/core/src/kernel/transaction/mod.rs:221`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

underlying error in the log store transactional layer.
