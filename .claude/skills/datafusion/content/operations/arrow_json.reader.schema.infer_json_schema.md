# `arrow_json::reader::schema::infer_json_schema`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_json.reader.schema.infer_json_schema.json).

<a id="op-3dc255502e1005cc9fe11e85"></a>
## infer_json_schema

`function` · `arrow_json::reader::schema::infer_json_schema` · arrow-json 59.3.0

```rust
fn infer_json_schema<R: BufRead>(reader: R, max_read_records: Option<usize>) -> Result<(arrow_schema::Schema, usize), arrow_schema::ArrowError>
```

Source: `src/reader/schema.rs:203`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

Infer the fields of a JSON file by reading the first n records of the buffer, with
`max_read_records` controlling the maximum number of records to read.

If `max_read_records` is not set, the whole file is read to infer its field types.

Returns inferred schema and number of records read.

This function will not seek back to the start of the `reader`. The user has to manage the
original file's cursor. This function is useful when the `reader`'s cursor is not available
(does not implement [`Seek`]), such is the case for compressed streams decoders.


Note that JSON is not able to represent all Arrow data types exactly. So the inferred schema
might be different from the schema of the original data that was encoded as JSON. For example,
JSON does not have different integer types, so all integers are inferred as `Int64`. Another
example is binary data, which is encoded as a [Base16] string in JSON and therefore inferred
as String type by this function.

[Base16]: https://en.wikipedia.org/wiki/Base16#Base16

# Examples
```
use std::fs::File;
use std::io::{BufReader, SeekFrom, Seek};
use flate2::read::GzDecoder;
use arrow_json::reader::infer_json_schema;

let mut file = File::open("test/data/mixed_arrays.json.gz").unwrap();

// file's cursor's offset at 0
let mut reader = BufReader::new(GzDecoder::new(&file));
let inferred_schema = infer_json_schema(&mut reader, None).unwrap();
// cursor's offset at end of file

// seek back to start so that the original file is usable again
file.seek(SeekFrom::Start(0)).unwrap();
```

Unresolved upstream links (retained, not inferred): ``Seek``.
