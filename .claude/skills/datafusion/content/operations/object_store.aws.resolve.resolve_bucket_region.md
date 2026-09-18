# `object_store::aws::resolve::resolve_bucket_region`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.aws.resolve.resolve_bucket_region.json).

<a id="op-38f2810ad70a1b11e2e6dfe8"></a>
## resolve_bucket_region

`function` · `object_store::aws::resolve::resolve_bucket_region` · object_store 0.13.2

```rust
async fn resolve_bucket_region(bucket: &str, client_options: &ClientOptions) -> Result<String>
```

Source: `src/aws/resolve.rs:49`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Get the bucket region using the [HeadBucket API]. This will fail if the bucket does not exist.

[HeadBucket API]: https://docs.aws.amazon.com/AmazonS3/latest/API/API_HeadBucket.html
