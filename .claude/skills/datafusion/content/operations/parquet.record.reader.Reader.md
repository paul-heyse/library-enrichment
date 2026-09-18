# `parquet::record::reader::Reader`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.record.reader.Reader.json).

<a id="op-810cceb034b57468f3e86d83"></a>
## Reader

`enum` · `parquet::record::reader::Reader` · parquet 59.3.0

```rust
enum Reader
```

Source: `src/record/reader.rs:330`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Reader tree for record assembly

<a id="op-7f52983638d35ab8b65d880b"></a>
## GroupReader

`variant` · `parquet::record::reader::Reader::GroupReader` · parquet 59.3.0

```rust
GroupReader
```

Source: `src/record/reader.rs:337`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Group (struct) reader with type information, definition level and list of child
readers. When it represents message type, type information is None

<a id="op-26223ceed6756f067256b3e3"></a>
## KeyValueReader

`variant` · `parquet::record::reader::Reader::KeyValueReader` · parquet 59.3.0

```rust
KeyValueReader
```

Source: `src/record/reader.rs:343`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Reader of key-value pairs, e.g. maps, contains type information, definition
level, repetition level, child reader for keys and child reader for values

<a id="op-76fd2a6acd84a71607d73b34"></a>
## OptionReader

`variant` · `parquet::record::reader::Reader::OptionReader` · parquet 59.3.0

```rust
OptionReader
```

Source: `src/record/reader.rs:334`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Optional reader with definition level of a parent and a reader

<a id="op-eed7856fdbad3d39c91381cc"></a>
## PrimitiveReader

`variant` · `parquet::record::reader::Reader::PrimitiveReader` · parquet 59.3.0

```rust
PrimitiveReader
```

Source: `src/record/reader.rs:332`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Primitive reader with type information and triplet iterator

<a id="op-191b819bf4b35ed58521e194"></a>
## RepeatedReader

`variant` · `parquet::record::reader::Reader::RepeatedReader` · parquet 59.3.0

```rust
RepeatedReader
```

Source: `src/record/reader.rs:340`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Reader for repeated values, e.g. lists, contains type information, definition
level, repetition level and a child reader

<a id="op-b7760bb224925f70afec4887"></a>
## fmt

`function` · `parquet::record::reader::Reader::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::record::reader::Reader", "path": "Reader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [621, 1], "end": [632, 2], "filename": "src/record/reader.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/record/reader.rs:622`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
