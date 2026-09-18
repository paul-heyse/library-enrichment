# `parquet::record::api::Row`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.record.api.Row.json).

<a id="op-8446812bf7ce8e0f4f9bf863"></a>
## Row

`struct` · `parquet::record::api::Row` · parquet 59.3.0

```rust
struct Row
```

Source: `src/record/api.rs:49`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

`Row` represents a nested Parquet record.

<a id="op-0fdcdba6a22e864f29d60faf"></a>
## clone

`function` · `parquet::record::api::Row::clone` · parquet 59.3.0

```rust
fn clone(&self) -> Row
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::record::api::Row", "path": "Row"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 10], "end": [48, 15], "filename": "src/record/api.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/record/api.rs:48`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c7df7d0e4676391e52a9876c"></a>
## eq

`function` · `parquet::record::api::Row::eq` · parquet 59.3.0

```rust
fn eq(&self, other: &Row) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::record::api::Row", "path": "Row"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 24], "end": [48, 33], "filename": "src/record/api.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/record/api.rs:48`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-74d491e7b05ee42682b75446"></a>
## fmt

`function` · `parquet::record::api::Row::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::record::api::Row", "path": "Row"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [308, 1], "end": [321, 2], "filename": "src/record/api.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/record/api.rs:309`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7c6885b8d0e1a3388b1015c1"></a>
## fmt

`function` · `parquet::record::api::Row::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::record::api::Row", "path": "Row"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 17], "end": [48, 22], "filename": "src/record/api.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/record/api.rs:48`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7dd5a6d7ddf1e6ba8cd822a6"></a>
## fmt

`function` · `parquet::record::api::Row::fmt` · parquet 59.3.0

```rust
fn fmt(&self, i: usize) -> &dyn fmt::Display
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::record::api::Row", "path": "Row"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [247, 1], "end": [256, 2], "filename": "src/record/api.rs"}, "trait": {"args": null, "id": "parquet::record::api::RowFormatter", "path": "RowFormatter"}, "trait_path": "parquet::record::api::RowFormatter"}`

Source: `src/record/api.rs:249`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Get Display reference for a given field.

<a id="op-9d52e39b368bb50294967d24"></a>
## get_bool

`function` · `parquet::record::api::Row::get_bool` · parquet 59.3.0

```rust
fn get_bool(&self, i: usize) -> Result<bool>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::record::api::Row", "path": "Row"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [258, 1], "end": [306, 2], "filename": "src/record/api.rs"}, "trait": {"args": null, "id": "parquet::record::api::RowAccessor", "path": "RowAccessor"}, "trait_path": "parquet::record::api::RowAccessor"}`

Source: `src/record/api.rs:267`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a93c105433fae5b5bc2ebaf6"></a>
## get_byte

`function` · `parquet::record::api::Row::get_byte` · parquet 59.3.0

```rust
fn get_byte(&self, i: usize) -> Result<i8>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::record::api::Row", "path": "Row"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [258, 1], "end": [306, 2], "filename": "src/record/api.rs"}, "trait": {"args": null, "id": "parquet::record::api::RowAccessor", "path": "RowAccessor"}, "trait_path": "parquet::record::api::RowAccessor"}`

Source: `src/record/api.rs:269`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2dae0f8f0899f2cdecd34032"></a>
## get_bytes

`function` · `parquet::record::api::Row::get_bytes` · parquet 59.3.0

```rust
fn get_bytes(&self, i: usize) -> Result<&ByteArray>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::record::api::Row", "path": "Row"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [258, 1], "end": [306, 2], "filename": "src/record/api.rs"}, "trait": {"args": null, "id": "parquet::record::api::RowAccessor", "path": "RowAccessor"}, "trait_path": "parquet::record::api::RowAccessor"}`

Source: `src/record/api.rs:299`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-36677dcf416b58b3ede7f5c3"></a>
## get_column_iter

`function` · `parquet::record::api::Row::get_column_iter` · parquet 59.3.0

```rust
fn get_column_iter(&self) -> RowColumnIter<'_>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::record::api::Row", "path": "Row"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [54, 1], "end": [119, 2], "filename": "src/record/api.rs"}, "trait": null, "trait_path": null}`

Source: `src/record/api.rs:101`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Get an iterator to go through all columns in the row.

# Example

```no_run
use std::fs::File;
use parquet::record::Row;
use parquet::file::reader::{FileReader, SerializedFileReader};

let file = File::open("/path/to/file").unwrap();
let reader = SerializedFileReader::new(file).unwrap();
let row: Row = reader.get_row_iter(None).unwrap().next().unwrap().unwrap();
for (idx, (name, field)) in row.get_column_iter().enumerate() {
    println!("column index: {}, column name: {}, column value: {}", idx, name, field);
}
```

<a id="op-60a468aaab9f87dce3779006"></a>
## get_decimal

`function` · `parquet::record::api::Row::get_decimal` · parquet 59.3.0

```rust
fn get_decimal(&self, i: usize) -> Result<&Decimal>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::record::api::Row", "path": "Row"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [258, 1], "end": [306, 2], "filename": "src/record/api.rs"}, "trait": {"args": null, "id": "parquet::record::api::RowAccessor", "path": "RowAccessor"}, "trait_path": "parquet::record::api::RowAccessor"}`

Source: `src/record/api.rs:295`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f0939504d6a9e707d0b54ad5"></a>
## get_double

`function` · `parquet::record::api::Row::get_double` · parquet 59.3.0

```rust
fn get_double(&self, i: usize) -> Result<f64>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::record::api::Row", "path": "Row"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [258, 1], "end": [306, 2], "filename": "src/record/api.rs"}, "trait": {"args": null, "id": "parquet::record::api::RowAccessor", "path": "RowAccessor"}, "trait_path": "parquet::record::api::RowAccessor"}`

Source: `src/record/api.rs:289`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7913ab9e527189bfdc241d3d"></a>
## get_float

`function` · `parquet::record::api::Row::get_float` · parquet 59.3.0

```rust
fn get_float(&self, i: usize) -> Result<f32>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::record::api::Row", "path": "Row"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [258, 1], "end": [306, 2], "filename": "src/record/api.rs"}, "trait": {"args": null, "id": "parquet::record::api::RowAccessor", "path": "RowAccessor"}, "trait_path": "parquet::record::api::RowAccessor"}`

Source: `src/record/api.rs:287`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2635e8159d40a5e48131c14b"></a>
## get_float16

`function` · `parquet::record::api::Row::get_float16` · parquet 59.3.0

```rust
fn get_float16(&self, i: usize) -> Result<f16>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::record::api::Row", "path": "Row"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [258, 1], "end": [306, 2], "filename": "src/record/api.rs"}, "trait": {"args": null, "id": "parquet::record::api::RowAccessor", "path": "RowAccessor"}, "trait_path": "parquet::record::api::RowAccessor"}`

Source: `src/record/api.rs:285`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8a7a2b446a04d03d036167c7"></a>
## get_group

`function` · `parquet::record::api::Row::get_group` · parquet 59.3.0

```rust
fn get_group(&self, i: usize) -> Result<&Row>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::record::api::Row", "path": "Row"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [258, 1], "end": [306, 2], "filename": "src/record/api.rs"}, "trait": {"args": null, "id": "parquet::record::api::RowAccessor", "path": "RowAccessor"}, "trait_path": "parquet::record::api::RowAccessor"}`

Source: `src/record/api.rs:301`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-12f48c710e57690597b297a6"></a>
## get_int

`function` · `parquet::record::api::Row::get_int` · parquet 59.3.0

```rust
fn get_int(&self, i: usize) -> Result<i32>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::record::api::Row", "path": "Row"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [258, 1], "end": [306, 2], "filename": "src/record/api.rs"}, "trait": {"args": null, "id": "parquet::record::api::RowAccessor", "path": "RowAccessor"}, "trait_path": "parquet::record::api::RowAccessor"}`

Source: `src/record/api.rs:273`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-eee1a18f162f99afe1c99709"></a>
## get_list

`function` · `parquet::record::api::Row::get_list` · parquet 59.3.0

```rust
fn get_list(&self, i: usize) -> Result<&List>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::record::api::Row", "path": "Row"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [258, 1], "end": [306, 2], "filename": "src/record/api.rs"}, "trait": {"args": null, "id": "parquet::record::api::RowAccessor", "path": "RowAccessor"}, "trait_path": "parquet::record::api::RowAccessor"}`

Source: `src/record/api.rs:303`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cb8307efd051549ccdd34857"></a>
## get_long

`function` · `parquet::record::api::Row::get_long` · parquet 59.3.0

```rust
fn get_long(&self, i: usize) -> Result<i64>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::record::api::Row", "path": "Row"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [258, 1], "end": [306, 2], "filename": "src/record/api.rs"}, "trait": {"args": null, "id": "parquet::record::api::RowAccessor", "path": "RowAccessor"}, "trait_path": "parquet::record::api::RowAccessor"}`

Source: `src/record/api.rs:275`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-abfce15de4af9998cf499970"></a>
## get_map

`function` · `parquet::record::api::Row::get_map` · parquet 59.3.0

```rust
fn get_map(&self, i: usize) -> Result<&Map>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::record::api::Row", "path": "Row"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [258, 1], "end": [306, 2], "filename": "src/record/api.rs"}, "trait": {"args": null, "id": "parquet::record::api::RowAccessor", "path": "RowAccessor"}, "trait_path": "parquet::record::api::RowAccessor"}`

Source: `src/record/api.rs:305`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a4b6e1600228ba05fe01f5f2"></a>
## get_short

`function` · `parquet::record::api::Row::get_short` · parquet 59.3.0

```rust
fn get_short(&self, i: usize) -> Result<i16>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::record::api::Row", "path": "Row"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [258, 1], "end": [306, 2], "filename": "src/record/api.rs"}, "trait": {"args": null, "id": "parquet::record::api::RowAccessor", "path": "RowAccessor"}, "trait_path": "parquet::record::api::RowAccessor"}`

Source: `src/record/api.rs:271`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7ab1773d28ce2bfce7e84666"></a>
## get_string

`function` · `parquet::record::api::Row::get_string` · parquet 59.3.0

```rust
fn get_string(&self, i: usize) -> Result<&String>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::record::api::Row", "path": "Row"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [258, 1], "end": [306, 2], "filename": "src/record/api.rs"}, "trait": {"args": null, "id": "parquet::record::api::RowAccessor", "path": "RowAccessor"}, "trait_path": "parquet::record::api::RowAccessor"}`

Source: `src/record/api.rs:297`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d09f77d36e801154f063c2ec"></a>
## get_timestamp_micros

`function` · `parquet::record::api::Row::get_timestamp_micros` · parquet 59.3.0

```rust
fn get_timestamp_micros(&self, i: usize) -> Result<i64>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::record::api::Row", "path": "Row"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [258, 1], "end": [306, 2], "filename": "src/record/api.rs"}, "trait": {"args": null, "id": "parquet::record::api::RowAccessor", "path": "RowAccessor"}, "trait_path": "parquet::record::api::RowAccessor"}`

Source: `src/record/api.rs:293`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c8008582a66d353c3659e9e5"></a>
## get_timestamp_millis

`function` · `parquet::record::api::Row::get_timestamp_millis` · parquet 59.3.0

```rust
fn get_timestamp_millis(&self, i: usize) -> Result<i64>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::record::api::Row", "path": "Row"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [258, 1], "end": [306, 2], "filename": "src/record/api.rs"}, "trait": {"args": null, "id": "parquet::record::api::RowAccessor", "path": "RowAccessor"}, "trait_path": "parquet::record::api::RowAccessor"}`

Source: `src/record/api.rs:291`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4860dcfba5f6461ce04281e5"></a>
## get_ubyte

`function` · `parquet::record::api::Row::get_ubyte` · parquet 59.3.0

```rust
fn get_ubyte(&self, i: usize) -> Result<u8>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::record::api::Row", "path": "Row"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [258, 1], "end": [306, 2], "filename": "src/record/api.rs"}, "trait": {"args": null, "id": "parquet::record::api::RowAccessor", "path": "RowAccessor"}, "trait_path": "parquet::record::api::RowAccessor"}`

Source: `src/record/api.rs:277`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-faee42b2586b36c8df36c5b9"></a>
## get_uint

`function` · `parquet::record::api::Row::get_uint` · parquet 59.3.0

```rust
fn get_uint(&self, i: usize) -> Result<u32>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::record::api::Row", "path": "Row"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [258, 1], "end": [306, 2], "filename": "src/record/api.rs"}, "trait": {"args": null, "id": "parquet::record::api::RowAccessor", "path": "RowAccessor"}, "trait_path": "parquet::record::api::RowAccessor"}`

Source: `src/record/api.rs:281`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f6aaa239293d65c7945f5549"></a>
## get_ulong

`function` · `parquet::record::api::Row::get_ulong` · parquet 59.3.0

```rust
fn get_ulong(&self, i: usize) -> Result<u64>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::record::api::Row", "path": "Row"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [258, 1], "end": [306, 2], "filename": "src/record/api.rs"}, "trait": {"args": null, "id": "parquet::record::api::RowAccessor", "path": "RowAccessor"}, "trait_path": "parquet::record::api::RowAccessor"}`

Source: `src/record/api.rs:283`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7970f676fd78bd6a02aa0347"></a>
## get_ushort

`function` · `parquet::record::api::Row::get_ushort` · parquet 59.3.0

```rust
fn get_ushort(&self, i: usize) -> Result<u16>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::record::api::Row", "path": "Row"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [258, 1], "end": [306, 2], "filename": "src/record/api.rs"}, "trait": {"args": null, "id": "parquet::record::api::RowAccessor", "path": "RowAccessor"}, "trait_path": "parquet::record::api::RowAccessor"}`

Source: `src/record/api.rs:279`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-97a3e13c22efffaeb3f05668"></a>
## into_columns

`function` · `parquet::record::api::Row::into_columns` · parquet 59.3.0

```rust
fn into_columns(self) -> Vec<(String, Field)>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::record::api::Row", "path": "Row"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [54, 1], "end": [119, 2], "filename": "src/record/api.rs"}, "trait": null, "trait_path": null}`

Source: `src/record/api.rs:81`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Move columns data out of the row. Useful to avoid internal data cloning.

# Example

```no_run
use std::fs::File;
use parquet::record::Row;
use parquet::file::reader::{FileReader, SerializedFileReader};

let file = File::open("/path/to/file").unwrap();
let reader = SerializedFileReader::new(file).unwrap();
let row: Row = reader.get_row_iter(None).unwrap().next().unwrap().unwrap();
let columns = row.into_columns();
println!("row columns: {:?}", columns);

```

<a id="op-3a9b6f1c9f0fd4b38e6b3a0e"></a>
## is_null

`function` · `parquet::record::api::Row::is_null` · parquet 59.3.0

```rust
fn is_null(&self, i: usize) -> Result<bool>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::record::api::Row", "path": "Row"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [258, 1], "end": [306, 2], "filename": "src/record/api.rs"}, "trait": {"args": null, "id": "parquet::record::api::RowAccessor", "path": "RowAccessor"}, "trait_path": "parquet::record::api::RowAccessor"}`

Source: `src/record/api.rs:259`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fcccb9e68ebcea8181d28ae6"></a>
## len

`function` · `parquet::record::api::Row::len` · parquet 59.3.0

```rust
fn len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::record::api::Row", "path": "Row"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [54, 1], "end": [119, 2], "filename": "src/record/api.rs"}, "trait": null, "trait_path": null}`

Source: `src/record/api.rs:61`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Get the number of fields in this row.

<a id="op-6bae4878bfc8d2a63ecdceef"></a>
## new

`function` · `parquet::record::api::Row::new` · parquet 59.3.0

```rust
fn new(fields: Vec<(String, Field)>) -> Row
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::record::api::Row", "path": "Row"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [54, 1], "end": [119, 2], "filename": "src/record/api.rs"}, "trait": null, "trait_path": null}`

Source: `src/record/api.rs:56`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Constructs a `Row` from the list of `fields` and returns it.

<a id="op-7c01107279b4bf86097e5f9d"></a>
## to_json_value

`function` · `parquet::record::api::Row::to_json_value` · parquet 59.3.0

```rust
fn to_json_value(&self) -> Value
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::record::api::Row", "path": "Row"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [54, 1], "end": [119, 2], "filename": "src/record/api.rs"}, "trait": null, "trait_path": null}`

Source: `src/record/api.rs:111`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Converts the row into a JSON object.
