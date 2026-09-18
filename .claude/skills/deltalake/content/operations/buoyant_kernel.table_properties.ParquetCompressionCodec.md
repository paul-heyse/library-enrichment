# `buoyant_kernel::table_properties::ParquetCompressionCodec`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.table_properties.ParquetCompressionCodec.json).

<a id="op-7cfc983ef61989a83b2c6d60"></a>
## ParquetCompressionCodec

`enum` · `buoyant_kernel::table_properties::ParquetCompressionCodec` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
enum ParquetCompressionCodec
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_properties/mod.rs#L370).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_properties/mod.rs:370`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Compression codec for newly written Parquet data files, controlled by the
`delta.parquet.compression.codec` table property.

Per the Delta protocol, parsing is case-insensitive, and `none` is accepted as an alias for
`uncompressed`. When the property is absent, writers SHOULD default to [`Self::Zstd`](../operations/buoyant_kernel.table_properties.ParquetCompressionCodec.md#op-77aaec8e41daa6caa3431be5).

See [Table Properties] in the Delta protocol.

[Table Properties]: https://github.com/delta-io/delta/blob/master/PROTOCOL.md#table-properties

<a id="op-ce338de9dbe46eff1927a3fa"></a>
## Err

`assoc_type` · `buoyant_kernel::table_properties::ParquetCompressionCodec::Err` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type Err = ParseError
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_properties/mod.rs#L368).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::table_properties::ParquetCompressionCodec", "path": "ParquetCompressionCodec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [368, 26], "end": [368, 36], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_properties/mod.rs"}, "trait": {"args": null, "id": "core::str::traits::FromStr", "path": "FromStr"}, "trait_path": "core::str::traits::FromStr"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_properties/mod.rs:368`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a99504004f335d325c512356"></a>
## Error

`assoc_type` · `buoyant_kernel::table_properties::ParquetCompressionCodec::Error` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type Error = ParseError
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_properties/mod.rs#L368).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::table_properties::ParquetCompressionCodec", "path": "ParquetCompressionCodec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [368, 26], "end": [368, 36], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_properties/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"primitive": "str"}}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_properties/mod.rs:368`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fe47986f673bcc1fa3ff3cfb"></a>
## Gzip

`variant` · `buoyant_kernel::table_properties::ParquetCompressionCodec::Gzip` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Gzip
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_properties/mod.rs#L379).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_properties/mod.rs:379`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

`gzip`.

<a id="op-7cff173f8773e423e2ea1eda"></a>
## Lz4

`variant` · `buoyant_kernel::table_properties::ParquetCompressionCodec::Lz4` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Lz4
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_properties/mod.rs#L381).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_properties/mod.rs:381`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

`lz4`. Deprecated by the Delta protocol (Hadoop framing); prefer [`Self::Lz4Raw`](../operations/buoyant_kernel.table_properties.ParquetCompressionCodec.md#op-c7dedfe532a6ed40b09b1f7d).

<a id="op-c7dedfe532a6ed40b09b1f7d"></a>
## Lz4Raw

`variant` · `buoyant_kernel::table_properties::ParquetCompressionCodec::Lz4Raw` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Lz4Raw
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_properties/mod.rs#L383).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_properties/mod.rs:383`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

`lz4_raw`. LZ4 block format.

<a id="op-a5997fcd195dce54a4a95536"></a>
## Snappy

`variant` · `buoyant_kernel::table_properties::ParquetCompressionCodec::Snappy` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Snappy
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_properties/mod.rs#L377).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_properties/mod.rs:377`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

`snappy`.

<a id="op-99d99d58d5567a5c1f0b5a4e"></a>
## Uncompressed

`variant` · `buoyant_kernel::table_properties::ParquetCompressionCodec::Uncompressed` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Uncompressed
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_properties/mod.rs#L375).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_properties/mod.rs:375`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

`uncompressed` (alias: `none`). No compression.

<a id="op-77aaec8e41daa6caa3431be5"></a>
## Zstd

`variant` · `buoyant_kernel::table_properties::ParquetCompressionCodec::Zstd` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Zstd
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_properties/mod.rs#L372).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_properties/mod.rs:372`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

`zstd`. Recommended fallback per the Delta protocol when the property is absent.

<a id="op-2d9f24ba5c6160ac1f373ba4"></a>
## clone

`function` · `buoyant_kernel::table_properties::ParquetCompressionCodec::clone` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> ParquetCompressionCodec
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_properties/mod.rs#L368).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::table_properties::ParquetCompressionCodec", "path": "ParquetCompressionCodec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [368, 59], "end": [368, 64], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_properties/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_properties/mod.rs:368`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e178a258f44d358552f82652"></a>
## eq

`function` · `buoyant_kernel::table_properties::ParquetCompressionCodec::eq` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eq(&self, other: &ParquetCompressionCodec) -> bool
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_properties/mod.rs#L368).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::table_properties::ParquetCompressionCodec", "path": "ParquetCompressionCodec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [368, 66], "end": [368, 75], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_properties/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_properties/mod.rs:368`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2062077b7ccc098360dd5391"></a>
## fmt

`function` · `buoyant_kernel::table_properties::ParquetCompressionCodec::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::result::Result<(), ::core::fmt::Error>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_properties/mod.rs#L368).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::table_properties::ParquetCompressionCodec", "path": "ParquetCompressionCodec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [368, 17], "end": [368, 24], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_properties/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_properties/mod.rs:368`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-be1bf33abb4cdf31dcaaf5c5"></a>
## fmt

`function` · `buoyant_kernel::table_properties::ParquetCompressionCodec::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_properties/mod.rs#L368).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::table_properties::ParquetCompressionCodec", "path": "ParquetCompressionCodec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [368, 10], "end": [368, 15], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_properties/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_properties/mod.rs:368`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-128ccbaf626dfda947ee5d79"></a>
## from_str

`function` · `buoyant_kernel::table_properties::ParquetCompressionCodec::from_str` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn from_str(s: &str) -> ::core::result::Result<ParquetCompressionCodec, <Self as ::core::str::FromStr>::Err>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_properties/mod.rs#L368).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::table_properties::ParquetCompressionCodec", "path": "ParquetCompressionCodec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [368, 26], "end": [368, 36], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_properties/mod.rs"}, "trait": {"args": null, "id": "core::str::traits::FromStr", "path": "FromStr"}, "trait_path": "core::str::traits::FromStr"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_properties/mod.rs:368`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-36a6e2de7cc059988eb0c257"></a>
## try_from

`function` · `buoyant_kernel::table_properties::ParquetCompressionCodec::try_from` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn try_from(s: &str) -> ::core::result::Result<ParquetCompressionCodec, <Self as ::core::convert::TryFrom>::Error>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_properties/mod.rs#L368).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::table_properties::ParquetCompressionCodec", "path": "ParquetCompressionCodec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [368, 26], "end": [368, 36], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_properties/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"primitive": "str"}}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_properties/mod.rs:368`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.
