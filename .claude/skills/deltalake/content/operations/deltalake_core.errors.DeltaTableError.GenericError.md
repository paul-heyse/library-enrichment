# `deltalake_core::errors::DeltaTableError::GenericError`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.errors.DeltaTableError.GenericError.json).

<a id="op-90a89ee94451a45e6b69246e"></a>
## source

`struct_field` · `deltalake_core::errors::DeltaTableError::GenericError::source` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
source: Box<dyn std::error::Error + Send + Sync + 'static>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/errors.rs#L196).

Source: `crates/core/src/errors.rs:196`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Source error
