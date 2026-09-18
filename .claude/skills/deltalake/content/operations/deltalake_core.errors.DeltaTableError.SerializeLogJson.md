# `deltalake_core::errors::DeltaTableError::SerializeLogJson`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.errors.DeltaTableError.SerializeLogJson.json).

<a id="op-c229ac3933379e0cd2f65e95"></a>
## json_err

`struct_field` · `deltalake_core::errors::DeltaTableError::SerializeLogJson::json_err` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
json_err: serde_json::error::Error
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/errors.rs#L185).

Source: `crates/core/src/errors.rs:185`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

JSON serialization error
