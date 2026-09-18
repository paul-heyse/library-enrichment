# `parquet::bloom_filter::Sbbf`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.bloom_filter.Sbbf.json).

<a id="op-47eb960cc6ee6f46d8423ca5"></a>
## Sbbf

`struct` · `parquet::bloom_filter::Sbbf` · parquet 59.3.0

```rust
struct Sbbf
```

Source: `src/bloom_filter/mod.rs:321`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

A split block Bloom filter (SBBF).

An SBBF partitions its bit space into fixed-size 256-bit (32-byte) blocks, each fitting in a
single CPU cache line. Each block contains eight 32-bit words, aligned with SIMD lanes for
parallel bit manipulation. When checking membership, only one block is accessed per query,
eliminating the cache-miss penalty of standard Bloom filters.

## Sizing and folding

Filters are initially sized for a maximum expected number of distinct values (NDV) via
[`Sbbf::new_with_ndv_fpp`](../operations/parquet.bloom_filter.Sbbf.md#op-dab59b8a97c422c2a05e474c). After all values are inserted, the filter is compacted by
calling [`Sbbf::fold_to_target_fpp`](../operations/parquet.bloom_filter.Sbbf.md#op-943c66e12ec73793b98e3132), which folds the filter down to the smallest size
that still meets the target false positive probability.

The creation of this structure is based on the [`crate::file::properties::BloomFilterProperties`](../operations/parquet.file.properties.BloomFilterProperties.md#op-d8c6090fa6f2cfe781d4070c)
struct set via [`crate::file::properties::WriterProperties`](../operations/parquet.file.properties.WriterProperties.md#op-1a8b0462c7a132d1d5004af2) and is thus hidden by default.

<a id="op-4690bd8e0262446cd5ff1eda"></a>
## check

`function` · `parquet::bloom_filter::Sbbf::check` · parquet 59.3.0

```rust
fn check<T: AsBytes + ?Sized>(&self, value: &T) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::bloom_filter::Sbbf", "path": "Sbbf"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [379, 1], "end": [754, 2], "filename": "src/bloom_filter/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/bloom_filter/mod.rs:556`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Check if an [AsBytes](../operations/parquet.data_type.AsBytes.md#op-df19664d53a774fa5f1d6462) value is probably present or definitely absent in the filter

<a id="op-e8b3b5d6f7374f53d080876b"></a>
## clone

`function` · `parquet::bloom_filter::Sbbf::clone` · parquet 59.3.0

```rust
fn clone(&self) -> Sbbf
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::bloom_filter::Sbbf", "path": "Sbbf"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [320, 17], "end": [320, 22], "filename": "src/bloom_filter/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/bloom_filter/mod.rs:320`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-aec6838d414ef548a7ffe7e8"></a>
## fmt

`function` · `parquet::bloom_filter::Sbbf::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::bloom_filter::Sbbf", "path": "Sbbf"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [320, 10], "end": [320, 15], "filename": "src/bloom_filter/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/bloom_filter/mod.rs:320`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-943c66e12ec73793b98e3132"></a>
## fold_to_target_fpp

`function` · `parquet::bloom_filter::Sbbf::fold_to_target_fpp` · parquet 59.3.0

```rust
fn fold_to_target_fpp(&mut self, target_fpp: f64)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::bloom_filter::Sbbf", "path": "Sbbf"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [379, 1], "end": [754, 2], "filename": "src/bloom_filter/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/bloom_filter/mod.rs:618`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Fold the bloom filter down to the smallest size that still meets the target FPP
(False Positive Percentage).

Folds the filter by merging groups of adjacent blocks via bitwise OR, where each
fold level halves the number of blocks. The fold count is chosen as the maximum
number of folds whose estimated FPP stays within `target_fpp`. The filter stops
at a minimum size of 1 block (32 bytes).

## How it works

SBBFs use multiplicative hashing for block selection:

```text
block_index = ((hash >> 32) * num_blocks) >> 32
```

A single fold halves the block count: when `num_blocks` is halved, the new index
becomes `floor(original_index / 2)`, so blocks `2i` and `2i+1` map to the same
position. More generally, `k` folds reduce the block count by `2^k`, merging
groups of `2^k` adjacent blocks in a single pass:

```text
folded[i] = blocks[i*2^k] | blocks[i*2^k + 1] | ... | blocks[i*2^k + 2^k - 1]
```

This differs from standard Bloom filter folding, which merges the two halves
(`B[i] | B[i + m/2]`) because standard filters use modular hashing where
`h(x) mod (m/2)` maps indices `i` and `i + m/2` to the same position.

## Correctness

Folding **never introduces false negatives**. Every bit that was set in the original
filter remains set in the folded filter (via bitwise OR). The only effect is a controlled
increase in FPP as set bits from different blocks are merged together.
This is was originally proven in [Sailhan & Stehr 2012] for standard bloom filters and is empirically
demonstrated for SBBFs in Lemma 1 and Lemma 2 of the tests.

## References

[Sailhan & Stehr 2012]: https://doi.org/10.1109/GreenCom.2012.16

<a id="op-c0ffe29c53b5b1da991aaa25"></a>
## from_bytes

`function` · `parquet::bloom_filter::Sbbf::from_bytes` · parquet 59.3.0

```rust
fn from_bytes(bytes: &[u8]) -> Result<Self, ParquetError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::bloom_filter::Sbbf", "path": "Sbbf"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [379, 1], "end": [754, 2], "filename": "src/bloom_filter/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/bloom_filter/mod.rs:729`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Reads a Sbff from Thrift encoded bytes

# Examples

```no_run
# use parquet::errors::Result;
# use parquet::bloom_filter::Sbbf;
# fn main() -> Result<()> {
// In a real application, you would read serialized bloom filter bytes from a cache.
// This example demonstrates the deserialization process.
// Assuming you have bloom filter bytes from a Parquet file:
# let serialized_bytes: Vec<u8> = vec![];
let bloom_filter = Sbbf::from_bytes(&serialized_bytes)?;
// Now you can use the bloom filter to check for values
if bloom_filter.check(&"some_value") {
    println!("Value might be present (or false positive)");
} else {
    println!("Value is definitely not present");
}
# Ok(())
# }
```

<a id="op-9a93e6db91ce8d5fe29ce5d0"></a>
## insert

`function` · `parquet::bloom_filter::Sbbf::insert` · parquet 59.3.0

```rust
fn insert<T: AsBytes + ?Sized>(&mut self, value: &T)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::bloom_filter::Sbbf", "path": "Sbbf"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [379, 1], "end": [754, 2], "filename": "src/bloom_filter/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/bloom_filter/mod.rs:545`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Insert an [AsBytes](../operations/parquet.data_type.AsBytes.md#op-df19664d53a774fa5f1d6462) value into the filter

<a id="op-b9c21bcd4a4439f083e0a9b9"></a>
## new

`function` · `parquet::bloom_filter::Sbbf::new` · parquet 59.3.0

```rust
fn new(bitset: &[u8]) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::bloom_filter::Sbbf", "path": "Sbbf"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [379, 1], "end": [754, 2], "filename": "src/bloom_filter/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/bloom_filter/mod.rs:403`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Creates a new [Sbbf](../operations/parquet.bloom_filter.Sbbf.md#op-47eb960cc6ee6f46d8423ca5) from a raw byte slice.

<a id="op-dab59b8a97c422c2a05e474c"></a>
## new_with_ndv_fpp

`function` · `parquet::bloom_filter::Sbbf::new_with_ndv_fpp` · parquet 59.3.0

```rust
fn new_with_ndv_fpp(ndv: u64, fpp: f64) -> Result<Self, ParquetError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::bloom_filter::Sbbf", "path": "Sbbf"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [379, 1], "end": [754, 2], "filename": "src/bloom_filter/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/bloom_filter/mod.rs:382`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Create a new [Sbbf](../operations/parquet.bloom_filter.Sbbf.md#op-47eb960cc6ee6f46d8423ca5) with given number of distinct values and false positive probability.
Will return an error if `fpp` is greater than or equal to 1.0 or less than 0.0.

<a id="op-9fa2eb49b10fd3519841a3d5"></a>
## new_with_num_of_bytes

`function` · `parquet::bloom_filter::Sbbf::new_with_num_of_bytes` · parquet 59.3.0

```rust
fn new_with_num_of_bytes(num_bytes: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::bloom_filter::Sbbf", "path": "Sbbf"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [379, 1], "end": [754, 2], "filename": "src/bloom_filter/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/bloom_filter/mod.rs:394`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Create a new [Sbbf](../operations/parquet.bloom_filter.Sbbf.md#op-47eb960cc6ee6f46d8423ca5) with given number of bytes, the exact number of bytes will be adjusted
to the next power of two bounded by [BITSET_MIN_LENGTH](../operations/parquet.bloom_filter.BITSET_MIN_LENGTH.md#op-65b75db588b6e018cdc7bd85) and [BITSET_MAX_LENGTH](../operations/parquet.bloom_filter.BITSET_MAX_LENGTH.md#op-10db3716e4217119663fd14c).

<a id="op-63738dc98abacd84c4277797"></a>
## num_blocks

`function` · `parquet::bloom_filter::Sbbf::num_blocks` · parquet 59.3.0

```rust
fn num_blocks(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::bloom_filter::Sbbf", "path": "Sbbf"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [379, 1], "end": [754, 2], "filename": "src/bloom_filter/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/bloom_filter/mod.rs:574`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns the number of blocks in this bloom filter.

<a id="op-1bc12cc1f6e669377e9cd563"></a>
## read_from_column_chunk

`function` · `parquet::bloom_filter::Sbbf::read_from_column_chunk` · parquet 59.3.0

```rust
fn read_from_column_chunk<R: ChunkReader>(column_metadata: &ColumnChunkMetaData, reader: &R) -> Result<Option<Self>, ParquetError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::bloom_filter::Sbbf", "path": "Sbbf"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [379, 1], "end": [754, 2], "filename": "src/bloom_filter/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/bloom_filter/mod.rs:471`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Read a new bloom filter from the given offset in the given reader.

<a id="op-93c56d91885ef800de8aa093"></a>
## write

`function` · `parquet::bloom_filter::Sbbf::write` · parquet 59.3.0

```rust
fn write<W: Write>(&self, writer: W) -> Result<(), ParquetError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::bloom_filter::Sbbf", "path": "Sbbf"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [379, 1], "end": [754, 2], "filename": "src/bloom_filter/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/bloom_filter/mod.rs:421`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Write the bloom filter data (header and then bitset) to the output. This doesn't
flush the writer in order to boost performance of bulk writing all blocks. Caller
must remember to flush the writer.
This method usually is used in conjunction with [`Self::from_bytes`](../operations/parquet.bloom_filter.Sbbf.md#op-c0ffe29c53b5b1da991aaa25) for serialization/deserialization.

<a id="op-147c3294b19540d6eea3105d"></a>
## write_bitset

`function` · `parquet::bloom_filter::Sbbf::write_bitset` · parquet 59.3.0

```rust
fn write_bitset<W: Write>(&self, writer: W) -> Result<(), ParquetError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::bloom_filter::Sbbf", "path": "Sbbf"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [379, 1], "end": [754, 2], "filename": "src/bloom_filter/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/bloom_filter/mod.rs:445`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Write the bitset in serialized form to the writer.
