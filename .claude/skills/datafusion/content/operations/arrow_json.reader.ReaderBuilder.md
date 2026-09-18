# `arrow_json::reader::ReaderBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_json.reader.ReaderBuilder.json).

<a id="op-9f26fb6438509b4a7359ec7e"></a>
## ReaderBuilder

`struct` · `arrow_json::reader::ReaderBuilder` · arrow-json 59.3.0

```rust
struct ReaderBuilder
```

Source: `src/reader/mod.rs:188`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

A builder for [`Reader`](../operations/arrow_json.reader.Reader.md#op-065551256893ce5f2d76aab8) and [`Decoder`](../operations/arrow_json.reader.Decoder.md#op-9a10b6ca230ecd7bed81532a)

<a id="op-fc17db480f353492ff447fe0"></a>
## build

`function` · `arrow_json::reader::ReaderBuilder::build` · arrow-json 59.3.0

```rust
fn build<R: BufRead>(self, reader: R) -> Result<Reader<R>, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_json::reader::ReaderBuilder", "path": "ReaderBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [199, 1], "end": [354, 2], "filename": "src/reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader/mod.rs:318`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

Create a [`Reader`](../operations/arrow_json.reader.Reader.md#op-065551256893ce5f2d76aab8) with the provided [`BufRead`]

Unresolved upstream links (retained, not inferred): ``BufRead``.

<a id="op-3a0f4593dd9fc592734185a4"></a>
## build_decoder

`function` · `arrow_json::reader::ReaderBuilder::build_decoder` · arrow-json 59.3.0

```rust
fn build_decoder(self) -> Result<Decoder, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_json::reader::ReaderBuilder", "path": "ReaderBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [199, 1], "end": [354, 2], "filename": "src/reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader/mod.rs:326`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

Create a [`Decoder`](../operations/arrow_json.reader.Decoder.md#op-9a10b6ca230ecd7bed81532a)

<a id="op-fb0aaabb174231be90a3eb48"></a>
## new

`function` · `arrow_json::reader::ReaderBuilder::new` · arrow-json 59.3.0

```rust
fn new(schema: SchemaRef) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_json::reader::ReaderBuilder", "path": "ReaderBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [199, 1], "end": [354, 2], "filename": "src/reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader/mod.rs:208`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

Create a new [`ReaderBuilder`](../operations/arrow_json.reader.ReaderBuilder.md#op-9f26fb6438509b4a7359ec7e) with the provided [`SchemaRef`](../operations/arrow_schema.schema.SchemaRef.md#op-e48a1bb89d62307cfab4093a)

This could be obtained using [`infer_json_schema`] if not known

Any columns not present in `schema` will be ignored, unless `strict_mode` is set to true.
In this case, an error is returned when a column is missing from `schema`.

[`infer_json_schema`]: crate::reader::infer_json_schema

<a id="op-b47ab921d3316e0c26ba8119"></a>
## new_with_field

`function` · `arrow_json::reader::ReaderBuilder::new_with_field` · arrow-json 59.3.0

```rust
fn new_with_field(field: impl Into<FieldRef>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_json::reader::ReaderBuilder", "path": "ReaderBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [199, 1], "end": [354, 2], "filename": "src/reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader/mod.rs:250`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

Create a new [`ReaderBuilder`](../operations/arrow_json.reader.ReaderBuilder.md#op-9f26fb6438509b4a7359ec7e) that will parse JSON values of `field.data_type()`

Unlike [`ReaderBuilder::new`](../operations/arrow_json.reader.ReaderBuilder.md#op-fb0aaabb174231be90a3eb48) this does not require the root of the JSON data
to be an object, i.e. `{..}`, allowing for parsing of any valid JSON value(s)

```
# use std::sync::Arc;
# use arrow_array::cast::AsArray;
# use arrow_array::types::Int32Type;
# use arrow_json::ReaderBuilder;
# use arrow_schema::{DataType, Field};
// Root of JSON schema is a numeric type
let data = "1\n2\n3\n";
let field = Arc::new(Field::new("int", DataType::Int32, true));
let mut reader = ReaderBuilder::new_with_field(field.clone()).build(data.as_bytes()).unwrap();
let b = reader.next().unwrap().unwrap();
let values = b.column(0).as_primitive::<Int32Type>().values();
assert_eq!(values, &[1, 2, 3]);

// Root of JSON schema is a list type
let data = "[1, 2, 3, 4, 5, 6, 7]\n[1, 2, 3]";
let field = Field::new_list("int", field.clone(), true);
let mut reader = ReaderBuilder::new_with_field(field).build(data.as_bytes()).unwrap();
let b = reader.next().unwrap().unwrap();
let list = b.column(0).as_list::<i32>();

assert_eq!(list.offsets().as_ref(), &[0, 7, 10]);
let list_values = list.values().as_primitive::<Int32Type>();
assert_eq!(list_values.values(), &[1, 2, 3, 4, 5, 6, 7, 1, 2, 3]);
```

<a id="op-8c1c3662c79d81c2a3f8ac88"></a>
## with_batch_size

`function` · `arrow_json::reader::ReaderBuilder::with_batch_size` · arrow-json 59.3.0

```rust
fn with_batch_size(self, batch_size: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_json::reader::ReaderBuilder", "path": "ReaderBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [199, 1], "end": [354, 2], "filename": "src/reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader/mod.rs:263`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

Sets the batch size in rows to read

<a id="op-a19ea9e8436490260b82f4a1"></a>
## with_coerce_primitive

`function` · `arrow_json::reader::ReaderBuilder::with_coerce_primitive` · arrow-json 59.3.0

```rust
fn with_coerce_primitive(self, coerce_primitive: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_json::reader::ReaderBuilder", "path": "ReaderBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [199, 1], "end": [354, 2], "filename": "src/reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader/mod.rs:269`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

Sets if the decoder should coerce primitive values (bool and number) into string
when the Schema's column is Utf8 or LargeUtf8.

<a id="op-2d82db05923f8cac2e552db4"></a>
## with_ignore_type_conflicts

`function` · `arrow_json::reader::ReaderBuilder::with_ignore_type_conflicts` · arrow-json 59.3.0

```rust
fn with_ignore_type_conflicts(self, ignore_type_conflicts: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_json::reader::ReaderBuilder", "path": "ReaderBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [199, 1], "end": [354, 2], "filename": "src/reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader/mod.rs:310`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

Sets whether the decoder should produce NULL instead of returning an error if it encounters
value that can not be parsed into the specified column type.

For example, if the type is declared to be a nullable array of `DataType::Int32` but the
reader encounters a string value `"foo"` and the value `ignore_type_conflicts` is:

* `false` (the default): The reader will return an error.

* `true`: The reader will fill in NULL value for that array element.

NOTE: An inferred NULL due to a type conflict will still produce parsing errors for
non-nullable fields, the same as any other NULL or missing value.

<a id="op-7ce37a6c7ce7c1ecca2d9fd5"></a>
## with_strict_mode

`function` · `arrow_json::reader::ReaderBuilder::with_strict_mode` · arrow-json 59.3.0

```rust
fn with_strict_mode(self, strict_mode: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_json::reader::ReaderBuilder", "path": "ReaderBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [199, 1], "end": [354, 2], "filename": "src/reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader/mod.rs:281`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

Sets if the decoder should return an error if it encounters a column not
present in `schema`. If `struct_mode` is `ListOnly` the value of
`strict_mode` is effectively `true`. It is required for all fields of
the struct to be in the list: without field names, there is no way to
determine which field is missing.

<a id="op-462f52daa45e5e6184f4e184"></a>
## with_struct_mode

`function` · `arrow_json::reader::ReaderBuilder::with_struct_mode` · arrow-json 59.3.0

```rust
fn with_struct_mode(self, struct_mode: StructMode) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_json::reader::ReaderBuilder", "path": "ReaderBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [199, 1], "end": [354, 2], "filename": "src/reader/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader/mod.rs:291`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

Set the [`StructMode`](../operations/arrow_json.StructMode.md#op-a862c9daf0b023e9659efe1e) for the reader, which determines whether structs
can be decoded from JSON as objects or lists. For more details refer to
the enum documentation. Default is to use `ObjectOnly`.
