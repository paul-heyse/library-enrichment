# `arrow_json::reader::schema`

Crate `arrow-json` · 3 public items · structured records in [`model/arrow_json.reader.schema.json`](../model/arrow_json.reader.schema.json)

## infer_json_schema

`function` · `arrow_json::reader::schema::infer_json_schema`

```rust
fn infer_json_schema<R: BufRead>(reader: R, max_read_records: Option<usize>) -> Result<(arrow_schema::Schema, usize), arrow_schema::ArrowError>
```

[Full member, field, variant and typed contracts](../operations/arrow_json.reader.schema.infer_json_schema.md).


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

---

## infer_json_schema_from_iterator

`function` · `arrow_json::reader::schema::infer_json_schema_from_iterator`

```rust
fn infer_json_schema_from_iterator<I, V>(value_iter: I) -> Result<arrow_schema::Schema, arrow_schema::ArrowError> where I: Iterator<Item = Result<V, arrow_schema::ArrowError>>, V: Borrow<serde_json::Value>
```

[Full member, field, variant and typed contracts](../operations/arrow_json.reader.schema.infer_json_schema_from_iterator.md).


Infer the fields of a JSON file by reading all items from the JSON Value Iterator.

The following type coercion logic is implemented:
* `Int64` and `Float64` are converted to `Float64`
* Lists and scalars are coerced to a list of a compatible scalar
* All other cases are coerced to `Utf8` (String)

Note that the above coercion logic is different from what Spark has, where it would default to
String type in case of List and Scalar values appeared in the same field.

The reason we diverge here is because we don't have utilities to deal with JSON data once it's
interpreted as Strings. We should match Spark's behavior once we added more JSON parsing
kernels in the future.

---

## infer_json_schema_from_seekable

`function` · `arrow_json::reader::schema::infer_json_schema_from_seekable`

```rust
fn infer_json_schema_from_seekable<R: BufRead + Seek>(reader: R, max_read_records: Option<usize>) -> Result<(arrow_schema::Schema, usize), arrow_schema::ArrowError>
```

[Full member, field, variant and typed contracts](../operations/arrow_json.reader.schema.infer_json_schema_from_seekable.md).


Infer the fields of a JSON file by reading the first n records of the file, with
`max_read_records` controlling the maximum number of records to read.

If `max_read_records` is not set, the whole file is read to infer its field types.

Returns inferred schema and number of records read.

Contrary to [`infer_json_schema`], this function will seek back to the start of the `reader`.
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

---
