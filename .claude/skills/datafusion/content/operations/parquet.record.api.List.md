# `parquet::record::api::List`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.record.api.List.json).

<a id="op-14cbffc544b88ff42060e342"></a>
## List

`struct` · `parquet::record::api::List` · parquet 59.3.0

```rust
struct List
```

Source: `src/record/api.rs:325`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

`List` represents a list which contains an array of elements.

<a id="op-997806e7692e89dbf71b58a4"></a>
## clone

`function` · `parquet::record::api::List::clone` · parquet 59.3.0

```rust
fn clone(&self) -> List
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::record::api::List", "path": "List"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [324, 10], "end": [324, 15], "filename": "src/record/api.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/record/api.rs:324`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-923eec8d5a5e1e9a8f78141d"></a>
## elements

`function` · `parquet::record::api::List::elements` · parquet 59.3.0

```rust
fn elements(&self) -> &[Field]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::record::api::List", "path": "List"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [330, 1], "end": [340, 2], "filename": "src/record/api.rs"}, "trait": null, "trait_path": null}`

Source: `src/record/api.rs:337`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Get the reference to the elements in this list

<a id="op-b1fbe4ac4e387319e0b4a123"></a>
## eq

`function` · `parquet::record::api::List::eq` · parquet 59.3.0

```rust
fn eq(&self, other: &List) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::record::api::List", "path": "List"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [324, 24], "end": [324, 33], "filename": "src/record/api.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/record/api.rs:324`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c7fc8eacbcc163206dfaff07"></a>
## fmt

`function` · `parquet::record::api::List::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::record::api::List", "path": "List"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [324, 17], "end": [324, 22], "filename": "src/record/api.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/record/api.rs:324`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-44bcfbd5d33a24ff6d4ece2e"></a>
## get_bool

`function` · `parquet::record::api::List::get_bool` · parquet 59.3.0

```rust
fn get_bool(&self, i: usize) -> Result<bool>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::record::api::List", "path": "List"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [429, 1], "end": [469, 2], "filename": "src/record/api.rs"}, "trait": {"args": null, "id": "parquet::record::api::ListAccessor", "path": "ListAccessor"}, "trait_path": "parquet::record::api::ListAccessor"}`

Source: `src/record/api.rs:430`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fca1977c7932270acd2c46ec"></a>
## get_byte

`function` · `parquet::record::api::List::get_byte` · parquet 59.3.0

```rust
fn get_byte(&self, i: usize) -> Result<i8>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::record::api::List", "path": "List"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [429, 1], "end": [469, 2], "filename": "src/record/api.rs"}, "trait": {"args": null, "id": "parquet::record::api::ListAccessor", "path": "ListAccessor"}, "trait_path": "parquet::record::api::ListAccessor"}`

Source: `src/record/api.rs:432`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-41c28a38d89b68376c03f2a8"></a>
## get_bytes

`function` · `parquet::record::api::List::get_bytes` · parquet 59.3.0

```rust
fn get_bytes(&self, i: usize) -> Result<&ByteArray>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::record::api::List", "path": "List"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [429, 1], "end": [469, 2], "filename": "src/record/api.rs"}, "trait": {"args": null, "id": "parquet::record::api::ListAccessor", "path": "ListAccessor"}, "trait_path": "parquet::record::api::ListAccessor"}`

Source: `src/record/api.rs:462`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-106cbc25edeeac3927e18926"></a>
## get_decimal

`function` · `parquet::record::api::List::get_decimal` · parquet 59.3.0

```rust
fn get_decimal(&self, i: usize) -> Result<&Decimal>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::record::api::List", "path": "List"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [429, 1], "end": [469, 2], "filename": "src/record/api.rs"}, "trait": {"args": null, "id": "parquet::record::api::ListAccessor", "path": "ListAccessor"}, "trait_path": "parquet::record::api::ListAccessor"}`

Source: `src/record/api.rs:458`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-573befa939523fc283be4bab"></a>
## get_double

`function` · `parquet::record::api::List::get_double` · parquet 59.3.0

```rust
fn get_double(&self, i: usize) -> Result<f64>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::record::api::List", "path": "List"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [429, 1], "end": [469, 2], "filename": "src/record/api.rs"}, "trait": {"args": null, "id": "parquet::record::api::ListAccessor", "path": "ListAccessor"}, "trait_path": "parquet::record::api::ListAccessor"}`

Source: `src/record/api.rs:452`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-111667e3e0d9afa3979ceddc"></a>
## get_float

`function` · `parquet::record::api::List::get_float` · parquet 59.3.0

```rust
fn get_float(&self, i: usize) -> Result<f32>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::record::api::List", "path": "List"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [429, 1], "end": [469, 2], "filename": "src/record/api.rs"}, "trait": {"args": null, "id": "parquet::record::api::ListAccessor", "path": "ListAccessor"}, "trait_path": "parquet::record::api::ListAccessor"}`

Source: `src/record/api.rs:450`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-68d2475e64f9de8a181fc92a"></a>
## get_float16

`function` · `parquet::record::api::List::get_float16` · parquet 59.3.0

```rust
fn get_float16(&self, i: usize) -> Result<f16>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::record::api::List", "path": "List"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [429, 1], "end": [469, 2], "filename": "src/record/api.rs"}, "trait": {"args": null, "id": "parquet::record::api::ListAccessor", "path": "ListAccessor"}, "trait_path": "parquet::record::api::ListAccessor"}`

Source: `src/record/api.rs:448`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-89a536b8e9df98d1adfb90a7"></a>
## get_group

`function` · `parquet::record::api::List::get_group` · parquet 59.3.0

```rust
fn get_group(&self, i: usize) -> Result<&Row>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::record::api::List", "path": "List"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [429, 1], "end": [469, 2], "filename": "src/record/api.rs"}, "trait": {"args": null, "id": "parquet::record::api::ListAccessor", "path": "ListAccessor"}, "trait_path": "parquet::record::api::ListAccessor"}`

Source: `src/record/api.rs:464`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b59fa62e2f3ec304ec07f783"></a>
## get_int

`function` · `parquet::record::api::List::get_int` · parquet 59.3.0

```rust
fn get_int(&self, i: usize) -> Result<i32>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::record::api::List", "path": "List"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [429, 1], "end": [469, 2], "filename": "src/record/api.rs"}, "trait": {"args": null, "id": "parquet::record::api::ListAccessor", "path": "ListAccessor"}, "trait_path": "parquet::record::api::ListAccessor"}`

Source: `src/record/api.rs:436`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b405f2515f71d0e9ab598491"></a>
## get_list

`function` · `parquet::record::api::List::get_list` · parquet 59.3.0

```rust
fn get_list(&self, i: usize) -> Result<&List>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::record::api::List", "path": "List"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [429, 1], "end": [469, 2], "filename": "src/record/api.rs"}, "trait": {"args": null, "id": "parquet::record::api::ListAccessor", "path": "ListAccessor"}, "trait_path": "parquet::record::api::ListAccessor"}`

Source: `src/record/api.rs:466`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c8a2cb43d243a8c5e9f8dee5"></a>
## get_long

`function` · `parquet::record::api::List::get_long` · parquet 59.3.0

```rust
fn get_long(&self, i: usize) -> Result<i64>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::record::api::List", "path": "List"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [429, 1], "end": [469, 2], "filename": "src/record/api.rs"}, "trait": {"args": null, "id": "parquet::record::api::ListAccessor", "path": "ListAccessor"}, "trait_path": "parquet::record::api::ListAccessor"}`

Source: `src/record/api.rs:438`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-33daa1fa7a44606714bd9c6c"></a>
## get_map

`function` · `parquet::record::api::List::get_map` · parquet 59.3.0

```rust
fn get_map(&self, i: usize) -> Result<&Map>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::record::api::List", "path": "List"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [429, 1], "end": [469, 2], "filename": "src/record/api.rs"}, "trait": {"args": null, "id": "parquet::record::api::ListAccessor", "path": "ListAccessor"}, "trait_path": "parquet::record::api::ListAccessor"}`

Source: `src/record/api.rs:468`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3337e3c673c7ee308803d9ad"></a>
## get_short

`function` · `parquet::record::api::List::get_short` · parquet 59.3.0

```rust
fn get_short(&self, i: usize) -> Result<i16>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::record::api::List", "path": "List"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [429, 1], "end": [469, 2], "filename": "src/record/api.rs"}, "trait": {"args": null, "id": "parquet::record::api::ListAccessor", "path": "ListAccessor"}, "trait_path": "parquet::record::api::ListAccessor"}`

Source: `src/record/api.rs:434`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c856230634a97968ee46af42"></a>
## get_string

`function` · `parquet::record::api::List::get_string` · parquet 59.3.0

```rust
fn get_string(&self, i: usize) -> Result<&String>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::record::api::List", "path": "List"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [429, 1], "end": [469, 2], "filename": "src/record/api.rs"}, "trait": {"args": null, "id": "parquet::record::api::ListAccessor", "path": "ListAccessor"}, "trait_path": "parquet::record::api::ListAccessor"}`

Source: `src/record/api.rs:460`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-04007f0015e8684761ce2caf"></a>
## get_timestamp_micros

`function` · `parquet::record::api::List::get_timestamp_micros` · parquet 59.3.0

```rust
fn get_timestamp_micros(&self, i: usize) -> Result<i64>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::record::api::List", "path": "List"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [429, 1], "end": [469, 2], "filename": "src/record/api.rs"}, "trait": {"args": null, "id": "parquet::record::api::ListAccessor", "path": "ListAccessor"}, "trait_path": "parquet::record::api::ListAccessor"}`

Source: `src/record/api.rs:456`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a1e09a25b87ddbef378c8275"></a>
## get_timestamp_millis

`function` · `parquet::record::api::List::get_timestamp_millis` · parquet 59.3.0

```rust
fn get_timestamp_millis(&self, i: usize) -> Result<i64>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::record::api::List", "path": "List"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [429, 1], "end": [469, 2], "filename": "src/record/api.rs"}, "trait": {"args": null, "id": "parquet::record::api::ListAccessor", "path": "ListAccessor"}, "trait_path": "parquet::record::api::ListAccessor"}`

Source: `src/record/api.rs:454`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-602a6b4a404956a2d9c2f3e4"></a>
## get_ubyte

`function` · `parquet::record::api::List::get_ubyte` · parquet 59.3.0

```rust
fn get_ubyte(&self, i: usize) -> Result<u8>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::record::api::List", "path": "List"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [429, 1], "end": [469, 2], "filename": "src/record/api.rs"}, "trait": {"args": null, "id": "parquet::record::api::ListAccessor", "path": "ListAccessor"}, "trait_path": "parquet::record::api::ListAccessor"}`

Source: `src/record/api.rs:440`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-925824fd0a6ee88a575fd395"></a>
## get_uint

`function` · `parquet::record::api::List::get_uint` · parquet 59.3.0

```rust
fn get_uint(&self, i: usize) -> Result<u32>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::record::api::List", "path": "List"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [429, 1], "end": [469, 2], "filename": "src/record/api.rs"}, "trait": {"args": null, "id": "parquet::record::api::ListAccessor", "path": "ListAccessor"}, "trait_path": "parquet::record::api::ListAccessor"}`

Source: `src/record/api.rs:444`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-930abfed847157d3453977f5"></a>
## get_ulong

`function` · `parquet::record::api::List::get_ulong` · parquet 59.3.0

```rust
fn get_ulong(&self, i: usize) -> Result<u64>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::record::api::List", "path": "List"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [429, 1], "end": [469, 2], "filename": "src/record/api.rs"}, "trait": {"args": null, "id": "parquet::record::api::ListAccessor", "path": "ListAccessor"}, "trait_path": "parquet::record::api::ListAccessor"}`

Source: `src/record/api.rs:446`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7566e41c3019e8d95d18253a"></a>
## get_ushort

`function` · `parquet::record::api::List::get_ushort` · parquet 59.3.0

```rust
fn get_ushort(&self, i: usize) -> Result<u16>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::record::api::List", "path": "List"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [429, 1], "end": [469, 2], "filename": "src/record/api.rs"}, "trait": {"args": null, "id": "parquet::record::api::ListAccessor", "path": "ListAccessor"}, "trait_path": "parquet::record::api::ListAccessor"}`

Source: `src/record/api.rs:442`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7a99cb30436c2485e21b33c1"></a>
## len

`function` · `parquet::record::api::List::len` · parquet 59.3.0

```rust
fn len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::record::api::List", "path": "List"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [330, 1], "end": [340, 2], "filename": "src/record/api.rs"}, "trait": null, "trait_path": null}`

Source: `src/record/api.rs:332`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Get the number of fields in this row
