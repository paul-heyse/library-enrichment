# `object_store::gcp`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.gcp.json).

<a id="op-397cb506fa80f7a4f9953b4d"></a>
## gcp

`module` · `object_store::gcp` · object_store 0.13.2

```rust
mod gcp
```

Source: `src/gcp/mod.rs:18`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

An object store implementation for Google Cloud Storage

## Multipart uploads

[Multipart uploads](https://cloud.google.com/storage/docs/multipart-uploads)
can be initiated with the [`ObjectStore::put_multipart_opts`](../operations/object_store.ObjectStore.md#op-27dfce2ef1fd51c4e2938336) method. If neither
[`MultipartUpload::complete`](../operations/object_store.upload.MultipartUpload.md#op-a1a4e1e4e03df4fa1f4622b1) nor [`MultipartUpload::abort`](../operations/object_store.upload.MultipartUpload.md#op-d0ab4a61fdda48f1d5be2d47) is invoked, you may
have parts uploaded to GCS but not used, that you will be charged for. It is recommended
you configure a [lifecycle rule] to abort incomplete multipart uploads after a certain
period of time to avoid being charged for storing partial uploads.

## Using HTTP/2

Google Cloud Storage supports both HTTP/2 and HTTP/1. HTTP/1 is used by default
because it allows much higher throughput in our benchmarks (see
[#5194](https://github.com/apache/arrow-rs/issues/5194)). HTTP/2 can be
enabled by setting [crate::ClientConfigKey::Http1Only](../operations/object_store.client.ClientConfigKey.md#op-ae7a0c48c4e1eb5f71a29bd0) to false.

[lifecycle rule]: https://cloud.google.com/storage/docs/lifecycle#abort-mpu
