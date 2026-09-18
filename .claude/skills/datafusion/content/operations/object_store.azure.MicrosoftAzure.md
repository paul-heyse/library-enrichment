# `object_store::azure::MicrosoftAzure`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.azure.MicrosoftAzure.json).

<a id="op-a50607143b9beb78c537c27a"></a>
## MicrosoftAzure

`struct` · `object_store::azure::MicrosoftAzure` · object_store 0.13.2

```rust
struct MicrosoftAzure
```

Source: `src/azure/mod.rs:63`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Interface for [Microsoft Azure Blob Storage](https://azure.microsoft.com/en-us/services/storage/blobs/).

<a id="op-4b212e4c7fa750eca3708dca"></a>
## abort_multipart

`function` · `object_store::azure::MicrosoftAzure::abort_multipart` · object_store 0.13.2

```rust
async fn abort_multipart(&self, _: &Path, _: &MultipartId) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::azure::MicrosoftAzure", "path": "MicrosoftAzure"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [289, 1], "end": [320, 2], "filename": "src/azure/mod.rs"}, "trait": {"args": null, "id": "object_store::multipart::MultipartStore", "path": "MultipartStore"}, "trait_path": "object_store::multipart::MultipartStore"}`

Source: `src/azure/mod.rs:315`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-13ac752f4e356bdef7e5e0b7"></a>
## complete_multipart

`function` · `object_store::azure::MicrosoftAzure::complete_multipart` · object_store 0.13.2

```rust
async fn complete_multipart(&self, path: &Path, _: &MultipartId, parts: Vec<PartId>) -> Result<PutResult>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::azure::MicrosoftAzure", "path": "MicrosoftAzure"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [289, 1], "end": [320, 2], "filename": "src/azure/mod.rs"}, "trait": {"args": null, "id": "object_store::multipart::MultipartStore", "path": "MultipartStore"}, "trait_path": "object_store::multipart::MultipartStore"}`

Source: `src/azure/mod.rs:304`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f61660d64ff177986255d708"></a>
## copy_opts

`function` · `object_store::azure::MicrosoftAzure::copy_opts` · object_store 0.13.2

```rust
async fn copy_opts(&self, from: &Path, to: &Path, options: CopyOptions) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::azure::MicrosoftAzure", "path": "MicrosoftAzure"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [91, 1], "end": [183, 2], "filename": "src/azure/mod.rs"}, "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}, "trait_path": "object_store::ObjectStore"}`

Source: `src/azure/mod.rs:172`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-98f79729515b77110e0cf99c"></a>
## create_multipart

`function` · `object_store::azure::MicrosoftAzure::create_multipart` · object_store 0.13.2

```rust
async fn create_multipart(&self, _: &Path) -> Result<MultipartId>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::azure::MicrosoftAzure", "path": "MicrosoftAzure"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [289, 1], "end": [320, 2], "filename": "src/azure/mod.rs"}, "trait": {"args": null, "id": "object_store::multipart::MultipartStore", "path": "MultipartStore"}, "trait_path": "object_store::multipart::MultipartStore"}`

Source: `src/azure/mod.rs:290`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-45fa2875481bb21884fffce1"></a>
## credentials

`function` · `object_store::azure::MicrosoftAzure::credentials` · object_store 0.13.2

```rust
fn credentials(&self) -> &AzureCredentialProvider
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::azure::MicrosoftAzure", "path": "MicrosoftAzure"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [67, 1], "end": [77, 2], "filename": "src/azure/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/azure/mod.rs:69`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Returns the [`AzureCredentialProvider`](../operations/object_store.azure.AzureCredentialProvider.md#op-83656ccde14f028af0230a2a) used by [`MicrosoftAzure`](../operations/object_store.azure.MicrosoftAzure.md#op-a50607143b9beb78c537c27a)

<a id="op-4b51a7fe73de89994a59f5b0"></a>
## delete_stream

`function` · `object_store::azure::MicrosoftAzure::delete_stream` · object_store 0.13.2

```rust
fn delete_stream(&self, locations: BoxStream<'static, Result<Path>>) -> BoxStream<'static, Result<Path>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::azure::MicrosoftAzure", "path": "MicrosoftAzure"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [91, 1], "end": [183, 2], "filename": "src/azure/mod.rs"}, "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}, "trait_path": "object_store::ObjectStore"}`

Source: `src/azure/mod.rs:144`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-06d49ef9ebd4c1a72ef05e18"></a>
## fmt

`function` · `object_store::azure::MicrosoftAzure::fmt` · object_store 0.13.2

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::azure::MicrosoftAzure", "path": "MicrosoftAzure"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [79, 1], "end": [88, 2], "filename": "src/azure/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/azure/mod.rs:80`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2c958e8b9c923278e2e2d349"></a>
## fmt

`function` · `object_store::azure::MicrosoftAzure::fmt` · object_store 0.13.2

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::azure::MicrosoftAzure", "path": "MicrosoftAzure"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [62, 10], "end": [62, 15], "filename": "src/azure/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/azure/mod.rs:62`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f8367350766a4568e565c4d1"></a>
## get_opts

`function` · `object_store::azure::MicrosoftAzure::get_opts` · object_store 0.13.2

```rust
async fn get_opts(&self, location: &Path, options: GetOptions) -> Result<GetResult>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::azure::MicrosoftAzure", "path": "MicrosoftAzure"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [91, 1], "end": [183, 2], "filename": "src/azure/mod.rs"}, "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}, "trait_path": "object_store::ObjectStore"}`

Source: `src/azure/mod.rs:117`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0e5c5d4c32dd31cfa9e18eb9"></a>
## list

`function` · `object_store::azure::MicrosoftAzure::list` · object_store 0.13.2

```rust
fn list(&self, prefix: Option<&Path>) -> BoxStream<'static, Result<ObjectMeta>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::azure::MicrosoftAzure", "path": "MicrosoftAzure"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [91, 1], "end": [183, 2], "filename": "src/azure/mod.rs"}, "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}, "trait_path": "object_store::ObjectStore"}`

Source: `src/azure/mod.rs:121`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dae6fefab37ba71dd0c2cf04"></a>
## list_paginated

`function` · `object_store::azure::MicrosoftAzure::list_paginated` · object_store 0.13.2

```rust
async fn list_paginated(&self, prefix: Option<&str>, opts: PaginatedListOptions) -> Result<PaginatedListResult>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::azure::MicrosoftAzure", "path": "MicrosoftAzure"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [323, 1], "end": [331, 2], "filename": "src/azure/mod.rs"}, "trait": {"args": null, "id": "object_store::list::PaginatedListStore", "path": "PaginatedListStore"}, "trait_path": "object_store::list::PaginatedListStore"}`

Source: `src/azure/mod.rs:324`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f5f390a8165f24c8514a573d"></a>
## list_with_delimiter

`function` · `object_store::azure::MicrosoftAzure::list_with_delimiter` · object_store 0.13.2

```rust
async fn list_with_delimiter(&self, prefix: Option<&Path>) -> Result<ListResult>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::azure::MicrosoftAzure", "path": "MicrosoftAzure"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [91, 1], "end": [183, 2], "filename": "src/azure/mod.rs"}, "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}, "trait_path": "object_store::ObjectStore"}`

Source: `src/azure/mod.rs:168`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-aac0d240e36a247599060910"></a>
## list_with_offset

`function` · `object_store::azure::MicrosoftAzure::list_with_offset` · object_store 0.13.2

```rust
fn list_with_offset(&self, prefix: Option<&Path>, offset: &Path) -> BoxStream<'static, Result<ObjectMeta>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::azure::MicrosoftAzure", "path": "MicrosoftAzure"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [91, 1], "end": [183, 2], "filename": "src/azure/mod.rs"}, "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}, "trait_path": "object_store::ObjectStore"}`

Source: `src/azure/mod.rs:125`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-23148f9bbfcbde34f3618ef1"></a>
## put_multipart_opts

`function` · `object_store::azure::MicrosoftAzure::put_multipart_opts` · object_store 0.13.2

```rust
async fn put_multipart_opts(&self, location: &Path, opts: PutMultipartOptions) -> Result<Box<dyn MultipartUpload>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::azure::MicrosoftAzure", "path": "MicrosoftAzure"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [91, 1], "end": [183, 2], "filename": "src/azure/mod.rs"}, "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}, "trait_path": "object_store::ObjectStore"}`

Source: `src/azure/mod.rs:101`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4071ae8c9e770bcabbbb1905"></a>
## put_opts

`function` · `object_store::azure::MicrosoftAzure::put_opts` · object_store 0.13.2

```rust
async fn put_opts(&self, location: &Path, payload: PutPayload, opts: PutOptions) -> Result<PutResult>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::azure::MicrosoftAzure", "path": "MicrosoftAzure"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [91, 1], "end": [183, 2], "filename": "src/azure/mod.rs"}, "trait": {"args": null, "id": "object_store::ObjectStore", "path": "ObjectStore"}, "trait_path": "object_store::ObjectStore"}`

Source: `src/azure/mod.rs:92`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9432e75ec4b63441c87fd68e"></a>
## put_part

`function` · `object_store::azure::MicrosoftAzure::put_part` · object_store 0.13.2

```rust
async fn put_part(&self, path: &Path, _: &MultipartId, part_idx: usize, data: PutPayload) -> Result<PartId>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::azure::MicrosoftAzure", "path": "MicrosoftAzure"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [289, 1], "end": [320, 2], "filename": "src/azure/mod.rs"}, "trait": {"args": null, "id": "object_store::multipart::MultipartStore", "path": "MultipartStore"}, "trait_path": "object_store::multipart::MultipartStore"}`

Source: `src/azure/mod.rs:294`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c7f98d3908341cfcbb748417"></a>
## signed_url

`function` · `object_store::azure::MicrosoftAzure::signed_url` · object_store 0.13.2

```rust
async fn signed_url(&self, method: Method, path: &Path, expires_in: Duration) -> Result<Url>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::azure::MicrosoftAzure", "path": "MicrosoftAzure"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [186, 1], "end": [239, 2], "filename": "src/azure/mod.rs"}, "trait": {"args": null, "id": "object_store::signer::Signer", "path": "Signer"}, "trait_path": "object_store::signer::Signer"}`

Source: `src/azure/mod.rs:217`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Create a URL containing the relevant [Service SAS] query parameters that authorize a request
via `method` to the resource at `path` valid for the duration specified in `expires_in`.

[Service SAS]: https://learn.microsoft.com/en-us/rest/api/storageservices/create-service-sas

# Example

This example returns a URL that will enable a user to upload a file to
"some-folder/some-file.txt" in the next hour.

```
# async fn example() -> Result<(), Box<dyn std::error::Error>> {
# use object_store::{azure::MicrosoftAzureBuilder, path::Path, signer::Signer};
# use reqwest::Method;
# use std::time::Duration;
#
let azure = MicrosoftAzureBuilder::new()
    .with_account("my-account")
    .with_access_key("my-access-key")
    .with_container_name("my-container")
    .build()?;

let url = azure.signed_url(
    Method::PUT,
    &Path::from("some-folder/some-file.txt"),
    Duration::from_secs(60 * 60)
).await?;
#     Ok(())
# }
```

<a id="op-dfe3cd98ba324eb706fe1715"></a>
## signed_urls

`function` · `object_store::azure::MicrosoftAzure::signed_urls` · object_store 0.13.2

```rust
async fn signed_urls(&self, method: Method, paths: &[Path], expires_in: Duration) -> Result<Vec<Url>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::azure::MicrosoftAzure", "path": "MicrosoftAzure"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [186, 1], "end": [239, 2], "filename": "src/azure/mod.rs"}, "trait": {"args": null, "id": "object_store::signer::Signer", "path": "Signer"}, "trait_path": "object_store::signer::Signer"}`

Source: `src/azure/mod.rs:224`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.
