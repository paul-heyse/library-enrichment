# `arrow_row`

Crate `arrow-row` · 9 public items · structured records in [`model/arrow_row.json`](../model/arrow_row.json)

## encode_dictionary_values

`function` · `arrow_row::encode_dictionary_values`

Also reachable as `arrow::row::encode_dictionary_values`

```rust
fn encode_dictionary_values<K: ArrowDictionaryKeyType>(data: &mut [u8], offsets: &mut [usize], column: &DictionaryArray<K>, values: &Rows, null: &Row<'_>)
```

[Full member, field, variant and typed contracts](../operations/arrow_row.encode_dictionary_values.md).


Encode dictionary values not preserving the dictionary encoding

---

## OwnedRow

`struct` · `arrow_row::OwnedRow`

Also reachable as `arrow::row::OwnedRow`

```rust
struct OwnedRow
```

**Implements**: `core::convert::AsRef`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd

**Methods** (1)

```rust
fn row(&self) -> Row<'_>
```

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &[u8]
```

[Full member, field, variant and typed contracts](../operations/arrow_row.OwnedRow.md).


Owned version of a [`Row`] that can be moved/cloned freely.

This contains the data for the one specific row (not the entire buffer of all rows).

---

## Row

`struct` · `arrow_row::Row`

Also reachable as `arrow::row::Row`

```rust
struct Row<'a>
```

**Implements**: `core::convert::AsRef`

**Derives**: Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd

**Methods** (2)

```rust
fn data(&self) -> &'a [u8]
fn owned(&self) -> OwnedRow
```

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &[u8]
```

[Full member, field, variant and typed contracts](../operations/arrow_row.Row.md).


A comparable representation of a row.

See the [module level documentation](self) for more details.

Two [`Row`] can only be compared if they both belong to [`Rows`]
returned by calls to [`RowConverter::convert_columns`] on the same
[`RowConverter`]. If different [`RowConverter`]s are used, any
ordering established by comparing the [`Row`] is arbitrary.

---

## RowConverter

`struct` · `arrow_row::RowConverter`

Also reachable as `arrow::row::RowConverter`

```rust
struct RowConverter
```

**Derives**: Debug

**Methods** (10)

```rust
fn append(&self, rows: &mut Rows, columns: &[ArrayRef]) -> Result<(), ArrowError>
fn convert_columns(&self, columns: &[ArrayRef]) -> Result<Rows, ArrowError>
fn convert_rows<'a, I>(&self, rows: I) -> Result<Vec<ArrayRef>, ArrowError> where I: IntoIterator<Item = Row<'a>>
fn empty_rows(&self, row_capacity: usize, data_capacity: usize) -> Rows
fn from_binary(&self, array: BinaryArray) -> Rows
fn new(fields: Vec<SortField>) -> Result<Self, ArrowError>
fn parser(&self) -> RowParser
unsafe fn parser_skip_utf8_validation(&self) -> RowParser
fn size(&self) -> usize
fn supports_fields(fields: &[SortField]) -> bool
```

[Full member, field, variant and typed contracts](../operations/arrow_row.RowConverter.md).


 Converts [`ArrayRef`] columns into a [row-oriented](self) format.

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

 See [`UnionArray`] for more details on union types.

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

---

## RowParser

`struct` · `arrow_row::RowParser`

Also reachable as `arrow::row::RowParser`

```rust
struct RowParser
```

**Derives**: Debug

**Methods** (1)

```rust
fn parse<'a>(&'a self, bytes: &'a [u8]) -> Row<'a>
```

[Full member, field, variant and typed contracts](../operations/arrow_row.RowParser.md).


A [`RowParser`] can be created from a [`RowConverter`] and used to parse bytes to [`Row`]

---

## Rows

`struct` · `arrow_row::Rows`

Also reachable as `arrow::row::Rows`

```rust
struct Rows
```

**Derives**: Clone, Debug

**Methods** (11)

```rust
fn clear(&mut self)
fn iter(&self) -> RowsIter<'_>
fn lengths(&self) -> RowLengthIter<'_>
fn num_rows(&self) -> usize
fn push(&mut self, row: Row<'_>)
fn reserve(&mut self, row_capacity: usize, data_capacity: usize)
fn row(&self, row: usize) -> Row<'_>
fn row_len(&self, row: usize) -> usize
unsafe fn row_unchecked(&self, index: usize) -> Row<'_>
fn size(&self) -> usize
fn try_into_binary(self) -> Result<BinaryArray, ArrowError>
```

[Full member, field, variant and typed contracts](../operations/arrow_row.Rows.md).


A row-oriented representation of arrow data, that is normalized for comparison.

See the [module level documentation](self) and [`RowConverter`] for more details.

---

## RowsIter

`struct` · `arrow_row::RowsIter`

Also reachable as `arrow::row::RowsIter`

```rust
struct RowsIter<'a>
```

**Implements**: `core::iter::traits::double_ended::DoubleEndedIterator`, `core::iter::traits::exact_size::ExactSizeIterator`, `core::iter::traits::iterator::Iterator`

**Derives**: Debug

**via `core::iter::traits::double_ended::DoubleEndedIterator`**

```rust
fn next_back(&mut self) -> Option<Self::Item>
```

**via `core::iter::traits::exact_size::ExactSizeIterator`**

```rust
fn len(&self) -> usize
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<Self::Item>
fn size_hint(&self) -> (usize, Option<usize>)
```

[Full member, field, variant and typed contracts](../operations/arrow_row.RowsIter.md).


An iterator over [`Rows`]

---

## SortField

`struct` · `arrow_row::SortField`

Also reachable as `arrow::row::SortField`

```rust
struct SortField
```

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn new(data_type: DataType) -> Self
fn new_with_options(data_type: DataType, options: SortOptions) -> Self
fn size(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/arrow_row.SortField.md).


Configure the data type and sort order for a given column

---

## RowLengthIter

`type_alias` · `arrow_row::RowLengthIter`

Also reachable as `arrow::row::RowLengthIter`

```rust
type RowLengthIter<'a> = std::iter::Map<std::slice::Windows<'a, usize>, fn(&'a [usize]) -> usize>
```

[Full member, field, variant and typed contracts](../operations/arrow_row.RowLengthIter.md).


The iterator type for [`Rows::lengths`]

---
