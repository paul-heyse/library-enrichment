# `arrow_ipc::gen::File`

Crate `arrow-ipc` · 13 public items · structured records in [`model/arrow_ipc.gen.File.json`](../model/arrow_ipc.gen.File.json)

## FooterOffset

`enum` · `arrow_ipc::gen::File::FooterOffset`

Also reachable as `arrow::ipc::FooterOffset`, `arrow_ipc::FooterOffset`

```rust
enum FooterOffset
```

---

## finish_footer_buffer

`function` · `arrow_ipc::gen::File::finish_footer_buffer`

Also reachable as `arrow::ipc::finish_footer_buffer`, `arrow_ipc::finish_footer_buffer`

```rust
fn finish_footer_buffer<'a, 'b, A: flatbuffers::Allocator + 'a>(fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>, root: flatbuffers::WIPOffset<Footer<'a>>)
```

---

## finish_size_prefixed_footer_buffer

`function` · `arrow_ipc::gen::File::finish_size_prefixed_footer_buffer`

Also reachable as `arrow::ipc::finish_size_prefixed_footer_buffer`, `arrow_ipc::finish_size_prefixed_footer_buffer`

```rust
fn finish_size_prefixed_footer_buffer<'a, 'b, A: flatbuffers::Allocator + 'a>(fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>, root: flatbuffers::WIPOffset<Footer<'a>>)
```

---

## root_as_footer

`function` · `arrow_ipc::gen::File::root_as_footer`

Also reachable as `arrow::ipc::root_as_footer`, `arrow_ipc::root_as_footer`

```rust
fn root_as_footer(buf: &[u8]) -> Result<Footer<'_>, flatbuffers::InvalidFlatbuffer>
```

Verifies that a buffer of bytes contains a `Footer`
and returns it.
Note that verification is still experimental and may not
catch every error, or be maximally performant. For the
previous, unchecked, behavior use
`root_as_footer_unchecked`.

---

## root_as_footer_unchecked

`function` · `arrow_ipc::gen::File::root_as_footer_unchecked`

Also reachable as `arrow::ipc::root_as_footer_unchecked`, `arrow_ipc::root_as_footer_unchecked`

```rust
unsafe fn root_as_footer_unchecked(buf: &[u8]) -> Footer<'_>
```

Assumes, without verification, that a buffer of bytes contains a Footer and returns it.
# Safety
Callers must trust the given bytes do indeed contain a valid `Footer`.

---

## root_as_footer_with_opts

`function` · `arrow_ipc::gen::File::root_as_footer_with_opts`

Also reachable as `arrow::ipc::root_as_footer_with_opts`, `arrow_ipc::root_as_footer_with_opts`

```rust
fn root_as_footer_with_opts<'b, 'o>(opts: &'o flatbuffers::VerifierOptions, buf: &'b [u8]) -> Result<Footer<'b>, flatbuffers::InvalidFlatbuffer>
```

Verifies, with the given options, that a buffer of bytes
contains a `Footer` and returns it.
Note that verification is still experimental and may not
catch every error, or be maximally performant. For the
previous, unchecked, behavior use
`root_as_footer_unchecked`.

---

## size_prefixed_root_as_footer

`function` · `arrow_ipc::gen::File::size_prefixed_root_as_footer`

Also reachable as `arrow::ipc::size_prefixed_root_as_footer`, `arrow_ipc::size_prefixed_root_as_footer`

```rust
fn size_prefixed_root_as_footer(buf: &[u8]) -> Result<Footer<'_>, flatbuffers::InvalidFlatbuffer>
```

Verifies that a buffer of bytes contains a size prefixed
`Footer` and returns it.
Note that verification is still experimental and may not
catch every error, or be maximally performant. For the
previous, unchecked, behavior use
`size_prefixed_root_as_footer_unchecked`.

---

## size_prefixed_root_as_footer_unchecked

`function` · `arrow_ipc::gen::File::size_prefixed_root_as_footer_unchecked`

Also reachable as `arrow::ipc::size_prefixed_root_as_footer_unchecked`, `arrow_ipc::size_prefixed_root_as_footer_unchecked`

```rust
unsafe fn size_prefixed_root_as_footer_unchecked(buf: &[u8]) -> Footer<'_>
```

Assumes, without verification, that a buffer of bytes contains a size prefixed Footer and returns it.
# Safety
Callers must trust the given bytes do indeed contain a valid size prefixed `Footer`.

---

## size_prefixed_root_as_footer_with_opts

`function` · `arrow_ipc::gen::File::size_prefixed_root_as_footer_with_opts`

Also reachable as `arrow::ipc::size_prefixed_root_as_footer_with_opts`, `arrow_ipc::size_prefixed_root_as_footer_with_opts`

```rust
fn size_prefixed_root_as_footer_with_opts<'b, 'o>(opts: &'o flatbuffers::VerifierOptions, buf: &'b [u8]) -> Result<Footer<'b>, flatbuffers::InvalidFlatbuffer>
```

Verifies, with the given verifier options, that a buffer of
bytes contains a size prefixed `Footer` and returns
it. Note that verification is still experimental and may not
catch every error, or be maximally performant. For the
previous, unchecked, behavior use
`root_as_footer_unchecked`.

---

## Block

`struct` · `arrow_ipc::gen::File::Block`

Also reachable as `arrow::ipc::Block`, `arrow_ipc::Block`

```rust
struct Block
```

**Implements**: `flatbuffers::follow::Follow`, `flatbuffers::push::Push`, `flatbuffers::verifier::SimpleToVerifyInSlice`, `flatbuffers::verifier::Verifiable`

**Derives**: Clone, Copy, Debug, Default, PartialEq, StructuralPartialEq

**Methods** (7)

```rust
fn bodyLength(&self) -> i64
fn metaDataLength(&self) -> i32
fn new(offset: i64, metaDataLength: i32, bodyLength: i64) -> Self
fn offset(&self) -> i64
fn set_bodyLength(&mut self, x: i64)
fn set_metaDataLength(&mut self, x: i32)
fn set_offset(&mut self, x: i64)
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

---

## Footer

`struct` · `arrow_ipc::gen::File::Footer`

Also reachable as `arrow::ipc::Footer`, `arrow_ipc::Footer`

```rust
struct Footer<'a>
```

**Fields**: `_tab`

**Implements**: `flatbuffers::follow::Follow`, `flatbuffers::verifier::Verifiable`

**Derives**: Clone, Copy, Debug, PartialEq, StructuralPartialEq

**Methods** (7)

```rust
fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr, A: flatbuffers::Allocator + 'bldr>(_fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>, args: &'args FooterArgs<'args>) -> flatbuffers::WIPOffset<Footer<'bldr>>
fn custom_metadata(&self) -> Option<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<KeyValue<'a>>>>
fn dictionaries(&self) -> Option<flatbuffers::Vector<'a, Block>>
unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self
fn recordBatches(&self) -> Option<flatbuffers::Vector<'a, Block>>
fn schema(&self) -> Option<Schema<'a>>
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

----------------------------------------------------------------------
Arrow File metadata

---

## FooterArgs

`struct` · `arrow_ipc::gen::File::FooterArgs`

Also reachable as `arrow::ipc::FooterArgs`, `arrow_ipc::FooterArgs`

```rust
struct FooterArgs<'a>
```

**Fields**: `version`, `schema`, `dictionaries`, `recordBatches`, `custom_metadata`

**Derives**: Default

---

## FooterBuilder

`struct` · `arrow_ipc::gen::File::FooterBuilder`

Also reachable as `arrow::ipc::FooterBuilder`, `arrow_ipc::FooterBuilder`

```rust
struct FooterBuilder<'a: 'b, 'b, A: flatbuffers::Allocator + 'a>
```

**Methods** (7)

```rust
fn add_custom_metadata(&mut self, custom_metadata: flatbuffers::WIPOffset<flatbuffers::Vector<'b, flatbuffers::ForwardsUOffset<KeyValue<'b>>>>)
fn add_dictionaries(&mut self, dictionaries: flatbuffers::WIPOffset<flatbuffers::Vector<'b, Block>>)
fn add_recordBatches(&mut self, recordBatches: flatbuffers::WIPOffset<flatbuffers::Vector<'b, Block>>)
fn add_schema(&mut self, schema: flatbuffers::WIPOffset<Schema<'b>>)
fn add_version(&mut self, version: MetadataVersion)
fn finish(self) -> flatbuffers::WIPOffset<Footer<'a>>
fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>) -> FooterBuilder<'a, 'b, A>
```

---
