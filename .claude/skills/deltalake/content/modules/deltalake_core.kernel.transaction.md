# `deltalake_core::kernel::transaction`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.kernel.transaction.json).

<a id="op-d4f4352df3f88442ab86f5de"></a>
## transaction

`module` · `deltalake_core::kernel::transaction` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
mod transaction
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/mod.rs#L1).

Source: `crates/core/src/kernel/transaction/mod.rs:1`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

 Add a commit entry to the Delta Table.
 This module provides a unified interface for modifying commit behavior and attributes

 [`CommitProperties`](../operations/deltalake_core.kernel.transaction.CommitProperties.md#op-0dd291fe008825d5408d61a7) provides an unified client interface for all Delta operations.
 Internally this is used to initialize a [`CommitBuilder`](../operations/deltalake_core.kernel.transaction.CommitBuilder.md#op-8e35b7f8912ddf52f59854e0).

 For advanced use cases [`CommitBuilder`](../operations/deltalake_core.kernel.transaction.CommitBuilder.md#op-8e35b7f8912ddf52f59854e0) can be used which allows
 finer control over the commit process. The builder can be converted
 into a future the yield either a [`PreparedCommit`](../operations/deltalake_core.kernel.transaction.PreparedCommit.md#op-d4ed120c429e31136363108d) or a [`FinalizedCommit`](../operations/deltalake_core.kernel.transaction.FinalizedCommit.md#op-c7dc3c28452e5b512659d272).

 A [`PreparedCommit`](../operations/deltalake_core.kernel.transaction.PreparedCommit.md#op-d4ed120c429e31136363108d) represents a temporary commit marker written to storage.
 To convert to a [`FinalizedCommit`](../operations/deltalake_core.kernel.transaction.FinalizedCommit.md#op-c7dc3c28452e5b512659d272) an atomic rename is attempted. If the rename fails
 then conflict resolution is performed and the atomic rename is tried for the latest version.

<pre>
                                          Client Interface
        ┌─────────────────────────────┐
        │      Commit Properties      │
        │                             │
        │ Public commit interface for │
        │     all Delta Operations    │
        │                             │
        └─────────────┬───────────────┘
                      │
 ─────────────────────┼────────────────────────────────────
                      │
                      ▼                  Advanced Interface
        ┌─────────────────────────────┐
        │       Commit Builder        │
        │                             │
        │   Advanced entry point for  │
        │     creating a commit       │
        └─────────────┬───────────────┘
                      │
                      ▼
     ┌───────────────────────────────────┐
     │                                   │
     │ ┌───────────────────────────────┐ │
     │ │        Prepared Commit        │ │
     │ │                               │ │
     │ │     Represents a temporary    │ │
     │ │   commit marker written to    │ │
     │ │           storage             │ │
     │ └──────────────┬────────────────┘ │
     │                │                  │
     │                ▼                  │
     │ ┌───────────────────────────────┐ │
     │ │       Finalize Commit         │ │
     │ │                               │ │
     │ │   Convert the commit marker   │ │
     │ │   to a commit using atomic    │ │
     │ │         operations            │ │
     │ │                               │ │
     │ └───────────────────────────────┘ │
     │                                   │
     └────────────────┬──────────────────┘
                      │
                      ▼
       ┌───────────────────────────────┐
       │          Post Commit          │
       │                               │
       │ Commit that was materialized  │
       │ to storage with post commit   │
       │      hooks to be executed     │
       └──────────────┬────────────────┘
                      │
                      ▼
       ┌───────────────────────────────┐
       │        Finalized Commit       │
       │                               │
       │ Commit that was materialized  │
       │         to storage            │
       │                               │
       └───────────────────────────────┘
</pre>
