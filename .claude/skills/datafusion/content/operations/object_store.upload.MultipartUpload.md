# `object_store::upload::MultipartUpload`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.upload.MultipartUpload.json).

<a id="op-d1bf78ac32fe2ad950a9d5d0"></a>
## MultipartUpload

`trait` · `object_store::upload::MultipartUpload` · object_store 0.13.2

```rust
trait MultipartUpload: Send + std::fmt::Debug
```

Source: `src/upload.rs:43`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

A trait allowing writing an object in fixed size chunks

Consecutive chunks of data can be written by calling [`MultipartUpload::put_part`](../operations/object_store.upload.MultipartUpload.md#op-81347236585009acee547d0a) and polling
the returned futures to completion. Multiple futures returned by [`MultipartUpload::put_part`](../operations/object_store.upload.MultipartUpload.md#op-81347236585009acee547d0a)
may be polled in parallel, allowing for concurrent uploads.

Once all part uploads have been polled to completion, the upload can be completed by
calling [`MultipartUpload::complete`](../operations/object_store.upload.MultipartUpload.md#op-a1a4e1e4e03df4fa1f4622b1). This will make the entire uploaded object visible
as an atomic operation.It is implementation behind behaviour if [`MultipartUpload::complete`](../operations/object_store.upload.MultipartUpload.md#op-a1a4e1e4e03df4fa1f4622b1)
is called before all [`UploadPart`](../operations/object_store.upload.UploadPart.md#op-b21b01cdc9dcbe60e8c4f765) have been polled to completion.

<a id="op-d0ab4a61fdda48f1d5be2d47"></a>
## abort

`function` · `object_store::upload::MultipartUpload::abort` · object_store 0.13.2

```rust
async fn abort(&mut self) -> Result<()>
```

Source: `src/upload.rs:94`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Abort the multipart upload

If a [`MultipartUpload`](../operations/object_store.upload.MultipartUpload.md#op-d1bf78ac32fe2ad950a9d5d0) is dropped without calling [`MultipartUpload::complete`](../operations/object_store.upload.MultipartUpload.md#op-a1a4e1e4e03df4fa1f4622b1),
some object stores will automatically clean up any previously uploaded parts.
However, some stores, such as S3 and GCS, cannot perform cleanup on drop.
As such [`MultipartUpload::abort`](../operations/object_store.upload.MultipartUpload.md#op-d0ab4a61fdda48f1d5be2d47) can be invoked to perform this cleanup.

It will not be possible to call `abort` in all failure scenarios, for example
non-graceful shutdown of the calling application. It is therefore recommended
object stores are configured with lifecycle rules to automatically cleanup
unused parts older than some threshold. See [crate::aws](../modules/object_store.aws.md#op-91f16db41cdd768042f8c13f) and [crate::gcp](../modules/object_store.gcp.md#op-397cb506fa80f7a4f9953b4d)
for more information.

It is implementation defined behaviour to call [`MultipartUpload::abort`](../operations/object_store.upload.MultipartUpload.md#op-d0ab4a61fdda48f1d5be2d47)
on an already completed or aborted [`MultipartUpload`](../operations/object_store.upload.MultipartUpload.md#op-d1bf78ac32fe2ad950a9d5d0)

<a id="op-a1a4e1e4e03df4fa1f4622b1"></a>
## complete

`function` · `object_store::upload::MultipartUpload::complete` · object_store 0.13.2

```rust
async fn complete(&mut self) -> Result<PutResult>
```

Source: `src/upload.rs:77`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Complete the multipart upload

It is implementation defined behaviour if this method is called before polling
all [`UploadPart`](../operations/object_store.upload.UploadPart.md#op-b21b01cdc9dcbe60e8c4f765) returned by [`MultipartUpload::put_part`](../operations/object_store.upload.MultipartUpload.md#op-81347236585009acee547d0a) to completion. Additionally,
it is implementation defined behaviour to call [`MultipartUpload::complete`](../operations/object_store.upload.MultipartUpload.md#op-a1a4e1e4e03df4fa1f4622b1)
on an already completed or aborted [`MultipartUpload`](../operations/object_store.upload.MultipartUpload.md#op-d1bf78ac32fe2ad950a9d5d0).

<a id="op-81347236585009acee547d0a"></a>
## put_part

`function` · `object_store::upload::MultipartUpload::put_part` · object_store 0.13.2

```rust
fn put_part(&mut self, data: PutPayload) -> UploadPart
```

Source: `src/upload.rs:69`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Upload the next part

Most stores require that all parts excluding the last are at least 5 MiB, and some
further require that all parts excluding the last be the same size, e.g. [R2].
Clients wanting to maximise compatibility should therefore perform writes in
fixed size blocks larger than 5 MiB.

Implementations may invoke this method multiple times and then await on the
returned futures in parallel

```no_run
# use futures_util::StreamExt;
# use object_store::MultipartUpload;
#
# async fn test() {
#
let mut upload: Box<&dyn MultipartUpload> = todo!();
let p1 = upload.put_part(vec![0; 10 * 1024 * 1024].into());
let p2 = upload.put_part(vec![1; 10 * 1024 * 1024].into());
futures_util::future::try_join(p1, p2).await.unwrap();
upload.complete().await.unwrap();
# }
```

[R2]: https://developers.cloudflare.com/r2/objects/multipart-objects/#limitations
