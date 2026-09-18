# `deltalake_core::errors::DeltaTableError::UnsupportedColumnMapping`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.errors.DeltaTableError.UnsupportedColumnMapping.json).

<a id="op-c4b78d80b4e215fdd59657c1"></a>
## mode

`struct_field` · `deltalake_core::errors::DeltaTableError::UnsupportedColumnMapping::mode` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
mode: ColumnMappingOperation
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/errors.rs#L238).

Source: `crates/core/src/errors.rs:238`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Whether the unsupported access was a read or a write.

<a id="op-90838b1721494e15755e13b3"></a>
## operation

`struct_field` · `deltalake_core::errors::DeltaTableError::UnsupportedColumnMapping::operation` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
operation: String
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/errors.rs#L240).

Source: `crates/core/src/errors.rs:240`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Human-readable description of the operation (e.g. "ADD COLUMN").
