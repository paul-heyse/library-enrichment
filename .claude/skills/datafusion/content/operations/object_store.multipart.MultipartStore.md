# `object_store::multipart::MultipartStore`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.multipart.MultipartStore.json).

<a id="op-df886cc201d4dd79df33cad8"></a>
## MultipartStore

`trait` · `object_store::multipart::MultipartStore` · object_store 0.13.2

```rust
trait MultipartStore: Send + Sync + 'static
```

Source: `src/multipart.rs:45`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

A low-level interface for interacting with multipart upload APIs

Most use-cases should prefer [`ObjectStore::put_multipart_opts`] as this is supported by more
backends, including [`LocalFileSystem`], and automatically handles uploading fixed
size parts of sufficient size in parallel

[`ObjectStore::put_multipart_opts`]: crate::ObjectStore::put_multipart_opts
[`LocalFileSystem`]: crate::local::LocalFileSystem

<a id="op-4db7d1a6be4d316ff0e48c15"></a>
## abort_multipart

`function` · `object_store::multipart::MultipartStore::abort_multipart` · object_store 0.13.2

```rust
async fn abort_multipart(&self, path: &Path, id: &MultipartId) -> Result<()>
```

Source: `src/multipart.rs:83`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Aborts a multipart upload

<a id="op-75ce3e06ad82c4924e6f4b01"></a>
## complete_multipart

`function` · `object_store::multipart::MultipartStore::complete_multipart` · object_store 0.13.2

```rust
async fn complete_multipart(&self, path: &Path, id: &MultipartId, parts: Vec<PartId>) -> Result<PutResult>
```

Source: `src/multipart.rs:75`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Completes a multipart upload

The `i`'th value of `parts` must be a [`PartId`](../operations/object_store.multipart.PartId.md#op-7d9f763a7604a6bb02e2bddf) returned by a call to [`Self::put_part`](../operations/object_store.multipart.MultipartStore.md#op-13d79cfbe53330cea363f6c7)
with a `part_idx` of `i`, and the same `path` and `id` as provided to this method. Calling
this method with out of sequence or repeated [`PartId`](../operations/object_store.multipart.PartId.md#op-7d9f763a7604a6bb02e2bddf), or [`PartId`](../operations/object_store.multipart.PartId.md#op-7d9f763a7604a6bb02e2bddf) returned for other
values of `path` or `id`, will result in implementation-defined behaviour

<a id="op-712bc970b3f88f8899456802"></a>
## create_multipart

`function` · `object_store::multipart::MultipartStore::create_multipart` · object_store 0.13.2

```rust
async fn create_multipart(&self, path: &Path) -> Result<MultipartId>
```

Source: `src/multipart.rs:47`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Creates a new multipart upload, returning the [`MultipartId`](../operations/object_store.MultipartId.md#op-90297522487b10495ee1e5c9)

<a id="op-13d79cfbe53330cea363f6c7"></a>
## put_part

`function` · `object_store::multipart::MultipartStore::put_part` · object_store 0.13.2

```rust
async fn put_part(&self, path: &Path, id: &MultipartId, part_idx: usize, data: PutPayload) -> Result<PartId>
```

Source: `src/multipart.rs:61`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Uploads a new part with index `part_idx`

`part_idx` should be an integer in the range `0..N` where `N` is the number of
parts in the upload. Parts may be uploaded concurrently and in any order.

Most stores require that all parts excluding the last are at least 5 MiB, and some
further require that all parts excluding the last be the same size, e.g. [R2].
[`WriteMultipart`] performs writes in fixed size blocks of 5 MiB, and clients wanting
to maximise compatibility should look to do likewise.

[R2]: https://developers.cloudflare.com/r2/objects/multipart-objects/#limitations
[`WriteMultipart`]: crate::upload::WriteMultipart
