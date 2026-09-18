# `deltalake_core::errors::DeltaTableError::InvalidDateTimeString`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.errors.DeltaTableError.InvalidDateTimeString.json).

<a id="op-4781b7790cd0c9018b43b8dd"></a>
## source

`struct_field` · `deltalake_core::errors::DeltaTableError::InvalidDateTimeString::source` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
source: chrono::ParseError
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/errors.rs#L98).

Source: `crates/core/src/errors.rs:98`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Parse error details returned of the datetime string parse error.
