# `deltalake_core::operations::merge::barrier`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.operations.merge.barrier.json).

<a id="op-d5ccdf9cce2ee75b86b6a679"></a>
## barrier

`module` · `deltalake_core::operations::merge::barrier` · deltalake-core 1.0.0+58f07cd6

Access: **internal_module**. Canonical source location is not automatically a valid import path.

```rust
mod barrier
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/merge/barrier.rs#L1).

Source: `crates/core/src/operations/merge/barrier.rs:1`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Merge Barrier determines which files have modifications during the merge operation

For every unique path in the input stream, a barrier is established. If any
single record for a file contains any delete, update, or insert operations
then the barrier for the file is opened and can be sent downstream.
To determine if a file contains zero changes, the input stream is
exhausted. Afterwards, records are then dropped.

Bookkeeping is maintained to determine which files have modifications, so
they can be removed from the delta log.
