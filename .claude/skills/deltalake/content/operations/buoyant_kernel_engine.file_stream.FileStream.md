# `buoyant_kernel_engine::file_stream::FileStream`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel_engine.file_stream.FileStream.json).

<a id="op-bac006aaae68732071a379c6"></a>
## FileStream

`struct` · `buoyant_kernel_engine::file_stream::FileStream` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct FileStream
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/file_stream.rs#L74).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/file_stream.rs:74`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

A stream that iterates record batch by record batch, file over file.

<a id="op-0ce472ed7bfeb9f70b7c1f98"></a>
## Item

`assoc_type` · `buoyant_kernel_engine::file_stream::FileStream::Item` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type Item = Result<RecordBatch, Error>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/file_stream.rs#L224).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel_engine::file_stream::FileStream", "path": "FileStream"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [223, 1], "end": [229, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/file_stream.rs"}, "trait": {"args": null, "id": "futures_core::stream::Stream", "path": "Stream"}, "trait_path": "futures_core::stream::Stream"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/file_stream.rs:224`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-94260a361ad889d45733f9c8"></a>
## new

`function` · `buoyant_kernel_engine::file_stream::FileStream::new` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn new(files: impl IntoIterator<Item = FileMeta>, schema: ArrowSchemaRef, file_opener: Box<dyn FileOpener>) -> DeltaResult<Self>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/file_stream.rs#L94).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel_engine::file_stream::FileStream", "path": "FileStream"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [92, 1], "end": [221, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/file_stream.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/file_stream.rs:94`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Create a new `FileStream` using the given `FileOpener` to scan underlying files

<a id="op-c0588ab66641e11f853e13e1"></a>
## poll_next

`function` · `buoyant_kernel_engine::file_stream::FileStream::poll_next` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn poll_next(Pin<&mut self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/file_stream.rs#L226).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel_engine::file_stream::FileStream", "path": "FileStream"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [223, 1], "end": [229, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/file_stream.rs"}, "trait": {"args": null, "id": "futures_core::stream::Stream", "path": "Stream"}, "trait_path": "futures_core::stream::Stream"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/file_stream.rs:226`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cda63fb4c418a429e3d55611"></a>
## with_on_error

`function` · `buoyant_kernel_engine::file_stream::FileStream::with_on_error` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_on_error(self, on_error: OnError) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/file_stream.rs#L112).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel_engine::file_stream::FileStream", "path": "FileStream"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [92, 1], "end": [221, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/file_stream.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/file_stream.rs:112`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Specify the behavior when an error occurs opening or scanning a file

If `OnError::Skip` the stream will skip files which encounter an error and continue
If `OnError:Fail` (default) the stream will fail and stop processing when an error occurs

<a id="op-58db4380c6a614cd35b0f123"></a>
## file_iter

`struct_field` · `buoyant_kernel_engine::file_stream::FileStream::file_iter` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
file_iter: std::collections::VecDeque<delta_kernel::FileMeta>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/file_stream.rs#L76).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/file_stream.rs:76`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

An iterator over input files.

<a id="op-d1ea1cb8716a260b43f22edd"></a>
## file_opener

`struct_field` · `buoyant_kernel_engine::file_stream::FileStream::file_opener` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
file_opener: Box<dyn FileOpener>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/file_stream.rs#L85).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/file_stream.rs:85`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

A closure that takes a reader and an optional remaining number of lines
(before reaching the limit) and returns a batch iterator. If the file reader
is not capable of limiting the number of records in the last batch, the file
stream will take care of truncating it.

<a id="op-f5a30604dcd9a73eaeef4e29"></a>
## on_error

`struct_field` · `buoyant_kernel_engine::file_stream::FileStream::on_error` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
on_error: OnError
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/file_stream.rs#L89).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/file_stream.rs:89`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Describes the behavior of the `FileStream` if file opening or scanning fails

<a id="op-ec9e975488fba6a908571657"></a>
## projected_schema

`struct_field` · `buoyant_kernel_engine::file_stream::FileStream::projected_schema` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
projected_schema: delta_kernel::arrow::datatypes::SchemaRef
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/file_stream.rs#L80).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/file_stream.rs:80`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The stream schema (file schema including partition columns and after
projection).

<a id="op-22a47cf709d767212c6a9d86"></a>
## state

`struct_field` · `buoyant_kernel_engine::file_stream::FileStream::state` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
state: FileStreamState
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/file_stream.rs#L87).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/file_stream.rs:87`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The stream state
