# `buoyant_kernel::log_segment::crc_replay`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.log_segment.crc_replay.json).

<a id="op-10434dea6ef33f766f91db32"></a>
## crc_replay

`module` · `buoyant_kernel::log_segment::crc_replay` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_module**. Canonical source location is not automatically a valid import path.

```rust
mod crc_replay
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/log_segment/crc_replay.rs#L1).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/log_segment/crc_replay.rs:1`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Reverse log replay for incremental CRC construction.

Reverse-replays a log segment's commit files to produce a [`CrcDelta`] covering
commits `(X, Y]`. Per the incremental equation `Crc[X] + CrcDelta = Crc[Y]`, that
delta is applied to a stale base via [`Crc::apply`].

Unresolved upstream links (retained, not inferred): ``CrcDelta``, ``Crc::apply``.
