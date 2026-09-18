# `buoyant_kernel::crc::file_size_histogram::FileSizeHistogram`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.crc.file_size_histogram.FileSizeHistogram.json).

<a id="op-c8fc21baeab039dee041e99d"></a>
## FileSizeHistogram

`struct` · `buoyant_kernel::crc::file_size_histogram::FileSizeHistogram` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct FileSizeHistogram
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/crc/file_size_histogram.rs#L70).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/crc/file_size_histogram.rs:70`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Tracks the distribution of file sizes across predefined bins.

Each bin `i` covers the range `[sorted_bin_boundaries[i], sorted_bin_boundaries[i+1])`,
with the last bin extending to infinity. The histogram records both the count of files
and the total bytes in each bin.

See the [Delta protocol spec] for the full schema definition.

[Delta protocol spec]: https://github.com/delta-io/delta/blob/master/PROTOCOL.md#file-size-histogram-schema

<a id="op-c15bda01a2dbaeb3b89debae"></a>
## clone

`function` · `buoyant_kernel::crc::file_size_histogram::FileSizeHistogram::clone` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> FileSizeHistogram
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/crc/file_size_histogram.rs#L68).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::crc::file_size_histogram::FileSizeHistogram", "path": "FileSizeHistogram"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [68, 17], "end": [68, 22], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/crc/file_size_histogram.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/crc/file_size_histogram.rs:68`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0812166f361aab4e3572c86b"></a>
## deserialize

`function` · `buoyant_kernel::crc::file_size_histogram::FileSizeHistogram::deserialize` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/crc/file_size_histogram.rs#L68).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::crc::file_size_histogram::FileSizeHistogram", "path": "FileSizeHistogram"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [68, 50], "end": [68, 61], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/crc/file_size_histogram.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/crc/file_size_histogram.rs:68`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0a65eeb17f377bbf963b2447"></a>
## eq

`function` · `buoyant_kernel::crc::file_size_histogram::FileSizeHistogram::eq` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eq(&self, other: &FileSizeHistogram) -> bool
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/crc/file_size_histogram.rs#L68).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::crc::file_size_histogram::FileSizeHistogram", "path": "FileSizeHistogram"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [68, 24], "end": [68, 33], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/crc/file_size_histogram.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/crc/file_size_histogram.rs:68`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c97c2377e6aa5d9d8721a98e"></a>
## file_counts

`function` · `buoyant_kernel::crc::file_size_histogram::FileSizeHistogram::file_counts` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn file_counts(&self) -> &[i64]
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/crc/file_size_histogram.rs#L95).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::crc::file_size_histogram::FileSizeHistogram", "path": "FileSizeHistogram"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [81, 1], "end": [276, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/crc/file_size_histogram.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/crc/file_size_histogram.rs:95`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Returns the file count for each bin.

Each element is the number of files whose size falls within the corresponding
bin's range. Length matches [`sorted_bin_boundaries()`](Self::sorted_bin_boundaries).

<a id="op-e2aa27ea8aa395b0b2a6ec4f"></a>
## fmt

`function` · `buoyant_kernel::crc::file_size_histogram::FileSizeHistogram::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/crc/file_size_histogram.rs#L68).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::crc::file_size_histogram::FileSizeHistogram", "path": "FileSizeHistogram"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [68, 10], "end": [68, 15], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/crc/file_size_histogram.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/crc/file_size_histogram.rs:68`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1581df00f32ca5c772801c19"></a>
## serialize

`function` · `buoyant_kernel::crc::file_size_histogram::FileSizeHistogram::serialize` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/crc/file_size_histogram.rs#L68).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::crc::file_size_histogram::FileSizeHistogram", "path": "FileSizeHistogram"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [68, 39], "end": [68, 48], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/crc/file_size_histogram.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/crc/file_size_histogram.rs:68`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-46cd2a3c41d9f2579ffbd190"></a>
## sorted_bin_boundaries

`function` · `buoyant_kernel::crc::file_size_histogram::FileSizeHistogram::sorted_bin_boundaries` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn sorted_bin_boundaries(&self) -> &[i64]
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/crc/file_size_histogram.rs#L87).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::crc::file_size_histogram::FileSizeHistogram", "path": "FileSizeHistogram"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [81, 1], "end": [276, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/crc/file_size_histogram.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/crc/file_size_histogram.rs:87`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Returns the sorted bin boundaries defining the histogram ranges.

Each element represents the inclusive lower bound of a bin. The bin covers
`[sorted_bin_boundaries[i], sorted_bin_boundaries[i+1])`, with the last bin
extending to infinity. The first element is always 0.

<a id="op-ebba3ead91ef0ecb7afe53ad"></a>
## total_bytes

`function` · `buoyant_kernel::crc::file_size_histogram::FileSizeHistogram::total_bytes` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn total_bytes(&self) -> &[i64]
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/crc/file_size_histogram.rs#L103).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::crc::file_size_histogram::FileSizeHistogram", "path": "FileSizeHistogram"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [81, 1], "end": [276, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/crc/file_size_histogram.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/crc/file_size_histogram.rs:103`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Returns the total bytes for each bin.

Each element is the sum of file sizes within the corresponding bin's range.
Length matches [`sorted_bin_boundaries()`](Self::sorted_bin_boundaries).

<a id="op-7e69be8309e71ea5979d6ec5"></a>
## file_counts

`struct_field` · `buoyant_kernel::crc::file_size_histogram::FileSizeHistogram::file_counts` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
file_counts: Vec<i64>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/crc/file_size_histogram.rs#L76).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/crc/file_size_histogram.rs:76`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Count of files in each bin. Length must match `sorted_bin_boundaries`.

<a id="op-033ae521af593b06df600a44"></a>
## sorted_bin_boundaries

`struct_field` · `buoyant_kernel::crc::file_size_histogram::FileSizeHistogram::sorted_bin_boundaries` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
sorted_bin_boundaries: Vec<i64>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/crc/file_size_histogram.rs#L74).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/crc/file_size_histogram.rs:74`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

A sorted array of bin boundaries where each element represents the start of a bin
(inclusive) and the next element represents the end of the bin (exclusive). The first
element must be 0.

<a id="op-d611a0d31aa19da341240294"></a>
## total_bytes

`struct_field` · `buoyant_kernel::crc::file_size_histogram::FileSizeHistogram::total_bytes` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
total_bytes: Vec<i64>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/crc/file_size_histogram.rs#L78).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/crc/file_size_histogram.rs:78`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Total bytes of files in each bin. Length must match `sorted_bin_boundaries`.
