# `parquet::file::properties::CdcOptions`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.file.properties.CdcOptions.json).

<a id="op-e03256b07941edddd28e75b2"></a>
## CdcOptions

`struct` · `parquet::file::properties::CdcOptions` · parquet 59.3.0

```rust
struct CdcOptions
```

Source: `src/file/properties.rs:95`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

EXPERIMENTAL: Options for content-defined chunking (CDC).

Content-defined chunking is an experimental feature that optimizes parquet
files for content addressable storage (CAS) systems by writing data pages
according to content-defined chunk boundaries. This allows for more
efficient deduplication of data across files, hence more efficient network
transfers and storage.

Each content-defined chunk is written as a separate parquet data page. The
following options control the chunks' size and the chunking process. Note
that the chunk size is calculated based on the logical value of the data,
before any encoding or compression is applied.

<a id="op-69d78c61d7108266e027cd52"></a>
## clone

`function` · `parquet::file::properties::CdcOptions::clone` · parquet 59.3.0

```rust
fn clone(&self) -> CdcOptions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::CdcOptions", "path": "CdcOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [94, 17], "end": [94, 22], "filename": "src/file/properties.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/file/properties.rs:94`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6247dba794c97841f5b26dfb"></a>
## default

`function` · `parquet::file::properties::CdcOptions::default` · parquet 59.3.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::CdcOptions", "path": "CdcOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [124, 1], "end": [132, 2], "filename": "src/file/properties.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/file/properties.rs:125`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-60fa7158561b3f781fb767ae"></a>
## eq

`function` · `parquet::file::properties::CdcOptions::eq` · parquet 59.3.0

```rust
fn eq(&self, other: &CdcOptions) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::CdcOptions", "path": "CdcOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [94, 30], "end": [94, 39], "filename": "src/file/properties.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/file/properties.rs:94`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6ff2347d4e2d0108179538f4"></a>
## fmt

`function` · `parquet::file::properties::CdcOptions::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::CdcOptions", "path": "CdcOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [94, 10], "end": [94, 15], "filename": "src/file/properties.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/file/properties.rs:94`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6017d084e26f20d9793452ab"></a>
## max_chunk_size

`struct_field` · `parquet::file::properties::CdcOptions::max_chunk_size` · parquet 59.3.0

```rust
max_chunk_size: usize
```

Source: `src/file/properties.rs:109`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Maximum chunk size in bytes, default is 1024 KiB.
The chunker will create a new chunk whenever the chunk size exceeds this value.
Note that the parquet writer has a related [`data_page_size_limit`] property that
controls the maximum size of a parquet data page after encoding. While setting
`data_page_size_limit` to a smaller value than `max_chunk_size` doesn't affect
the chunking effectiveness, it results in more small parquet data pages.

[`data_page_size_limit`]: WriterPropertiesBuilder::set_data_page_size_limit

<a id="op-1566931cc57a3e8f3f405351"></a>
## min_chunk_size

`struct_field` · `parquet::file::properties::CdcOptions::min_chunk_size` · parquet 59.3.0

```rust
min_chunk_size: usize
```

Source: `src/file/properties.rs:100`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Minimum chunk size in bytes, default is 256 KiB.
The rolling hash will not be updated until this size is reached for each chunk.
Note that all data sent through the hash function is counted towards the chunk
size, including definition and repetition levels if present.

<a id="op-96e659b8a6925a948507144a"></a>
## norm_level

`struct_field` · `parquet::file::properties::CdcOptions::norm_level` · parquet 59.3.0

```rust
norm_level: i32
```

Source: `src/file/properties.rs:121`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Number of bit adjustment to the gearhash mask in order to center the chunk size
around the average size more aggressively, default is 0.
Increasing the normalization level increases the probability of finding a chunk,
improving the deduplication ratio, but also increasing the number of small chunks
resulting in many small parquet data pages. The default value provides a good
balance between deduplication ratio and fragmentation.
Use norm_level=1 or norm_level=2 to reach a higher deduplication ratio at the
expense of fragmentation. Negative values can also be used to reduce the
probability of finding a chunk, resulting in larger chunks and fewer data pages.
Note that values outside [-3, 3] are not recommended, prefer using the default
value of 0 for most use cases.
