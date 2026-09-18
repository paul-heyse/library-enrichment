# `object_store::aws`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.aws.json).

<a id="op-91f16db41cdd768042f8c13f"></a>
## aws

`module` · `object_store::aws` · object_store 0.13.2

```rust
mod aws
```

Source: `src/aws/mod.rs:18`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

An object store implementation for S3

## Multipart uploads

Multipart uploads can be initiated with the [`ObjectStore::put_multipart_opts`](../operations/object_store.ObjectStore.md#op-27dfce2ef1fd51c4e2938336) method.

If the writer fails for any reason, you may have parts uploaded to AWS but not
used that you will be charged for. [`MultipartUpload::abort`](../operations/object_store.upload.MultipartUpload.md#op-d0ab4a61fdda48f1d5be2d47) may be invoked to drop
these unneeded parts, however, it is recommended that you consider implementing
[automatic cleanup] of unused parts that are older than some threshold.

[automatic cleanup]: https://aws.amazon.com/blogs/aws/s3-lifecycle-management-update-support-for-multipart-uploads-and-delete-markers/
