# `object_store::aws::resolve`

Crate `object_store` · 1 public items · structured records in [`model/object_store.aws.resolve.json`](../model/object_store.aws.resolve.json)

## resolve_bucket_region

`function` · `object_store::aws::resolve::resolve_bucket_region`

Also reachable as `object_store::aws::resolve_bucket_region`

```rust
async fn resolve_bucket_region(bucket: &str, client_options: &ClientOptions) -> Result<String>
```

Get the bucket region using the [HeadBucket API]. This will fail if the bucket does not exist.

[HeadBucket API]: https://docs.aws.amazon.com/AmazonS3/latest/API/API_HeadBucket.html

---
