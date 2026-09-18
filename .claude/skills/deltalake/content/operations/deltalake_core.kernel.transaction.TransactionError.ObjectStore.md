# `deltalake_core::kernel::transaction::TransactionError::ObjectStore`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.kernel.transaction.TransactionError.ObjectStore.json).

<a id="op-ad8811988b525210206bb587"></a>
## source

`struct_field` · `deltalake_core::kernel::transaction::TransactionError::ObjectStore::source` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
source: object_store::Error
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/mod.rs#L189).

Source: `crates/core/src/kernel/transaction/mod.rs:189`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Storage error details when reading the delta log object failed.
