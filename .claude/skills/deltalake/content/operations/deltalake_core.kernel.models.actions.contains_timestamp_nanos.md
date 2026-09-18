# `deltalake_core::kernel::models::actions::contains_timestamp_nanos`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.kernel.models.actions.contains_timestamp_nanos.json).

<a id="op-42d82ec031404d0a6646dc39"></a>
## contains_timestamp_nanos

`function` · `deltalake_core::kernel::models::actions::contains_timestamp_nanos` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn contains_timestamp_nanos<'a>(fields: impl Iterator<Item = &'a delta_kernel::schema::StructField>) -> bool
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/actions.rs#L176).

Source: `crates/core/src/kernel/models/actions.rs:176`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

checks if table contains timestamp_nanos or timestamp_nanos_ntz in any
field including nested fields. Both primitive types require the same
`timestampNanos` table feature.
