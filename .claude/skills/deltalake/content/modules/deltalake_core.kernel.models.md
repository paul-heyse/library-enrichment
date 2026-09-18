# `deltalake_core::kernel::models`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.kernel.models.json).

<a id="op-e8d6cac4460814925b12a55e"></a>
## models

`module` · `deltalake_core::kernel::models` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
mod models
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/models/mod.rs#L1).

Source: `crates/core/src/kernel/models/mod.rs:1`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Core Delta log action models (Add, Remove, Metadata, Protocol, ...) and related types.
Actions are the fundamental unit of work in Delta Lake. Each action performs a single atomic
operation on the state of a Delta table. Actions are stored in the `_delta_log` directory of a
Delta table in JSON format. The log is a time series of actions that represent all the changes
made to a table.
