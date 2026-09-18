# `deltalake_core::kernel::schema::cast::set_cast_nanos_timestamps_to_micros`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.kernel.schema.cast.set_cast_nanos_timestamps_to_micros.json).

<a id="op-de29e037a17794a7fd60c01e"></a>
## set_cast_nanos_timestamps_to_micros

`function` · `deltalake_core::kernel::schema::cast::set_cast_nanos_timestamps_to_micros` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn set_cast_nanos_timestamps_to_micros(cast: bool)
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/schema/cast/mod.rs#L29).

Source: `crates/core/src/kernel/schema/cast/mod.rs:29`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Set whether casting nanosecond timestamps to microsecond timestamps happens.
