# `buoyant_kernel::crc`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.crc.json).

<a id="op-0d7d1ae0b9a0eec2a59370b8"></a>
## crc

`module` · `buoyant_kernel::crc` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
mod crc
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/crc/mod.rs#L1).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/crc/mod.rs:1`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

CRC (version checksum) file support.

A [CRC file] contains a snapshot of table state at a specific version, which can be used to
optimize log replay operations like reading Protocol/Metadata, domain metadata, set
transactions, and ICT.

[`Crc`](../operations/buoyant_kernel.crc.Crc.md#op-eb0003c051215379df8b89fa) holds the in-memory state using shapes that make kernel queries easy: typed
state enums (`FileStatsState`, `DomainMetadataState`, `SetTransactionState`) and `HashMap`s
keyed by id, instead of the flat scalars and arrays of the on-disk format. It (de)serializes
to/from JSON via the private `CrcRaw` serde intermediate, which mirrors the wire format
exactly.

[CRC file]: https://github.com/delta-io/delta/blob/master/PROTOCOL.md#version-checksum-file
