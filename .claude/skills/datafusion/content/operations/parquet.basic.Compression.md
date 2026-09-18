# `parquet::basic::Compression`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.basic.Compression.json).

<a id="op-c5a5132a156c388f8a10fcad"></a>
## Compression

`enum` · `parquet::basic::Compression` · parquet 59.3.0

```rust
enum Compression
```

Source: `src/basic.rs:662`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Supported block compression algorithms.

Block compression can yield non-trivial improvements to storage efficiency at the expense
of potentially significantly worse encode and decode performance. Many applications,
especially those making use of high-throughput and low-cost commodity object storage,
may find storage efficiency less important than decode throughput, and therefore may
wish to not make use of block compression.

The writers in this crate default to no block compression for this reason.

Applications that do still wish to use block compression, will find [`Compression::ZSTD`](../operations/parquet.basic.Compression.md#op-84aadab8c2faa0fef643b40e)
to provide a good balance of compression, performance, and ecosystem support. Alternatively,
[`Compression::LZ4_RAW`](../operations/parquet.basic.Compression.md#op-1decb5f30e2f8f39b1daa4b9) provides much faster decompression speeds, at the cost of typically
worse compression ratios. However, it is not as widely supported by the ecosystem, with the
Hadoop ecosystem historically favoring the non-standard and now deprecated [`Compression::LZ4`](../operations/parquet.basic.Compression.md#op-6d42043c45e7abe31397dbd4).

<a id="op-e3850c59965fea8eb8a2d7b7"></a>
## BROTLI

`variant` · `parquet::basic::Compression::BROTLI` · parquet 59.3.0

```rust
BROTLI
```

Source: `src/basic.rs:672`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

[Brotli compression](https://datatracker.ietf.org/doc/html/rfc7932)

<a id="op-fb85ba3ae8b533c46aa71eb3"></a>
## Err

`assoc_type` · `parquet::basic::Compression::Err` · parquet 59.3.0

```rust
Err
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::Compression", "path": "Compression"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [743, 1], "end": [791, 2], "filename": "src/basic.rs"}, "trait": {"args": null, "id": "core::str::traits::FromStr", "path": "FromStr"}, "trait_path": "core::str::traits::FromStr"}`

Source: `src/basic.rs:744`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3be57a7a869023137f80d0de"></a>
## GZIP

`variant` · `parquet::basic::Compression::GZIP` · parquet 59.3.0

```rust
GZIP
```

Source: `src/basic.rs:668`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

[Gzip compression](https://www.ietf.org/rfc/rfc1952.txt)

<a id="op-6d42043c45e7abe31397dbd4"></a>
## LZ4

`variant` · `parquet::basic::Compression::LZ4` · parquet 59.3.0

```rust
LZ4
```

Source: `src/basic.rs:674`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

[LZ4 compression](https://lz4.org/), [(deprecated)](https://issues.apache.org/jira/browse/PARQUET-2032)

<a id="op-1decb5f30e2f8f39b1daa4b9"></a>
## LZ4_RAW

`variant` · `parquet::basic::Compression::LZ4_RAW` · parquet 59.3.0

```rust
LZ4_RAW
```

Source: `src/basic.rs:678`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

[LZ4 compression](https://lz4.org/).

<a id="op-7c2f7b688675f1d4bb661388"></a>
## LZO

`variant` · `parquet::basic::Compression::LZO` · parquet 59.3.0

```rust
LZO
```

Source: `src/basic.rs:670`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

[LZO compression](https://en.wikipedia.org/wiki/Lempel%E2%80%93Ziv%E2%80%93Oberhumer)

<a id="op-8113c77a08a796a6c8d7251c"></a>
## SNAPPY

`variant` · `parquet::basic::Compression::SNAPPY` · parquet 59.3.0

```rust
SNAPPY
```

Source: `src/basic.rs:666`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

[Snappy compression](https://en.wikipedia.org/wiki/Snappy_(compression))

<a id="op-ebe52b4d38843c53828b0f98"></a>
## UNCOMPRESSED

`variant` · `parquet::basic::Compression::UNCOMPRESSED` · parquet 59.3.0

```rust
UNCOMPRESSED
```

Source: `src/basic.rs:664`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No compression.

<a id="op-84aadab8c2faa0fef643b40e"></a>
## ZSTD

`variant` · `parquet::basic::Compression::ZSTD` · parquet 59.3.0

```rust
ZSTD
```

Source: `src/basic.rs:676`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

[ZSTD compression](https://datatracker.ietf.org/doc/html/rfc8878)

<a id="op-4f74fe5e3906b229353c251d"></a>
## clone

`function` · `parquet::basic::Compression::clone` · parquet 59.3.0

```rust
fn clone(&self) -> Compression
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::Compression", "path": "Compression"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [660, 17], "end": [660, 22], "filename": "src/basic.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/basic.rs:660`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-30499e2d77bcc814efa36b77"></a>
## eq

`function` · `parquet::basic::Compression::eq` · parquet 59.3.0

```rust
fn eq(&self, other: &Compression) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::Compression", "path": "Compression"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [660, 30], "end": [660, 39], "filename": "src/basic.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/basic.rs:660`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6d38d96d00e6fc9f2e9b4348"></a>
## fmt

`function` · `parquet::basic::Compression::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::Compression", "path": "Compression"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1178, 1], "end": [1182, 2], "filename": "src/basic.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/basic.rs:1179`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d91603b4ae906c7f0d49eab1"></a>
## fmt

`function` · `parquet::basic::Compression::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::Compression", "path": "Compression"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [660, 10], "end": [660, 15], "filename": "src/basic.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/basic.rs:660`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fe04a97c0a30263499945cbd"></a>
## from

`function` · `parquet::basic::Compression::from` · parquet 59.3.0

```rust
fn from(value: CompressionCodec) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::Compression", "path": "Compression"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [681, 1], "end": [694, 2], "filename": "src/basic.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "parquet::basic::CompressionCodec", "path": "CompressionCodec"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/basic.rs:682`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1b66d630372308f910407a7c"></a>
## from_str

`function` · `parquet::basic::Compression::from_str` · parquet 59.3.0

```rust
fn from_str(s: &str) -> std::result::Result<Self, Self::Err>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::Compression", "path": "Compression"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [743, 1], "end": [791, 2], "filename": "src/basic.rs"}, "trait": {"args": null, "id": "core::str::traits::FromStr", "path": "FromStr"}, "trait_path": "core::str::traits::FromStr"}`

Source: `src/basic.rs:746`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
