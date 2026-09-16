# `buoyant_kernel::crc::file_size_histogram`

Crate `buoyant_kernel` · 1 public items · structured records in [`model/buoyant_kernel.crc.file_size_histogram.json`](../model/buoyant_kernel.crc.file_size_histogram.json)

## FileSizeHistogram

`struct` · `buoyant_kernel::crc::file_size_histogram::FileSizeHistogram`

Also reachable as `buoyant_kernel::FileSizeHistogram`, `buoyant_kernel::crc::FileSizeHistogram`, `delta_kernel::crc::file_size_histogram::FileSizeHistogram`

```rust
struct FileSizeHistogram
```

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn file_counts(&self) -> &[i64]
fn sorted_bin_boundaries(&self) -> &[i64]
fn total_bytes(&self) -> &[i64]
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Tracks the distribution of file sizes across predefined bins.

Each bin `i` covers the range `[sorted_bin_boundaries[i], sorted_bin_boundaries[i+1])`,
with the last bin extending to infinity. The histogram records both the count of files
and the total bytes in each bin.

See the [Delta protocol spec] for the full schema definition.

[Delta protocol spec]: https://github.com/delta-io/delta/blob/master/PROTOCOL.md#file-size-histogram-schema

---
