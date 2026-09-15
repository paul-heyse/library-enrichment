# `arrow_ipc::gen::Tensor`

Crate `arrow-ipc` · 16 public items · structured records in [`model/arrow_ipc.gen.Tensor.json`](../model/arrow_ipc.gen.Tensor.json)

## TensorDimOffset

`enum` · `arrow_ipc::gen::Tensor::TensorDimOffset`

Also reachable as `arrow::ipc::TensorDimOffset`, `arrow_ipc::TensorDimOffset`

```rust
enum TensorDimOffset
```

---

## TensorOffset

`enum` · `arrow_ipc::gen::Tensor::TensorOffset`

Also reachable as `arrow::ipc::TensorOffset`, `arrow_ipc::TensorOffset`

```rust
enum TensorOffset
```

---

## finish_size_prefixed_tensor_buffer

`function` · `arrow_ipc::gen::Tensor::finish_size_prefixed_tensor_buffer`

Also reachable as `arrow::ipc::finish_size_prefixed_tensor_buffer`, `arrow_ipc::finish_size_prefixed_tensor_buffer`

```rust
fn finish_size_prefixed_tensor_buffer<'a, 'b, A: flatbuffers::Allocator + 'a>(fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>, root: flatbuffers::WIPOffset<Tensor<'a>>)
```

---

## finish_tensor_buffer

`function` · `arrow_ipc::gen::Tensor::finish_tensor_buffer`

Also reachable as `arrow::ipc::finish_tensor_buffer`, `arrow_ipc::finish_tensor_buffer`

```rust
fn finish_tensor_buffer<'a, 'b, A: flatbuffers::Allocator + 'a>(fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>, root: flatbuffers::WIPOffset<Tensor<'a>>)
```

---

## root_as_tensor

`function` · `arrow_ipc::gen::Tensor::root_as_tensor`

Also reachable as `arrow::ipc::root_as_tensor`, `arrow_ipc::root_as_tensor`

```rust
fn root_as_tensor(buf: &[u8]) -> Result<Tensor<'_>, flatbuffers::InvalidFlatbuffer>
```

Verifies that a buffer of bytes contains a `Tensor`
and returns it.
Note that verification is still experimental and may not
catch every error, or be maximally performant. For the
previous, unchecked, behavior use
`root_as_tensor_unchecked`.

---

## root_as_tensor_unchecked

`function` · `arrow_ipc::gen::Tensor::root_as_tensor_unchecked`

Also reachable as `arrow::ipc::root_as_tensor_unchecked`, `arrow_ipc::root_as_tensor_unchecked`

```rust
unsafe fn root_as_tensor_unchecked(buf: &[u8]) -> Tensor<'_>
```

Assumes, without verification, that a buffer of bytes contains a Tensor and returns it.
# Safety
Callers must trust the given bytes do indeed contain a valid `Tensor`.

---

## root_as_tensor_with_opts

`function` · `arrow_ipc::gen::Tensor::root_as_tensor_with_opts`

Also reachable as `arrow::ipc::root_as_tensor_with_opts`, `arrow_ipc::root_as_tensor_with_opts`

```rust
fn root_as_tensor_with_opts<'b, 'o>(opts: &'o flatbuffers::VerifierOptions, buf: &'b [u8]) -> Result<Tensor<'b>, flatbuffers::InvalidFlatbuffer>
```

Verifies, with the given options, that a buffer of bytes
contains a `Tensor` and returns it.
Note that verification is still experimental and may not
catch every error, or be maximally performant. For the
previous, unchecked, behavior use
`root_as_tensor_unchecked`.

---

## size_prefixed_root_as_tensor

`function` · `arrow_ipc::gen::Tensor::size_prefixed_root_as_tensor`

Also reachable as `arrow::ipc::size_prefixed_root_as_tensor`, `arrow_ipc::size_prefixed_root_as_tensor`

```rust
fn size_prefixed_root_as_tensor(buf: &[u8]) -> Result<Tensor<'_>, flatbuffers::InvalidFlatbuffer>
```

Verifies that a buffer of bytes contains a size prefixed
`Tensor` and returns it.
Note that verification is still experimental and may not
catch every error, or be maximally performant. For the
previous, unchecked, behavior use
`size_prefixed_root_as_tensor_unchecked`.

---

## size_prefixed_root_as_tensor_unchecked

`function` · `arrow_ipc::gen::Tensor::size_prefixed_root_as_tensor_unchecked`

Also reachable as `arrow::ipc::size_prefixed_root_as_tensor_unchecked`, `arrow_ipc::size_prefixed_root_as_tensor_unchecked`

```rust
unsafe fn size_prefixed_root_as_tensor_unchecked(buf: &[u8]) -> Tensor<'_>
```

Assumes, without verification, that a buffer of bytes contains a size prefixed Tensor and returns it.
# Safety
Callers must trust the given bytes do indeed contain a valid size prefixed `Tensor`.

---

## size_prefixed_root_as_tensor_with_opts

`function` · `arrow_ipc::gen::Tensor::size_prefixed_root_as_tensor_with_opts`

Also reachable as `arrow::ipc::size_prefixed_root_as_tensor_with_opts`, `arrow_ipc::size_prefixed_root_as_tensor_with_opts`

```rust
fn size_prefixed_root_as_tensor_with_opts<'b, 'o>(opts: &'o flatbuffers::VerifierOptions, buf: &'b [u8]) -> Result<Tensor<'b>, flatbuffers::InvalidFlatbuffer>
```

Verifies, with the given verifier options, that a buffer of
bytes contains a size prefixed `Tensor` and returns
it. Note that verification is still experimental and may not
catch every error, or be maximally performant. For the
previous, unchecked, behavior use
`root_as_tensor_unchecked`.

---

## Tensor

`struct` · `arrow_ipc::gen::Tensor::Tensor`

Also reachable as `arrow::ipc::Tensor`, `arrow_ipc::Tensor`

```rust
struct Tensor<'a>
```

**Fields**: `_tab`

**Implements**: `flatbuffers::follow::Follow`, `flatbuffers::verifier::Verifiable`

**Derives**: Clone, Copy, Debug, PartialEq, StructuralPartialEq

**Methods** (33)

```rust
fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr, A: flatbuffers::Allocator + 'bldr>(_fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>, args: &'args TensorArgs<'args>) -> flatbuffers::WIPOffset<Tensor<'bldr>>
fn data(&self) -> &'a Buffer
unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self
fn shape(&self) -> flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<TensorDim<'a>>>
fn strides(&self) -> Option<flatbuffers::Vector<'a, i64>>
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

---

## TensorArgs

`struct` · `arrow_ipc::gen::Tensor::TensorArgs`

Also reachable as `arrow::ipc::TensorArgs`, `arrow_ipc::TensorArgs`

```rust
struct TensorArgs<'a>
```

**Fields**: `type_type`, `type_`, `shape`, `strides`, `data`

**Derives**: Default

---

## TensorBuilder

`struct` · `arrow_ipc::gen::Tensor::TensorBuilder`

Also reachable as `arrow::ipc::TensorBuilder`, `arrow_ipc::TensorBuilder`

```rust
struct TensorBuilder<'a: 'b, 'b, A: flatbuffers::Allocator + 'a>
```

**Methods** (7)

```rust
fn add_data(&mut self, data: &Buffer)
fn add_shape(&mut self, shape: flatbuffers::WIPOffset<flatbuffers::Vector<'b, flatbuffers::ForwardsUOffset<TensorDim<'b>>>>)
fn add_strides(&mut self, strides: flatbuffers::WIPOffset<flatbuffers::Vector<'b, i64>>)
fn add_type_(&mut self, type_: flatbuffers::WIPOffset<flatbuffers::UnionWIPOffset>)
fn add_type_type(&mut self, type_type: Type)
fn finish(self) -> flatbuffers::WIPOffset<Tensor<'a>>
fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>) -> TensorBuilder<'a, 'b, A>
```

---

## TensorDim

`struct` · `arrow_ipc::gen::Tensor::TensorDim`

Also reachable as `arrow::ipc::TensorDim`, `arrow_ipc::TensorDim`

```rust
struct TensorDim<'a>
```

**Fields**: `_tab`

**Implements**: `flatbuffers::follow::Follow`, `flatbuffers::verifier::Verifiable`

**Derives**: Clone, Copy, Debug, PartialEq, StructuralPartialEq

**Methods** (4)

```rust
fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr, A: flatbuffers::Allocator + 'bldr>(_fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>, args: &'args TensorDimArgs<'args>) -> flatbuffers::WIPOffset<TensorDim<'bldr>>
unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self
fn name(&self) -> Option<&'a str>
fn size(&self) -> i64
```

**via `flatbuffers::follow::Follow`**

```rust
unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner
```

**via `flatbuffers::verifier::Verifiable`**

```rust
fn run_verifier(v: &mut flatbuffers::Verifier<'_, '_>, pos: usize) -> Result<(), flatbuffers::InvalidFlatbuffer>
```

----------------------------------------------------------------------
Data structures for dense tensors
Shape data for a single axis in a tensor

---

## TensorDimArgs

`struct` · `arrow_ipc::gen::Tensor::TensorDimArgs`

Also reachable as `arrow::ipc::TensorDimArgs`, `arrow_ipc::TensorDimArgs`

```rust
struct TensorDimArgs<'a>
```

**Fields**: `size`, `name`

**Derives**: Default

---

## TensorDimBuilder

`struct` · `arrow_ipc::gen::Tensor::TensorDimBuilder`

Also reachable as `arrow::ipc::TensorDimBuilder`, `arrow_ipc::TensorDimBuilder`

```rust
struct TensorDimBuilder<'a: 'b, 'b, A: flatbuffers::Allocator + 'a>
```

**Methods** (4)

```rust
fn add_name(&mut self, name: flatbuffers::WIPOffset<&'b str>)
fn add_size(&mut self, size: i64)
fn finish(self) -> flatbuffers::WIPOffset<TensorDim<'a>>
fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>) -> TensorDimBuilder<'a, 'b, A>
```

---
