# `arrow_ipc::gen::SparseTensor`

Crate `arrow-ipc` · 33 public items · structured records in [`model/arrow_ipc.gen.SparseTensor.json`](../model/arrow_ipc.gen.SparseTensor.json)

## ENUM_MAX_SPARSE_MATRIX_COMPRESSED_AXIS

`constant` · `arrow_ipc::gen::SparseTensor::ENUM_MAX_SPARSE_MATRIX_COMPRESSED_AXIS`

> **Deprecated** — since 2.0.0: Use associated constants instead. This will no longer be generated in 2021.

Also reachable as `arrow::ipc::ENUM_MAX_SPARSE_MATRIX_COMPRESSED_AXIS`, `arrow_ipc::ENUM_MAX_SPARSE_MATRIX_COMPRESSED_AXIS`

```rust
const ENUM_MAX_SPARSE_MATRIX_COMPRESSED_AXIS: i16 = 1
```

[Full member, field, variant and typed contracts](../operations/arrow_ipc.gen.SparseTensor.ENUM_MAX_SPARSE_MATRIX_COMPRESSED_AXIS.md).


---

## ENUM_MAX_SPARSE_TENSOR_INDEX

`constant` · `arrow_ipc::gen::SparseTensor::ENUM_MAX_SPARSE_TENSOR_INDEX`

> **Deprecated** — since 2.0.0: Use associated constants instead. This will no longer be generated in 2021.

Also reachable as `arrow::ipc::ENUM_MAX_SPARSE_TENSOR_INDEX`, `arrow_ipc::ENUM_MAX_SPARSE_TENSOR_INDEX`

```rust
const ENUM_MAX_SPARSE_TENSOR_INDEX: u8 = 3
```

[Full member, field, variant and typed contracts](../operations/arrow_ipc.gen.SparseTensor.ENUM_MAX_SPARSE_TENSOR_INDEX.md).


---

## ENUM_MIN_SPARSE_MATRIX_COMPRESSED_AXIS

`constant` · `arrow_ipc::gen::SparseTensor::ENUM_MIN_SPARSE_MATRIX_COMPRESSED_AXIS`

> **Deprecated** — since 2.0.0: Use associated constants instead. This will no longer be generated in 2021.

Also reachable as `arrow::ipc::ENUM_MIN_SPARSE_MATRIX_COMPRESSED_AXIS`, `arrow_ipc::ENUM_MIN_SPARSE_MATRIX_COMPRESSED_AXIS`

```rust
const ENUM_MIN_SPARSE_MATRIX_COMPRESSED_AXIS: i16 = 0
```

[Full member, field, variant and typed contracts](../operations/arrow_ipc.gen.SparseTensor.ENUM_MIN_SPARSE_MATRIX_COMPRESSED_AXIS.md).


---

## ENUM_MIN_SPARSE_TENSOR_INDEX

`constant` · `arrow_ipc::gen::SparseTensor::ENUM_MIN_SPARSE_TENSOR_INDEX`

> **Deprecated** — since 2.0.0: Use associated constants instead. This will no longer be generated in 2021.

Also reachable as `arrow::ipc::ENUM_MIN_SPARSE_TENSOR_INDEX`, `arrow_ipc::ENUM_MIN_SPARSE_TENSOR_INDEX`

```rust
const ENUM_MIN_SPARSE_TENSOR_INDEX: u8 = 0
```

[Full member, field, variant and typed contracts](../operations/arrow_ipc.gen.SparseTensor.ENUM_MIN_SPARSE_TENSOR_INDEX.md).


---

## ENUM_VALUES_SPARSE_MATRIX_COMPRESSED_AXIS

`constant` · `arrow_ipc::gen::SparseTensor::ENUM_VALUES_SPARSE_MATRIX_COMPRESSED_AXIS`

> **Deprecated** — since 2.0.0: Use associated constants instead. This will no longer be generated in 2021.

Also reachable as `arrow::ipc::ENUM_VALUES_SPARSE_MATRIX_COMPRESSED_AXIS`, `arrow_ipc::ENUM_VALUES_SPARSE_MATRIX_COMPRESSED_AXIS`

```rust
const ENUM_VALUES_SPARSE_MATRIX_COMPRESSED_AXIS: [SparseMatrixCompressedAxis; 2] = _
```

[Full member, field, variant and typed contracts](../operations/arrow_ipc.gen.SparseTensor.ENUM_VALUES_SPARSE_MATRIX_COMPRESSED_AXIS.md).


---

## ENUM_VALUES_SPARSE_TENSOR_INDEX

`constant` · `arrow_ipc::gen::SparseTensor::ENUM_VALUES_SPARSE_TENSOR_INDEX`

> **Deprecated** — since 2.0.0: Use associated constants instead. This will no longer be generated in 2021.

Also reachable as `arrow::ipc::ENUM_VALUES_SPARSE_TENSOR_INDEX`, `arrow_ipc::ENUM_VALUES_SPARSE_TENSOR_INDEX`

```rust
const ENUM_VALUES_SPARSE_TENSOR_INDEX: [SparseTensorIndex; 4] = _
```

[Full member, field, variant and typed contracts](../operations/arrow_ipc.gen.SparseTensor.ENUM_VALUES_SPARSE_TENSOR_INDEX.md).


---

## SparseMatrixIndexCSXOffset

`enum` · `arrow_ipc::gen::SparseTensor::SparseMatrixIndexCSXOffset`

Also reachable as `arrow::ipc::SparseMatrixIndexCSXOffset`, `arrow_ipc::SparseMatrixIndexCSXOffset`

```rust
enum SparseMatrixIndexCSXOffset
```

[Full member, field, variant and typed contracts](../operations/arrow_ipc.gen.SparseTensor.SparseMatrixIndexCSXOffset.md).


---

## SparseTensorIndexCOOOffset

`enum` · `arrow_ipc::gen::SparseTensor::SparseTensorIndexCOOOffset`

Also reachable as `arrow::ipc::SparseTensorIndexCOOOffset`, `arrow_ipc::SparseTensorIndexCOOOffset`

```rust
enum SparseTensorIndexCOOOffset
```

[Full member, field, variant and typed contracts](../operations/arrow_ipc.gen.SparseTensor.SparseTensorIndexCOOOffset.md).


---

## SparseTensorIndexCSFOffset

`enum` · `arrow_ipc::gen::SparseTensor::SparseTensorIndexCSFOffset`

Also reachable as `arrow::ipc::SparseTensorIndexCSFOffset`, `arrow_ipc::SparseTensorIndexCSFOffset`

```rust
enum SparseTensorIndexCSFOffset
```

[Full member, field, variant and typed contracts](../operations/arrow_ipc.gen.SparseTensor.SparseTensorIndexCSFOffset.md).


---

## SparseTensorOffset

`enum` · `arrow_ipc::gen::SparseTensor::SparseTensorOffset`

Also reachable as `arrow::ipc::SparseTensorOffset`, `arrow_ipc::SparseTensorOffset`

```rust
enum SparseTensorOffset
```

[Full member, field, variant and typed contracts](../operations/arrow_ipc.gen.SparseTensor.SparseTensorOffset.md).


---

## finish_size_prefixed_sparse_tensor_buffer

`function` · `arrow_ipc::gen::SparseTensor::finish_size_prefixed_sparse_tensor_buffer`

Also reachable as `arrow::ipc::finish_size_prefixed_sparse_tensor_buffer`, `arrow_ipc::finish_size_prefixed_sparse_tensor_buffer`

```rust
fn finish_size_prefixed_sparse_tensor_buffer<'a, 'b, A: flatbuffers::Allocator + 'a>(fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>, root: flatbuffers::WIPOffset<SparseTensor<'a>>)
```

[Full member, field, variant and typed contracts](../operations/arrow_ipc.gen.SparseTensor.finish_size_prefixed_sparse_tensor_buffer.md).


---

## finish_sparse_tensor_buffer

`function` · `arrow_ipc::gen::SparseTensor::finish_sparse_tensor_buffer`

Also reachable as `arrow::ipc::finish_sparse_tensor_buffer`, `arrow_ipc::finish_sparse_tensor_buffer`

```rust
fn finish_sparse_tensor_buffer<'a, 'b, A: flatbuffers::Allocator + 'a>(fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>, root: flatbuffers::WIPOffset<SparseTensor<'a>>)
```

[Full member, field, variant and typed contracts](../operations/arrow_ipc.gen.SparseTensor.finish_sparse_tensor_buffer.md).


---

## root_as_sparse_tensor

`function` · `arrow_ipc::gen::SparseTensor::root_as_sparse_tensor`

Also reachable as `arrow::ipc::root_as_sparse_tensor`, `arrow_ipc::root_as_sparse_tensor`

```rust
fn root_as_sparse_tensor(buf: &[u8]) -> Result<SparseTensor<'_>, flatbuffers::InvalidFlatbuffer>
```

[Full member, field, variant and typed contracts](../operations/arrow_ipc.gen.SparseTensor.root_as_sparse_tensor.md).


Verifies that a buffer of bytes contains a `SparseTensor`
and returns it.
Note that verification is still experimental and may not
catch every error, or be maximally performant. For the
previous, unchecked, behavior use
`root_as_sparse_tensor_unchecked`.

---

## root_as_sparse_tensor_unchecked

`function` · `arrow_ipc::gen::SparseTensor::root_as_sparse_tensor_unchecked`

Also reachable as `arrow::ipc::root_as_sparse_tensor_unchecked`, `arrow_ipc::root_as_sparse_tensor_unchecked`

```rust
unsafe fn root_as_sparse_tensor_unchecked(buf: &[u8]) -> SparseTensor<'_>
```

[Full member, field, variant and typed contracts](../operations/arrow_ipc.gen.SparseTensor.root_as_sparse_tensor_unchecked.md).


Assumes, without verification, that a buffer of bytes contains a SparseTensor and returns it.
# Safety
Callers must trust the given bytes do indeed contain a valid `SparseTensor`.

---

## root_as_sparse_tensor_with_opts

`function` · `arrow_ipc::gen::SparseTensor::root_as_sparse_tensor_with_opts`

Also reachable as `arrow::ipc::root_as_sparse_tensor_with_opts`, `arrow_ipc::root_as_sparse_tensor_with_opts`

```rust
fn root_as_sparse_tensor_with_opts<'b, 'o>(opts: &'o flatbuffers::VerifierOptions, buf: &'b [u8]) -> Result<SparseTensor<'b>, flatbuffers::InvalidFlatbuffer>
```

[Full member, field, variant and typed contracts](../operations/arrow_ipc.gen.SparseTensor.root_as_sparse_tensor_with_opts.md).


Verifies, with the given options, that a buffer of bytes
contains a `SparseTensor` and returns it.
Note that verification is still experimental and may not
catch every error, or be maximally performant. For the
previous, unchecked, behavior use
`root_as_sparse_tensor_unchecked`.

---

## size_prefixed_root_as_sparse_tensor

`function` · `arrow_ipc::gen::SparseTensor::size_prefixed_root_as_sparse_tensor`

Also reachable as `arrow::ipc::size_prefixed_root_as_sparse_tensor`, `arrow_ipc::size_prefixed_root_as_sparse_tensor`

```rust
fn size_prefixed_root_as_sparse_tensor(buf: &[u8]) -> Result<SparseTensor<'_>, flatbuffers::InvalidFlatbuffer>
```

[Full member, field, variant and typed contracts](../operations/arrow_ipc.gen.SparseTensor.size_prefixed_root_as_sparse_tensor.md).


Verifies that a buffer of bytes contains a size prefixed
`SparseTensor` and returns it.
Note that verification is still experimental and may not
catch every error, or be maximally performant. For the
previous, unchecked, behavior use
`size_prefixed_root_as_sparse_tensor_unchecked`.

---

## size_prefixed_root_as_sparse_tensor_unchecked

`function` · `arrow_ipc::gen::SparseTensor::size_prefixed_root_as_sparse_tensor_unchecked`

Also reachable as `arrow::ipc::size_prefixed_root_as_sparse_tensor_unchecked`, `arrow_ipc::size_prefixed_root_as_sparse_tensor_unchecked`

```rust
unsafe fn size_prefixed_root_as_sparse_tensor_unchecked(buf: &[u8]) -> SparseTensor<'_>
```

[Full member, field, variant and typed contracts](../operations/arrow_ipc.gen.SparseTensor.size_prefixed_root_as_sparse_tensor_unchecked.md).


Assumes, without verification, that a buffer of bytes contains a size prefixed SparseTensor and returns it.
# Safety
Callers must trust the given bytes do indeed contain a valid size prefixed `SparseTensor`.

---

## size_prefixed_root_as_sparse_tensor_with_opts

`function` · `arrow_ipc::gen::SparseTensor::size_prefixed_root_as_sparse_tensor_with_opts`

Also reachable as `arrow::ipc::size_prefixed_root_as_sparse_tensor_with_opts`, `arrow_ipc::size_prefixed_root_as_sparse_tensor_with_opts`

```rust
fn size_prefixed_root_as_sparse_tensor_with_opts<'b, 'o>(opts: &'o flatbuffers::VerifierOptions, buf: &'b [u8]) -> Result<SparseTensor<'b>, flatbuffers::InvalidFlatbuffer>
```

[Full member, field, variant and typed contracts](../operations/arrow_ipc.gen.SparseTensor.size_prefixed_root_as_sparse_tensor_with_opts.md).


Verifies, with the given verifier options, that a buffer of
bytes contains a size prefixed `SparseTensor` and returns
it. Note that verification is still experimental and may not
catch every error, or be maximally performant. For the
previous, unchecked, behavior use
`root_as_sparse_tensor_unchecked`.

---

## SparseMatrixCompressedAxis

`struct` · `arrow_ipc::gen::SparseTensor::SparseMatrixCompressedAxis`

Also reachable as `arrow::ipc::SparseMatrixCompressedAxis`, `arrow_ipc::SparseMatrixCompressedAxis`

```rust
struct SparseMatrixCompressedAxis
```

**Implements**: `flatbuffers::endian_scalar::EndianScalar`, `flatbuffers::follow::Follow`, `flatbuffers::push::Push`, `flatbuffers::verifier::SimpleToVerifyInSlice`, `flatbuffers::verifier::Verifiable`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (1)

```rust
fn variant_name(self) -> Option<&'static str>
```

**via `flatbuffers::endian_scalar::EndianScalar`**

```rust
fn from_little_endian(v: i16) -> Self
fn to_little_endian(self) -> i16
```

**via `flatbuffers::follow::Follow`**

```rust
unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner
```

**via `flatbuffers::push::Push`**

```rust
unsafe fn push(&self, dst: &mut [u8], _written_len: usize)
```

**via `flatbuffers::verifier::Verifiable`**

```rust
fn run_verifier(v: &mut flatbuffers::Verifier<'_, '_>, pos: usize) -> Result<(), flatbuffers::InvalidFlatbuffer>
```

[Full member, field, variant and typed contracts](../operations/arrow_ipc.gen.SparseTensor.SparseMatrixCompressedAxis.md).


---

## SparseMatrixIndexCSX

`struct` · `arrow_ipc::gen::SparseTensor::SparseMatrixIndexCSX`

Also reachable as `arrow::ipc::SparseMatrixIndexCSX`, `arrow_ipc::SparseMatrixIndexCSX`

```rust
struct SparseMatrixIndexCSX<'a>
```

**Fields**: `_tab`

**Implements**: `flatbuffers::follow::Follow`, `flatbuffers::verifier::Verifiable`

**Derives**: Clone, Copy, Debug, PartialEq, StructuralPartialEq

**Methods** (7)

```rust
fn compressedAxis(&self) -> SparseMatrixCompressedAxis
fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr, A: flatbuffers::Allocator + 'bldr>(_fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>, args: &'args SparseMatrixIndexCSXArgs<'args>) -> flatbuffers::WIPOffset<SparseMatrixIndexCSX<'bldr>>
fn indicesBuffer(&self) -> &'a Buffer
fn indicesType(&self) -> Int<'a>
fn indptrBuffer(&self) -> &'a Buffer
fn indptrType(&self) -> Int<'a>
unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self
```

**via `flatbuffers::follow::Follow`**

```rust
unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner
```

**via `flatbuffers::verifier::Verifiable`**

```rust
fn run_verifier(v: &mut flatbuffers::Verifier<'_, '_>, pos: usize) -> Result<(), flatbuffers::InvalidFlatbuffer>
```

[Full member, field, variant and typed contracts](../operations/arrow_ipc.gen.SparseTensor.SparseMatrixIndexCSX.md).


Compressed Sparse format, that is matrix-specific.

---

## SparseMatrixIndexCSXArgs

`struct` · `arrow_ipc::gen::SparseTensor::SparseMatrixIndexCSXArgs`

Also reachable as `arrow::ipc::SparseMatrixIndexCSXArgs`, `arrow_ipc::SparseMatrixIndexCSXArgs`

```rust
struct SparseMatrixIndexCSXArgs<'a>
```

**Fields**: `compressedAxis`, `indptrType`, `indptrBuffer`, `indicesType`, `indicesBuffer`

**Derives**: Default

[Full member, field, variant and typed contracts](../operations/arrow_ipc.gen.SparseTensor.SparseMatrixIndexCSXArgs.md).


---

## SparseMatrixIndexCSXBuilder

`struct` · `arrow_ipc::gen::SparseTensor::SparseMatrixIndexCSXBuilder`

Also reachable as `arrow::ipc::SparseMatrixIndexCSXBuilder`, `arrow_ipc::SparseMatrixIndexCSXBuilder`

```rust
struct SparseMatrixIndexCSXBuilder<'a: 'b, 'b, A: flatbuffers::Allocator + 'a>
```

**Methods** (7)

```rust
fn add_compressedAxis(&mut self, compressedAxis: SparseMatrixCompressedAxis)
fn add_indicesBuffer(&mut self, indicesBuffer: &Buffer)
fn add_indicesType(&mut self, indicesType: flatbuffers::WIPOffset<Int<'b>>)
fn add_indptrBuffer(&mut self, indptrBuffer: &Buffer)
fn add_indptrType(&mut self, indptrType: flatbuffers::WIPOffset<Int<'b>>)
fn finish(self) -> flatbuffers::WIPOffset<SparseMatrixIndexCSX<'a>>
fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>) -> SparseMatrixIndexCSXBuilder<'a, 'b, A>
```

[Full member, field, variant and typed contracts](../operations/arrow_ipc.gen.SparseTensor.SparseMatrixIndexCSXBuilder.md).


---

## SparseTensor

`struct` · `arrow_ipc::gen::SparseTensor::SparseTensor`

Also reachable as `arrow::ipc::SparseTensor`, `arrow_ipc::SparseTensor`

```rust
struct SparseTensor<'a>
```

**Fields**: `_tab`

**Implements**: `flatbuffers::follow::Follow`, `flatbuffers::verifier::Verifiable`

**Derives**: Clone, Copy, Debug, PartialEq, StructuralPartialEq

**Methods** (38)

```rust
fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr, A: flatbuffers::Allocator + 'bldr>(_fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>, args: &'args SparseTensorArgs<'args>) -> flatbuffers::WIPOffset<SparseTensor<'bldr>>
fn data(&self) -> &'a Buffer
unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self
fn non_zero_length(&self) -> i64
fn shape(&self) -> flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<TensorDim<'a>>>
fn sparseIndex(&self) -> flatbuffers::Table<'a>
fn sparseIndex_as_sparse_matrix_index_csx(&self) -> Option<SparseMatrixIndexCSX<'a>>
fn sparseIndex_as_sparse_tensor_index_coo(&self) -> Option<SparseTensorIndexCOO<'a>>
fn sparseIndex_as_sparse_tensor_index_csf(&self) -> Option<SparseTensorIndexCSF<'a>>
fn sparseIndex_type(&self) -> SparseTensorIndex
fn type_(&self) -> flatbuffers::Table<'a>
fn type_as_binary(&self) -> Option<Binary<'a>>
fn type_as_binary_view(&self) -> Option<BinaryView<'a>>
fn type_as_bool(&self) -> Option<Bool<'a>>
fn type_as_date(&self) -> Option<Date<'a>>
fn type_as_decimal(&self) -> Option<Decimal<'a>>
fn type_as_duration(&self) -> Option<Duration<'a>>
fn type_as_fixed_size_binary(&self) -> Option<FixedSizeBinary<'a>>
fn type_as_fixed_size_list(&self) -> Option<FixedSizeList<'a>>
fn type_as_floating_point(&self) -> Option<FloatingPoint<'a>>
fn type_as_int(&self) -> Option<Int<'a>>
fn type_as_interval(&self) -> Option<Interval<'a>>
fn type_as_large_binary(&self) -> Option<LargeBinary<'a>>
fn type_as_large_list(&self) -> Option<LargeList<'a>>
fn type_as_large_list_view(&self) -> Option<LargeListView<'a>>
fn type_as_large_utf_8(&self) -> Option<LargeUtf8<'a>>
fn type_as_list(&self) -> Option<List<'a>>
fn type_as_list_view(&self) -> Option<ListView<'a>>
fn type_as_map(&self) -> Option<Map<'a>>
fn type_as_null(&self) -> Option<Null<'a>>
fn type_as_run_end_encoded(&self) -> Option<RunEndEncoded<'a>>
fn type_as_struct_(&self) -> Option<Struct_<'a>>
fn type_as_time(&self) -> Option<Time<'a>>
fn type_as_timestamp(&self) -> Option<Timestamp<'a>>
fn type_as_union(&self) -> Option<Union<'a>>
fn type_as_utf_8(&self) -> Option<Utf8<'a>>
fn type_as_utf_8_view(&self) -> Option<Utf8View<'a>>
fn type_type(&self) -> Type
```

**via `flatbuffers::follow::Follow`**

```rust
unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner
```

**via `flatbuffers::verifier::Verifiable`**

```rust
fn run_verifier(v: &mut flatbuffers::Verifier<'_, '_>, pos: usize) -> Result<(), flatbuffers::InvalidFlatbuffer>
```

[Full member, field, variant and typed contracts](../operations/arrow_ipc.gen.SparseTensor.SparseTensor.md).


---

## SparseTensorArgs

`struct` · `arrow_ipc::gen::SparseTensor::SparseTensorArgs`

Also reachable as `arrow::ipc::SparseTensorArgs`, `arrow_ipc::SparseTensorArgs`

```rust
struct SparseTensorArgs<'a>
```

**Fields**: `type_type`, `type_`, `shape`, `non_zero_length`, `sparseIndex_type`, `sparseIndex`, `data`

**Derives**: Default

[Full member, field, variant and typed contracts](../operations/arrow_ipc.gen.SparseTensor.SparseTensorArgs.md).


---

## SparseTensorBuilder

`struct` · `arrow_ipc::gen::SparseTensor::SparseTensorBuilder`

Also reachable as `arrow::ipc::SparseTensorBuilder`, `arrow_ipc::SparseTensorBuilder`

```rust
struct SparseTensorBuilder<'a: 'b, 'b, A: flatbuffers::Allocator + 'a>
```

**Methods** (9)

```rust
fn add_data(&mut self, data: &Buffer)
fn add_non_zero_length(&mut self, non_zero_length: i64)
fn add_shape(&mut self, shape: flatbuffers::WIPOffset<flatbuffers::Vector<'b, flatbuffers::ForwardsUOffset<TensorDim<'b>>>>)
fn add_sparseIndex(&mut self, sparseIndex: flatbuffers::WIPOffset<flatbuffers::UnionWIPOffset>)
fn add_sparseIndex_type(&mut self, sparseIndex_type: SparseTensorIndex)
fn add_type_(&mut self, type_: flatbuffers::WIPOffset<flatbuffers::UnionWIPOffset>)
fn add_type_type(&mut self, type_type: Type)
fn finish(self) -> flatbuffers::WIPOffset<SparseTensor<'a>>
fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>) -> SparseTensorBuilder<'a, 'b, A>
```

[Full member, field, variant and typed contracts](../operations/arrow_ipc.gen.SparseTensor.SparseTensorBuilder.md).


---

## SparseTensorIndex

`struct` · `arrow_ipc::gen::SparseTensor::SparseTensorIndex`

Also reachable as `arrow::ipc::SparseTensorIndex`, `arrow_ipc::SparseTensorIndex`

```rust
struct SparseTensorIndex
```

**Implements**: `flatbuffers::endian_scalar::EndianScalar`, `flatbuffers::follow::Follow`, `flatbuffers::push::Push`, `flatbuffers::verifier::SimpleToVerifyInSlice`, `flatbuffers::verifier::Verifiable`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (1)

```rust
fn variant_name(self) -> Option<&'static str>
```

**via `flatbuffers::endian_scalar::EndianScalar`**

```rust
fn from_little_endian(v: u8) -> Self
fn to_little_endian(self) -> u8
```

**via `flatbuffers::follow::Follow`**

```rust
unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner
```

**via `flatbuffers::push::Push`**

```rust
unsafe fn push(&self, dst: &mut [u8], _written_len: usize)
```

**via `flatbuffers::verifier::Verifiable`**

```rust
fn run_verifier(v: &mut flatbuffers::Verifier<'_, '_>, pos: usize) -> Result<(), flatbuffers::InvalidFlatbuffer>
```

[Full member, field, variant and typed contracts](../operations/arrow_ipc.gen.SparseTensor.SparseTensorIndex.md).


---

## SparseTensorIndexCOO

`struct` · `arrow_ipc::gen::SparseTensor::SparseTensorIndexCOO`

Also reachable as `arrow::ipc::SparseTensorIndexCOO`, `arrow_ipc::SparseTensorIndexCOO`

```rust
struct SparseTensorIndexCOO<'a>
```

**Fields**: `_tab`

**Implements**: `flatbuffers::follow::Follow`, `flatbuffers::verifier::Verifiable`

**Derives**: Clone, Copy, Debug, PartialEq, StructuralPartialEq

**Methods** (6)

```rust
fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr, A: flatbuffers::Allocator + 'bldr>(_fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>, args: &'args SparseTensorIndexCOOArgs<'args>) -> flatbuffers::WIPOffset<SparseTensorIndexCOO<'bldr>>
fn indicesBuffer(&self) -> &'a Buffer
fn indicesStrides(&self) -> Option<flatbuffers::Vector<'a, i64>>
fn indicesType(&self) -> Int<'a>
unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self
fn isCanonical(&self) -> bool
```

**via `flatbuffers::follow::Follow`**

```rust
unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner
```

**via `flatbuffers::verifier::Verifiable`**

```rust
fn run_verifier(v: &mut flatbuffers::Verifier<'_, '_>, pos: usize) -> Result<(), flatbuffers::InvalidFlatbuffer>
```

[Full member, field, variant and typed contracts](../operations/arrow_ipc.gen.SparseTensor.SparseTensorIndexCOO.md).


----------------------------------------------------------------------
EXPERIMENTAL: Data structures for sparse tensors
Coordinate (COO) format of sparse tensor index.

COO's index list are represented as a NxM matrix,
where N is the number of non-zero values,
and M is the number of dimensions of a sparse tensor.

indicesBuffer stores the location and size of the data of this indices
matrix.  The value type and the stride of the indices matrix is
specified in indicesType and indicesStrides fields.

For example, let X be a 2x3x4x5 tensor, and it has the following
6 non-zero values:
```text
  X[0, 1, 2, 0] := 1
  X[1, 1, 2, 3] := 2
  X[0, 2, 1, 0] := 3
  X[0, 1, 3, 0] := 4
  X[0, 1, 2, 1] := 5
  X[1, 2, 0, 4] := 6
```
In COO format, the index matrix of X is the following 4x6 matrix:
```text
  [[0, 0, 0, 0, 1, 1],
   [1, 1, 1, 2, 1, 2],
   [2, 2, 3, 1, 2, 0],
   [0, 1, 0, 0, 3, 4]]
```
When isCanonical is true, the indices is sorted in lexicographical order
(row-major order), and it does not have duplicated entries.  Otherwise,
the indices may not be sorted, or may have duplicated entries.

---

## SparseTensorIndexCOOArgs

`struct` · `arrow_ipc::gen::SparseTensor::SparseTensorIndexCOOArgs`

Also reachable as `arrow::ipc::SparseTensorIndexCOOArgs`, `arrow_ipc::SparseTensorIndexCOOArgs`

```rust
struct SparseTensorIndexCOOArgs<'a>
```

**Fields**: `indicesType`, `indicesStrides`, `indicesBuffer`, `isCanonical`

**Derives**: Default

[Full member, field, variant and typed contracts](../operations/arrow_ipc.gen.SparseTensor.SparseTensorIndexCOOArgs.md).


---

## SparseTensorIndexCOOBuilder

`struct` · `arrow_ipc::gen::SparseTensor::SparseTensorIndexCOOBuilder`

Also reachable as `arrow::ipc::SparseTensorIndexCOOBuilder`, `arrow_ipc::SparseTensorIndexCOOBuilder`

```rust
struct SparseTensorIndexCOOBuilder<'a: 'b, 'b, A: flatbuffers::Allocator + 'a>
```

**Methods** (6)

```rust
fn add_indicesBuffer(&mut self, indicesBuffer: &Buffer)
fn add_indicesStrides(&mut self, indicesStrides: flatbuffers::WIPOffset<flatbuffers::Vector<'b, i64>>)
fn add_indicesType(&mut self, indicesType: flatbuffers::WIPOffset<Int<'b>>)
fn add_isCanonical(&mut self, isCanonical: bool)
fn finish(self) -> flatbuffers::WIPOffset<SparseTensorIndexCOO<'a>>
fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>) -> SparseTensorIndexCOOBuilder<'a, 'b, A>
```

[Full member, field, variant and typed contracts](../operations/arrow_ipc.gen.SparseTensor.SparseTensorIndexCOOBuilder.md).


---

## SparseTensorIndexCSF

`struct` · `arrow_ipc::gen::SparseTensor::SparseTensorIndexCSF`

Also reachable as `arrow::ipc::SparseTensorIndexCSF`, `arrow_ipc::SparseTensorIndexCSF`

```rust
struct SparseTensorIndexCSF<'a>
```

**Fields**: `_tab`

**Implements**: `flatbuffers::follow::Follow`, `flatbuffers::verifier::Verifiable`

**Derives**: Clone, Copy, Debug, PartialEq, StructuralPartialEq

**Methods** (7)

```rust
fn axisOrder(&self) -> flatbuffers::Vector<'a, i32>
fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr, A: flatbuffers::Allocator + 'bldr>(_fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>, args: &'args SparseTensorIndexCSFArgs<'args>) -> flatbuffers::WIPOffset<SparseTensorIndexCSF<'bldr>>
fn indicesBuffers(&self) -> flatbuffers::Vector<'a, Buffer>
fn indicesType(&self) -> Int<'a>
fn indptrBuffers(&self) -> flatbuffers::Vector<'a, Buffer>
fn indptrType(&self) -> Int<'a>
unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self
```

**via `flatbuffers::follow::Follow`**

```rust
unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner
```

**via `flatbuffers::verifier::Verifiable`**

```rust
fn run_verifier(v: &mut flatbuffers::Verifier<'_, '_>, pos: usize) -> Result<(), flatbuffers::InvalidFlatbuffer>
```

[Full member, field, variant and typed contracts](../operations/arrow_ipc.gen.SparseTensor.SparseTensorIndexCSF.md).


Compressed Sparse Fiber (CSF) sparse tensor index.

---

## SparseTensorIndexCSFArgs

`struct` · `arrow_ipc::gen::SparseTensor::SparseTensorIndexCSFArgs`

Also reachable as `arrow::ipc::SparseTensorIndexCSFArgs`, `arrow_ipc::SparseTensorIndexCSFArgs`

```rust
struct SparseTensorIndexCSFArgs<'a>
```

**Fields**: `indptrType`, `indptrBuffers`, `indicesType`, `indicesBuffers`, `axisOrder`

**Derives**: Default

[Full member, field, variant and typed contracts](../operations/arrow_ipc.gen.SparseTensor.SparseTensorIndexCSFArgs.md).


---

## SparseTensorIndexCSFBuilder

`struct` · `arrow_ipc::gen::SparseTensor::SparseTensorIndexCSFBuilder`

Also reachable as `arrow::ipc::SparseTensorIndexCSFBuilder`, `arrow_ipc::SparseTensorIndexCSFBuilder`

```rust
struct SparseTensorIndexCSFBuilder<'a: 'b, 'b, A: flatbuffers::Allocator + 'a>
```

**Methods** (7)

```rust
fn add_axisOrder(&mut self, axisOrder: flatbuffers::WIPOffset<flatbuffers::Vector<'b, i32>>)
fn add_indicesBuffers(&mut self, indicesBuffers: flatbuffers::WIPOffset<flatbuffers::Vector<'b, Buffer>>)
fn add_indicesType(&mut self, indicesType: flatbuffers::WIPOffset<Int<'b>>)
fn add_indptrBuffers(&mut self, indptrBuffers: flatbuffers::WIPOffset<flatbuffers::Vector<'b, Buffer>>)
fn add_indptrType(&mut self, indptrType: flatbuffers::WIPOffset<Int<'b>>)
fn finish(self) -> flatbuffers::WIPOffset<SparseTensorIndexCSF<'a>>
fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>) -> SparseTensorIndexCSFBuilder<'a, 'b, A>
```

[Full member, field, variant and typed contracts](../operations/arrow_ipc.gen.SparseTensor.SparseTensorIndexCSFBuilder.md).


---

## SparseTensorIndexUnionTableOffset

`struct` · `arrow_ipc::gen::SparseTensor::SparseTensorIndexUnionTableOffset`

Also reachable as `arrow::ipc::SparseTensorIndexUnionTableOffset`, `arrow_ipc::SparseTensorIndexUnionTableOffset`

```rust
struct SparseTensorIndexUnionTableOffset
```

[Full member, field, variant and typed contracts](../operations/arrow_ipc.gen.SparseTensor.SparseTensorIndexUnionTableOffset.md).


---
