# `arrow_json::reader`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_json.reader.json).

<a id="op-53dfc6a8d86f7f7c1bf482e7"></a>
## reader

`module` · `arrow_json::reader` · arrow-json 59.3.0

```rust
mod reader
```

Source: `src/reader/mod.rs:18`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

JSON reader

This JSON reader allows JSON records to be read into the Arrow memory
model. Records are loaded in batches and are then converted from the record-oriented
representation to the columnar arrow data model.

The reader ignores whitespace between JSON values, including `\n` and `\r`, allowing
parsing of sequences of one or more arbitrarily formatted JSON values, including
but not limited to newline-delimited JSON.

# Basic Usage

[`Reader`](../operations/arrow_json.reader.Reader.md#op-065551256893ce5f2d76aab8) can be used directly with synchronous data sources, such as [`std::fs::File`]

```
# use arrow_schema::*;
# use std::fs::File;
# use std::io::BufReader;
# use std::sync::Arc;

let schema = Arc::new(Schema::new(vec![
    Field::new("a", DataType::Float64, false),
    Field::new("b", DataType::Float64, false),
    Field::new("c", DataType::Boolean, true),
]));

let file = File::open("test/data/basic.json").unwrap();

let mut json = arrow_json::ReaderBuilder::new(schema).build(BufReader::new(file)).unwrap();
let batch = json.next().unwrap().unwrap();
```

# Async Usage

The lower-level [`Decoder`](../operations/arrow_json.reader.Decoder.md#op-9a10b6ca230ecd7bed81532a) can be integrated with various forms of async data streams,
and is designed to be agnostic to the various different kinds of async IO primitives found
within the Rust ecosystem.

For example, see below for how it can be used with an arbitrary `Stream` of `Bytes`

```
# use std::task::{Poll, ready};
# use bytes::{Buf, Bytes};
# use arrow_schema::ArrowError;
# use futures::stream::{Stream, StreamExt};
# use arrow_array::RecordBatch;
# use arrow_json::reader::Decoder;
#
fn decode_stream<S: Stream<Item = Bytes> + Unpin>(
    mut decoder: Decoder,
    mut input: S,
) -> impl Stream<Item = Result<RecordBatch, ArrowError>> {
    let mut buffered = Bytes::new();
    futures::stream::poll_fn(move |cx| {
        loop {
            if buffered.is_empty() {
                buffered = match ready!(input.poll_next_unpin(cx)) {
                    Some(b) => b,
                    None => break,
                };
            }
            let decoded = match decoder.decode(buffered.as_ref()) {
                Ok(decoded) => decoded,
                Err(e) => return Poll::Ready(Some(Err(e))),
            };
            let read = buffered.len();
            buffered.advance(decoded);
            if decoded != read {
                break
            }
        }

        Poll::Ready(decoder.flush().transpose())
    })
}

```

In a similar vein, it can also be used with tokio-based IO primitives

```
# use std::sync::Arc;
# use arrow_schema::{DataType, Field, Schema};
# use std::pin::Pin;
# use std::task::{Poll, ready};
# use futures::{Stream, TryStreamExt};
# use tokio::io::AsyncBufRead;
# use arrow_array::RecordBatch;
# use arrow_json::reader::Decoder;
# use arrow_schema::ArrowError;
fn decode_stream<R: AsyncBufRead + Unpin>(
    mut decoder: Decoder,
    mut reader: R,
) -> impl Stream<Item = Result<RecordBatch, ArrowError>> {
    futures::stream::poll_fn(move |cx| {
        loop {
            let b = match ready!(Pin::new(&mut reader).poll_fill_buf(cx)) {
                Ok(b) if b.is_empty() => break,
                Ok(b) => b,
                Err(e) => return Poll::Ready(Some(Err(e.into()))),
            };
            let read = b.len();
            let decoded = match decoder.decode(b) {
                Ok(decoded) => decoded,
                Err(e) => return Poll::Ready(Some(Err(e))),
            };
            Pin::new(&mut reader).consume(decoded);
            if decoded != read {
                break;
            }
        }

        Poll::Ready(decoder.flush().transpose())
    })
}
```


Unresolved upstream links (retained, not inferred): ``std::fs::File``.
