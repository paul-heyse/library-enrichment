# `buoyant_kernel::crc::delta`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.crc.delta.json).

<a id="op-1a483375207828c9f6849e83"></a>
## delta

`module` · `buoyant_kernel::crc::delta` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_module**. Canonical source location is not automatically a valid import path.

```rust
mod delta
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/crc/delta.rs#L1).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/crc/delta.rs:1`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Incremental CRC state updates via commit deltas.

A [`CrcDelta`] aggregates the CRC-relevant changes from commits `(X, Y]`. Applying it to a
base CRC at `X` yields the CRC at `Y`: `Crc[X] + CrcDelta = Crc[Y]`. [`Crc::apply`] is the
consumer.

Unresolved upstream links (retained, not inferred): ``CrcDelta``, ``Crc::apply``.
