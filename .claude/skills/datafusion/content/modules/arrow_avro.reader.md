# `arrow_avro::reader`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_avro.reader.json).

<a id="op-d0f1dd18a131a112e4533284"></a>
## reader

`module` · `arrow_avro::reader` · arrow-avro 59.3.0

```rust
mod reader
```

Source: `src/reader/mod.rs:18`. [Exact documentation build](https://docs.rs/crate/arrow-avro/59.3.0/json).

 Core functionality for reading Avro data into Arrow arrays

 Implements the primary reader interface and record decoding logic.
 Avro reader

 Facilities to read Apache Avro–encoded data into Arrow's `RecordBatch` format.

 ### Limitations

- **Avro unions with > 127 branches are not supported.**
  When decoding Avro unions to Arrow `UnionArray`, Arrow stores the union
  type identifiers in an **8‑bit signed** buffer (`i8`). This implies a
  practical limit of **127** distinct branch ids. Inputs that resolve to
  more than 127 branches will return an error. If you truly need more,
  model the schema as a **union of unions**, per the Arrow format spec.

  See: Arrow Columnar Format — Dense Union (“types buffer: 8‑bit signed;
  a union with more than 127 possible types can be modeled as a union of
  unions”).

 This module exposes three layers of the API surface, from highest to lowest-level:

 * [`ReaderBuilder`](crate::reader::ReaderBuilder): configures how Avro is read (batch size, strict union handling,
   string representation, reader schema, etc.) and produces either:
   * a `Reader` for **Avro Object Container Files (OCF)** read from any `BufRead`, or
   * a low-level `Decoder` for **single‑object encoded** Avro bytes and Confluent
     **Schema Registry** framed messages.
 * [`Reader`](crate::reader::Reader): a convenient, synchronous iterator over `RecordBatch` decoded from an OCF
   input. Implements [`Iterator<Item = Result<RecordBatch, ArrowError>>`] and
   `RecordBatchReader`.
 * [`Decoder`](crate::reader::Decoder): a push‑based row decoder that consumes SOE framed Avro bytes and yields ready
   `RecordBatch` values when batches fill. This is suitable for integrating with async
   byte streams, network protocols, or other custom data sources.

 ## Encodings and when to use which type

 * **Object Container File (OCF)**: A self‑describing file format with a header containing
   the writer schema, optional compression codec, and a sync marker, followed by one or
   more data blocks. Use `Reader` for this format. See the Avro 1.11.1 specification
   (“Object Container Files”). <https://avro.apache.org/docs/1.11.1/specification/#object-container-files>
 * **Single‑Object Encoding**: A stream‑friendly framing that prefixes each record body with
   the 2‑byte marker `0xC3 0x01` followed by the **8‑byte little‑endian CRC‑64‑AVRO Rabin
   fingerprint** of the writer schema, then the Avro binary body. Use `Decoder` with a
   populated `SchemaStore` to resolve fingerprints to full schemas.
   See “Single object encoding” in the Avro 1.11.1 spec.
   <https://avro.apache.org/docs/1.11.1/specification/#single-object-encoding>
 * **Confluent Schema Registry wire format**: A 1‑byte magic `0x00`, a **4‑byte big‑endian**
   schema ID, then the Avro‑encoded body. Use `Decoder` with a `SchemaStore` configured
   for `FingerprintAlgorithm::Id` and entries keyed by `Fingerprint::Id`. See
   Confluent’s “Wire format” documentation.
   <https://docs.confluent.io/platform/current/schema-registry/fundamentals/serdes-develop/index.html#wire-format>
 * **Apicurio Schema Registry wire format**: A 1‑byte magic `0x00`, a **8‑byte big‑endian**
   global schema ID, then the Avro‑encoded body. Use `Decoder` with a `SchemaStore` configured
   for `FingerprintAlgorithm::Id64` and entries keyed by `Fingerprint::Id64`. See
   Apicurio’s “Avro SerDe” documentation.
   <https://www.apicur.io/registry/docs/apicurio-registry/1.3.3.Final/getting-started/assembly-using-kafka-client-serdes.html#registry-serdes-types-avro-registry>

 ## Basic file usage (OCF)

 Use `ReaderBuilder::build` to construct a `Reader` from any `BufRead`. The doctest below
 creates a tiny OCF in memory using `AvroWriter` and then reads it back.

 ```
 use std::io::Cursor;
 use std::sync::Arc;
 use arrow_array::{ArrayRef, Int32Array, RecordBatch};
 use arrow_schema::{DataType, Field, Schema};
 use arrow_avro::writer::AvroWriter;
 use arrow_avro::reader::ReaderBuilder;

 # fn main() -> Result<(), Box<dyn std::error::Error>> {
 // Build a minimal Arrow schema and batch
 let schema = Schema::new(vec![Field::new("id", DataType::Int32, false)]);
 let batch = RecordBatch::try_new(
     Arc::new(schema.clone()),
     vec![Arc::new(Int32Array::from(vec![1, 2, 3])) as ArrayRef],
 )?;

 // Write an Avro OCF to memory
 let buffer: Vec<u8> = Vec::new();
 let mut writer = AvroWriter::new(buffer, schema.clone())?;
 writer.write(&batch)?;
 writer.finish()?;
 let bytes = writer.into_inner();

 // Read it back with ReaderBuilder
 let mut reader = ReaderBuilder::new().build(Cursor::new(bytes))?;
 let out = reader.next().unwrap()?;
 assert_eq!(out.num_rows(), 3);
 # Ok(()) }
 ```

 ## Streaming usage (single‑object / Confluent / Apicurio)

 The `Decoder` lets you integrate Avro decoding with **any** source of bytes by
 periodically calling `Decoder::decode` with new data and calling `Decoder::flush`
 to get a `RecordBatch` once at least one row is complete.

 The example below shows how to decode from an arbitrary stream of `bytes::Bytes` using
 `futures` utilities. Note: this is illustrative and keeps a single in‑memory `Bytes`
 buffer for simplicity—real applications typically maintain a rolling buffer.

 ```
 use bytes::{Buf, Bytes};
 use futures::{Stream, StreamExt};
 use std::task::{Poll, ready};
 use arrow_array::RecordBatch;
 use arrow_avro::{reader::Decoder, errors::AvroError};

 /// Decode a stream of Avro-framed bytes into RecordBatch values.
 fn decode_stream<S: Stream<Item = Bytes> + Unpin>(
     mut decoder: Decoder,
     mut input: S,
 ) -> impl Stream<Item = Result<RecordBatch, AvroError>> {
     let mut buffered = Bytes::new();
     futures::stream::poll_fn(move |cx| {
         loop {
             if buffered.is_empty() {
                 buffered = match ready!(input.poll_next_unpin(cx)) {
                     Some(b) => b,
                     None => break, // EOF
                 };
             }
             // Feed as much as possible
             let decoded = match decoder.decode(buffered.as_ref()) {
                 Ok(n) => n,
                 Err(e) => return Poll::Ready(Some(Err(e))),
             };
             let read = buffered.len();
             buffered.advance(decoded);
             if decoded != read {
                 // decoder made partial progress; request more bytes
                 break
             }
         }
         // Return a batch if one or more rows are complete
         Poll::Ready(decoder.flush().transpose())
     })
 }
 ```

 ### Building and using a `Decoder` for **single‑object encoding** (Rabin fingerprints)

 The doctest below **writes** a single‑object framed record using the Avro writer
 (no manual varints) for the writer schema
 (`{"type":"record","name":"User","fields":[{"name":"id","type":"long"}]}`)
 and then decodes it into a `RecordBatch`.

 ```
 use std::sync::Arc;
 use std::collections::HashMap;
 use arrow_array::{ArrayRef, Int64Array, RecordBatch};
 use arrow_schema::{DataType, Field, Schema};
 use arrow_avro::schema::{AvroSchema, SchemaStore, SCHEMA_METADATA_KEY, FingerprintStrategy};
 use arrow_avro::writer::{WriterBuilder, format::AvroSoeFormat};
 use arrow_avro::reader::ReaderBuilder;

 # fn main() -> Result<(), Box<dyn std::error::Error>> {
 // Register the writer schema (Rabin fingerprint by default).
 let mut store = SchemaStore::new();
 let avro_schema = AvroSchema::new(r#"{"type":"record","name":"User","fields":[
   {"name":"id","type":"long"}]}"#.to_string());
 let _fp = store.register(avro_schema.clone())?;

 // Create a single-object framed record { id: 42 } with the Avro writer.
 let mut md = HashMap::new();
 md.insert(SCHEMA_METADATA_KEY.to_string(), avro_schema.json_string.clone());
 let arrow = Schema::new_with_metadata(vec![Field::new("id", DataType::Int64, false)], md);
 let batch = RecordBatch::try_new(
     Arc::new(arrow.clone()),
     vec![Arc::new(Int64Array::from(vec![42])) as ArrayRef],
 )?;
 let mut w = WriterBuilder::new(arrow)
     .with_fingerprint_strategy(FingerprintStrategy::Rabin) // SOE prefix
     .build::<_, AvroSoeFormat>(Vec::new())?;
 w.write(&batch)?;
 w.finish()?;
 let frame = w.into_inner(); // C3 01 + fp + Avro body

 // Decode with a `Decoder`
 let mut dec = ReaderBuilder::new()
   .with_writer_schema_store(store)
   .with_batch_size(1024)
   .build_decoder()?;

 dec.decode(&frame)?;
 let out = dec.flush()?.expect("one batch");
 assert_eq!(out.num_rows(), 1);
 # Ok(()) }
 ```

 See Avro 1.11.1 “Single object encoding” for details of the 2‑byte marker
 and little‑endian CRC‑64‑AVRO fingerprint:
 <https://avro.apache.org/docs/1.11.1/specification/#single-object-encoding>

 ### Building and using a `Decoder` for **Confluent Schema Registry** framing

 The Confluent wire format is: 1‑byte magic `0x00`, then a **4‑byte big‑endian** schema ID,
 then the Avro body. The doctest below crafts two messages for the same schema ID and
 decodes them into a single `RecordBatch` with two rows.

 ```
 use std::sync::Arc;
 use std::collections::HashMap;
 use arrow_array::{ArrayRef, Int64Array, StringArray, RecordBatch};
 use arrow_schema::{DataType, Field, Schema};
 use arrow_avro::schema::{AvroSchema, SchemaStore, Fingerprint, FingerprintAlgorithm, SCHEMA_METADATA_KEY, FingerprintStrategy};
 use arrow_avro::writer::{WriterBuilder, format::AvroSoeFormat};
 use arrow_avro::reader::ReaderBuilder;

 # fn main() -> Result<(), Box<dyn std::error::Error>> {
 // Set up a store keyed by numeric IDs (Confluent).
 let mut store = SchemaStore::new_with_type(FingerprintAlgorithm::Id);
 let schema_id = 7u32;
 let avro_schema = AvroSchema::new(r#"{"type":"record","name":"User","fields":[
   {"name":"id","type":"long"}, {"name":"name","type":"string"}]}"#.to_string());
 store.set(Fingerprint::Id(schema_id), avro_schema.clone())?;

 // Write two Confluent-framed messages {id:1,name:"a"} and {id:2,name:"b"}.
 fn msg(id: i64, name: &str, schema: &AvroSchema, schema_id: u32) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
     let mut md = HashMap::new();
     md.insert(SCHEMA_METADATA_KEY.to_string(), schema.json_string.clone());
     let arrow = Schema::new_with_metadata(
         vec![Field::new("id", DataType::Int64, false), Field::new("name", DataType::Utf8, false)],
         md,
     );
     let batch = RecordBatch::try_new(
         Arc::new(arrow.clone()),
         vec![
           Arc::new(Int64Array::from(vec![id])) as ArrayRef,
           Arc::new(StringArray::from(vec![name])) as ArrayRef,
         ],
     )?;
     let mut w = WriterBuilder::new(arrow)
         .with_fingerprint_strategy(FingerprintStrategy::Id(schema_id)) // 0x00 + ID + body
         .build::<_, AvroSoeFormat>(Vec::new())?;
     w.write(&batch)?; w.finish()?;
     Ok(w.into_inner())
 }
 let m1 = msg(1, "a", &avro_schema, schema_id)?;
 let m2 = msg(2, "b", &avro_schema, schema_id)?;

 // Decode both into a single batch.
 let mut dec = ReaderBuilder::new()
   .with_writer_schema_store(store)
   .with_batch_size(1024)
   .build_decoder()?;
 dec.decode(&m1)?;
 dec.decode(&m2)?;
 let batch = dec.flush()?.expect("batch");
 assert_eq!(batch.num_rows(), 2);
 # Ok(()) }
 ```

 See Confluent’s “Wire format” notes: magic byte `0x00`, 4‑byte **big‑endian** schema ID,
 then the Avro‑encoded payload.
 <https://docs.confluent.io/platform/current/schema-registry/fundamentals/serdes-develop/index.html#wire-format>

 ## Schema resolution (reader vs. writer schemas)

 Avro supports resolving data written with one schema (“writer”) into another (“reader”)
 using rules like **field aliases**, **default values**, and **numeric promotions**.
 In practice this lets you evolve schemas over time while remaining compatible with old data.

 *Spec background:* See Avro’s **Schema Resolution** (aliases, defaults) and the Confluent
 **Wire format** (magic `0x00` + big‑endian schema id + Avro body).
 <https://avro.apache.org/docs/1.11.1/specification/#schema-resolution>
 <https://docs.confluent.io/platform/current/schema-registry/fundamentals/serdes-develop/index.html#wire-format>

 ### OCF example: rename a field and add a default via a reader schema

 Below we write an OCF with a *writer schema* having fields `id: long`, `name: string`.
 We then read it with a *reader schema* that:
 - **renames** `name` to `full_name` via `aliases`, and
 - **adds** `is_active: boolean` with a **default** value `true`.

 ```
 use std::io::Cursor;
 use std::sync::Arc;
 use arrow_array::{ArrayRef, Int64Array, StringArray, RecordBatch};
 use arrow_schema::{DataType, Field, Schema};
 use arrow_avro::writer::AvroWriter;
 use arrow_avro::reader::ReaderBuilder;
 use arrow_avro::schema::AvroSchema;

 # fn main() -> Result<(), Box<dyn std::error::Error>> {
 // Writer (past version): { id: long, name: string }
 let writer_arrow = Schema::new(vec![
     Field::new("id", DataType::Int64, false),
     Field::new("name", DataType::Utf8, false),
 ]);
 let batch = RecordBatch::try_new(
     Arc::new(writer_arrow.clone()),
     vec![
         Arc::new(Int64Array::from(vec![1, 2])) as ArrayRef,
         Arc::new(StringArray::from(vec!["a", "b"])) as ArrayRef,
     ],
 )?;

 // Write an OCF entirely in memory
 let mut w = AvroWriter::new(Vec::<u8>::new(), writer_arrow)?;
 w.write(&batch)?;
 w.finish()?;
 let bytes = w.into_inner();

 // Reader (current version):
 //  - record name "topLevelRecord" matches the crate's default for OCF
 //  - rename `name` -> `full_name` using aliases (optional)
 let reader_json = r#"
 {
   "type": "record",
   "name": "topLevelRecord",
   "fields": [
     { "name": "id", "type": "long" },
     { "name": "full_name", "type": ["null","string"], "aliases": ["name"], "default": null },
     { "name": "is_active", "type": "boolean", "default": true }
   ]
 }"#;

 let mut reader = ReaderBuilder::new()
   .with_reader_schema(AvroSchema::new(reader_json.to_string()))
   .build(Cursor::new(bytes))?;

 let out = reader.next().unwrap()?;
 assert_eq!(out.num_rows(), 2);
 # Ok(()) }
 ```

 ### Confluent single‑object example: resolve *past* writer versions to the topic’s **current** reader schema

 In this scenario, the **reader schema** is the topic’s *current* schema, while the two
 **writer schemas** registered under Confluent IDs **1** and **2** represent *past versions*.
 The decoder uses the reader schema to resolve both versions.

 ```
 use std::sync::Arc;
 use std::collections::HashMap;
 use arrow_avro::reader::ReaderBuilder;
 use arrow_avro::schema::{
     AvroSchema, Fingerprint, FingerprintAlgorithm, SchemaStore,
     SCHEMA_METADATA_KEY, FingerprintStrategy,
 };
 use arrow_array::{ArrayRef, Int32Array, Int64Array, StringArray, RecordBatch};
 use arrow_schema::{DataType, Field, Schema};

 fn main() -> Result<(), Box<dyn std::error::Error>> {
     // Reader: current topic schema (no reader-added fields)
     //   {"type":"record","name":"User","fields":[
     //     {"name":"id","type":"long"},
     //     {"name":"name","type":"string"}]}
     let reader_schema = AvroSchema::new(
         r#"{"type":"record","name":"User",
             "fields":[{"name":"id","type":"long"},{"name":"name","type":"string"}]}"#
             .to_string(),
     );

     // Register two *writer* schemas under Confluent IDs 0 and 1
     let writer_v0 = AvroSchema::new(
         r#"{"type":"record","name":"User",
             "fields":[{"name":"id","type":"int"},{"name":"name","type":"string"}]}"#
             .to_string(),
     );
     let writer_v1 = AvroSchema::new(
         r#"{"type":"record","name":"User",
             "fields":[{"name":"id","type":"long"},{"name":"name","type":"string"},
                       {"name":"email","type":["null","string"],"default":null}]}"#
             .to_string(),
     );

     let id_v0: u32 = 0;
     let id_v1: u32 = 1;

     let mut store = SchemaStore::new_with_type(FingerprintAlgorithm::Id); // integer IDs
     store.set(Fingerprint::Id(id_v0), writer_v0.clone())?;
     store.set(Fingerprint::Id(id_v1), writer_v1.clone())?;

     // Write two Confluent-framed messages using each writer version
     // frame0: writer v0 body {id:1001_i32, name:"v0-alice"}
     let mut md0 = HashMap::new();
     md0.insert(SCHEMA_METADATA_KEY.to_string(), writer_v0.json_string.clone());
     let arrow0 = Schema::new_with_metadata(
         vec![Field::new("id", DataType::Int32, false),
              Field::new("name", DataType::Utf8, false)], md0);
     let batch0 = RecordBatch::try_new(
         Arc::new(arrow0.clone()),
         vec![Arc::new(Int32Array::from(vec![1001])) as ArrayRef,
              Arc::new(StringArray::from(vec!["v0-alice"])) as ArrayRef])?;
     let mut w0 = arrow_avro::writer::WriterBuilder::new(arrow0)
         .with_fingerprint_strategy(FingerprintStrategy::Id(id_v0))
         .build::<_, arrow_avro::writer::format::AvroSoeFormat>(Vec::new())?;
     w0.write(&batch0)?; w0.finish()?;
     let frame0 = w0.into_inner(); // 0x00 + id_v0 + body

     // frame1: writer v1 body {id:2002_i64, name:"v1-bob", email: Some("bob@example.com")}
     let mut md1 = HashMap::new();
    md1.insert(SCHEMA_METADATA_KEY.to_string(), writer_v1.json_string.clone());
     let arrow1 = Schema::new_with_metadata(
         vec![Field::new("id", DataType::Int64, false),
              Field::new("name", DataType::Utf8, false),
              Field::new("email", DataType::Utf8, true)], md1);
     let batch1 = RecordBatch::try_new(
         Arc::new(arrow1.clone()),
         vec![Arc::new(Int64Array::from(vec![2002])) as ArrayRef,
              Arc::new(StringArray::from(vec!["v1-bob"])) as ArrayRef,
              Arc::new(StringArray::from(vec![Some("bob@example.com")])) as ArrayRef])?;
     let mut w1 = arrow_avro::writer::WriterBuilder::new(arrow1)
         .with_fingerprint_strategy(FingerprintStrategy::Id(id_v1))
         .build::<_, arrow_avro::writer::format::AvroSoeFormat>(Vec::new())?;
     w1.write(&batch1)?; w1.finish()?;
     let frame1 = w1.into_inner(); // 0x00 + id_v1 + body

     // Build a streaming Decoder that understands Confluent framing
     let mut decoder = ReaderBuilder::new()
         .with_reader_schema(reader_schema)
         .with_writer_schema_store(store)
         .with_batch_size(8) // small demo batches
         .build_decoder()?;

     // Decode each whole frame, then drain completed rows with flush()
     let mut total_rows = 0usize;

     let consumed0 = decoder.decode(&frame0)?;
     assert_eq!(consumed0, frame0.len(), "decoder must consume the whole frame");
     while let Some(batch) = decoder.flush()? { total_rows += batch.num_rows(); }

     let consumed1 = decoder.decode(&frame1)?;
     assert_eq!(consumed1, frame1.len(), "decoder must consume the whole frame");
     while let Some(batch) = decoder.flush()? { total_rows += batch.num_rows(); }

     // We sent 2 records so we should get 2 rows (possibly one per flush)
     assert_eq!(total_rows, 2);
     Ok(())
 }
 ```

 ## Schema evolution and batch boundaries

 `Decoder` supports mid‑stream schema changes when the input framing carries a schema
 fingerprint (single‑object or Confluent). When a new fingerprint is observed:

 * If the current `RecordBatch` is **empty**, the decoder switches to the new schema
   immediately.
 * If not, the decoder finishes the current batch first and only then switches.

 Consequently, the schema of batches produced by `Decoder::flush` may change over time,
 and `Decoder` intentionally does **not** implement `RecordBatchReader`. In contrast,
 `Reader` (OCF) has a single writer schema for the entire file and therefore implements
 `RecordBatchReader`.

 ## Performance & memory

 * `batch_size` controls the maximum number of rows per `RecordBatch`. Larger batches
   amortize per‑batch overhead; smaller batches reduce peak memory usage and latency.
 * When `utf8_view` is enabled, string columns use Arrow’s `StringViewArray`, which can
   reduce allocations for short strings.
 * For OCF, blocks may be compressed; `Reader` will decompress using the codec specified
   in the file header and feed uncompressed bytes to the row `Decoder`.

 ## Error handling

 * Incomplete inputs return parse errors with "Unexpected EOF"; callers typically provide
   more bytes and try again.
 * If a fingerprint is unknown to the provided `SchemaStore`, decoding fails with a
   descriptive error. Populate the store up front to avoid this.

 ---

Unresolved upstream links (retained, not inferred): ``Iterator<Item = Result<RecordBatch, ArrowError>>``.
