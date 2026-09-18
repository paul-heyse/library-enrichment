# `deltalake_core::kernel::error::Error::InvalidInvariantJson`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.kernel.error.Error.InvalidInvariantJson.json).

<a id="op-9d8d7d184057f68792e6fa95"></a>
## json_err

`struct_field` · `deltalake_core::kernel::error::Error::InvalidInvariantJson::json_err` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
json_err: serde_json::error::Error
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/error.rs#L63).

Source: `crates/core/src/kernel/error.rs:63`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

JSON error details returned when parsing the invariant expression JSON.

<a id="op-60d55f09ed36d3ed6ec3ddf7"></a>
## line

`struct_field` · `deltalake_core::kernel::error::Error::InvalidInvariantJson::line` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
line: String
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/error.rs#L65).

Source: `crates/core/src/kernel/error.rs:65`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Invariant expression.
