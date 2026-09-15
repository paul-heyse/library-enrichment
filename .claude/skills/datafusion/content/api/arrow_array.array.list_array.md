# `arrow_array::array::list_array`

Crate `arrow-array` · 4 public items · structured records in [`model/arrow_array.array.list_array.json`](../model/arrow_array.array.list_array.json)

## GenericListArray

`struct` · `arrow_array::array::list_array::GenericListArray`

```rust
struct GenericListArray<OffsetSize: OffsetSizeTrait>
```

**Implements**: `arrow_array::array::Array`, `arrow_array::array::ListLikeArray`, `core::convert::From`

**Derives**: Clone, Debug, PartialEq

**Methods** (16)

```rust
fn from_iter_primitive<T, P, I>(iter: I) -> Self where T: ArrowPrimitiveType, P: IntoIterator<Item = Option<<T as ArrowPrimitiveType>::Native>>, I: IntoIterator<Item = Option<P>>
fn from_nested_iter<B, T, P, I>(iter: I) -> Self where B: ArrayBuilder + Default + Extend<Option<T>>, P: IntoIterator<Item = Option<T>>, I: IntoIterator<Item = Option<P>>
fn into_parts(self) -> (FieldRef, OffsetBuffer<OffsetSize>, ArrayRef, Option<NullBuffer>)
fn iter<'a>(&'a self) -> GenericListArrayIter<'a, OffsetSize>
fn new(field: FieldRef, offsets: OffsetBuffer<OffsetSize>, values: ArrayRef, nulls: Option<NullBuffer>) -> Self
fn new_null(field: FieldRef, len: usize) -> Self
unsafe fn new_unchecked(field: FieldRef, offsets: OffsetBuffer<OffsetSize>, values: ArrayRef, nulls: Option<NullBuffer>) -> Self
fn offsets(&self) -> &OffsetBuffer<OffsetSize>
fn slice(&self, offset: usize, length: usize) -> Self
fn try_new(field: FieldRef, offsets: OffsetBuffer<OffsetSize>, values: ArrayRef, nulls: Option<NullBuffer>) -> Result<Self, ArrowError>
fn value(&self, i: usize) -> ArrayRef
fn value_length(&self, i: usize) -> OffsetSize
fn value_offsets(&self) -> &[OffsetSize]
fn value_type(&self) -> DataType
unsafe fn value_unchecked(&self, i: usize) -> ArrayRef
fn values(&self) -> &ArrayRef
```

**via `arrow_array::array::Array`**

```rust
fn as_any(&self) -> &dyn Any
fn claim(&self, pool: &dyn arrow_buffer::MemoryPool)
fn data_type(&self) -> &DataType
fn get_array_memory_size(&self) -> usize
fn get_buffer_memory_size(&self) -> usize
fn into_data(self) -> ArrayData
fn is_empty(&self) -> bool
fn len(&self) -> usize
fn logical_null_count(&self) -> usize
fn nulls(&self) -> Option<&NullBuffer>
fn offset(&self) -> usize
fn shrink_to_fit(&mut self)
fn slice(&self, offset: usize, length: usize) -> ArrayRef
fn to_data(&self) -> ArrayData
```

**via `arrow_array::array::ListLikeArray`**

```rust
fn element_range(&self, index: usize) -> std::ops::Range<usize>
fn values(&self) -> &ArrayRef
```

**via `core::convert::From`**

```rust
fn from(data: ArrayData) -> Self
fn from(value: FixedSizeListArray) -> Self
```

An array of [variable length lists], similar to JSON arrays
(e.g. `["A", "B", "C"]`). This struct specifically represents
the [list layout]. Refer to [`GenericListViewArray`] for the
[list-view layout].

Lists are represented using `offsets` into a `values` child
array. Offsets are stored in two adjacent entries of an
[`OffsetBuffer`].

Arrow defines [`ListArray`] with `i32` offsets and
[`LargeListArray`] with `i64` offsets.

Use [`GenericListBuilder`] to construct a [`GenericListArray`].

# Representation

A [`ListArray`] can represent a list of values of any other
supported Arrow type. Each element of the `ListArray` itself is
a list which may be empty, may contain NULL and non-null values,
or may itself be NULL.

For example, the `ListArray` shown in the following diagram stores
lists of strings. Note that `[]` represents an empty (length
0), but non NULL list.

```text
┌─────────────┐
│   [A,B,C]   │
├─────────────┤
│     []      │
├─────────────┤
│    NULL     │
├─────────────┤
│     [D]     │
├─────────────┤
│  [NULL, F]  │
└─────────────┘
```

The `values` are stored in a child [`StringArray`] and the offsets
are stored in an [`OffsetBuffer`] as shown in the following
diagram. The logical values and offsets are shown on the left, and
the actual `ListArray` encoding on the right.

```text
                                        ┌ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─
                                                                ┌ ─ ─ ─ ─ ─ ─ ┐    │
 ┌─────────────┐  ┌───────┐             │     ┌───┐   ┌───┐       ┌───┐ ┌───┐
 │   [A,B,C]   │  │ (0,3) │                   │ 1 │   │ 0 │     │ │ 1 │ │ A │ │ 0  │
 ├─────────────┤  ├───────┤             │     ├───┤   ├───┤       ├───┤ ├───┤
 │ [] (empty)  │  │ (3,3) │                   │ 1 │   │ 3 │     │ │ 1 │ │ B │ │ 1  │
 ├─────────────┤  ├───────┤             │     ├───┤   ├───┤       ├───┤ ├───┤
 │    NULL     │  │ (3,3) │                   │ 0 │   │ 3 │     │ │ 1 │ │ C │ │ 2  │
 ├─────────────┤  ├───────┤             │     ├───┤   ├───┤       ├───┤ ├───┤
 │     [D]     │  │ (3,4) │                   │ 1 │   │ 3 │     │ │ 1 │ │ D │ │ 3  │
 ├─────────────┤  ├───────┤             │     ├───┤   ├───┤       ├───┤ ├───┤
 │  [NULL, F]  │  │ (4,6) │                   │ 1 │   │ 4 │     │ │ 0 │ │ ? │ │ 4  │
 └─────────────┘  └───────┘             │     └───┘   ├───┤       ├───┤ ├───┤
                                                      │ 6 │     │ │ 1 │ │ F │ │ 5  │
                                        │  Validity   └───┘       └───┘ └───┘
    Logical       Logical                  (nulls)   Offsets    │    Values   │    │
     Values       Offsets               │                           (Array)
                                                                └ ─ ─ ─ ─ ─ ─ ┘    │
                (offsets[i],            │   ListArray
               offsets[i+1])                                                       │
                                        └ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─
```

# Slicing

Slicing a `ListArray` creates a new `ListArray` without copying any data,
but this means the [`Self::values`] and [`Self::offsets`] may have "unused" data

For example, calling `slice(1, 3)` on the `ListArray` in the above example
would result in the following. Note

1. `Values` array is unchanged
2. `Offsets` do not start at `0`, nor cover all values in the Values array.

```text
                                ┌ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─
                                                        ┌ ─ ─ ─ ─ ─ ─ ┐    │  ╔═══╗
                                │                         ╔═══╗ ╔═══╗         ║   ║  Not used
                                                        │ ║ 1 ║ ║ A ║ │ 0  │  ╚═══╝
 ┌─────────────┐  ┌───────┐     │     ┌───┐   ┌───┐       ╠═══╣ ╠═══╣
 │ [] (empty)  │  │ (3,3) │           │ 1 │   │ 3 │     │ ║ 1 ║ ║ B ║ │ 1  │
 ├─────────────┤  ├───────┤     │     ├───┤   ├───┤       ╠═══╣ ╠═══╣
 │    NULL     │  │ (3,3) │           │ 0 │   │ 3 │     │ ║ 1 ║ ║ C ║ │ 2  │
 ├─────────────┤  ├───────┤     │     ├───┤   ├───┤       ╚═══╝ ╚═══╝
 │     [D]     │  │ (3,4) │           │ 1 │   │ 3 │     │ │ 1 │ │ D │ │ 3  │
 └─────────────┘  └───────┘     │     └───┘   ├───┤       ╔═══╗ ╔═══╗
                                              │ 4 │     │ ║ 0 ║ ║ ? ║ │ 4  │
                                │             └───┘       ╠═══╣ ╠═══╣
                                                        │ ║ 1 ║ ║ F ║ │ 5  │
                                │  Validity               ╚═══╝ ╚═══╝
    Logical       Logical          (nulls)   Offsets    │    Values   │    │
     Values       Offsets       │                           (Array)
                                                        └ ─ ─ ─ ─ ─ ─ ┘    │
                (offsets[i],    │   ListArray
               offsets[i+1])                                               │
                                └ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─
```

[`StringArray`]: crate::array::StringArray
[`GenericListViewArray`]: crate::array::GenericListViewArray
[variable length lists]: https://arrow.apache.org/docs/format/Columnar.html#variable-size-list-layout
[list layout]: https://arrow.apache.org/docs/format/Columnar.html#list-layout
[list-view layout]: https://arrow.apache.org/docs/format/Columnar.html#listview-layout

---

## OffsetSizeTrait

`trait` · `arrow_array::array::list_array::OffsetSizeTrait`

```rust
trait OffsetSizeTrait: ArrowNativeType + std::ops::AddAssign + Integer + num_traits::CheckedAdd + num_traits::CheckedSub
```

A type that can be used within a variable-size array to encode offset information

See [`ListArray`], [`LargeListArray`], [`BinaryArray`], [`LargeBinaryArray`],
[`StringArray`] and [`LargeStringArray`]

[`BinaryArray`]: crate::array::BinaryArray
[`LargeBinaryArray`]: crate::array::LargeBinaryArray
[`StringArray`]: crate::array::StringArray
[`LargeStringArray`]: crate::array::LargeStringArray

---

## LargeListArray

`type_alias` · `arrow_array::array::list_array::LargeListArray`

```rust
type LargeListArray = GenericListArray<i64>
```

**Implements**: `datafusion_common::heap_size::DFHeapSize`

A [`GenericListArray`] of variable size lists, storing offsets as `i64`.

See [`LargeListBuilder`](crate::builder::LargeListBuilder) for how to construct a [`LargeListArray`]

---

## ListArray

`type_alias` · `arrow_array::array::list_array::ListArray`

```rust
type ListArray = GenericListArray<i32>
```

**Implements**: `core::convert::From`, `datafusion_common::heap_size::DFHeapSize`

**via `core::convert::From`**

```rust
fn from(value: MapArray) -> Self
```

A [`GenericListArray`] of variable size lists, storing offsets as `i32`.

See [`ListBuilder`](crate::builder::ListBuilder) for how to construct a [`ListArray`]

---
