# `arrow_ipc::gen::Message`

Crate `arrow-ipc` · 38 public items · structured records in [`model/arrow_ipc.gen.Message.json`](../model/arrow_ipc.gen.Message.json)

## ENUM_MAX_BODY_COMPRESSION_METHOD

`constant` · `arrow_ipc::gen::Message::ENUM_MAX_BODY_COMPRESSION_METHOD`

> **Deprecated** — since 2.0.0: Use associated constants instead. This will no longer be generated in 2021.

Also reachable as `arrow::ipc::ENUM_MAX_BODY_COMPRESSION_METHOD`, `arrow_ipc::ENUM_MAX_BODY_COMPRESSION_METHOD`

```rust
const ENUM_MAX_BODY_COMPRESSION_METHOD: i8 = 0
```

---

## ENUM_MAX_COMPRESSION_TYPE

`constant` · `arrow_ipc::gen::Message::ENUM_MAX_COMPRESSION_TYPE`

> **Deprecated** — since 2.0.0: Use associated constants instead. This will no longer be generated in 2021.

Also reachable as `arrow::ipc::ENUM_MAX_COMPRESSION_TYPE`, `arrow_ipc::ENUM_MAX_COMPRESSION_TYPE`

```rust
const ENUM_MAX_COMPRESSION_TYPE: i8 = 1
```

---

## ENUM_MAX_MESSAGE_HEADER

`constant` · `arrow_ipc::gen::Message::ENUM_MAX_MESSAGE_HEADER`

> **Deprecated** — since 2.0.0: Use associated constants instead. This will no longer be generated in 2021.

Also reachable as `arrow::ipc::ENUM_MAX_MESSAGE_HEADER`, `arrow_ipc::ENUM_MAX_MESSAGE_HEADER`

```rust
const ENUM_MAX_MESSAGE_HEADER: u8 = 5
```

---

## ENUM_MIN_BODY_COMPRESSION_METHOD

`constant` · `arrow_ipc::gen::Message::ENUM_MIN_BODY_COMPRESSION_METHOD`

> **Deprecated** — since 2.0.0: Use associated constants instead. This will no longer be generated in 2021.

Also reachable as `arrow::ipc::ENUM_MIN_BODY_COMPRESSION_METHOD`, `arrow_ipc::ENUM_MIN_BODY_COMPRESSION_METHOD`

```rust
const ENUM_MIN_BODY_COMPRESSION_METHOD: i8 = 0
```

---

## ENUM_MIN_COMPRESSION_TYPE

`constant` · `arrow_ipc::gen::Message::ENUM_MIN_COMPRESSION_TYPE`

> **Deprecated** — since 2.0.0: Use associated constants instead. This will no longer be generated in 2021.

Also reachable as `arrow::ipc::ENUM_MIN_COMPRESSION_TYPE`, `arrow_ipc::ENUM_MIN_COMPRESSION_TYPE`

```rust
const ENUM_MIN_COMPRESSION_TYPE: i8 = 0
```

---

## ENUM_MIN_MESSAGE_HEADER

`constant` · `arrow_ipc::gen::Message::ENUM_MIN_MESSAGE_HEADER`

> **Deprecated** — since 2.0.0: Use associated constants instead. This will no longer be generated in 2021.

Also reachable as `arrow::ipc::ENUM_MIN_MESSAGE_HEADER`, `arrow_ipc::ENUM_MIN_MESSAGE_HEADER`

```rust
const ENUM_MIN_MESSAGE_HEADER: u8 = 0
```

---

## ENUM_VALUES_BODY_COMPRESSION_METHOD

`constant` · `arrow_ipc::gen::Message::ENUM_VALUES_BODY_COMPRESSION_METHOD`

> **Deprecated** — since 2.0.0: Use associated constants instead. This will no longer be generated in 2021.

Also reachable as `arrow::ipc::ENUM_VALUES_BODY_COMPRESSION_METHOD`, `arrow_ipc::ENUM_VALUES_BODY_COMPRESSION_METHOD`

```rust
const ENUM_VALUES_BODY_COMPRESSION_METHOD: [BodyCompressionMethod; 1] = _
```

---

## ENUM_VALUES_COMPRESSION_TYPE

`constant` · `arrow_ipc::gen::Message::ENUM_VALUES_COMPRESSION_TYPE`

> **Deprecated** — since 2.0.0: Use associated constants instead. This will no longer be generated in 2021.

Also reachable as `arrow::ipc::ENUM_VALUES_COMPRESSION_TYPE`, `arrow_ipc::ENUM_VALUES_COMPRESSION_TYPE`

```rust
const ENUM_VALUES_COMPRESSION_TYPE: [CompressionType; 2] = _
```

---

## ENUM_VALUES_MESSAGE_HEADER

`constant` · `arrow_ipc::gen::Message::ENUM_VALUES_MESSAGE_HEADER`

> **Deprecated** — since 2.0.0: Use associated constants instead. This will no longer be generated in 2021.

Also reachable as `arrow::ipc::ENUM_VALUES_MESSAGE_HEADER`, `arrow_ipc::ENUM_VALUES_MESSAGE_HEADER`

```rust
const ENUM_VALUES_MESSAGE_HEADER: [MessageHeader; 6] = _
```

---

## BodyCompressionOffset

`enum` · `arrow_ipc::gen::Message::BodyCompressionOffset`

Also reachable as `arrow::ipc::BodyCompressionOffset`, `arrow_ipc::BodyCompressionOffset`

```rust
enum BodyCompressionOffset
```

---

## DictionaryBatchOffset

`enum` · `arrow_ipc::gen::Message::DictionaryBatchOffset`

Also reachable as `arrow::ipc::DictionaryBatchOffset`, `arrow_ipc::DictionaryBatchOffset`

```rust
enum DictionaryBatchOffset
```

---

## MessageOffset

`enum` · `arrow_ipc::gen::Message::MessageOffset`

Also reachable as `arrow::ipc::MessageOffset`, `arrow_ipc::MessageOffset`

```rust
enum MessageOffset
```

---

## RecordBatchOffset

`enum` · `arrow_ipc::gen::Message::RecordBatchOffset`

Also reachable as `arrow::ipc::RecordBatchOffset`, `arrow_ipc::RecordBatchOffset`

```rust
enum RecordBatchOffset
```

---

## finish_message_buffer

`function` · `arrow_ipc::gen::Message::finish_message_buffer`

Also reachable as `arrow::ipc::finish_message_buffer`, `arrow_ipc::finish_message_buffer`

```rust
fn finish_message_buffer<'a, 'b, A: flatbuffers::Allocator + 'a>(fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>, root: flatbuffers::WIPOffset<Message<'a>>)
```

---

## finish_size_prefixed_message_buffer

`function` · `arrow_ipc::gen::Message::finish_size_prefixed_message_buffer`

Also reachable as `arrow::ipc::finish_size_prefixed_message_buffer`, `arrow_ipc::finish_size_prefixed_message_buffer`

```rust
fn finish_size_prefixed_message_buffer<'a, 'b, A: flatbuffers::Allocator + 'a>(fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>, root: flatbuffers::WIPOffset<Message<'a>>)
```

---

## root_as_message

`function` · `arrow_ipc::gen::Message::root_as_message`

Also reachable as `arrow::ipc::root_as_message`, `arrow_ipc::root_as_message`

```rust
fn root_as_message(buf: &[u8]) -> Result<Message<'_>, flatbuffers::InvalidFlatbuffer>
```

Verifies that a buffer of bytes contains a `Message`
and returns it.
Note that verification is still experimental and may not
catch every error, or be maximally performant. For the
previous, unchecked, behavior use
`root_as_message_unchecked`.

---

## root_as_message_unchecked

`function` · `arrow_ipc::gen::Message::root_as_message_unchecked`

Also reachable as `arrow::ipc::root_as_message_unchecked`, `arrow_ipc::root_as_message_unchecked`

```rust
unsafe fn root_as_message_unchecked(buf: &[u8]) -> Message<'_>
```

Assumes, without verification, that a buffer of bytes contains a Message and returns it.
# Safety
Callers must trust the given bytes do indeed contain a valid `Message`.

---

## root_as_message_with_opts

`function` · `arrow_ipc::gen::Message::root_as_message_with_opts`

Also reachable as `arrow::ipc::root_as_message_with_opts`, `arrow_ipc::root_as_message_with_opts`

```rust
fn root_as_message_with_opts<'b, 'o>(opts: &'o flatbuffers::VerifierOptions, buf: &'b [u8]) -> Result<Message<'b>, flatbuffers::InvalidFlatbuffer>
```

Verifies, with the given options, that a buffer of bytes
contains a `Message` and returns it.
Note that verification is still experimental and may not
catch every error, or be maximally performant. For the
previous, unchecked, behavior use
`root_as_message_unchecked`.

---

## size_prefixed_root_as_message

`function` · `arrow_ipc::gen::Message::size_prefixed_root_as_message`

Also reachable as `arrow::ipc::size_prefixed_root_as_message`, `arrow_ipc::size_prefixed_root_as_message`

```rust
fn size_prefixed_root_as_message(buf: &[u8]) -> Result<Message<'_>, flatbuffers::InvalidFlatbuffer>
```

Verifies that a buffer of bytes contains a size prefixed
`Message` and returns it.
Note that verification is still experimental and may not
catch every error, or be maximally performant. For the
previous, unchecked, behavior use
`size_prefixed_root_as_message_unchecked`.

---

## size_prefixed_root_as_message_unchecked

`function` · `arrow_ipc::gen::Message::size_prefixed_root_as_message_unchecked`

Also reachable as `arrow::ipc::size_prefixed_root_as_message_unchecked`, `arrow_ipc::size_prefixed_root_as_message_unchecked`

```rust
unsafe fn size_prefixed_root_as_message_unchecked(buf: &[u8]) -> Message<'_>
```

Assumes, without verification, that a buffer of bytes contains a size prefixed Message and returns it.
# Safety
Callers must trust the given bytes do indeed contain a valid size prefixed `Message`.

---

## size_prefixed_root_as_message_with_opts

`function` · `arrow_ipc::gen::Message::size_prefixed_root_as_message_with_opts`

Also reachable as `arrow::ipc::size_prefixed_root_as_message_with_opts`, `arrow_ipc::size_prefixed_root_as_message_with_opts`

```rust
fn size_prefixed_root_as_message_with_opts<'b, 'o>(opts: &'o flatbuffers::VerifierOptions, buf: &'b [u8]) -> Result<Message<'b>, flatbuffers::InvalidFlatbuffer>
```

Verifies, with the given verifier options, that a buffer of
bytes contains a size prefixed `Message` and returns
it. Note that verification is still experimental and may not
catch every error, or be maximally performant. For the
previous, unchecked, behavior use
`root_as_message_unchecked`.

---

## BodyCompression

`struct` · `arrow_ipc::gen::Message::BodyCompression`

Also reachable as `arrow::ipc::BodyCompression`, `arrow_ipc::BodyCompression`

```rust
struct BodyCompression<'a>
```

**Fields**: `_tab`

**Implements**: `flatbuffers::follow::Follow`, `flatbuffers::verifier::Verifiable`

**Derives**: Clone, Copy, Debug, PartialEq, StructuralPartialEq

**Methods** (4)

```rust
fn codec(&self) -> CompressionType
fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr, A: flatbuffers::Allocator + 'bldr>(_fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>, args: &'args BodyCompressionArgs) -> flatbuffers::WIPOffset<BodyCompression<'bldr>>
unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self
fn method(&self) -> BodyCompressionMethod
```

**via `flatbuffers::follow::Follow`**

```rust
unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner
```

**via `flatbuffers::verifier::Verifiable`**

```rust
fn run_verifier(v: &mut flatbuffers::Verifier<'_, '_>, pos: usize) -> Result<(), flatbuffers::InvalidFlatbuffer>
```

Optional compression for the memory buffers constituting IPC message
bodies. Intended for use with RecordBatch but could be used for other
message types

---

## BodyCompressionArgs

`struct` · `arrow_ipc::gen::Message::BodyCompressionArgs`

Also reachable as `arrow::ipc::BodyCompressionArgs`, `arrow_ipc::BodyCompressionArgs`

```rust
struct BodyCompressionArgs
```

**Fields**: `codec`, `method`

**Derives**: Default

---

## BodyCompressionBuilder

`struct` · `arrow_ipc::gen::Message::BodyCompressionBuilder`

Also reachable as `arrow::ipc::BodyCompressionBuilder`, `arrow_ipc::BodyCompressionBuilder`

```rust
struct BodyCompressionBuilder<'a: 'b, 'b, A: flatbuffers::Allocator + 'a>
```

**Methods** (4)

```rust
fn add_codec(&mut self, codec: CompressionType)
fn add_method(&mut self, method: BodyCompressionMethod)
fn finish(self) -> flatbuffers::WIPOffset<BodyCompression<'a>>
fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>) -> BodyCompressionBuilder<'a, 'b, A>
```

---

## BodyCompressionMethod

`struct` · `arrow_ipc::gen::Message::BodyCompressionMethod`

Also reachable as `arrow::ipc::BodyCompressionMethod`, `arrow_ipc::BodyCompressionMethod`

```rust
struct BodyCompressionMethod
```

**Implements**: `flatbuffers::endian_scalar::EndianScalar`, `flatbuffers::follow::Follow`, `flatbuffers::push::Push`, `flatbuffers::verifier::SimpleToVerifyInSlice`, `flatbuffers::verifier::Verifiable`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (1)

```rust
fn variant_name(self) -> Option<&'static str>
```

**via `flatbuffers::endian_scalar::EndianScalar`**

```rust
fn from_little_endian(v: i8) -> Self
fn to_little_endian(self) -> i8
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

Provided for forward compatibility in case we need to support different
strategies for compressing the IPC message body (like whole-body
compression rather than buffer-level) in the future

---

## CompressionType

`struct` · `arrow_ipc::gen::Message::CompressionType`

Also reachable as `arrow::ipc::CompressionType`, `arrow_ipc::CompressionType`

```rust
struct CompressionType
```

**Implements**: `flatbuffers::endian_scalar::EndianScalar`, `flatbuffers::follow::Follow`, `flatbuffers::push::Push`, `flatbuffers::verifier::SimpleToVerifyInSlice`, `flatbuffers::verifier::Verifiable`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (1)

```rust
fn variant_name(self) -> Option<&'static str>
```

**via `flatbuffers::endian_scalar::EndianScalar`**

```rust
fn from_little_endian(v: i8) -> Self
fn to_little_endian(self) -> i8
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

---

## DictionaryBatch

`struct` · `arrow_ipc::gen::Message::DictionaryBatch`

Also reachable as `arrow::ipc::DictionaryBatch`, `arrow_ipc::DictionaryBatch`

```rust
struct DictionaryBatch<'a>
```

**Fields**: `_tab`

**Implements**: `flatbuffers::follow::Follow`, `flatbuffers::verifier::Verifiable`

**Derives**: Clone, Copy, Debug, PartialEq, StructuralPartialEq

**Methods** (5)

```rust
fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr, A: flatbuffers::Allocator + 'bldr>(_fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>, args: &'args DictionaryBatchArgs<'args>) -> flatbuffers::WIPOffset<DictionaryBatch<'bldr>>
fn data(&self) -> Option<RecordBatch<'a>>
fn id(&self) -> i64
unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self
fn isDelta(&self) -> bool
```

**via `flatbuffers::follow::Follow`**

```rust
unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner
```

**via `flatbuffers::verifier::Verifiable`**

```rust
fn run_verifier(v: &mut flatbuffers::Verifier<'_, '_>, pos: usize) -> Result<(), flatbuffers::InvalidFlatbuffer>
```

For sending dictionary encoding information. Any Field can be
dictionary-encoded, but in this case none of its children may be
dictionary-encoded.
There is one vector / column per dictionary, but that vector / column
may be spread across multiple dictionary batches by using the isDelta
flag

---

## DictionaryBatchArgs

`struct` · `arrow_ipc::gen::Message::DictionaryBatchArgs`

Also reachable as `arrow::ipc::DictionaryBatchArgs`, `arrow_ipc::DictionaryBatchArgs`

```rust
struct DictionaryBatchArgs<'a>
```

**Fields**: `id`, `data`, `isDelta`

**Derives**: Default

---

## DictionaryBatchBuilder

`struct` · `arrow_ipc::gen::Message::DictionaryBatchBuilder`

Also reachable as `arrow::ipc::DictionaryBatchBuilder`, `arrow_ipc::DictionaryBatchBuilder`

```rust
struct DictionaryBatchBuilder<'a: 'b, 'b, A: flatbuffers::Allocator + 'a>
```

**Methods** (5)

```rust
fn add_data(&mut self, data: flatbuffers::WIPOffset<RecordBatch<'b>>)
fn add_id(&mut self, id: i64)
fn add_isDelta(&mut self, isDelta: bool)
fn finish(self) -> flatbuffers::WIPOffset<DictionaryBatch<'a>>
fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>) -> DictionaryBatchBuilder<'a, 'b, A>
```

---

## FieldNode

`struct` · `arrow_ipc::gen::Message::FieldNode`

Also reachable as `arrow::ipc::FieldNode`, `arrow_ipc::FieldNode`

```rust
struct FieldNode
```

**Implements**: `flatbuffers::follow::Follow`, `flatbuffers::push::Push`, `flatbuffers::verifier::SimpleToVerifyInSlice`, `flatbuffers::verifier::Verifiable`

**Derives**: Clone, Copy, Debug, Default, PartialEq, StructuralPartialEq

**Methods** (5)

```rust
fn length(&self) -> i64
fn new(length: i64, null_count: i64) -> Self
fn null_count(&self) -> i64
fn set_length(&mut self, x: i64)
fn set_null_count(&mut self, x: i64)
```

**via `flatbuffers::follow::Follow`**

```rust
unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner
```

**via `flatbuffers::push::Push`**

```rust
fn alignment() -> flatbuffers::PushAlignment
unsafe fn push(&self, dst: &mut [u8], _written_len: usize)
```

**via `flatbuffers::verifier::Verifiable`**

```rust
fn run_verifier(v: &mut flatbuffers::Verifier<'_, '_>, pos: usize) -> Result<(), flatbuffers::InvalidFlatbuffer>
```

----------------------------------------------------------------------
Data structures for describing a table row batch (a collection of
equal-length Arrow arrays)
Metadata about a field at some level of a nested type tree (but not
its children).

For example, a `List<Int16>` with values `[[1, 2, 3], null, [4], [5, 6], null]`
would have {length: 5, null_count: 2} for its List node, and {length: 6,
null_count: 0} for its Int16 node, as separate FieldNode structs

---

## Message

`struct` · `arrow_ipc::gen::Message::Message`

Also reachable as `arrow::ipc::Message`, `arrow_ipc::Message`

```rust
struct Message<'a>
```

**Fields**: `_tab`

**Implements**: `flatbuffers::follow::Follow`, `flatbuffers::verifier::Verifiable`

**Derives**: Clone, Copy, Debug, PartialEq, StructuralPartialEq

**Methods** (12)

```rust
fn bodyLength(&self) -> i64
fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr, A: flatbuffers::Allocator + 'bldr>(_fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>, args: &'args MessageArgs<'args>) -> flatbuffers::WIPOffset<Message<'bldr>>
fn custom_metadata(&self) -> Option<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<KeyValue<'a>>>>
fn header(&self) -> Option<flatbuffers::Table<'a>>
fn header_as_dictionary_batch(&self) -> Option<DictionaryBatch<'a>>
fn header_as_record_batch(&self) -> Option<RecordBatch<'a>>
fn header_as_schema(&self) -> Option<Schema<'a>>
fn header_as_sparse_tensor(&self) -> Option<SparseTensor<'a>>
fn header_as_tensor(&self) -> Option<Tensor<'a>>
fn header_type(&self) -> MessageHeader
unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self
fn version(&self) -> MetadataVersion
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

## MessageArgs

`struct` · `arrow_ipc::gen::Message::MessageArgs`

Also reachable as `arrow::ipc::MessageArgs`, `arrow_ipc::MessageArgs`

```rust
struct MessageArgs<'a>
```

**Fields**: `version`, `header_type`, `header`, `bodyLength`, `custom_metadata`

**Derives**: Default

---

## MessageBuilder

`struct` · `arrow_ipc::gen::Message::MessageBuilder`

Also reachable as `arrow::ipc::MessageBuilder`, `arrow_ipc::MessageBuilder`

```rust
struct MessageBuilder<'a: 'b, 'b, A: flatbuffers::Allocator + 'a>
```

**Methods** (7)

```rust
fn add_bodyLength(&mut self, bodyLength: i64)
fn add_custom_metadata(&mut self, custom_metadata: flatbuffers::WIPOffset<flatbuffers::Vector<'b, flatbuffers::ForwardsUOffset<KeyValue<'b>>>>)
fn add_header(&mut self, header: flatbuffers::WIPOffset<flatbuffers::UnionWIPOffset>)
fn add_header_type(&mut self, header_type: MessageHeader)
fn add_version(&mut self, version: MetadataVersion)
fn finish(self) -> flatbuffers::WIPOffset<Message<'a>>
fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>) -> MessageBuilder<'a, 'b, A>
```

---

## MessageHeader

`struct` · `arrow_ipc::gen::Message::MessageHeader`

Also reachable as `arrow::ipc::MessageHeader`, `arrow_ipc::MessageHeader`

```rust
struct MessageHeader
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

----------------------------------------------------------------------
The root Message type
This union enables us to easily send different message types without
redundant storage, and in the future we can easily add new message types.

Arrow implementations do not need to implement all of the message types,
which may include experimental metadata types. For maximum compatibility,
it is best to send data using RecordBatch

---

## MessageHeaderUnionTableOffset

`struct` · `arrow_ipc::gen::Message::MessageHeaderUnionTableOffset`

Also reachable as `arrow::ipc::MessageHeaderUnionTableOffset`, `arrow_ipc::MessageHeaderUnionTableOffset`

```rust
struct MessageHeaderUnionTableOffset
```

---

## RecordBatch

`struct` · `arrow_ipc::gen::Message::RecordBatch`

Also reachable as `arrow::ipc::RecordBatch`, `arrow_ipc::RecordBatch`

```rust
struct RecordBatch<'a>
```

**Fields**: `_tab`

**Implements**: `flatbuffers::follow::Follow`, `flatbuffers::verifier::Verifiable`

**Derives**: Clone, Copy, Debug, PartialEq, StructuralPartialEq

**Methods** (7)

```rust
fn buffers(&self) -> Option<flatbuffers::Vector<'a, Buffer>>
fn compression(&self) -> Option<BodyCompression<'a>>
fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr, A: flatbuffers::Allocator + 'bldr>(_fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>, args: &'args RecordBatchArgs<'args>) -> flatbuffers::WIPOffset<RecordBatch<'bldr>>
unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self
fn length(&self) -> i64
fn nodes(&self) -> Option<flatbuffers::Vector<'a, FieldNode>>
fn variadicBufferCounts(&self) -> Option<flatbuffers::Vector<'a, i64>>
```

**via `flatbuffers::follow::Follow`**

```rust
unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner
```

**via `flatbuffers::verifier::Verifiable`**

```rust
fn run_verifier(v: &mut flatbuffers::Verifier<'_, '_>, pos: usize) -> Result<(), flatbuffers::InvalidFlatbuffer>
```

A data header describing the shared memory layout of a "record" or "row"
batch. Some systems call this a "row batch" internally and others a "record
batch".

---

## RecordBatchArgs

`struct` · `arrow_ipc::gen::Message::RecordBatchArgs`

Also reachable as `arrow::ipc::RecordBatchArgs`, `arrow_ipc::RecordBatchArgs`

```rust
struct RecordBatchArgs<'a>
```

**Fields**: `length`, `nodes`, `buffers`, `compression`, `variadicBufferCounts`

**Derives**: Default

---

## RecordBatchBuilder

`struct` · `arrow_ipc::gen::Message::RecordBatchBuilder`

Also reachable as `arrow::ipc::RecordBatchBuilder`, `arrow_ipc::RecordBatchBuilder`

```rust
struct RecordBatchBuilder<'a: 'b, 'b, A: flatbuffers::Allocator + 'a>
```

**Methods** (7)

```rust
fn add_buffers(&mut self, buffers: flatbuffers::WIPOffset<flatbuffers::Vector<'b, Buffer>>)
fn add_compression(&mut self, compression: flatbuffers::WIPOffset<BodyCompression<'b>>)
fn add_length(&mut self, length: i64)
fn add_nodes(&mut self, nodes: flatbuffers::WIPOffset<flatbuffers::Vector<'b, FieldNode>>)
fn add_variadicBufferCounts(&mut self, variadicBufferCounts: flatbuffers::WIPOffset<flatbuffers::Vector<'b, i64>>)
fn finish(self) -> flatbuffers::WIPOffset<RecordBatch<'a>>
fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>) -> RecordBatchBuilder<'a, 'b, A>
```

---
