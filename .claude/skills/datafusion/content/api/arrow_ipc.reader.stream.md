# `arrow_ipc::reader::stream`

Crate `arrow-ipc` · 1 public items · structured records in [`model/arrow_ipc.reader.stream.json`](../model/arrow_ipc.reader.stream.json)

## StreamDecoder

`struct` · `arrow_ipc::reader::stream::StreamDecoder`

```rust
struct StreamDecoder
```

**Derives**: Debug, Default

**Methods** (6)

```rust
fn decode(&mut self, buffer: &mut Buffer) -> Result<Option<RecordBatch>, ArrowError>
fn finish(&mut self) -> Result<(), ArrowError>
fn new() -> Self
fn schema(&self) -> Option<SchemaRef>
fn with_require_alignment(self, require_alignment: bool) -> Self
unsafe fn with_skip_validation(self, skip_validation: bool) -> Self
```

[Full member, field, variant and typed contracts](../operations/arrow_ipc.reader.stream.StreamDecoder.md).


A low-level interface for reading [`RecordBatch`] data from a stream of bytes

See [StreamReader](crate::reader::StreamReader) for a higher-level interface

---
