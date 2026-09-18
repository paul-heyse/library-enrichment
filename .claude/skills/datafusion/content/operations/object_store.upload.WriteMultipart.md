# `object_store::upload::WriteMultipart`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.upload.WriteMultipart.json).

<a id="op-abbbcfc53690f534bb386f40"></a>
## WriteMultipart

`struct` · `object_store::upload::WriteMultipart` · object_store 0.13.2

```rust
struct WriteMultipart
```

Source: `src/upload.rs:123`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

A synchronous write API for uploading data in parallel in fixed size chunks

Uses multiple tokio tasks in a [`JoinSet`] to multiplex upload tasks in parallel

The design also takes inspiration from [`Sink`] with [`WriteMultipart::wait_for_capacity`](../operations/object_store.upload.WriteMultipart.md#op-109b9faf11030fda78698665)
allowing back pressure on producers, prior to buffering the next part. However, unlike
[`Sink`] this back pressure is optional, allowing integration with synchronous producers

[`Sink`]: futures_util::sink::Sink

Unresolved upstream links (retained, not inferred): ``JoinSet``, `futures_util::sink::Sink`.

<a id="op-3340164c99b87c78679d9429"></a>
## abort

`function` · `object_store::upload::WriteMultipart::abort` · object_store 0.13.2

```rust
async fn abort(self) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::upload::WriteMultipart", "path": "WriteMultipart"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [134, 1], "end": [243, 2], "filename": "src/upload.rs"}, "trait": null, "trait_path": null}`

Source: `src/upload.rs:220`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Abort this upload, attempting to clean up any successfully uploaded parts

<a id="op-314a5bcb54fe509141885d85"></a>
## finish

`function` · `object_store::upload::WriteMultipart::finish` · object_store 0.13.2

```rust
async fn finish(self) -> Result<PutResult>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::upload::WriteMultipart", "path": "WriteMultipart"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [134, 1], "end": [243, 2], "filename": "src/upload.rs"}, "trait": null, "trait_path": null}`

Source: `src/upload.rs:226`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Flush final chunk, and await completion of all in-flight requests

<a id="op-df5f7665bf42f2c1ace57a63"></a>
## fmt

`function` · `object_store::upload::WriteMultipart::fmt` · object_store 0.13.2

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::upload::WriteMultipart", "path": "WriteMultipart"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [122, 10], "end": [122, 15], "filename": "src/upload.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/upload.rs:122`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2bdf7983af40e792bf84e588"></a>
## new

`function` · `object_store::upload::WriteMultipart::new` · object_store 0.13.2

```rust
fn new(upload: Box<dyn MultipartUpload>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::upload::WriteMultipart", "path": "WriteMultipart"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [134, 1], "end": [243, 2], "filename": "src/upload.rs"}, "trait": null, "trait_path": null}`

Source: `src/upload.rs:136`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Create a new [`WriteMultipart`](../operations/object_store.upload.WriteMultipart.md#op-abbbcfc53690f534bb386f40) that will upload using 5MB chunks

<a id="op-c91454348e9caefc5e6daea3"></a>
## new_with_chunk_size

`function` · `object_store::upload::WriteMultipart::new_with_chunk_size` · object_store 0.13.2

```rust
fn new_with_chunk_size(upload: Box<dyn MultipartUpload>, chunk_size: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::upload::WriteMultipart", "path": "WriteMultipart"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [134, 1], "end": [243, 2], "filename": "src/upload.rs"}, "trait": null, "trait_path": null}`

Source: `src/upload.rs:141`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Create a new [`WriteMultipart`](../operations/object_store.upload.WriteMultipart.md#op-abbbcfc53690f534bb386f40) that will upload in fixed `chunk_size` sized chunks

<a id="op-9bd8fee1ace28269b0fa61bc"></a>
## poll_for_capacity

`function` · `object_store::upload::WriteMultipart::poll_for_capacity` · object_store 0.13.2

```rust
fn poll_for_capacity(&mut self, cx: &mut Context<'_>, max_concurrency: usize) -> Poll<Result<()>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::upload::WriteMultipart", "path": "WriteMultipart"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [134, 1], "end": [243, 2], "filename": "src/upload.rs"}, "trait": null, "trait_path": null}`

Source: `src/upload.rs:153`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Polls for there to be less than `max_concurrency` [`UploadPart`](../operations/object_store.upload.UploadPart.md#op-b21b01cdc9dcbe60e8c4f765) in progress

See [`Self::wait_for_capacity`](../operations/object_store.upload.WriteMultipart.md#op-109b9faf11030fda78698665) for an async version of this function

<a id="op-3c936867c15947d13e3ff422"></a>
## put

`function` · `object_store::upload::WriteMultipart::put` · object_store 0.13.2

```rust
fn put(&mut self, bytes: bytes::Bytes)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::upload::WriteMultipart", "path": "WriteMultipart"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [134, 1], "end": [243, 2], "filename": "src/upload.rs"}, "trait": null, "trait_path": null}`

Source: `src/upload.rs:202`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Put a chunk of data into this [`WriteMultipart`](../operations/object_store.upload.WriteMultipart.md#op-abbbcfc53690f534bb386f40) without copying

Data is buffered using [`PutPayloadMut::push`](../operations/object_store.payload.PutPayloadMut.md#op-0e62aad6b58e4ce810d0fb80). Implementations looking to
perform writes from non-owned buffers should prefer [`Self::write`](../operations/object_store.upload.WriteMultipart.md#op-4a5ac1f19dc9fdf478b14ee3) as this
will allow multiple calls to share the same underlying allocation.

See [`Self::write`](../operations/object_store.upload.WriteMultipart.md#op-4a5ac1f19dc9fdf478b14ee3) for information on backpressure

<a id="op-109b9faf11030fda78698665"></a>
## wait_for_capacity

`function` · `object_store::upload::WriteMultipart::wait_for_capacity` · object_store 0.13.2

```rust
async fn wait_for_capacity(&mut self, max_concurrency: usize) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::upload::WriteMultipart", "path": "WriteMultipart"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [134, 1], "end": [243, 2], "filename": "src/upload.rs"}, "trait": null, "trait_path": null}`

Source: `src/upload.rs:167`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Wait until there are less than `max_concurrency` [`UploadPart`](../operations/object_store.upload.UploadPart.md#op-b21b01cdc9dcbe60e8c4f765) in progress

See [`Self::poll_for_capacity`](../operations/object_store.upload.WriteMultipart.md#op-9bd8fee1ace28269b0fa61bc) for a [`Poll`] version of this function

Unresolved upstream links (retained, not inferred): ``Poll``.

<a id="op-4a5ac1f19dc9fdf478b14ee3"></a>
## write

`function` · `object_store::upload::WriteMultipart::write` · object_store 0.13.2

```rust
fn write(&mut self, buf: &[u8])
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::upload::WriteMultipart", "path": "WriteMultipart"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [134, 1], "end": [243, 2], "filename": "src/upload.rs"}, "trait": null, "trait_path": null}`

Source: `src/upload.rs:182`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Write data to this [`WriteMultipart`](../operations/object_store.upload.WriteMultipart.md#op-abbbcfc53690f534bb386f40)

Data is buffered using [`PutPayloadMut::extend_from_slice`](../operations/object_store.payload.PutPayloadMut.md#op-6a4b231ec02eddc2d2c5a96e). Implementations looking to
write data from owned buffers may prefer [`Self::put`](../operations/object_store.upload.WriteMultipart.md#op-3c936867c15947d13e3ff422) as this avoids copying.

Note this method is synchronous (not `async`) and will immediately
start new uploads as soon as the internal `chunk_size` is hit,
regardless of how many outstanding uploads are already in progress.

Back pressure can optionally be applied to producers by calling
[`Self::wait_for_capacity`](../operations/object_store.upload.WriteMultipart.md#op-109b9faf11030fda78698665) prior to calling this method
