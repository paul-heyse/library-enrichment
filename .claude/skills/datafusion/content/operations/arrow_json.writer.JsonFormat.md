# `arrow_json::writer::JsonFormat`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_json.writer.JsonFormat.json).

<a id="op-3a011970d5da2ba6a8b56ed1"></a>
## JsonFormat

`trait` · `arrow_json::writer::JsonFormat` · arrow-json 59.3.0

```rust
trait JsonFormat: Debug + Default
```

Source: `src/writer/mod.rs:119`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

This trait defines how to format a sequence of JSON objects to a
byte stream.

<a id="op-15026ee32c79db8f8d10e479"></a>
## end_row

`function` · `arrow_json::writer::JsonFormat::end_row` · arrow-json 59.3.0

```rust
fn end_row<W: Write>(&self, _writer: &mut W) -> Result<(), ArrowError>
```

Source: `src/writer/mod.rs:134`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

write any bytes needed for the end of each row

<a id="op-edc7ddc5f94e7ad519b647b5"></a>
## end_stream

`function` · `arrow_json::writer::JsonFormat::end_stream` · arrow-json 59.3.0

```rust
fn end_stream<W: Write>(&self, _writer: &mut W) -> Result<(), ArrowError>
```

Source: `src/writer/mod.rs:139`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

write any bytes needed for the start of each row

<a id="op-0a4b20531a242e60b0d1e6f5"></a>
## start_row

`function` · `arrow_json::writer::JsonFormat::start_row` · arrow-json 59.3.0

```rust
fn start_row<W: Write>(&self, _writer: &mut W, _is_first_row: bool) -> Result<(), ArrowError>
```

Source: `src/writer/mod.rs:128`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

write any bytes needed for the start of each row

<a id="op-2399bb5b7d95b582cfad3c12"></a>
## start_stream

`function` · `arrow_json::writer::JsonFormat::start_stream` · arrow-json 59.3.0

```rust
fn start_stream<W: Write>(&self, _writer: &mut W) -> Result<(), ArrowError>
```

Source: `src/writer/mod.rs:122`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

write any bytes needed at the start of the file to the writer
