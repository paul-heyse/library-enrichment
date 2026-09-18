# `buoyant_kernel::crc`

Crate `buoyant_kernel` · 2 public items · structured records in [`model/buoyant_kernel.crc.json`](../model/buoyant_kernel.crc.json)

## Crc

`struct` · `buoyant_kernel::crc::Crc`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.crc.Crc.md)

Also reachable as `delta_kernel::crc::Crc`

```rust
struct Crc
```

**Fields**: `version`, `metadata`, `protocol`, `in_commit_timestamp_opt`, `set_transaction_state`, `domain_metadata_state`

**Implements**: `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, Eq, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn file_stats(&self) -> Option<&FileStats>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error>
```

Parsed content of a CRC (version checksum) file.

A `Crc` is either (a) loaded from disk (deserialized from a `.crc` JSON file via
the private `CrcRaw` intermediate) or (b) computed in memory (built incrementally via
`Crc::apply`).

A CRC file must:
1. Be named `{version}.crc` with version zero-padded to 20 digits: `00000000000000000001.crc`
2. Be stored directly in the _delta_log directory alongside Delta log files
3. Contain exactly one JSON object with the schema mirrored by `CrcRaw`.

This struct and its fields are marked `pub`, but the `crc` module is only re-exported as `pub`
when the `internal-api` feature is enabled (otherwise `pub(crate)`). See `kernel/src/lib.rs`.

---

## DeletedRecordCountsHistogram

`struct` · `buoyant_kernel::crc::DeletedRecordCountsHistogram`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.crc.DeletedRecordCountsHistogram.md)

Also reachable as `delta_kernel::crc::DeletedRecordCountsHistogram`

```rust
struct DeletedRecordCountsHistogram
```

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

The [DeletedRecordCountsHistogram] object represents a histogram tracking the distribution of
deleted record counts across files in the table. Each bin in the histogram represents a range
of deletion counts and stores the number of files having that many deleted records.

The histogram bins correspond to the following ranges:
Bin 0: [0, 0] (files with no deletions)
Bin 1: [1, 9] (files with 1-9 deleted records)
Bin 2: [10, 99] (files with 10-99 deleted records)
Bin 3: [100, 999] (files with 100-999 deleted records)
Bin 4: [1000, 9999] (files with 1,000-9,999 deleted records)
Bin 5: [10000, 99999] (files with 10,000-99,999 deleted records)
Bin 6: [100000, 999999] (files with 100,000-999,999 deleted records)
Bin 7: [1000000, 9999999] (files with 1,000,000-9,999,999 deleted records)
Bin 8: [10000000, 2147483646] (files with 10,000,000 to 2,147,483,646 deleted records)
Bin 9: [2147483647, inf) (files with 2,147,483,647 or more deleted records)

[DeletedRecordCountsHistogram]: https://github.com/delta-io/delta/blob/master/PROTOCOL.md#deleted-record-counts-histogram-schema

---
