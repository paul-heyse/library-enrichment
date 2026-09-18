# `deltalake_core::operations::vacuum::Clock`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.operations.vacuum.Clock.json).

<a id="op-26819400faef6c1178a2ff10"></a>
## Clock

`trait` · `deltalake_core::operations::vacuum::Clock` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
trait Clock: Debug + Send + Sync
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/vacuum.rs#L165).

Source: `crates/core/src/operations/vacuum.rs:165`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

A source of time

<a id="op-563ffbc220beb7939783d2ab"></a>
## current_timestamp_millis

`function` · `deltalake_core::operations::vacuum::Clock::current_timestamp_millis` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn current_timestamp_millis(&self) -> i64
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/vacuum.rs#L167).

Source: `crates/core/src/operations/vacuum.rs:167`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

get the current time in milliseconds since epoch
