# `deltalake_core::kernel::error::Error::InvalidGenerationExpressionJson`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.kernel.error.Error.InvalidGenerationExpressionJson.json).

<a id="op-63c1d7f3ee222b45917a708c"></a>
## json_err

`struct_field` · `deltalake_core::kernel::error::Error::InvalidGenerationExpressionJson::json_err` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
json_err: serde_json::error::Error
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/error.rs#L72).

Source: `crates/core/src/kernel/error.rs:72`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

JSON error details returned when parsing the generation expression JSON.

<a id="op-d746a5b6636687e6dea483fd"></a>
## line

`struct_field` · `deltalake_core::kernel::error::Error::InvalidGenerationExpressionJson::line` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
line: String
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/error.rs#L74).

Source: `crates/core/src/kernel/error.rs:74`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Generation expression.
