# `deltalake_core::errors::DeltaTableError::InvalidData`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.errors.DeltaTableError.InvalidData.json).

<a id="op-7adac1bd9b517cbb94ec419a"></a>
## message

`struct_field` · `deltalake_core::errors::DeltaTableError::InvalidData::message` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
message: String
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/errors.rs#L105).

Source: `crates/core/src/errors.rs:105`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Action error details returned of the invalid action.
