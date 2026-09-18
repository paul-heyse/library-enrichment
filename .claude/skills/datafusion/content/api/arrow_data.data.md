# `arrow_data::data`

Crate `arrow-data` · 6 public items · structured records in [`model/arrow_data.data.json`](../model/arrow_data.data.json)

## BufferSpec

`enum` · `arrow_data::data::BufferSpec`

Also reachable as `arrow::array::BufferSpec`

```rust
enum BufferSpec
```

**Variants**: `FixedWidth`, `VariableWidth`, `BitMap`, `AlwaysNull`

**Derives**: Debug, Eq, PartialEq, StructuralPartialEq

[Full member, field, variant and typed contracts](../operations/arrow_data.data.BufferSpec.md).


Layout specification for a single data type buffer

---

## layout

`function` · `arrow_data::data::layout`

Also reachable as `arrow::array::layout`

```rust
fn layout(data_type: &arrow_schema::DataType) -> DataTypeLayout
```

[Full member, field, variant and typed contracts](../operations/arrow_data.data.layout.md).


Return the expected [`DataTypeLayout`] Arrays of this data
type are expected to have

---

## ArrayData

`struct` · `arrow_data::data::ArrayData`

Also reachable as `arrow::array::ArrayData`

```rust
struct ArrayData
```

**Implements**: `arrow_pyarrow::FromPyArrow`, `arrow_pyarrow::ToPyArrow`, `core::convert::From`

**Derives**: Clone, Debug, PartialEq

**Methods** (30)

```rust
fn align_buffers(&mut self)
fn buffer<T: ArrowNativeType>(&self, buffer: usize) -> &[T]
fn buffers(&self) -> &[Buffer]
const fn builder(data_type: DataType) -> ArrayDataBuilder
fn child_data(&self) -> &[ArrayData]
fn claim(&self, pool: &dyn arrow_buffer::MemoryPool)
const fn data_type(&self) -> &DataType
fn get_array_memory_size(&self) -> usize
fn get_buffer_memory_size(&self) -> usize
fn get_slice_memory_size(&self) -> Result<usize, ArrowError>
fn into_builder(self) -> ArrayDataBuilder
fn into_parts(self) -> (DataType, usize, Option<NullBuffer>, usize, Vec<Buffer>, Vec<ArrayData>)
const fn is_empty(&self) -> bool
fn is_null(&self, i: usize) -> bool
fn is_valid(&self, i: usize) -> bool
const fn len(&self) -> usize
fn new_empty(data_type: &DataType) -> Self
fn new_null(data_type: &DataType, len: usize) -> Self
unsafe fn new_unchecked(data_type: DataType, len: usize, null_count: Option<usize>, null_bit_buffer: Option<Buffer>, offset: usize, buffers: Vec<Buffer>, child_data: Vec<ArrayData>) -> Self
fn null_count(&self) -> usize
fn nulls(&self) -> Option<&NullBuffer>
const fn offset(&self) -> usize
fn ptr_eq(&self, other: &Self) -> bool
fn slice(&self, offset: usize, length: usize) -> ArrayData
fn try_new(data_type: DataType, len: usize, null_bit_buffer: Option<Buffer>, offset: usize, buffers: Vec<Buffer>, child_data: Vec<ArrayData>) -> Result<Self, ArrowError>
fn validate(&self) -> Result<(), ArrowError>
fn validate_data(&self) -> Result<(), ArrowError>
fn validate_full(&self) -> Result<(), ArrowError>
fn validate_nulls(&self) -> Result<(), ArrowError>
fn validate_values(&self) -> Result<(), ArrowError>
```

[Full member, field, variant and typed contracts](../operations/arrow_data.data.ArrayData.md).


A generic representation of Arrow array data which encapsulates common attributes
and operations for Arrow array.

Specific operations for different arrays types (e.g., primitive, list, struct)
are implemented in `Array`.

# Memory Layout

`ArrayData` has references to one or more underlying data buffers
and optional child ArrayData, depending on type as illustrated
below. Bitmaps are not shown for simplicity but they are stored
similarly to the buffers.

```text
                       offset
                      points to
┌───────────────────┐ start of  ┌───────┐       Different
│                   │   data    │       │     ArrayData may
│ArrayData {        │           │....   │     also refers to
│  data_type: ...   │   ─ ─ ─ ─▶│1234   │  ┌ ─  the same
│  offset: ... ─ ─ ─│─ ┘        │4372   │      underlying
│  len: ...    ─ ─ ─│─ ┐        │4888   │  │     buffer with different offset/len
│  buffers: [       │           │5882   │◀─
│    ...            │  │        │4323   │
│  ]                │   ─ ─ ─ ─▶│4859   │
│  child_data: [    │           │....   │
│    ...            │           │       │
│  ]                │           └───────┘
│}                  │
│                   │            Shared Buffer uses
│               │   │            bytes::Bytes to hold
└───────────────────┘            actual data values
          ┌ ─ ─ ┘

          ▼
┌───────────────────┐
│ArrayData {        │
│  ...              │
│}                  │
│                   │
└───────────────────┘

Child ArrayData may also have its own buffers and children
```

---

## ArrayDataBuilder

`struct` · `arrow_data::data::ArrayDataBuilder`

Also reachable as `arrow::array::ArrayDataBuilder`

```rust
struct ArrayDataBuilder
```

**Implements**: `core::convert::From`

**Derives**: Debug

**Methods** (16)

```rust
fn add_buffer(self, b: Buffer) -> Self
fn add_buffers<I: IntoIterator<Item = Buffer>>(self, bs: I) -> Self
fn add_child_data(self, r: ArrayData) -> Self
fn align_buffers(self, align_buffers: bool) -> Self
fn buffers(self, v: Vec<Buffer>) -> Self
fn build(self) -> Result<ArrayData, ArrowError>
unsafe fn build_unchecked(self) -> ArrayData
fn child_data(self, v: Vec<ArrayData>) -> Self
fn data_type(self, data_type: DataType) -> Self
const fn len(self, n: usize) -> Self
const fn new(data_type: DataType) -> Self
fn null_bit_buffer(self, buf: Option<Buffer>) -> Self
fn null_count(self, null_count: usize) -> Self
fn nulls(self, nulls: Option<NullBuffer>) -> Self
const fn offset(self, n: usize) -> Self
unsafe fn skip_validation(self, skip_validation: bool) -> Self
```

**via `core::convert::From`**

```rust
fn from(d: ArrayData) -> Self
```

[Full member, field, variant and typed contracts](../operations/arrow_data.data.ArrayDataBuilder.md).


Builder for [`ArrayData`] type

---

## DataTypeLayout

`struct` · `arrow_data::data::DataTypeLayout`

Also reachable as `arrow::array::DataTypeLayout`

```rust
struct DataTypeLayout
```

**Fields**: `buffers`, `can_contain_null_mask`, `variadic`

**Derives**: Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (6)

```rust
fn new_binary<T>() -> Self
fn new_empty() -> Self
fn new_fixed_width<T>() -> Self
fn new_list_view<T>() -> Self
fn new_nullable_empty() -> Self
fn new_view() -> Self
```

[Full member, field, variant and typed contracts](../operations/arrow_data.data.DataTypeLayout.md).


Layout specification for a data type

---

## ArrayDataRef

`type_alias` · `arrow_data::data::ArrayDataRef`

Also reachable as `arrow::array::ArrayDataRef`

```rust
type ArrayDataRef = std::sync::Arc<ArrayData>
```

[Full member, field, variant and typed contracts](../operations/arrow_data.data.ArrayDataRef.md).


A thread-safe, shared reference to the Arrow array data.

---
