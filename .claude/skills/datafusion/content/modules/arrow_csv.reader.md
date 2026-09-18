# `arrow_csv::reader`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_csv.reader.json).

<a id="op-bee90653886c075fc154b5b7"></a>
## reader

`module` · `arrow_csv::reader` · arrow-csv 59.3.0

```rust
mod reader
```

Source: `src/reader/mod.rs:18`. [Exact documentation build](https://docs.rs/crate/arrow-csv/59.3.0/json).

 CSV Reading: [`Reader`](../operations/arrow_csv.reader.Reader.md#op-80f5b4a82073800a3487a573) and [`ReaderBuilder`](../operations/arrow_csv.reader.ReaderBuilder.md#op-743461f30ad5174b593cb0bd)

 # Basic Usage

 This CSV reader allows CSV files to be read into the Arrow memory model. Records are
 loaded in batches and are then converted from row-based data to columnar data.

 Example:

 ```
 # use arrow_schema::*;
 # use arrow_csv::{Reader, ReaderBuilder};
 # use std::fs::File;
 # use std::sync::Arc;

 let schema = Schema::new(vec![
     Field::new("city", DataType::Utf8, false),
     Field::new("lat", DataType::Float64, false),
     Field::new("lng", DataType::Float64, false),
 ]);

 let file = File::open("test/data/uk_cities.csv").unwrap();

 let mut csv = ReaderBuilder::new(Arc::new(schema)).build(file).unwrap();
 let batch = csv.next().unwrap().unwrap();
 ```

 # Example: Numeric calculations on CSV
 This code finds the maximum value in column 0 of a CSV file containing
 ```csv
 c1,c2,c3,c4
 1,1.1,"hong kong",true
 3,323.12,"XiAn",false
 10,131323.12,"cheng du",false
 ```

 ```
 # use arrow_array::cast::AsArray;
 # use arrow_array::types::Int16Type;
 # use arrow_csv::ReaderBuilder;
 # use arrow_schema::{DataType, Field, Schema};
 # use std::fs::File;
 # use std::sync::Arc;
 // Open the example file
 let file = File::open("test/data/example.csv").unwrap();
 let csv_schema = Schema::new(vec![
     Field::new("c1", DataType::Int16, true),
     Field::new("c2", DataType::Float32, true),
     Field::new("c3", DataType::Utf8, true),
     Field::new("c4", DataType::Boolean, true),
 ]);
 let mut reader = ReaderBuilder::new(Arc::new(csv_schema))
     .with_header(true)
     .build(file)
     .unwrap();
 // find the maximum value in column 0 across all batches
 let mut max_c0 = 0;
 while let Some(r) = reader.next() {
   let r = r.unwrap(); // handle error
   // get the max value in column(0) for this batch
   let col = r.column(0).as_primitive::<Int16Type>();
   let batch_max = col.iter().max().flatten().unwrap_or_default();
   max_c0 = max_c0.max(batch_max);
 }
 assert_eq!(max_c0, 10);
```

 # Async Usage

 The lower-level [`Decoder`](../operations/arrow_csv.reader.Decoder.md#op-3b738a30f68a89852bb9bff5) can be integrated with various forms of async data streams,
 and is designed to be agnostic to the various different kinds of async IO primitives found
 within the Rust ecosystem.

 For example, see below for how it can be used with an arbitrary `Stream` of `Bytes`

 ```
 # use std::task::{Poll, ready};
 # use bytes::{Buf, Bytes};
 # use arrow_schema::ArrowError;
 # use futures::stream::{Stream, StreamExt};
 # use arrow_array::RecordBatch;
 # use arrow_csv::reader::Decoder;
 #
 fn decode_stream<S: Stream<Item = Bytes> + Unpin>(
     mut decoder: Decoder,
     mut input: S,
 ) -> impl Stream<Item = Result<RecordBatch, ArrowError>> {
     let mut buffered = Bytes::new();
     futures::stream::poll_fn(move |cx| {
         loop {
             if buffered.is_empty() {
                 if let Some(b) = ready!(input.poll_next_unpin(cx)) {
                     buffered = b;
                 }
                 // Note: don't break on `None` as the decoder needs
                 // to be called with an empty array to delimit the
                 // final record
             }
             let decoded = match decoder.decode(buffered.as_ref()) {
                 Ok(0) => break,
                 Ok(decoded) => decoded,
                 Err(e) => return Poll::Ready(Some(Err(e))),
             };
             buffered.advance(decoded);
         }

         Poll::Ready(decoder.flush().transpose())
     })
 }

 ```

 In a similar vein, it can also be used with tokio-based IO primitives

 ```
 # use std::pin::Pin;
 # use std::task::{Poll, ready};
 # use futures::Stream;
 # use tokio::io::AsyncBufRead;
 # use arrow_array::RecordBatch;
 # use arrow_csv::reader::Decoder;
 # use arrow_schema::ArrowError;
 fn decode_stream<R: AsyncBufRead + Unpin>(
     mut decoder: Decoder,
     mut reader: R,
 ) -> impl Stream<Item = Result<RecordBatch, ArrowError>> {
     futures::stream::poll_fn(move |cx| {
         loop {
             let b = match ready!(Pin::new(&mut reader).poll_fill_buf(cx)) {
                 Ok(b) => b,
                 Err(e) => return Poll::Ready(Some(Err(e.into()))),
             };
             let decoded = match decoder.decode(b) {
                 // Note: the decoder needs to be called with an empty
                 // array to delimit the final record
                 Ok(0) => break,
                 Ok(decoded) => decoded,
                 Err(e) => return Poll::Ready(Some(Err(e))),
             };
             Pin::new(&mut reader).consume(decoded);
         }

         Poll::Ready(decoder.flush().transpose())
     })
 }
 ```

