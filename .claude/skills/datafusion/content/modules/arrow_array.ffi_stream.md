# `arrow_array::ffi_stream`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.ffi_stream.json).

<a id="op-e9cf7aa86cbf6cb56dac636f"></a>
## ffi_stream

`module` · `arrow_array::ffi_stream` · arrow-array 59.3.0

```rust
mod ffi_stream
```

Source: `src/ffi_stream.rs:18`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Contains declarations to bind to the [C Stream Interface](https://arrow.apache.org/docs/format/CStreamInterface.html).

This module has two main interfaces:
One interface maps C ABI to native Rust types, i.e. convert c-pointers, c_char, to native rust.
This is handled by [FFI_ArrowArrayStream](../operations/arrow_array.ffi_stream.FFI_ArrowArrayStream.md#op-72b1ea752d3976ebd8228bf1).

The second interface is used to import `FFI_ArrowArrayStream` as Rust implementation `RecordBatch` reader.
This is handled by `ArrowArrayStreamReader`.

```ignore
# use std::fs::File;
# use std::sync::Arc;
# use arrow::error::Result;
# use arrow::ffi_stream::{export_reader_into_raw, ArrowArrayStreamReader, FFI_ArrowArrayStream};
# use arrow::ipc::reader::FileReader;
# use arrow::record_batch::RecordBatchReader;
# fn main() -> Result<()> {
// create an record batch reader natively
let file = File::open("arrow_file").unwrap();
let reader = Box::new(FileReader::try_new(file).unwrap());

// export it
let mut stream = FFI_ArrowArrayStream::empty();
unsafe { export_reader_into_raw(reader, &mut stream) };

// consumed and used by something else...

// import it
let stream_reader = unsafe { ArrowArrayStreamReader::from_raw(&mut stream).unwrap() };
let imported_schema = stream_reader.schema();

let mut produced_batches = vec![];
for batch in stream_reader {
     produced_batches.push(batch.unwrap());
}
Ok(())
}
```
