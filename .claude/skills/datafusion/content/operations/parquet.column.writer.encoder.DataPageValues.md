# `parquet::column::writer::encoder::DataPageValues`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.column.writer.encoder.DataPageValues.json).

<a id="op-5b69f3d77a16d7336cc23e98"></a>
## DataPageValues

`struct` · `parquet::column::writer::encoder::DataPageValues` · parquet 59.3.0

```rust
struct DataPageValues<T>
```

Source: `src/column/writer/encoder.rs:62`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

The encoded values for a data page, with optional statistics

<a id="op-765929c94f0866c22b9a91c4"></a>
## buf

`struct_field` · `parquet::column::writer::encoder::DataPageValues::buf` · parquet 59.3.0

```rust
buf: bytes::Bytes
```

Source: `src/column/writer/encoder.rs:63`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d6d219fa5d377f74e3beacd6"></a>
## encoding

`struct_field` · `parquet::column::writer::encoder::DataPageValues::encoding` · parquet 59.3.0

```rust
encoding: basic::Encoding
```

Source: `src/column/writer/encoder.rs:65`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-02f5571067fa26b6d01cb823"></a>
## max_value

`struct_field` · `parquet::column::writer::encoder::DataPageValues::max_value` · parquet 59.3.0

```rust
max_value: Option<T>
```

Source: `src/column/writer/encoder.rs:67`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6da10e6e331cc118b9e3b150"></a>
## min_value

`struct_field` · `parquet::column::writer::encoder::DataPageValues::min_value` · parquet 59.3.0

```rust
min_value: Option<T>
```

Source: `src/column/writer/encoder.rs:66`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-33828925d1395d83857a450f"></a>
## num_values

`struct_field` · `parquet::column::writer::encoder::DataPageValues::num_values` · parquet 59.3.0

```rust
num_values: usize
```

Source: `src/column/writer/encoder.rs:64`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-60d7f5d6b577d2512e1d61a8"></a>
## variable_length_bytes

`struct_field` · `parquet::column::writer::encoder::DataPageValues::variable_length_bytes` · parquet 59.3.0

```rust
variable_length_bytes: Option<i64>
```

Source: `src/column/writer/encoder.rs:68`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
