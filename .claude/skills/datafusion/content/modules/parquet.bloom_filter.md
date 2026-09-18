# `parquet::bloom_filter`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.bloom_filter.json).

<a id="op-af709d60395fc7196360df11"></a>
## bloom_filter

`module` · `parquet::bloom_filter` · parquet 59.3.0

```rust
mod bloom_filter
```

Source: `src/bloom_filter/mod.rs:18`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Bloom filter implementation specific to Parquet, as described
in the [spec][parquet-bf-spec].

# Bloom Filter Size

Parquet uses the [Split Block Bloom Filter][sbbf-paper] (SBBF) as its bloom filter
implementation. For each column upon which bloom filters are enabled, the offset and length of an SBBF
is stored in  the metadata for each row group in the parquet file. The size of each filter is
initialized using a calculation based on the desired number of distinct values (NDV) and false
positive probability (FPP). The FPP for a SBBF can be approximated as<sup>[1][bf-formulae]</sup>:

```text
f = (1 - e^(-k * n / m))^k
```

Where, `f` is the FPP, `k` the number of hash functions, `n` the NDV, and `m` the total number
of bits in the bloom filter. This can be re-arranged to determine the total number of bits
required to achieve a given FPP and NDV:

```text
m = -k * n / ln(1 - f^(1/k))
```

SBBFs use eight hash functions to cleanly fit in SIMD lanes<sup>[2][sbbf-paper]</sup>, therefore
`k` is set to 8. The SBBF will spread those `m` bits accross a set of `b` blocks that
are each 256 bits, i.e., 32 bytes, in size. The number of blocks is chosen as:

```text
b = NP2(m/8) / 32
```

Where, `NP2` denotes *the next power of two*, and `m` is divided by 8 to be represented as bytes.

Here is a table of calculated sizes for various FPP and NDV:

| NDV       | FPP       | b       | Size (KB) |
|-----------|-----------|---------|-----------|
| 10,000    | 0.1       | 256     | 8         |
| 10,000    | 0.01      | 512     | 16        |
| 10,000    | 0.001     | 1,024   | 32        |
| 10,000    | 0.0001    | 1,024   | 32        |
| 100,000   | 0.1       | 4,096   | 128       |
| 100,000   | 0.01      | 4,096   | 128       |
| 100,000   | 0.001     | 8,192   | 256       |
| 100,000   | 0.0001    | 16,384  | 512       |
| 100,000   | 0.00001   | 16,384  | 512       |
| 1,000,000 | 0.1       | 32,768  | 1,024     |
| 1,000,000 | 0.01      | 65,536  | 2,048     |
| 1,000,000 | 0.001     | 65,536  | 2,048     |
| 1,000,000 | 0.0001    | 131,072 | 4,096     |
| 1,000,000 | 0.00001   | 131,072 | 4,096     |
| 1,000,000 | 0.000001  | 262,144 | 8,192     |

# Structure: Filter → Blocks → Words → Bits

An SBBF is an array of **blocks**. Each block is 256 bits (32 bytes),
divided into eight 32-bit **words**. A word is just a `u32` — an array of
32 individual bits that can each be "set" (1) or "not set" (0).

```text
  Sbbf (the whole filter)
  ┌──────────┬──────────┬──────────┬─── ─── ──┬──────────┐
  │ Block 0  │ Block 1  │ Block 2  │   ...    │ Block N-1│
  └──────────┴──────────┴──────────┴─── ─── ──┴──────────┘
       │
       ▼
  One Block = 256 bits = 8 words
  ┌────────┬────────┬────────┬────────┬────────┬────────┬────────┬────────┐
  │ word 0 │ word 1 │ word 2 │ word 3 │ word 4 │ word 5 │ word 6 │ word 7 │
  │ (u32)  │ (u32)  │ (u32)  │ (u32)  │ (u32)  │ (u32)  │ (u32)  │ (u32)  │
  └────────┴────────┴────────┴────────┴────────┴────────┴────────┴────────┘
       │
       ▼
  One Word = 32 individual bits
  ┌─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┐
  │0│0│1│0│0│0│0│0│0│0│0│0│0│0│0│0│0│0│0│0│0│0│0│0│0│0│0│0│0│0│0│0│  ← bit 29 is set
  └─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┘
```

**Inserting** a value hashes it to a 64-bit number, then:
 1. The upper 32 bits pick which **block** (via `Sbbf::hash_to_block_index`).
 2. The lower 32 bits pick one bit position in each of the 8 **words** (via `Block::mask`).
    So each insert sets exactly **8 bits** (one per word) in a single block.

**Checking** does the same two steps and returns `true` only if all 8 bits
are already set — meaning the value was *probably* inserted (or is a false
positive).

# Bloom Filter Folding

After inserting all values into a bloom filter it can be "folded" to minimize it's size.
See [`Sbbf::fold_to_target_fpp`](../operations/parquet.bloom_filter.Sbbf.md#op-943c66e12ec73793b98e3132) for details  on the algorithm and its mathematical basis.

[parquet-bf-spec]: https://github.com/apache/parquet-format/blob/master/BloomFilter.md
[sbbf-paper]: https://arxiv.org/pdf/2101.01719
[bf-formulae]: http://tfk.mit.edu/pdf/bloom.pdf
