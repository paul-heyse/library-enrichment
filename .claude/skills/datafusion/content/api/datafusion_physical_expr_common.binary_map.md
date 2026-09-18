# `datafusion_physical_expr_common::binary_map`

Crate `datafusion-physical-expr-common` · 4 public items · structured records in [`model/datafusion_physical_expr_common.binary_map.json`](../model/datafusion_physical_expr_common.binary_map.json)

## INITIAL_BUFFER_CAPACITY

`constant` · `datafusion_physical_expr_common::binary_map::INITIAL_BUFFER_CAPACITY`

```rust
const INITIAL_BUFFER_CAPACITY: usize = _
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_expr_common.binary_map.INITIAL_BUFFER_CAPACITY.md).


The initial size, in bytes, of the string data

---

## OutputType

`enum` · `datafusion_physical_expr_common::binary_map::OutputType`

Also reachable as `datafusion_physical_expr::binary_map::OutputType`

```rust
enum OutputType
```

**Variants**: `Utf8`, `Utf8View`, `Binary`, `BinaryView`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

[Full member, field, variant and typed contracts](../operations/datafusion_physical_expr_common.binary_map.OutputType.md).


Should the output be a String or Binary?

---

## ArrowBytesMap

`struct` · `datafusion_physical_expr_common::binary_map::ArrowBytesMap`

```rust
struct ArrowBytesMap<O, V> where O: OffsetSizeTrait, V: Debug + PartialEq + Eq + Clone + Copy + Default
```

**Derives**: Debug

**Methods** (8)

```rust
fn insert_if_new<MP, OP>(&mut self, values: &ArrayRef, make_payload_fn: MP, observe_payload_fn: OP) where MP: FnMut(Option<&[u8]>) -> V, OP: FnMut(V)
fn into_state(self) -> ArrayRef
fn is_empty(&self) -> bool
fn len(&self) -> usize
fn new(output_type: OutputType) -> Self
fn non_null_len(&self) -> usize
fn size(&self) -> usize
fn take(&mut self) -> Self
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_expr_common.binary_map.ArrowBytesMap.md).


Optimized map for storing Arrow "bytes" types (`String`, `LargeString`,
`Binary`, and `LargeBinary`) values that can produce the set of keys on
output as `GenericBinaryArray` without copies.

Equivalent to `HashSet<String, V>` but with better performance if you need
to emit the keys as an Arrow `StringArray` / `BinaryArray`. For other
purposes it is the same as a `HashMap<String, V>`

# Generic Arguments

* `O`: OffsetSize (String/LargeString)
* `V`: payload type

# Description

This is a specialized HashMap with the following properties:

1. Optimized for storing and emitting Arrow byte types  (e.g.
   `StringArray` / `BinaryArray`) very efficiently by minimizing copying of
   the string values themselves, both when inserting and when emitting the
   final array.


2. Retains the insertion order of entries in the final array. The values are
   in the same order as they were inserted.

Note this structure can be used as a `HashSet` by specifying the value type
as `()`, as is done by [`ArrowBytesSet`].

This map is used by the special `COUNT DISTINCT` aggregate function to
store the distinct values, and by the `GROUP BY` operator to store
group values when they are a single string array.

# Example

The following diagram shows how the map would store the four strings
"Foo", NULL, "Bar", "TheQuickBrownFox":

* `hashtable` stores entries for each distinct string that has been
  inserted. The entries contain the payload as well as information about the
  value (either an offset or the actual bytes, see `Entry` docs for more
  details)

* `offsets` stores offsets into `buffer` for each distinct string value,
  following the same convention as the offsets in a `StringArray` or
  `LargeStringArray`.

* `buffer` stores the actual byte data

* `null`: stores the index and payload of the null value, in this case the
  second value (index 1)

```text
┌───────────────────────────────────┐    ┌─────┐    ┌────┐
│                ...                │    │  0  │    │FooB│
│ ┌──────────────────────────────┐  │    │  0  │    │arTh│
│ │      <Entry for "Bar">       │  │    │  3  │    │eQui│
│ │            len: 3            │  │    │  3  │    │ckBr│
│ │   offset_or_inline: "Bar"    │  │    │  6  │    │ownF│
│ │         payload:...          │  │    │     │    │ox  │
│ └──────────────────────────────┘  │    │     │    │    │
│                ...                │    └─────┘    └────┘
│ ┌──────────────────────────────┐  │
│ │<Entry for "TheQuickBrownFox">│  │    offsets    buffer
│ │           len: 16            │  │
│ │     offset_or_inline: 6      │  │    ┌───────────────┐
│ │         payload: ...         │  │    │    Some(1)    │
│ └──────────────────────────────┘  │    │ payload: ...  │
│                ...                │    └───────────────┘
└───────────────────────────────────┘
                                             null
              HashTable
```

# Entry Format

Entries stored in a [`ArrowBytesMap`] represents a value that is either
stored inline or in the buffer

This helps the case where there are many short (less than 8 bytes) strings
that are the same (e.g. "MA", "CA", "NY", "TX", etc)

```text
                                                               ┌──────────────────┐
                                                 ─ ─ ─ ─ ─ ─ ─▶│...               │
                                                │              │TheQuickBrownFox  │
                                                               │...               │
                                                │              │                  │
                                                               └──────────────────┘
                                                │               buffer of u8

                                                │
                       ┌────────────────┬───────────────┬───────────────┐
 Storing               │                │ starting byte │  length, in   │
 "TheQuickBrownFox"    │   hash value   │   offset in   │  bytes (not   │
 (long string)         │                │    buffer     │  characters)  │
                       └────────────────┴───────────────┴───────────────┘
                             8 bytes          8 bytes       4 or 8


                        ┌───────────────┬─┬─┬─┬─┬─┬─┬─┬─┬───────────────┐
Storing "foobar"        │               │ │ │ │ │ │ │ │ │  length, in   │
(short string)          │  hash value   │?│?│f│o│o│b│a│r│  bytes (not   │
                        │               │ │ │ │ │ │ │ │ │  characters)  │
                        └───────────────┴─┴─┴─┴─┴─┴─┴─┴─┴───────────────┘
                             8 bytes         8 bytes        4 or 8
```

---

## ArrowBytesSet

`struct` · `datafusion_physical_expr_common::binary_map::ArrowBytesSet`

Also reachable as `datafusion_physical_expr::binary_map::ArrowBytesSet`

```rust
struct ArrowBytesSet<O: OffsetSizeTrait>
```

**Derives**: Debug

**Methods** (8)

```rust
fn insert(&mut self, values: &ArrayRef)
fn into_state(self) -> ArrayRef
fn is_empty(&self) -> bool
fn len(&self) -> usize
fn new(output_type: OutputType) -> Self
fn non_null_len(&self) -> usize
fn size(&self) -> usize
fn take(&mut self) -> Self
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_expr_common.binary_map.ArrowBytesSet.md).


HashSet optimized for storing string or binary values that can produce that
the final set as a GenericStringArray with minimal copies.

---
