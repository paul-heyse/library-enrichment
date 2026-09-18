# `deltalake_core::kernel::models::actions::contains_timestampntz`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.kernel.models.actions.contains_timestampntz.json).

<a id="op-59ab8bfa1b5068afa5b55781"></a>
## contains_timestampntz

`function` · `deltalake_core::kernel::models::actions::contains_timestampntz` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn contains_timestampntz<'a>(fields: impl Iterator<Item = &'a delta_kernel::schema::StructField>) -> bool
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L168).

Source: `crates/core/src/kernel/models/actions.rs:168`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

checks if table contains timestamp_ntz in any field including nested fields.
