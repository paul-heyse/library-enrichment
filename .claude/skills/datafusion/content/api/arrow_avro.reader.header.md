# `arrow_avro::reader::header`

Crate `arrow-avro` · 3 public items · structured records in [`model/arrow_avro.reader.header.json`](../model/arrow_avro.reader.header.json)

## read_header_info

`function` · `arrow_avro::reader::header::read_header_info`

Also reachable as `arrow_avro::reader::read_header_info`

```rust
fn read_header_info<R: BufRead>(reader: R) -> Result<HeaderInfo, errors::AvroError>
```

Reads the Avro file header (magic, metadata, sync marker) from `reader`.

On success, returns the parsed [`HeaderInfo`] containing the header and its length in bytes.

---

## Header

`struct` · `arrow_avro::reader::header::Header`

```rust
struct Header
```

A decoded header for an [Object Container File](https://avro.apache.org/docs/1.11.1/specification/#object-container-files)

---

## HeaderInfo

`struct` · `arrow_avro::reader::header::HeaderInfo`

Also reachable as `arrow_avro::reader::HeaderInfo`

```rust
struct HeaderInfo
```

**Derives**: Clone

**Methods** (4)

```rust
fn compression(&self) -> Result<Option<CompressionCodec>, AvroError>
fn header_len(&self) -> u64
fn sync(&self) -> [u8; 16]
fn writer_schema(&self) -> Result<AvroSchema, AvroError>
```

Header information for an Avro OCF file.

The header can be parsed once and shared to construct multiple readers
for the same file, and so this struct is designed to be cheaply clonable.

---
