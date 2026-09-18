# `arrow_json`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_json.json).

<a id="op-9cfc2f2ebe48fa33164bfd99"></a>
## arrow_json

`module` · `arrow_json` · arrow-json 59.3.0

```rust
mod arrow_json
```

Source: `src/lib.rs:18`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

Transfer data between the Arrow memory format and JSON line-delimited records.

See the module level documentation for the
[`reader`](../modules/arrow_json.reader.md#op-53dfc6a8d86f7f7c1bf482e7) and [`writer`](../modules/arrow_json.writer.md#op-3211eef7ff6c9c882d353606) for usage examples.

# Binary Data uses `Base16` Encoding

As per [RFC7159] JSON cannot encode arbitrary binary data. This crate works around that
limitation by encoding/decoding binary data as a [hexadecimal] string (i.e.
[`Base16` encoding]).

Note that `Base16` only has 50% space efficiency (i.e., the encoded data is twice as large
as the original). If that is an issue, we recommend to convert binary data to/from a different
encoding format such as `Base64` instead. See the following example for details.

## `Base64` Encoding Example

[`Base64`] is a common [binary-to-text encoding] scheme with a space efficiency of 75%. The
following example shows how to use the [`arrow_cast`](../modules/arrow_cast.md#op-c44ca9952f804b390ba02fcf) crate to encode binary data to `Base64`
before converting it to JSON and how to decode it back.

```
# use std::io::Cursor;
# use std::sync::Arc;
# use arrow_array::{BinaryArray, RecordBatch, StringArray};
# use arrow_array::cast::AsArray;
use arrow_cast::base64::{b64_decode, b64_encode, BASE64_STANDARD};
# use arrow_json::{LineDelimitedWriter, ReaderBuilder};
#
// The data we want to write
let input = BinaryArray::from(vec![b"\xDE\x00\xFF".as_ref()]);

// Base64 encode it to a string
let encoded: StringArray = b64_encode(&BASE64_STANDARD, &input);

// Write the StringArray to JSON
let batch = RecordBatch::try_from_iter([("col", Arc::new(encoded) as _)]).unwrap();
let mut buf = Vec::with_capacity(1024);
let mut writer = LineDelimitedWriter::new(&mut buf);
writer.write(&batch).unwrap();
writer.finish().unwrap();

// Read the JSON data
let cursor = Cursor::new(buf);
let mut reader = ReaderBuilder::new(batch.schema()).build(cursor).unwrap();
let batch = reader.next().unwrap().unwrap();

// Reverse the base64 encoding
let col: BinaryArray = batch.column(0).as_string::<i32>().clone().into();
let output = b64_decode(&BASE64_STANDARD, &col).unwrap();

assert_eq!(input, output);
```

[RFC7159]: https://datatracker.ietf.org/doc/html/rfc7159#section-8.1
[binary-to-text encoding]: https://en.wikipedia.org/wiki/Binary-to-text_encoding
[hexadecimal]: https://en.wikipedia.org/wiki/Hexadecimal
[`Base16` encoding]: https://en.wikipedia.org/wiki/Base16#Base16
[`Base64`]: https://en.wikipedia.org/wiki/Base64
