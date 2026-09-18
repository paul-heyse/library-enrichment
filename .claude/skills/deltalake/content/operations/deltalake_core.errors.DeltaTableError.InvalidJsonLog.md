# `deltalake_core::errors::DeltaTableError::InvalidJsonLog`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.errors.DeltaTableError.InvalidJsonLog.json).

<a id="op-8e50842c100b57a466e8077a"></a>
## json_err

`struct_field` · `deltalake_core::errors::DeltaTableError::InvalidJsonLog::json_err` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
json_err: serde_json::error::Error
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/errors.rs#L64).

Source: `crates/core/src/errors.rs:64`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

JSON error details returned when parsing the record JSON.

<a id="op-69fb22f23e6de839325cfea5"></a>
## line

`struct_field` · `deltalake_core::errors::DeltaTableError::InvalidJsonLog::line` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
line: String
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/errors.rs#L66).

Source: `crates/core/src/errors.rs:66`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

invalid log entry content.

<a id="op-77a9e82e3564751eafdcb8ca"></a>
## version

`struct_field` · `deltalake_core::errors::DeltaTableError::InvalidJsonLog::version` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
version: kernel::Version
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/errors.rs#L68).

Source: `crates/core/src/errors.rs:68`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

corresponding table version for the log file.
