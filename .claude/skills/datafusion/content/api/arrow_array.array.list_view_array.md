# `arrow_array::array::list_view_array`

Crate `arrow-array` · 3 public items · structured records in [`model/arrow_array.array.list_view_array.json`](../model/arrow_array.array.list_view_array.json)

## GenericListViewArray

`struct` · `arrow_array::array::list_view_array::GenericListViewArray`

```rust
struct GenericListViewArray<OffsetSize: OffsetSizeTrait>
```

**Implements**: `arrow_array::array::Array`, `arrow_array::array::ListLikeArray`, `core::convert::From`

**Derives**: Clone, Debug, PartialEq

**Methods** (18)

```rust
fn from_iter_primitive<T, P, I>(iter: I) -> Self where T: ArrowPrimitiveType, P: IntoIterator<Item = Option<<T as ArrowPrimitiveType>::Native>>, I: IntoIterator<Item = Option<P>>
fn into_parts(self) -> (FieldRef, ScalarBuffer<OffsetSize>, ScalarBuffer<OffsetSize>, ArrayRef, Option<NullBuffer>)
fn iter(&self) -> GenericListViewArrayIter<'_, OffsetSize>
fn new(field: FieldRef, offsets: ScalarBuffer<OffsetSize>, sizes: ScalarBuffer<OffsetSize>, values: ArrayRef, nulls: Option<NullBuffer>) -> Self
fn new_null(field: FieldRef, len: usize) -> Self
unsafe fn new_unchecked(field: FieldRef, offsets: ScalarBuffer<OffsetSize>, sizes: ScalarBuffer<OffsetSize>, values: ArrayRef, nulls: Option<NullBuffer>) -> Self
fn offsets(&self) -> &ScalarBuffer<OffsetSize>
fn sizes(&self) -> &ScalarBuffer<OffsetSize>
fn slice(&self, offset: usize, length: usize) -> Self
fn try_new(field: FieldRef, offsets: ScalarBuffer<OffsetSize>, sizes: ScalarBuffer<OffsetSize>, values: ArrayRef, nulls: Option<NullBuffer>) -> Result<Self, ArrowError>
fn value(&self, i: usize) -> ArrayRef
fn value_offset(&self, i: usize) -> OffsetSize
fn value_offsets(&self) -> &[OffsetSize]
fn value_size(&self, i: usize) -> OffsetSize
fn value_sizes(&self) -> &[OffsetSize]
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
fn from(value: FixedSizeListArray) -> Self
fn from(value: GenericListArray<OffsetSize>) -> Self
fn from(data: ArrayData) -> Self
```

[Full member, field, variant and typed contracts](../operations/arrow_array.array.list_view_array.GenericListViewArray.md).


An array of [variable length lists], specifically in the [list-view layout].

Differs from [`GenericListArray`] (which represents the [list layout]) in that
the sizes of the child arrays are explicitly encoded in a separate buffer, instead
of being derived from the difference between subsequent offsets in the offset buffer.

This allows the offsets (and subsequently child data) to be out of order. It also
allows take / filter operations to be implemented without copying the underlying data.

# Representation

Given the same example array from [`GenericListArray`], it would be represented
as such via a list-view layout array:

```text
                                        ┌ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─
                                                                        ┌ ─ ─ ─ ─ ─ ─ ┐    │
 ┌─────────────┐  ┌───────┐             │     ┌───┐   ┌───┐   ┌───┐       ┌───┐ ┌───┐
 │   [A,B,C]   │  │ (0,3) │                   │ 1 │   │ 0 │   │ 3 │     │ │ 1 │ │ A │ │ 0  │
 ├─────────────┤  ├───────┤             │     ├───┤   ├───┤   ├───┤       ├───┤ ├───┤
 │      []     │  │ (3,0) │                   │ 1 │   │ 3 │   │ 0 │     │ │ 1 │ │ B │ │ 1  │
 ├─────────────┤  ├───────┤             │     ├───┤   ├───┤   ├───┤       ├───┤ ├───┤
 │    NULL     │  │ (?,?) │                   │ 0 │   │ ? │   │ ? │     │ │ 1 │ │ C │ │ 2  │
 ├─────────────┤  ├───────┤             │     ├───┤   ├───┤   ├───┤       ├───┤ ├───┤
 │     [D]     │  │ (4,1) │                   │ 1 │   │ 4 │   │ 1 │     │ │ ? │ │ ? │ │ 3  │
 ├─────────────┤  ├───────┤             │     ├───┤   ├───┤   ├───┤       ├───┤ ├───┤
 │  [NULL, F]  │  │ (5,2) │                   │ 1 │   │ 5 │   │ 2 │     │ │ 1 │ │ D │ │ 4  │
 └─────────────┘  └───────┘             │     └───┘   └───┘   └───┘       ├───┤ ├───┤
                                                                        │ │ 0 │ │ ? │ │ 5  │
    Logical       Logical               │  Validity  Offsets  Sizes       ├───┤ ├───┤
     Values       Offset                   (nulls)                      │ │ 1 │ │ F │ │ 6  │
                  & Size                │                                 └───┘ └───┘
                                                                        │    Values   │    │
                (offsets[i],            │   ListViewArray                   (Array)
                 sizes[i])                                              └ ─ ─ ─ ─ ─ ─ ┘    │
                                        └ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─
```

Another way of representing the same array but taking advantage of the offsets being out of order:

```text
                                        ┌ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─
                                                                        ┌ ─ ─ ─ ─ ─ ─ ┐    │
 ┌─────────────┐  ┌───────┐             │     ┌───┐   ┌───┐   ┌───┐       ┌───┐ ┌───┐
 │   [A,B,C]   │  │ (2,3) │                   │ 1 │   │ 2 │   │ 3 │     │ │ 0 │ │ ? │ │ 0  │
 ├─────────────┤  ├───────┤             │     ├───┤   ├───┤   ├───┤       ├───┤ ├───┤
 │      []     │  │ (0,0) │                   │ 1 │   │ 0 │   │ 0 │     │ │ 1 │ │ F │ │ 1  │
 ├─────────────┤  ├───────┤             │     ├───┤   ├───┤   ├───┤       ├───┤ ├───┤
 │    NULL     │  │ (?,?) │                   │ 0 │   │ ? │   │ ? │     │ │ 1 │ │ A │ │ 2  │
 ├─────────────┤  ├───────┤             │     ├───┤   ├───┤   ├───┤       ├───┤ ├───┤
 │     [D]     │  │ (5,1) │                   │ 1 │   │ 5 │   │ 1 │     │ │ 1 │ │ B │ │ 3  │
 ├─────────────┤  ├───────┤             │     ├───┤   ├───┤   ├───┤       ├───┤ ├───┤
 │  [NULL, F]  │  │ (0,2) │                   │ 1 │   │ 0 │   │ 2 │     │ │ 1 │ │ C │ │ 4  │
 └─────────────┘  └───────┘             │     └───┘   └───┘   └───┘       ├───┤ ├───┤
                                                                        │ │ 1 │ │ D │ │ 5  │
    Logical       Logical               │  Validity  Offsets  Sizes       └───┘ └───┘
     Values       Offset                   (nulls)                      │    Values   │    │
                  & Size                │                                   (Array)
                                                                        └ ─ ─ ─ ─ ─ ─ ┘    │
                (offsets[i],            │   ListViewArray
                 sizes[i])                                                                 │
                                        └ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─
```

[`GenericListArray`]: crate::array::GenericListArray
[variable length lists]: https://arrow.apache.org/docs/format/Columnar.html#variable-size-list-layout
[list layout]: https://arrow.apache.org/docs/format/Columnar.html#list-layout
[list-view layout]: https://arrow.apache.org/docs/format/Columnar.html#listview-layout

---

## LargeListViewArray

`type_alias` · `arrow_array::array::list_view_array::LargeListViewArray`

```rust
type LargeListViewArray = GenericListViewArray<i64>
```

**Implements**: `datafusion_common::heap_size::DFHeapSize`

[Full member, field, variant and typed contracts](../operations/arrow_array.array.list_view_array.LargeListViewArray.md).


A [`GenericListViewArray`] of variable size lists, storing offsets as `i64`.

---

## ListViewArray

`type_alias` · `arrow_array::array::list_view_array::ListViewArray`

```rust
type ListViewArray = GenericListViewArray<i32>
```

**Implements**: `datafusion_common::heap_size::DFHeapSize`

[Full member, field, variant and typed contracts](../operations/arrow_array.array.list_view_array.ListViewArray.md).


A [`GenericListViewArray`] of variable size lists, storing offsets as `i32`.

---
