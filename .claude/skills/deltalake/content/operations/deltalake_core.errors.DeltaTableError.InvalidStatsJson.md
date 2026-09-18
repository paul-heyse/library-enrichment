# `deltalake_core::errors::DeltaTableError::InvalidStatsJson`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.errors.DeltaTableError.InvalidStatsJson.json).

<a id="op-4e04acb0532df1b79ff18c16"></a>
## json_err

`struct_field` · `deltalake_core::errors::DeltaTableError::InvalidStatsJson::json_err` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
json_err: serde_json::error::Error
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/errors.rs#L75).

Source: `crates/core/src/errors.rs:75`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

JSON error details returned when parsing the stats JSON.
