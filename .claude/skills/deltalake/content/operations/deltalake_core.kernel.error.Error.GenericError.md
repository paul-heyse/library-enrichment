# `deltalake_core::kernel::error::Error::GenericError`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.kernel.error.Error.GenericError.json).

<a id="op-4466191a40db7f88ea97d77c"></a>
## source

`struct_field` · `deltalake_core::kernel::error::Error::GenericError::source` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
source: Box<dyn std::error::Error + Send + Sync + 'static>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/error.rs#L20).

Source: `crates/core/src/kernel/error.rs:20`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Source error
