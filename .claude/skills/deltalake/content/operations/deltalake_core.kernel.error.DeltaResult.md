# `deltalake_core::kernel::error::DeltaResult`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.kernel.error.DeltaResult.json).

<a id="op-718d5c621eb48e38a3cd189a"></a>
## DeltaResult

`type_alias` · `deltalake_core::kernel::error::DeltaResult` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type DeltaResult<T, E = Error> = std::result::Result<T, E>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/error.rs#L6).

Source: `crates/core/src/kernel/error.rs:6`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

A specialized [`Result`] type for Delta Lake operations.

Unresolved upstream links (retained, not inferred): ``Result``.
