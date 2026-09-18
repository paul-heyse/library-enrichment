# `arrow_row::RowConverter`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_row.RowConverter.json).

<a id="op-3289371bf6ccaf9946ba91e3"></a>
## RowConverter

`struct` · `arrow_row::RowConverter` · arrow-row 59.3.0

```rust
struct RowConverter
```

Source: `src/lib.rs:568`. [Exact documentation build](https://docs.rs/crate/arrow-row/59.3.0/json).

 Converts [`ArrayRef`](../operations/arrow_array.array.ArrayRef.md#op-657cc3ff3d24afcacda590e1) columns into a [row-oriented](self) format.

 *Note: The encoding of the row format may change from release to release.*

 ## Overview

 The row format is a variable length byte sequence created by
 concatenating the encoded form of each column. The encoding for
 each column depends on its datatype (and sort options).

 The encoding is carefully designed in such a way that escaping is
 unnecessary: it is never ambiguous as to whether a byte is part of
 a sentinel (e.g. null) or a value.

 ## Unsigned Integer Encoding

 A null integer is encoded as a `0_u8`, followed by a zero-ed number of bytes corresponding
 to the integer's length.

 A valid integer is encoded as `1_u8`, followed by the big-endian representation of the
 integer.

 ```text
               ┌──┬──┬──┬──┐      ┌──┬──┬──┬──┬──┐
    3          │03│00│00│00│      │01│00│00│00│03│
               └──┴──┴──┴──┘      └──┴──┴──┴──┴──┘
               ┌──┬──┬──┬──┐      ┌──┬──┬──┬──┬──┐
   258         │02│01│00│00│      │01│00│00│01│02│
               └──┴──┴──┴──┘      └──┴──┴──┴──┴──┘
               ┌──┬──┬──┬──┐      ┌──┬──┬──┬──┬──┐
  23423        │7F│5B│00│00│      │01│00│00│5B│7F│
               └──┴──┴──┴──┘      └──┴──┴──┴──┴──┘
               ┌──┬──┬──┬──┐      ┌──┬──┬──┬──┬──┐
  NULL         │??│??│??│??│      │00│00│00│00│00│
               └──┴──┴──┴──┘      └──┴──┴──┴──┴──┘

              32-bit (4 bytes)        Row Format
  Value        Little Endian
 ```

 ## Signed Integer Encoding

 Signed integers have their most significant sign bit flipped, and are then encoded in the
 same manner as an unsigned integer.

 ```text
        ┌──┬──┬──┬──┐       ┌──┬──┬──┬──┐       ┌──┬──┬──┬──┬──┐
     5  │05│00│00│00│       │05│00│00│80│       │01│80│00│00│05│
        └──┴──┴──┴──┘       └──┴──┴──┴──┘       └──┴──┴──┴──┴──┘
        ┌──┬──┬──┬──┐       ┌──┬──┬──┬──┐       ┌──┬──┬──┬──┬──┐
    -5  │FB│FF│FF│FF│       │FB│FF│FF│7F│       │01│7F│FF│FF│FB│
        └──┴──┴──┴──┘       └──┴──┴──┴──┘       └──┴──┴──┴──┴──┘

  Value  32-bit (4 bytes)    High bit flipped      Row Format
          Little Endian
 ```

 ## Float Encoding

 Floats are converted from IEEE 754 representation to a signed integer representation
 by flipping all bar the sign bit if they are negative.

 They are then encoded in the same manner as a signed integer.

 ## Fixed Length Bytes Encoding

 Fixed length bytes are encoded in the same fashion as primitive types above.

 For a fixed length array of length `n`:

 A null is encoded as `0_u8` null sentinel followed by `n` `0_u8` bytes

 A valid value is encoded as `1_u8` followed by the value bytes

 ## Variable Length Bytes (including Strings) Encoding

 A null is encoded as a `0_u8`.

 An empty byte array is encoded as `1_u8`.

 A non-null, non-empty byte array is encoded as `2_u8` followed by the byte array
 encoded using a block based scheme described below.

 The byte array is broken up into fixed-width blocks, each block is written in turn
 to the output, followed by `0xFF_u8`. The final block is padded to 32-bytes
 with `0_u8` and written to the output, followed by the un-padded length in bytes
 of this final block as a `u8`. The first 4 blocks have a length of 8, with subsequent
 blocks using a length of 32, this is to reduce space amplification for small strings.

 Note the following example encodings use a block size of 4 bytes for brevity:

 ```text
                       ┌───┬───┬───┬───┬───┬───┐
  "MEEP"               │02 │'M'│'E'│'E'│'P'│04 │
                       └───┴───┴───┴───┴───┴───┘

                       ┌───┐
  ""                   │01 |
                       └───┘

  NULL                 ┌───┐
                       │00 │
                       └───┘

 "Defenestration"      ┌───┬───┬───┬───┬───┬───┐
                       │02 │'D'│'e'│'f'│'e'│FF │
                       └───┼───┼───┼───┼───┼───┤
                           │'n'│'e'│'s'│'t'│FF │
                           ├───┼───┼───┼───┼───┤
                           │'r'│'a'│'t'│'r'│FF │
                           ├───┼───┼───┼───┼───┤
                           │'a'│'t'│'i'│'o'│FF │
                           ├───┼───┼───┼───┼───┤
                           │'n'│00 │00 │00 │01 │
                           └───┴───┴───┴───┴───┘
 ```

 This approach is loosely inspired by [COBS] encoding, and chosen over more traditional
 [byte stuffing] as it is more amenable to vectorisation, in particular AVX-256.

 ## Dictionary Encoding

 Dictionary encoded arrays are hydrated to their underlying values

 ## REE Encoding

 REE (Run End Encoding) arrays, A form of Run Length Encoding, are hydrated to their underlying values.

 ## Struct Encoding

 A null is encoded as a `0_u8`.

 A valid value is encoded as `1_u8` followed by the row encoding of each child.

 This encoding effectively flattens the schema in a depth-first fashion.

 For example

 ```text
 ┌───────┬────────────────────────┬───────┐
 │ Int32 │ Struct[Int32, Float32] │ Int32 │
 └───────┴────────────────────────┴───────┘
 ```

 Is encoded as

 ```text
 ┌───────┬───────────────┬───────┬─────────┬───────┐
 │ Int32 │ Null Sentinel │ Int32 │ Float32 │ Int32 │
 └───────┴───────────────┴───────┴─────────┴───────┘
 ```

 ## List and Map Encoding

 Lists are encoded by first encoding all child elements to the row format.

 The Map encoding is the same with the only difference being that the child elements are key-value pairs.

 A list/map value is then encoded as the concatenation of each of the child elements,
 separately encoded using the variable length encoding described above, followed
 by the variable length encoding of an empty byte array.

 For example given:

 ```text
 [1_u8, 2_u8, 3_u8]
 [1_u8, null]
 []
 null
 ```

 The elements would be converted to:

 ```text
     ┌──┬──┐     ┌──┬──┐     ┌──┬──┐     ┌──┬──┐        ┌──┬──┐
  1  │01│01│  2  │01│02│  3  │01│03│  1  │01│01│  null  │00│00│
     └──┴──┘     └──┴──┘     └──┴──┘     └──┴──┘        └──┴──┘
```

 Which would be encoded as

 ```text
                         ┌──┬──┬──┬──┬──┬──┬──┬──┬──┬──┬──┬──┬──┬──┬──┬──┬──┬──┬──┐
  [1_u8, 2_u8, 3_u8]     │02│01│01│00│00│02│02│01│02│00│00│02│02│01│03│00│00│02│01│
                         └──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┘
                          └──── 1_u8 ────┘   └──── 2_u8 ────┘  └──── 3_u8 ────┘

                         ┌──┬──┬──┬──┬──┬──┬──┬──┬──┬──┬──┬──┬──┐
  [1_u8, null]           │02│01│01│00│00│02│02│00│00│00│00│02│01│
                         └──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┘
                          └──── 1_u8 ────┘   └──── null ────┘

```

 With `[]` represented by an empty byte array, and `null` a null byte array.

 ### Map Equality

 **Note:** Maps with different order of keys are not equal in row format
 i.e. row_format({"hello": 1, "world": 2}) != row_format({"world": 2, "hello": 1})

 ```
 # use std::sync::Arc;
 # use arrow_row::{RowConverter, SortField};
 # use arrow_array::{Array, Int32Array, StringArray, MapArray};

 // [{ "hello": 1, "world": 2 }, { "hey": 3, "you": 4 }]
 let map_1 = MapArray::from_vec_of_maps::<StringArray, Int32Array, _, _>(vec![
   Some(vec![("hello", Some(1)), ("world", Some(2))]),
   Some(vec![("hey", Some(3)), ("you", Some(4))]),
 ], false);
 // [{ "world": 2, "hello": 1 }, { "hey": 3, "you": 4 }]
 let map_2 = MapArray::from_vec_of_maps::<StringArray, Int32Array, _, _>(vec![
   Some(vec![("world", Some(2)), ("hello", Some(1))]),
   Some(vec![("hey", Some(3)), ("you", Some(4))]),
 ], false);

 let converter = RowConverter::new(vec![SortField::new(map_1.data_type().clone())]).unwrap();

 let map_1_rows = converter.convert_columns(&[Arc::new(map_1)]).unwrap();
 let map_2_rows = converter.convert_columns(&[Arc::new(map_2)]).unwrap();

 // Attention! these maps are NOT equal since the order of the keys are different
 assert_ne!(map_1_rows.row(0), map_2_rows.row(0));

 // These maps ARE equal since the order and the content of the keys and values are the same
 assert_eq!(map_1_rows.row(1), map_2_rows.row(1));
 ```

 If you DO want to treat maps with different order of keys as the same in row format you should canonicalize them first.

 ## Fixed Size List Encoding

 Fixed Size Lists are encoded by first encoding all child elements to the row format.

 A non-null list value is then encoded as 0x01 followed by the concatenation of each
 of the child elements. A null list value is encoded as a null marker.

 For example given:

 ```text
 [1_u8, 2_u8]
 [3_u8, null]
 null
 ```

 The elements would be converted to:

 ```text
     ┌──┬──┐     ┌──┬──┐     ┌──┬──┐        ┌──┬──┐
  1  │01│01│  2  │01│02│  3  │01│03│  null  │00│00│
     └──┴──┘     └──┴──┘     └──┴──┘        └──┴──┘
```

 Which would be encoded as

 ```text
                 ┌──┬──┬──┬──┬──┐
  [1_u8, 2_u8]   │01│01│01│01│02│
                 └──┴──┴──┴──┴──┘
                     └ 1 ┘ └ 2 ┘
                 ┌──┬──┬──┬──┬──┐
  [3_u8, null]   │01│01│03│00│00│
                 └──┴──┴──┴──┴──┘
                     └ 1 ┘ └null┘
                 ┌──┐
  null           │00│
                 └──┘

```

 ## ListView Encoding

 ListView arrays differ from List arrays in their representation: instead of using
 consecutive offset pairs to define each list, ListView uses explicit offset and size
 pairs for each element. This allows ListView elements to reference arbitrary (potentially
 overlapping) regions of the child array.

 Despite this structural difference, ListView uses the **same row encoding as List**.
 Each list value is encoded as the concatenation of its child elements (each separately
 variable-length encoded), followed by a variable-length encoded empty byte array terminator.

 **Important**: When a ListView is decoded back from row format, it is still a
 ListView, but any child element sharing that may have existed in the original
 (where multiple list entries could reference overlapping regions of the child
 array) is **not preserved** - each list's children are decoded independently
 with sequential offsets.

 For example, given a ListView with offset/size pairs:

 ```text
 offsets: [0, 1, 0]
 sizes:   [2, 2, 0]
 values:  [1_u8, 2_u8, 3_u8]

 Resulting lists:
 [1_u8, 2_u8]  (offset=0, size=2 -> values[0..2])
 [2_u8, 3_u8]  (offset=1, size=2 -> values[1..3])
 []            (offset=0, size=0 -> empty)
 ```

 The elements would be encoded identically to List encoding:

 ```text
                         ┌──┬──┬──┬──┬──┬──┬──┬──┬──┬──┬──┬──┬──┐
  [1_u8, 2_u8]           │02│01│01│00│00│02│02│01│02│00│00│02│01│
                         └──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┘
                          └──── 1_u8 ────┘   └──── 2_u8 ────┘

                         ┌──┬──┬──┬──┬──┬──┬──┬──┬──┬──┬──┬──┬──┐
  [2_u8, 3_u8]           │02│01│02│00│00│02│02│01│03│00│00│02│01│
                         └──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┘
                          └──── 2_u8 ────┘   └──── 3_u8 ────┘

```

 Note that element `2_u8` appears in both encoded rows, even though it was shared
 in the original ListView, and `[]` is represented by an empty byte array.

 ## Union Encoding

 A union value is encoded as a single type-id byte followed by the row encoding of the selected child value.
 The type-id byte is always present; union arrays have no top-level null marker, so nulls are represented by the child encoding.

 For example, given a union of Int32 (type_id = 0) and Utf8 (type_id = 1):

 ```text
                           ┌──┬──────────────┐
  3                        │00│01│80│00│00│03│
                           └──┴──────────────┘
                            │  └─ signed integer encoding (non-null)
                            └──── type_id

                           ┌──┬────────────────────────────────┐
 "abc"                     │01│02│'a'│'b'│'c'│00│00│00│00│00│03│
                           └──┴────────────────────────────────┘
                            │  └─ string encoding (non-null)
                            └──── type_id

                           ┌──┬──────────────┐
 null Int32                │00│00│00│00│00│00│
                           └──┴──────────────┘
                            │  └─ signed integer encoding (null)
                            └──── type_id

                           ┌──┬──┐
 null Utf8                 │01│00│
                           └──┴──┘
                            │  └─ string encoding (null)
                            └──── type_id
 ```

 See [`UnionArray`](../operations/arrow_array.array.union_array.UnionArray.md#op-39e2f188616dc0298ba644ac) for more details on union types.

 # Ordering

 ## Float Ordering

 Floats are totally ordered in accordance to the `totalOrder` predicate as defined
 in the IEEE 754 (2008 revision) floating point standard.

 The ordering established by this does not always agree with the
 [`PartialOrd`] and [`PartialEq`] implementations of `f32`. For example,
 they consider negative and positive zero equal, while this does not

 ## Null Ordering

 The encoding described above will order nulls first, this can be inverted by representing
 nulls as `0xFF_u8` instead of `0_u8`

 ## Union Ordering

 Values of the same type are ordered according to the ordering of that type.
 Values of different types are ordered by their type id.
 The type_id is negated when descending order is specified.

 ## Reverse Column Ordering

 The order of a given column can be reversed by negating the encoded bytes of non-null values

 [COBS]: https://en.wikipedia.org/wiki/Consistent_Overhead_Byte_Stuffing
 [byte stuffing]: https://en.wikipedia.org/wiki/High-Level_Data_Link_Control#Asynchronous_framing

Unresolved upstream links (retained, not inferred): ``PartialOrd``, ``PartialEq``.

<a id="op-69a9ab377eed19a94e263b06"></a>
## append

`function` · `arrow_row::RowConverter::append` · arrow-row 59.3.0

```rust
fn append(&self, rows: &mut Rows, columns: &[ArrayRef]) -> Result<(), ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_row::RowConverter", "path": "RowConverter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [956, 1], "end": [1276, 2], "filename": "src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `src/lib.rs:1041`. [Exact documentation build](https://docs.rs/crate/arrow-row/59.3.0/json).

Convert [`ArrayRef`](../operations/arrow_array.array.ArrayRef.md#op-657cc3ff3d24afcacda590e1) columns appending to an existing [`Rows`](../operations/arrow_row.Rows.md#op-21ce3501b5312cc762135114)

See [`Row`](../operations/arrow_row.Row.md#op-a32be7fffe0a916746ead966) for information on when [`Row`](../operations/arrow_row.Row.md#op-a32be7fffe0a916746ead966) can be compared

# Panics

Panics if
* The schema of `columns` does not match that provided to [`RowConverter::new`](../operations/arrow_row.RowConverter.md#op-0eb3cadddfeaafa7cf8f988c)
* The provided [`Rows`](../operations/arrow_row.Rows.md#op-21ce3501b5312cc762135114) were not created by this [`RowConverter`](../operations/arrow_row.RowConverter.md#op-3289371bf6ccaf9946ba91e3)

```
# use std::sync::Arc;
# use std::collections::HashSet;
# use arrow_array::cast::AsArray;
# use arrow_array::StringArray;
# use arrow_row::{Row, RowConverter, SortField};
# use arrow_schema::DataType;
#
let converter = RowConverter::new(vec![SortField::new(DataType::Utf8)]).unwrap();
let a1 = StringArray::from(vec!["hello", "world"]);
let a2 = StringArray::from(vec!["a", "a", "hello"]);

let mut rows = converter.empty_rows(5, 128);
converter.append(&mut rows, &[Arc::new(a1)]).unwrap();
converter.append(&mut rows, &[Arc::new(a2)]).unwrap();

let back = converter.convert_rows(&rows).unwrap();
let values: Vec<_> = back[0].as_string::<i32>().iter().map(Option::unwrap).collect();
assert_eq!(&values, &["hello", "world", "a", "a", "hello"]);
```

<a id="op-610a34b7b5b57c4d480dfde7"></a>
## convert_columns

`function` · `arrow_row::RowConverter::convert_columns` · arrow-row 59.3.0

```rust
fn convert_columns(&self, columns: &[ArrayRef]) -> Result<Rows, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_row::RowConverter", "path": "RowConverter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [956, 1], "end": [1276, 2], "filename": "src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `src/lib.rs:1004`. [Exact documentation build](https://docs.rs/crate/arrow-row/59.3.0/json).

Convert [`ArrayRef`](../operations/arrow_array.array.ArrayRef.md#op-657cc3ff3d24afcacda590e1) columns into [`Rows`](../operations/arrow_row.Rows.md#op-21ce3501b5312cc762135114)

See [`Row`](../operations/arrow_row.Row.md#op-a32be7fffe0a916746ead966) for information on when [`Row`](../operations/arrow_row.Row.md#op-a32be7fffe0a916746ead966) can be compared

See [`Self::convert_rows`](../operations/arrow_row.RowConverter.md#op-772d3ca81b1fb03bb6f2e3a4) for converting [`Rows`](../operations/arrow_row.Rows.md#op-21ce3501b5312cc762135114) back into [`ArrayRef`](../operations/arrow_array.array.ArrayRef.md#op-657cc3ff3d24afcacda590e1)

# Panics

Panics if the schema of `columns` does not match that provided to [`RowConverter::new`](../operations/arrow_row.RowConverter.md#op-0eb3cadddfeaafa7cf8f988c)

<a id="op-772d3ca81b1fb03bb6f2e3a4"></a>
## convert_rows

`function` · `arrow_row::RowConverter::convert_rows` · arrow-row 59.3.0

```rust
fn convert_rows<'a, I>(&self, rows: I) -> Result<Vec<ArrayRef>, ArrowError> where I: IntoIterator<Item = Row<'a>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_row::RowConverter", "path": "RowConverter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [956, 1], "end": [1276, 2], "filename": "src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `src/lib.rs:1113`. [Exact documentation build](https://docs.rs/crate/arrow-row/59.3.0/json).

Convert [`Rows`](../operations/arrow_row.Rows.md#op-21ce3501b5312cc762135114) columns into [`ArrayRef`](../operations/arrow_array.array.ArrayRef.md#op-657cc3ff3d24afcacda590e1)

See [`Self::convert_columns`](../operations/arrow_row.RowConverter.md#op-610a34b7b5b57c4d480dfde7) for converting [`ArrayRef`](../operations/arrow_array.array.ArrayRef.md#op-657cc3ff3d24afcacda590e1) into [`Rows`](../operations/arrow_row.Rows.md#op-21ce3501b5312cc762135114)

# Panics

Panics if the rows were not produced by this [`RowConverter`](../operations/arrow_row.RowConverter.md#op-3289371bf6ccaf9946ba91e3)

<a id="op-cfb061e7cbeb709e581edaea"></a>
## empty_rows

`function` · `arrow_row::RowConverter::empty_rows` · arrow-row 59.3.0

```rust
fn empty_rows(&self, row_capacity: usize, data_capacity: usize) -> Rows
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_row::RowConverter", "path": "RowConverter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [956, 1], "end": [1276, 2], "filename": "src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `src/lib.rs:1177`. [Exact documentation build](https://docs.rs/crate/arrow-row/59.3.0/json).

Returns an empty [`Rows`](../operations/arrow_row.Rows.md#op-21ce3501b5312cc762135114) with capacity for `row_capacity` rows with
a total length of `data_capacity`

This can be used to buffer a selection of [`Row`](../operations/arrow_row.Row.md#op-a32be7fffe0a916746ead966)

```
# use std::sync::Arc;
# use std::collections::HashSet;
# use arrow_array::cast::AsArray;
# use arrow_array::StringArray;
# use arrow_row::{Row, RowConverter, SortField};
# use arrow_schema::DataType;
#
let converter = RowConverter::new(vec![SortField::new(DataType::Utf8)]).unwrap();
let array = StringArray::from(vec!["hello", "world", "a", "a", "hello"]);

// Convert to row format and deduplicate
let converted = converter.convert_columns(&[Arc::new(array)]).unwrap();
let mut distinct_rows = converter.empty_rows(3, 100);
let mut dedup: HashSet<Row> = HashSet::with_capacity(3);
converted.iter().filter(|row| dedup.insert(*row)).for_each(|row| distinct_rows.push(row));

// Note: we could skip buffering and feed the filtered iterator directly
// into convert_rows, this is done for demonstration purposes only
let distinct = converter.convert_rows(&distinct_rows).unwrap();
let values: Vec<_> = distinct[0].as_string::<i32>().iter().map(Option::unwrap).collect();
assert_eq!(&values, &["hello", "world", "a"]);
```

<a id="op-8bdecee307b852c8e836a334"></a>
## fmt

`function` · `arrow_row::RowConverter::fmt` · arrow-row 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_row::RowConverter", "path": "RowConverter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [567, 10], "end": [567, 15], "filename": "src/lib.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/lib.rs:567`. [Exact documentation build](https://docs.rs/crate/arrow-row/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-917e88735bd670bb31698c8c"></a>
## from_binary

`function` · `arrow_row::RowConverter::from_binary` · arrow-row 59.3.0

```rust
fn from_binary(&self, array: BinaryArray) -> Rows
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_row::RowConverter", "path": "RowConverter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [956, 1], "end": [1276, 2], "filename": "src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `src/lib.rs:1217`. [Exact documentation build](https://docs.rs/crate/arrow-row/59.3.0/json).

Create a new [Rows](../operations/arrow_row.Rows.md#op-21ce3501b5312cc762135114) instance from the given binary data.

```
# use std::sync::Arc;
# use std::collections::HashSet;
# use arrow_array::cast::AsArray;
# use arrow_array::StringArray;
# use arrow_row::{OwnedRow, Row, RowConverter, RowParser, SortField};
# use arrow_schema::DataType;
#
let converter = RowConverter::new(vec![SortField::new(DataType::Utf8)]).unwrap();
let array = StringArray::from(vec!["hello", "world", "a", "a", "hello"]);
let rows = converter.convert_columns(&[Arc::new(array)]).unwrap();

// We can convert rows into binary format and back in batch.
let values: Vec<OwnedRow> = rows.iter().map(|r| r.owned()).collect();
let binary = rows.try_into_binary().expect("known-small array");
let converted = converter.from_binary(binary.clone());
assert!(converted.iter().eq(values.iter().map(|r| r.row())));
```

# Panics

This function expects the passed [BinaryArray](../operations/arrow_array.array.binary_array.BinaryArray.md#op-7129d9dc68ce402f881c540c) to contain valid row data as produced by this
[RowConverter](../operations/arrow_row.RowConverter.md#op-3289371bf6ccaf9946ba91e3). It will panic if any rows are null. Operations on the returned [Rows](../operations/arrow_row.Rows.md#op-21ce3501b5312cc762135114) may
panic if the data is malformed.

<a id="op-0eb3cadddfeaafa7cf8f988c"></a>
## new

`function` · `arrow_row::RowConverter::new` · arrow-row 59.3.0

```rust
fn new(fields: Vec<SortField>) -> Result<Self, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_row::RowConverter", "path": "RowConverter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [956, 1], "end": [1276, 2], "filename": "src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `src/lib.rs:958`. [Exact documentation build](https://docs.rs/crate/arrow-row/59.3.0/json).

Create a new [`RowConverter`](../operations/arrow_row.RowConverter.md#op-3289371bf6ccaf9946ba91e3) with the provided schema

<a id="op-4e836f01a5d467c50abda04a"></a>
## parser

`function` · `arrow_row::RowConverter::parser` · arrow-row 59.3.0

```rust
fn parser(&self) -> RowParser
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_row::RowConverter", "path": "RowConverter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [956, 1], "end": [1276, 2], "filename": "src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `src/lib.rs:1255`. [Exact documentation build](https://docs.rs/crate/arrow-row/59.3.0/json).

Returns a [`RowParser`](../operations/arrow_row.RowParser.md#op-955916db32b3a0fe58460e3a) that can be used to parse [`Row`](../operations/arrow_row.Row.md#op-a32be7fffe0a916746ead966) from bytes

<a id="op-b0909ae4fa1f7f96e04f05c4"></a>
## parser_skip_utf8_validation

`function` · `arrow_row::RowConverter::parser_skip_utf8_validation` · arrow-row 59.3.0

```rust
unsafe fn parser_skip_utf8_validation(&self) -> RowParser
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_row::RowConverter", "path": "RowConverter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [956, 1], "end": [1276, 2], "filename": "src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `src/lib.rs:1263`. [Exact documentation build](https://docs.rs/crate/arrow-row/59.3.0/json).

Like [`Self::parser`](../operations/arrow_row.RowConverter.md#op-4e836f01a5d467c50abda04a) but skips UTF-8 validation on decode.

# Safety
The caller must ensure all row bytes contain valid UTF-8 for string columns.

<a id="op-05e5654254329417ff05780b"></a>
## size

`function` · `arrow_row::RowConverter::size` · arrow-row 59.3.0

```rust
fn size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_row::RowConverter", "path": "RowConverter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [956, 1], "end": [1276, 2], "filename": "src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `src/lib.rs:1270`. [Exact documentation build](https://docs.rs/crate/arrow-row/59.3.0/json).

Returns the size of this instance in bytes

Includes the size of `Self`.

<a id="op-79616c2881c517486a6328ea"></a>
## supports_fields

`function` · `arrow_row::RowConverter::supports_fields` · arrow-row 59.3.0

```rust
fn supports_fields(fields: &[SortField]) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_row::RowConverter", "path": "RowConverter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [956, 1], "end": [1276, 2], "filename": "src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `src/lib.rs:973`. [Exact documentation build](https://docs.rs/crate/arrow-row/59.3.0/json).

Check if the given fields are supported by the row format.
