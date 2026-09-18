# `deltalake_core::errors::DeltaTableError::Io`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.errors.DeltaTableError.Io.json).

<a id="op-895c763cd8eb653013c4ca23"></a>
## source

`struct_field` · `deltalake_core::errors::DeltaTableError::Io::source` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
source: std::io::Error
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/errors.rs#L143).

Source: `crates/core/src/errors.rs:143`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Source error details returned while reading the log record.
