# `buoyant_kernel::crc::state`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.crc.state.json).

<a id="op-0598ff6cd034f0280cf0a78a"></a>
## state

`module` · `buoyant_kernel::crc::state` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_module**. Canonical source location is not automatically a valid import path.

```rust
mod state
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/crc/state.rs#L1).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/crc/state.rs:1`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Typed state enums for CRC tracking.

Each enum encodes both *what we know* about a slice of CRC state and *the trustworthiness*
of those values. Variants carry data exactly when it makes sense for that state, making
invalid reads unrepresentable: the compiler prevents reading an absolute count from a
degraded state because the field does not exist there.

- [`FileStatsState`](../operations/buoyant_kernel.crc.state.FileStatsState.md#op-fa49cf56a10f2e93f21b3b8a) tracks file-stat validity (Complete / Indeterminate).
- [`DomainMetadataState`](../operations/buoyant_kernel.crc.state.DomainMetadataState.md#op-1bdc582d72a8cd8896b7ca99) tracks domain-metadata completeness (Complete / Partial).
- [`SetTransactionState`](../operations/buoyant_kernel.crc.state.SetTransactionState.md#op-813a79731eedededb5c6ef13) tracks set-transaction completeness (Complete / Partial).
