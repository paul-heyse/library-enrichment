# `deltalake_core::delta_datafusion::session::create_session_state_with_spill_config`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.delta_datafusion.session.create_session_state_with_spill_config.json).

<a id="op-6718e986726f6ba946bb0cd3"></a>
## create_session_state_with_spill_config

`function` · `deltalake_core::delta_datafusion::session::create_session_state_with_spill_config` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn create_session_state_with_spill_config(max_spill_size: Option<usize>, max_temp_directory_size: Option<u64>) -> datafusion::execution::SessionState
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/session.rs#L38).

Source: `crates/core/src/delta_datafusion/session.rs:38`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Create a [`SessionState`] with optional spill-to-disk configuration.

When either parameter is `Some`, a [`FairSpillPool`] memory pool and a sized
[`DiskManagerBuilder`] are wired into the runtime environment so that
DataFusion can spill intermediate results to disk instead of running out of
memory.

# Arguments
* `max_spill_size` – Maximum bytes kept in memory before spilling. `None` uses DataFusion's default (unbounded) pool.
* `max_temp_directory_size` – Maximum disk space for temporary spill files. `None` uses DataFusion's default disk manager.

Unresolved upstream links (retained, not inferred): ``FairSpillPool``, ``DiskManagerBuilder``, ``SessionState``.
