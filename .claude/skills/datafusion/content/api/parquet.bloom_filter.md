# `parquet::bloom_filter`

Crate `parquet` · 4 public items · structured records in [`model/parquet.bloom_filter.json`](../model/parquet.bloom_filter.json)

## BITSET_MAX_LENGTH

`constant` · `parquet::bloom_filter::BITSET_MAX_LENGTH`

```rust
const BITSET_MAX_LENGTH: usize = _
```

The maximum number of bytes for a bloom filter bitset.

---

## BITSET_MIN_LENGTH

`constant` · `parquet::bloom_filter::BITSET_MIN_LENGTH`

```rust
const BITSET_MIN_LENGTH: usize = 32
```

The minimum number of bytes for a bloom filter bitset.

---

## BloomFilterHeader

`struct` · `parquet::bloom_filter::BloomFilterHeader`

```rust
struct BloomFilterHeader
```

**Fields**: `num_bytes`, `algorithm`, `hash`, `compression`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

Bloom filter header is stored at beginning of Bloom filter data of each column
and followed by its bitset.

---

## Sbbf

`struct` · `parquet::bloom_filter::Sbbf`

```rust
struct Sbbf
```

**Derives**: Clone, Debug

**Methods** (11)

```rust
fn check<T: AsBytes + ?Sized>(&self, value: &T) -> bool
fn fold_to_target_fpp(&mut self, target_fpp: f64)
fn from_bytes(bytes: &[u8]) -> Result<Self, ParquetError>
fn insert<T: AsBytes + ?Sized>(&mut self, value: &T)
fn new(bitset: &[u8]) -> Self
fn new_with_ndv_fpp(ndv: u64, fpp: f64) -> Result<Self, ParquetError>
fn new_with_num_of_bytes(num_bytes: usize) -> Self
fn num_blocks(&self) -> usize
fn read_from_column_chunk<R: ChunkReader>(column_metadata: &ColumnChunkMetaData, reader: &R) -> Result<Option<Self>, ParquetError>
fn write<W: Write>(&self, writer: W) -> Result<(), ParquetError>
fn write_bitset<W: Write>(&self, writer: W) -> Result<(), ParquetError>
```

A split block Bloom filter (SBBF).

An SBBF partitions its bit space into fixed-size 256-bit (32-byte) blocks, each fitting in a
single CPU cache line. Each block contains eight 32-bit words, aligned with SIMD lanes for
parallel bit manipulation. When checking membership, only one block is accessed per query,
eliminating the cache-miss penalty of standard Bloom filters.

## Sizing and folding

Filters are initially sized for a maximum expected number of distinct values (NDV) via
[`Sbbf::new_with_ndv_fpp`]. After all values are inserted, the filter is compacted by
calling [`Sbbf::fold_to_target_fpp`], which folds the filter down to the smallest size
that still meets the target false positive probability.

The creation of this structure is based on the [`crate::file::properties::BloomFilterProperties`]
struct set via [`crate::file::properties::WriterProperties`] and is thus hidden by default.

---
