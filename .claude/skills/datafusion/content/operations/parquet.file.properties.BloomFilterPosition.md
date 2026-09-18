# `parquet::file::properties::BloomFilterPosition`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.file.properties.BloomFilterPosition.json).

<a id="op-c1910810a69dafe2fbcd850a"></a>
## BloomFilterPosition

`enum` · `parquet::file::properties::BloomFilterPosition` · parquet 59.3.0

```rust
enum BloomFilterPosition
```

Source: `src/file/properties.rs:173`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Where in the file [`ArrowWriter`](crate::arrow::arrow_writer::ArrowWriter) should
write Bloom filters

Basic constant, which is not part of the Thrift definition.

<a id="op-dbf7423ef960804006cd9a3f"></a>
## AfterRowGroup

`variant` · `parquet::file::properties::BloomFilterPosition::AfterRowGroup` · parquet 59.3.0

```rust
AfterRowGroup
```

Source: `src/file/properties.rs:178`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Write Bloom Filters of each row group right after the row group

This saves memory by writing it as soon as it is computed, at the cost
of data locality for readers

<a id="op-b949679da273859762926932"></a>
## End

`variant` · `parquet::file::properties::BloomFilterPosition::End` · parquet 59.3.0

```rust
End
```

Source: `src/file/properties.rs:183`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Write Bloom Filters at the end of the file

This allows better data locality for readers, at the cost of memory usage
for writers.

<a id="op-cb5c468d69fb630c47e0982b"></a>
## clone

`function` · `parquet::file::properties::BloomFilterPosition::clone` · parquet 59.3.0

```rust
fn clone(&self) -> BloomFilterPosition
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::BloomFilterPosition", "path": "BloomFilterPosition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [172, 17], "end": [172, 22], "filename": "src/file/properties.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/file/properties.rs:172`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a875d1a6834a5525d420cdf9"></a>
## eq

`function` · `parquet::file::properties::BloomFilterPosition::eq` · parquet 59.3.0

```rust
fn eq(&self, other: &BloomFilterPosition) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::BloomFilterPosition", "path": "BloomFilterPosition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [172, 30], "end": [172, 39], "filename": "src/file/properties.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/file/properties.rs:172`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5fa4565eff24ef7dae51e8e1"></a>
## fmt

`function` · `parquet::file::properties::BloomFilterPosition::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::BloomFilterPosition", "path": "BloomFilterPosition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [172, 10], "end": [172, 15], "filename": "src/file/properties.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/file/properties.rs:172`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
