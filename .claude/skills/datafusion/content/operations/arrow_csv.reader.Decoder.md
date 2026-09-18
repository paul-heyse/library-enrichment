# `arrow_csv::reader::Decoder`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_csv.reader.Decoder.json).

<a id="op-3b738a30f68a89852bb9bff5"></a>
## Decoder

`struct` · `arrow_csv::reader::Decoder` · arrow-csv 59.3.0

```rust
struct Decoder
```

Source: `src/reader/mod.rs:611`. [Exact documentation build](https://docs.rs/crate/arrow-csv/59.3.0/json).

A push-based interface for decoding CSV data from an arbitrary byte stream

See [`Reader`](../operations/arrow_csv.reader.Reader.md#op-80f5b4a82073800a3487a573) for a higher-level interface for interface with [`Read`]

The push-based interface facilitates integration with sources that yield arbitrarily
delimited bytes ranges, such as [`BufRead`], or a chunked byte stream received from
object storage

```
# use std::io::BufRead;
# use arrow_array::RecordBatch;
# use arrow_csv::ReaderBuilder;
# use arrow_schema::{ArrowError, SchemaRef};
#
fn read_from_csv<R: BufRead>(
    mut reader: R,
    schema: SchemaRef,
    batch_size: usize,
) -> Result<impl Iterator<Item = Result<RecordBatch, ArrowError>>, ArrowError> {
    let mut decoder = ReaderBuilder::new(schema)
        .with_batch_size(batch_size)
        .build_decoder();

    let mut next = move || {
        loop {
            let buf = reader.fill_buf()?;
            let decoded = decoder.decode(buf)?;
            if decoded == 0 {
                break;
            }

            // Consume the number of bytes read
            reader.consume(decoded);
        }
        decoder.flush()
    };
    Ok(std::iter::from_fn(move || next().transpose()))
}
```

Unresolved upstream links (retained, not inferred): ``Read``, ``BufRead``.

<a id="op-841369c63265d0be4225e887"></a>
## capacity

`function` · `arrow_csv::reader::Decoder::capacity` · arrow-csv 59.3.0

```rust
fn capacity(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_csv::reader::Decoder", "path": "Decoder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [640, 1], "end": [707, 2], "filename": "src/reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader/mod.rs:704`. [Exact documentation build](https://docs.rs/crate/arrow-csv/59.3.0/json).

Returns the number of records that can be read before requiring a call to [`Self::flush`](../operations/arrow_csv.reader.Decoder.md#op-1a91ccf89a2d1c1953d1d1ef)

<a id="op-c19c3a0d0cae30a3c83a9923"></a>
## decode

`function` · `arrow_csv::reader::Decoder::decode` · arrow-csv 59.3.0

```rust
fn decode(&mut self, buf: &[u8]) -> Result<usize, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_csv::reader::Decoder", "path": "Decoder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [640, 1], "end": [707, 2], "filename": "src/reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader/mod.rs:650`. [Exact documentation build](https://docs.rs/crate/arrow-csv/59.3.0/json).

Decode records from `buf` returning the number of bytes read

This method returns once `batch_size` objects have been parsed since the
last call to [`Self::flush`](../operations/arrow_csv.reader.Decoder.md#op-1a91ccf89a2d1c1953d1d1ef), or `buf` is exhausted. Any remaining bytes
should be included in the next call to [`Self::decode`](../operations/arrow_csv.reader.Decoder.md#op-c19c3a0d0cae30a3c83a9923)

There is no requirement that `buf` contains a whole number of records, facilitating
integration with arbitrary byte streams, such as that yielded by [`BufRead`] or
network sources such as object storage

Unresolved upstream links (retained, not inferred): ``BufRead``.

<a id="op-1a91ccf89a2d1c1953d1d1ef"></a>
## flush

`function` · `arrow_csv::reader::Decoder::flush` · arrow-csv 59.3.0

```rust
fn flush(&mut self) -> Result<Option<RecordBatch>, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_csv::reader::Decoder", "path": "Decoder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [640, 1], "end": [707, 2], "filename": "src/reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader/mod.rs:685`. [Exact documentation build](https://docs.rs/crate/arrow-csv/59.3.0/json).

Flushes the currently buffered data to a [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34)

This should only be called after [`Self::decode`](../operations/arrow_csv.reader.Decoder.md#op-c19c3a0d0cae30a3c83a9923) has returned `Ok(0)`,
otherwise may return an error if part way through decoding a record

Returns `Ok(None)` if no buffered data

<a id="op-0ebf8b99561ead7a5cde3379"></a>
## fmt

`function` · `arrow_csv::reader::Decoder::fmt` · arrow-csv 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_csv::reader::Decoder", "path": "Decoder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [610, 10], "end": [610, 15], "filename": "src/reader/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/reader/mod.rs:610`. [Exact documentation build](https://docs.rs/crate/arrow-csv/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
