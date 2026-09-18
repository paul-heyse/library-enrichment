# `arrow_json::reader::Decoder`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_json.reader.Decoder.json).

<a id="op-9a10b6ca230ecd7bed81532a"></a>
## Decoder

`struct` · `arrow_json::reader::Decoder` · arrow-json 59.3.0

```rust
struct Decoder
```

Source: `src/reader/mod.rs:446`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

A low-level interface for reading JSON data from a byte stream

See [`Reader`](../operations/arrow_json.reader.Reader.md#op-065551256893ce5f2d76aab8) for a higher-level interface for interface with [`BufRead`]

The push-based interface facilitates integration with sources that yield arbitrarily
delimited bytes ranges, such as [`BufRead`], or a chunked byte stream received from
object storage

```
# use std::io::BufRead;
# use arrow_array::RecordBatch;
# use arrow_json::reader::{Decoder, ReaderBuilder};
# use arrow_schema::{ArrowError, SchemaRef};
#
fn read_from_json<R: BufRead>(
    mut reader: R,
    schema: SchemaRef,
) -> Result<impl Iterator<Item = Result<RecordBatch, ArrowError>>, ArrowError> {
    let mut decoder = ReaderBuilder::new(schema).build_decoder()?;
    let mut next = move || {
        loop {
            // Decoder is agnostic that buf doesn't contain whole records
            let buf = reader.fill_buf()?;
            if buf.is_empty() {
                break; // Input exhausted
            }
            let read = buf.len();
            let decoded = decoder.decode(buf)?;

            // Consume the number of bytes read
            reader.consume(decoded);
            if decoded != read {
                break; // Read batch size
            }
        }
        decoder.flush()
    };
    Ok(std::iter::from_fn(move || next().transpose()))
}
```

Unresolved upstream links (retained, not inferred): ``BufRead``.

<a id="op-432f1d1ec2cb18d7ee8c3aae"></a>
## decode

`function` · `arrow_json::reader::Decoder::decode` · arrow-json 59.3.0

```rust
fn decode(&mut self, buf: &[u8]) -> Result<usize, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_json::reader::Decoder", "path": "Decoder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [463, 1], "end": [705, 2], "filename": "src/reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader/mod.rs:472`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

Read JSON objects from `buf`, returning the number of bytes read

This method returns once `batch_size` objects have been parsed since the
last call to [`Self::flush`](../operations/arrow_json.reader.Decoder.md#op-f12a99309c412e731981123f), or `buf` is exhausted. Any remaining bytes
should be included in the next call to [`Self::decode`](../operations/arrow_json.reader.Decoder.md#op-432f1d1ec2cb18d7ee8c3aae)

There is no requirement that `buf` contains a whole number of records, facilitating
integration with arbitrary byte streams, such as those yielded by [`BufRead`]

Unresolved upstream links (retained, not inferred): ``BufRead``.

<a id="op-f12a99309c412e731981123f"></a>
## flush

`function` · `arrow_json::reader::Decoder::flush` · arrow-json 59.3.0

```rust
fn flush(&mut self) -> Result<Option<RecordBatch>, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_json::reader::Decoder", "path": "Decoder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [463, 1], "end": [705, 2], "filename": "src/reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader/mod.rs:677`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

Flushes the currently buffered data to a [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34)

Returns `Ok(None)` if no buffered data, i.e. [`Self::is_empty`](../operations/arrow_json.reader.Decoder.md#op-24c7a5aa3cf2c1dd2ba08219) is true.

Note: This will return an error if called part way through decoding a record,
i.e. [`Self::has_partial_record`](../operations/arrow_json.reader.Decoder.md#op-362bbde4ed37f541c6dfa5a3) is true.

<a id="op-e72ea76d4e47feb63e5383ff"></a>
## fmt

`function` · `arrow_json::reader::Decoder::fmt` · arrow-json 59.3.0

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_json::reader::Decoder", "path": "Decoder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [454, 1], "end": [461, 2], "filename": "src/reader/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/reader/mod.rs:455`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-362bbde4ed37f541c6dfa5a3"></a>
## has_partial_record

`function` · `arrow_json::reader::Decoder::has_partial_record` · arrow-json 59.3.0

```rust
fn has_partial_record(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_json::reader::Decoder", "path": "Decoder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [463, 1], "end": [705, 2], "filename": "src/reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader/mod.rs:657`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

True if the decoder is currently part way through decoding a record.

<a id="op-24c7a5aa3cf2c1dd2ba08219"></a>
## is_empty

`function` · `arrow_json::reader::Decoder::is_empty` · arrow-json 59.3.0

```rust
fn is_empty(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_json::reader::Decoder", "path": "Decoder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [463, 1], "end": [705, 2], "filename": "src/reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader/mod.rs:667`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

True if there are no records to flush, i.e. [`Self::len`](../operations/arrow_json.reader.Decoder.md#op-ba2932d7cbc6dcd7de2eeff8) is zero.

<a id="op-ba2932d7cbc6dcd7de2eeff8"></a>
## len

`function` · `arrow_json::reader::Decoder::len` · arrow-json 59.3.0

```rust
fn len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_json::reader::Decoder", "path": "Decoder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [463, 1], "end": [705, 2], "filename": "src/reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader/mod.rs:662`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

The number of unflushed records, including the partially decoded record (if any).

<a id="op-8556fc8721e544d5e48c86bb"></a>
## serialize

`function` · `arrow_json::reader::Decoder::serialize` · arrow-json 59.3.0

```rust
fn serialize<S: Serialize>(&mut self, rows: &[S]) -> Result<(), ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_json::reader::Decoder", "path": "Decoder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [463, 1], "end": [705, 2], "filename": "src/reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader/mod.rs:652`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

Serialize `rows` to this [`Decoder`](../operations/arrow_json.reader.Decoder.md#op-9a10b6ca230ecd7bed81532a)

This provides a simple way to convert [serde]-compatible datastructures into arrow
[`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34).

Custom conversion logic as described in [arrow_array::builder](../modules/arrow_array.builder.md#op-aacfbf3413ace5e290c06216) will likely outperform this,
especially where the schema is known at compile-time, however, this provides a mechanism
to get something up and running quickly

It can be used with [`serde_json::Value`]

```
# use std::sync::Arc;
# use serde_json::{Value, json};
# use arrow_array::cast::AsArray;
# use arrow_array::types::Float32Type;
# use arrow_json::ReaderBuilder;
# use arrow_schema::{DataType, Field, Schema};
let json = vec![json!({"float": 2.3}), json!({"float": 5.7})];

let schema = Schema::new(vec![Field::new("float", DataType::Float32, true)]);
let mut decoder = ReaderBuilder::new(Arc::new(schema)).build_decoder().unwrap();

decoder.serialize(&json).unwrap();
let batch = decoder.flush().unwrap().unwrap();
assert_eq!(batch.num_rows(), 2);
assert_eq!(batch.num_columns(), 1);
let values = batch.column(0).as_primitive::<Float32Type>().values();
assert_eq!(values, &[2.3, 5.7])
```

Or with arbitrary [`Serialize`] types

```
# use std::sync::Arc;
# use arrow_json::ReaderBuilder;
# use arrow_schema::{DataType, Field, Schema};
# use serde::Serialize;
# use arrow_array::cast::AsArray;
# use arrow_array::types::{Float32Type, Int32Type};
#
#[derive(Serialize)]
struct MyStruct {
    int32: i32,
    float: f32,
}

let schema = Schema::new(vec![
    Field::new("int32", DataType::Int32, false),
    Field::new("float", DataType::Float32, false),
]);

let rows = vec![
    MyStruct{ int32: 0, float: 3. },
    MyStruct{ int32: 4, float: 67.53 },
];

let mut decoder = ReaderBuilder::new(Arc::new(schema)).build_decoder().unwrap();
decoder.serialize(&rows).unwrap();

let batch = decoder.flush().unwrap().unwrap();

// Expect batch containing two columns
let int32 = batch.column(0).as_primitive::<Int32Type>();
assert_eq!(int32.values(), &[0, 4]);

let float = batch.column(1).as_primitive::<Float32Type>();
assert_eq!(float.values(), &[3., 67.53]);
```

Or even complex nested types

```
# use std::collections::BTreeMap;
# use std::sync::Arc;
# use arrow_array::StructArray;
# use arrow_cast::display::{ArrayFormatter, FormatOptions};
# use arrow_json::ReaderBuilder;
# use arrow_schema::{DataType, Field, Fields, Schema};
# use serde::Serialize;
#
#[derive(Serialize)]
struct MyStruct {
    int32: i32,
    list: Vec<f64>,
    nested: Vec<Option<Nested>>,
}

impl MyStruct {
    /// Returns the [`Fields`] for [`MyStruct`]
    fn fields() -> Fields {
        let nested = DataType::Struct(Nested::fields());
        Fields::from([
            Arc::new(Field::new("int32", DataType::Int32, false)),
            Arc::new(Field::new_list(
                "list",
                Field::new("element", DataType::Float64, false),
                false,
            )),
            Arc::new(Field::new_list(
                "nested",
                Field::new("element", nested, true),
                true,
            )),
        ])
    }
}

#[derive(Serialize)]
struct Nested {
    map: BTreeMap<String, Vec<String>>
}

impl Nested {
    /// Returns the [`Fields`] for [`Nested`]
    fn fields() -> Fields {
        let element = Field::new("element", DataType::Utf8, false);
        Fields::from([
            Arc::new(Field::new_map(
                "map",
                "entries",
                Field::new("key", DataType::Utf8, false),
                Field::new_list("value", element, false),
                false, // sorted
                false, // nullable
            ))
        ])
    }
}

let data = vec![
    MyStruct {
        int32: 34,
        list: vec![1., 2., 34.],
        nested: vec![
            None,
            Some(Nested {
                map: vec![
                    ("key1".to_string(), vec!["foo".to_string(), "bar".to_string()]),
                    ("key2".to_string(), vec!["baz".to_string()])
                ].into_iter().collect()
            })
        ]
    },
    MyStruct {
        int32: 56,
        list: vec![],
        nested: vec![]
    },
    MyStruct {
        int32: 24,
        list: vec![-1., 245.],
        nested: vec![None]
    }
];

let schema = Schema::new(MyStruct::fields());
let mut decoder = ReaderBuilder::new(Arc::new(schema)).build_decoder().unwrap();
decoder.serialize(&data).unwrap();
let batch = decoder.flush().unwrap().unwrap();
assert_eq!(batch.num_rows(), 3);
assert_eq!(batch.num_columns(), 3);

// Convert to StructArray to format
let s = StructArray::from(batch);
let options = FormatOptions::default().with_null("null");
let formatter = ArrayFormatter::try_new(&s, &options).unwrap();

assert_eq!(&formatter.value(0).to_string(), "{int32: 34, list: [1.0, 2.0, 34.0], nested: [null, {map: {key1: [foo, bar], key2: [baz]}}]}");
assert_eq!(&formatter.value(1).to_string(), "{int32: 56, list: [], nested: []}");
assert_eq!(&formatter.value(2).to_string(), "{int32: 24, list: [-1.0, 245.0], nested: [null]}");
```

Note: this ignores any batch size setting, and always decodes all rows

[serde]: https://docs.rs/serde/latest/serde/

Unresolved upstream links (retained, not inferred): ``Serialize``, ``serde_json::Value``.
