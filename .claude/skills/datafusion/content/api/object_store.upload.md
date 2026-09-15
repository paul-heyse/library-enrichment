# `object_store::upload`

Crate `object_store` · 3 public items · structured records in [`model/object_store.upload.json`](../model/object_store.upload.json)

## WriteMultipart

`struct` · `object_store::upload::WriteMultipart`

```rust
struct WriteMultipart
```

**Derives**: Debug

**Methods** (8)

```rust
async fn abort(self) -> Result<()>
async fn finish(self) -> Result<PutResult>
fn new(upload: Box<dyn MultipartUpload>) -> Self
fn new_with_chunk_size(upload: Box<dyn MultipartUpload>, chunk_size: usize) -> Self
fn poll_for_capacity(&mut self, cx: &mut Context<'_>, max_concurrency: usize) -> Poll<Result<()>>
fn put(&mut self, bytes: bytes::Bytes)
async fn wait_for_capacity(&mut self, max_concurrency: usize) -> Result<()>
fn write(&mut self, buf: &[u8])
```

A synchronous write API for uploading data in parallel in fixed size chunks

Uses multiple tokio tasks in a [`JoinSet`] to multiplex upload tasks in parallel

The design also takes inspiration from [`Sink`] with [`WriteMultipart::wait_for_capacity`]
allowing back pressure on producers, prior to buffering the next part. However, unlike
[`Sink`] this back pressure is optional, allowing integration with synchronous producers

[`Sink`]: futures_util::sink::Sink

---

## MultipartUpload

`trait` · `object_store::upload::MultipartUpload`

```rust
trait MultipartUpload: Send + std::fmt::Debug
```

**Implementors** (2)

- `alloc::boxed::Box`
- `object_store::limit::LimitUpload`

**Methods** (3)

```rust
async fn abort(&mut self) -> Result<()>
async fn complete(&mut self) -> Result<PutResult>
fn put_part(&mut self, data: PutPayload) -> UploadPart
```

A trait allowing writing an object in fixed size chunks

Consecutive chunks of data can be written by calling [`MultipartUpload::put_part`] and polling
the returned futures to completion. Multiple futures returned by [`MultipartUpload::put_part`]
may be polled in parallel, allowing for concurrent uploads.

Once all part uploads have been polled to completion, the upload can be completed by
calling [`MultipartUpload::complete`]. This will make the entire uploaded object visible
as an atomic operation.It is implementation behind behaviour if [`MultipartUpload::complete`]
is called before all [`UploadPart`] have been polled to completion.

---

## UploadPart

`type_alias` · `object_store::upload::UploadPart`

```rust
type UploadPart = futures_util::future::BoxFuture<'static, Result<()>>
```

An upload part request

---
