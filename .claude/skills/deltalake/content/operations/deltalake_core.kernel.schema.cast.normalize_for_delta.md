# `deltalake_core::kernel::schema::cast::normalize_for_delta`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.kernel.schema.cast.normalize_for_delta.json).

<a id="op-6601c2591b6129782f1a8b98"></a>
## normalize_for_delta

`function` · `deltalake_core::kernel::schema::cast::normalize_for_delta` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn normalize_for_delta(schema: &arrow_schema::SchemaRef) -> arrow_schema::SchemaRef
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/schema/cast/mod.rs#L325).

Source: `crates/core/src/kernel/schema/cast/mod.rs:325`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Normalize an Arrow schema so it can be safely written to a Delta table.

Delta does not support all Arrow types verbatim (for example nanosecond timestamps);
this rewrites such fields to the closest Delta-compatible representation, returning the
original schema untouched when no changes are required.
