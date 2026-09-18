# `deltalake_core::delta_datafusion::session::create_session`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.delta_datafusion.session.create_session.json).

<a id="op-45c70950a9bc56476e6885e2"></a>
## create_session

`function` · `deltalake_core::delta_datafusion::session::create_session` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn create_session() -> DeltaSessionContext
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/session.rs#L24).

Source: `crates/core/src/delta_datafusion/session.rs:24`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Create a default [`DeltaSessionContext`](../operations/deltalake_core.delta_datafusion.session.DeltaSessionContext.md#op-82b673d3dc76c6c4dfd9bc65), a DataFusion session pre-configured with the
settings delta-rs relies on (custom planner, object-store registration, etc.).
