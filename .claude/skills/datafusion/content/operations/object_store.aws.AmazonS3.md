# `object_store::aws::AmazonS3`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.aws.AmazonS3.json).

<a id="op-792ec7caebfaa6563754cd2e"></a>
## AmazonS3

`struct` · `object_store::aws::AmazonS3` · object_store 0.13.2

```rust
struct AmazonS3
```

Source: `src/aws/mod.rs:84`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Interface for [Amazon S3](https://aws.amazon.com/s3/).

<a id="op-6bab09f9bc9e15095fa10007"></a>
## abort_multipart

`function` · `object_store::aws::AmazonS3::abort_multipart` · object_store 0.13.2

```rust
async fn abort_multipart(&self, path: &Path, id: &MultipartId) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::aws::AmazonS3", "path": "AmazonS3"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [462, 1], "end": [500, 2], "filename": "src/aws/mod.rs"}, "trait": {"args": null, "id": "object_store::multipart::MultipartStore", "path": "MultipartStore"}, "trait_path": "object_store::multipart::MultipartStore"}`

Source: `src/aws/mod.rs:492`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c1108e2d604961375720165d"></a>
## clone

`function` · `object_store::aws::AmazonS3::clone` · object_store 0.13.2

```rust
fn clone(&self) -> AmazonS3
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::aws::AmazonS3", "path": "AmazonS3"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [83, 17], "end": [83, 22], "filename": "src/aws/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/aws/mod.rs:83`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cdf826789953051109f81408"></a>
## complete_multipart

`function` · `object_store::aws::AmazonS3::complete_multipart` · object_store 0.13.2

```rust
async fn complete_multipart(&self, path: &Path, id: &MultipartId, parts: Vec<PartId>) -> Result<PutResult>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::aws::AmazonS3", "path": "AmazonS3"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [462, 1], "end": [500, 2], "filename": "src/aws/mod.rs"}, "trait": {"args": null, "id": "object_store::multipart::MultipartStore", "path": "MultipartStore"}, "trait_path": "object_store::multipart::MultipartStore"}`

Source: `src/aws/mod.rs:481`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a10d553bf7c27e44ad086e37"></a>
## copy_opts

`function` · `object_store::aws::AmazonS3::copy_opts` · object_store 0.13.2

```rust
async fn copy_opts(&self, from: &Path, to: &Path, options: CopyOptions) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::aws::AmazonS3", "path": "AmazonS3"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [158, 1], "end": [397, 2], "filename": "src/aws/mod.rs"}, "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}, "trait_path": "object_store::ObjectStore"}`

Source: `src/aws/mod.rs:312`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4db798e521d266721ac22b95"></a>
## create_multipart

`function` · `object_store::aws::AmazonS3::create_multipart` · object_store 0.13.2

```rust
async fn create_multipart(&self, path: &Path) -> Result<MultipartId>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::aws::AmazonS3", "path": "AmazonS3"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [462, 1], "end": [500, 2], "filename": "src/aws/mod.rs"}, "trait": {"args": null, "id": "object_store::multipart::MultipartStore", "path": "MultipartStore"}, "trait_path": "object_store::multipart::MultipartStore"}`

Source: `src/aws/mod.rs:463`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bbf1836b785ca81da1615d2b"></a>
## credentials

`function` · `object_store::aws::AmazonS3::credentials` · object_store 0.13.2

```rust
fn credentials(&self) -> &AwsCredentialProvider
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::aws::AmazonS3", "path": "AmazonS3"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [94, 1], "end": [104, 2], "filename": "src/aws/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/aws/mod.rs:96`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Returns the [`AwsCredentialProvider`](../operations/object_store.aws.AwsCredentialProvider.md#op-c90753c8319c4a398efcba53) used by [`AmazonS3`](../operations/object_store.aws.AmazonS3.md#op-792ec7caebfaa6563754cd2e)

<a id="op-b55cc71bd43f267fc9b2bee9"></a>
## delete_stream

`function` · `object_store::aws::AmazonS3::delete_stream` · object_store 0.13.2

```rust
fn delete_stream(&self, locations: BoxStream<'static, Result<Path>>) -> BoxStream<'static, Result<Path>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::aws::AmazonS3", "path": "AmazonS3"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [158, 1], "end": [397, 2], "filename": "src/aws/mod.rs"}, "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}, "trait_path": "object_store::ObjectStore"}`

Source: `src/aws/mod.rs:262`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8e15e8018fb39519aefae384"></a>
## fmt

`function` · `object_store::aws::AmazonS3::fmt` · object_store 0.13.2

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::aws::AmazonS3", "path": "AmazonS3"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [88, 1], "end": [92, 2], "filename": "src/aws/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/aws/mod.rs:89`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e2c34da5c1f0c58e73c77d11"></a>
## fmt

`function` · `object_store::aws::AmazonS3::fmt` · object_store 0.13.2

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::aws::AmazonS3", "path": "AmazonS3"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [83, 10], "end": [83, 15], "filename": "src/aws/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/aws/mod.rs:83`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cafdbdf3cc6016377b528147"></a>
## get_opts

`function` · `object_store::aws::AmazonS3::get_opts` · object_store 0.13.2

```rust
async fn get_opts(&self, location: &Path, options: GetOptions) -> Result<GetResult>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::aws::AmazonS3", "path": "AmazonS3"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [158, 1], "end": [397, 2], "filename": "src/aws/mod.rs"}, "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}, "trait_path": "object_store::ObjectStore"}`

Source: `src/aws/mod.rs:258`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-817b5b82f1737d31ec3ea1f7"></a>
## list

`function` · `object_store::aws::AmazonS3::list` · object_store 0.13.2

```rust
fn list(&self, prefix: Option<&Path>) -> BoxStream<'static, Result<ObjectMeta>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::aws::AmazonS3", "path": "AmazonS3"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [158, 1], "end": [397, 2], "filename": "src/aws/mod.rs"}, "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}, "trait_path": "object_store::ObjectStore"}`

Source: `src/aws/mod.rs:286`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-db2af2c1e4682fe3af93937f"></a>
## list_paginated

`function` · `object_store::aws::AmazonS3::list_paginated` · object_store 0.13.2

```rust
async fn list_paginated(&self, prefix: Option<&str>, opts: PaginatedListOptions) -> Result<PaginatedListResult>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::aws::AmazonS3", "path": "AmazonS3"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [503, 1], "end": [511, 2], "filename": "src/aws/mod.rs"}, "trait": {"args": null, "id": "object_store::list::PaginatedListStore", "path": "PaginatedListStore"}, "trait_path": "object_store::list::PaginatedListStore"}`

Source: `src/aws/mod.rs:504`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cff9b8e800eb57e1396f5905"></a>
## list_with_delimiter

`function` · `object_store::aws::AmazonS3::list_with_delimiter` · object_store 0.13.2

```rust
async fn list_with_delimiter(&self, prefix: Option<&Path>) -> Result<ListResult>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::aws::AmazonS3", "path": "AmazonS3"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [158, 1], "end": [397, 2], "filename": "src/aws/mod.rs"}, "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}, "trait_path": "object_store::ObjectStore"}`

Source: `src/aws/mod.rs:308`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8addd3a9960468bc8d088578"></a>
## list_with_offset

`function` · `object_store::aws::AmazonS3::list_with_offset` · object_store 0.13.2

```rust
fn list_with_offset(&self, prefix: Option<&Path>, offset: &Path) -> BoxStream<'static, Result<ObjectMeta>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::aws::AmazonS3", "path": "AmazonS3"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [158, 1], "end": [397, 2], "filename": "src/aws/mod.rs"}, "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}, "trait_path": "object_store::ObjectStore"}`

Source: `src/aws/mod.rs:290`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f51491d8e6850ba0fb9d2cd9"></a>
## put_multipart_opts

`function` · `object_store::aws::AmazonS3::put_multipart_opts` · object_store 0.13.2

```rust
async fn put_multipart_opts(&self, location: &Path, opts: PutMultipartOptions) -> Result<Box<dyn MultipartUpload>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::aws::AmazonS3", "path": "AmazonS3"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [158, 1], "end": [397, 2], "filename": "src/aws/mod.rs"}, "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}, "trait_path": "object_store::ObjectStore"}`

Source: `src/aws/mod.rs:240`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6abb9d066d5e626ae455e133"></a>
## put_opts

`function` · `object_store::aws::AmazonS3::put_opts` · object_store 0.13.2

```rust
async fn put_opts(&self, location: &Path, payload: PutPayload, opts: PutOptions) -> Result<PutResult>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::aws::AmazonS3", "path": "AmazonS3"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [158, 1], "end": [397, 2], "filename": "src/aws/mod.rs"}, "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}, "trait_path": "object_store::ObjectStore"}`

Source: `src/aws/mod.rs:159`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-69cd1c0dcb2df9184ce33b1a"></a>
## put_part

`function` · `object_store::aws::AmazonS3::put_part` · object_store 0.13.2

```rust
async fn put_part(&self, path: &Path, id: &MultipartId, part_idx: usize, data: PutPayload) -> Result<PartId>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::aws::AmazonS3", "path": "AmazonS3"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [462, 1], "end": [500, 2], "filename": "src/aws/mod.rs"}, "trait": {"args": null, "id": "object_store::multipart::MultipartStore", "path": "MultipartStore"}, "trait_path": "object_store::multipart::MultipartStore"}`

Source: `src/aws/mod.rs:469`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-507be9a0dc15fc374b1e38de"></a>
## signed_url

`function` · `object_store::aws::AmazonS3::signed_url` · object_store 0.13.2

```rust
async fn signed_url(&self, method: Method, path: &Path, expires_in: Duration) -> Result<Url>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::aws::AmazonS3", "path": "AmazonS3"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [107, 1], "end": [155, 2], "filename": "src/aws/mod.rs"}, "trait": {"args": null, "id": "object_store::signer::Signer", "path": "Signer"}, "trait_path": "object_store::signer::Signer"}`

Source: `src/aws/mod.rs:140`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Create a URL containing the relevant [AWS SigV4] query parameters that authorize a request
via `method` to the resource at `path` valid for the duration specified in `expires_in`.

[AWS SigV4]: https://docs.aws.amazon.com/IAM/latest/UserGuide/create-signed-request.html

# Example

This example returns a URL that will enable a user to upload a file to
"some-folder/some-file.txt" in the next hour.

```
# async fn example() -> Result<(), Box<dyn std::error::Error>> {
# use object_store::{aws::AmazonS3Builder, path::Path, signer::Signer};
# use reqwest::Method;
# use std::time::Duration;
#
let region = "us-east-1";
let s3 = AmazonS3Builder::new()
    .with_region(region)
    .with_bucket_name("my-bucket")
    .with_access_key_id("my-access-key-id")
    .with_secret_access_key("my-secret-access-key")
    .build()?;

let url = s3.signed_url(
    Method::PUT,
    &Path::from("some-folder/some-file.txt"),
    Duration::from_secs(60 * 60)
).await?;
#     Ok(())
# }
```
