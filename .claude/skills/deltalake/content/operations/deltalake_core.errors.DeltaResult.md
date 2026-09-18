# `deltalake_core::errors::DeltaResult`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.errors.DeltaResult.json).

<a id="op-0ab2063e00e7d8f4da456811"></a>
## DeltaResult

`type_alias` · `deltalake_core::errors::DeltaResult` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type DeltaResult<T, E = DeltaTableError> = Result<T, E>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/errors.rs#L9).

Source: `crates/core/src/errors.rs:9`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

A result returned by delta-rs
