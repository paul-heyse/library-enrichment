# `parquet::bloom_filter::BloomFilterHeader`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.bloom_filter.BloomFilterHeader.json).

<a id="op-4968ea987be4eb4df4a1a37c"></a>
## BloomFilterHeader

`struct` · `parquet::bloom_filter::BloomFilterHeader` · parquet 59.3.0

```rust
struct BloomFilterHeader
```

Source: `src/bloom_filter/mod.rs:141`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Bloom filter header is stored at beginning of Bloom filter data of each column
and followed by its bitset.


<a id="op-13fbe8162d2872595a913f0f"></a>
## algorithm

`struct_field` · `parquet::bloom_filter::BloomFilterHeader::algorithm` · parquet 59.3.0

```rust
algorithm: basic::BloomFilterAlgorithm
```

Source: `src/bloom_filter/mod.rs:141`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

The algorithm for setting bits.

<a id="op-6b0570225f9c93a8d2e8fd22"></a>
## clone

`function` · `parquet::bloom_filter::BloomFilterHeader::clone` · parquet 59.3.0

```rust
fn clone(&self) -> BloomFilterHeader
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::bloom_filter::BloomFilterHeader", "path": "BloomFilterHeader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [141, 1], "end": [155, 2], "filename": "src/bloom_filter/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/bloom_filter/mod.rs:141`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3f66934be6fe24a192ed4843"></a>
## compression

`struct_field` · `parquet::bloom_filter::BloomFilterHeader::compression` · parquet 59.3.0

```rust
compression: basic::BloomFilterCompression
```

Source: `src/bloom_filter/mod.rs:141`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

The compression used in the Bloom filter

<a id="op-0cc58d06c9436bbc0d0068bf"></a>
## eq

`function` · `parquet::bloom_filter::BloomFilterHeader::eq` · parquet 59.3.0

```rust
fn eq(&self, other: &BloomFilterHeader) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::bloom_filter::BloomFilterHeader", "path": "BloomFilterHeader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [141, 1], "end": [155, 2], "filename": "src/bloom_filter/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/bloom_filter/mod.rs:141`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7e5b43e023c8106204b16d33"></a>
## fmt

`function` · `parquet::bloom_filter::BloomFilterHeader::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::bloom_filter::BloomFilterHeader", "path": "BloomFilterHeader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [141, 1], "end": [155, 2], "filename": "src/bloom_filter/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/bloom_filter/mod.rs:141`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-65c1ce68054ffe631e2187c7"></a>
## hash

`struct_field` · `parquet::bloom_filter::BloomFilterHeader::hash` · parquet 59.3.0

```rust
hash: basic::BloomFilterHash
```

Source: `src/bloom_filter/mod.rs:141`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

The hash function used for Bloom filter

<a id="op-7229adb5a85522acfdbfdfdf"></a>
## num_bytes

`struct_field` · `parquet::bloom_filter::BloomFilterHeader::num_bytes` · parquet 59.3.0

```rust
num_bytes: i32
```

Source: `src/bloom_filter/mod.rs:141`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

The size of bitset in bytes
