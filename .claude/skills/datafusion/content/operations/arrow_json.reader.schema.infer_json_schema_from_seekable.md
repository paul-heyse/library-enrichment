# `arrow_json::reader::schema::infer_json_schema_from_seekable`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_json.reader.schema.infer_json_schema_from_seekable.json).

<a id="op-292ac212ae619e2f78b217b0"></a>
## infer_json_schema_from_seekable

`function` · `arrow_json::reader::schema::infer_json_schema_from_seekable` · arrow-json 59.3.0

```rust
fn infer_json_schema_from_seekable<R: BufRead + Seek>(reader: R, max_read_records: Option<usize>) -> Result<(arrow_schema::Schema, usize), arrow_schema::ArrowError>
```

Source: `src/reader/schema.rs:155`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

Infer the fields of a JSON file by reading the first n records of the file, with
`max_read_records` controlling the maximum number of records to read.

If `max_read_records` is not set, the whole file is read to infer its field types.

Returns inferred schema and number of records read.

Contrary to [`infer_json_schema`](../operations/arrow_json.reader.schema.infer_json_schema.md#op-3dc255502e1005cc9fe11e85), this function will seek back to the start of the `reader`.
That way, the `reader` can be used immediately afterwards to create a [`Reader`].

# Examples
```
use std::fs::File;
use std::io::BufReader;
use arrow_json::reader::infer_json_schema_from_seekable;

let file = File::open("test/data/mixed_arrays.json").unwrap();
// file's cursor's offset at 0
let mut reader = BufReader::new(file);
let inferred_schema = infer_json_schema_from_seekable(&mut reader, None).unwrap();
// file's cursor's offset automatically set at 0
```

[`Reader`]: super::Reader
