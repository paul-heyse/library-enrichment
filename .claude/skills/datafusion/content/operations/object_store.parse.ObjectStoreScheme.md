# `object_store::parse::ObjectStoreScheme`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.parse.ObjectStoreScheme.json).

<a id="op-cfc57ebb4b0582b4fec3191b"></a>
## ObjectStoreScheme

`enum` · `object_store::parse::ObjectStoreScheme` · object_store 0.13.2

```rust
enum ObjectStoreScheme
```

Source: `src/parse.rs:66`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Recognizes various URL formats, identifying the relevant [`ObjectStore`](../operations/object_store.ObjectStore.md#op-94894eaf9e5f6b785baca8ca)

See [`ObjectStoreScheme::parse`](../operations/object_store.parse.ObjectStoreScheme.md#op-91f99d192a42f43dd68083df) for more details

# Supported formats:
- `file:///path/to/my/file` -> [`LocalFileSystem`](../operations/object_store.local.LocalFileSystem.md#op-972e6ecc4b3db74073f32c0b)
- `memory:///` -> [`InMemory`](../operations/object_store.memory.InMemory.md#op-2b004086fc4faa5587daef80)
- `s3://bucket/path` -> [`AmazonS3`](crate::aws::AmazonS3) (also supports `s3a`)
- `gs://bucket/path` -> [`GoogleCloudStorage`](crate::gcp::GoogleCloudStorage)
- `[az|abfs[s]]://container[@<account>.<host>]/path` -> [`MicrosoftAzure`](crate::azure::MicrosoftAzure)
- `http://mydomain/path` -> [`HttpStore`](crate::http::HttpStore)
- `https://mydomain/path` -> [`HttpStore`](crate::http::HttpStore)

There are also special cases for AWS and Azure for `https://{host?}/path` paths:
- `dfs.core.windows.net`, `blob.core.windows.net`, `dfs.fabric.microsoft.com`, `blob.fabric.microsoft.com` -> [`MicrosoftAzure`](crate::azure::MicrosoftAzure)
- `amazonaws.com` -> [`AmazonS3`](crate::aws::AmazonS3)
- `r2.cloudflarestorage.com` -> [`AmazonS3`](crate::aws::AmazonS3)


<a id="op-53f8a955b1a42000d2a7cdf3"></a>
## AmazonS3

`variant` · `object_store::parse::ObjectStoreScheme::AmazonS3` · object_store 0.13.2

```rust
AmazonS3
```

Source: `src/parse.rs:72`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Url corresponding to [`AmazonS3`](crate::aws::AmazonS3)

<a id="op-43149259aedc3e33087bbc6a"></a>
## GoogleCloudStorage

`variant` · `object_store::parse::ObjectStoreScheme::GoogleCloudStorage` · object_store 0.13.2

```rust
GoogleCloudStorage
```

Source: `src/parse.rs:74`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Url corresponding to [`GoogleCloudStorage`](crate::gcp::GoogleCloudStorage)

<a id="op-4422f851908986b94d2f1967"></a>
## Http

`variant` · `object_store::parse::ObjectStoreScheme::Http` · object_store 0.13.2

```rust
Http
```

Source: `src/parse.rs:78`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Url corresponding to [`HttpStore`](crate::http::HttpStore)

<a id="op-62078d2c413731f59ac8465c"></a>
## Local

`variant` · `object_store::parse::ObjectStoreScheme::Local` · object_store 0.13.2

```rust
Local
```

Source: `src/parse.rs:68`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Url corresponding to [`LocalFileSystem`](../operations/object_store.local.LocalFileSystem.md#op-972e6ecc4b3db74073f32c0b)

<a id="op-bd3fee2d698644c2103a1a87"></a>
## Memory

`variant` · `object_store::parse::ObjectStoreScheme::Memory` · object_store 0.13.2

```rust
Memory
```

Source: `src/parse.rs:70`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Url corresponding to [`InMemory`](../operations/object_store.memory.InMemory.md#op-2b004086fc4faa5587daef80)

<a id="op-a6f47cd5280c3199f19d3866"></a>
## MicrosoftAzure

`variant` · `object_store::parse::ObjectStoreScheme::MicrosoftAzure` · object_store 0.13.2

```rust
MicrosoftAzure
```

Source: `src/parse.rs:76`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Url corresponding to [`MicrosoftAzure`](crate::azure::MicrosoftAzure)

<a id="op-f731acedd40592b03ed04a18"></a>
## clone

`function` · `object_store::parse::ObjectStoreScheme::clone` · object_store 0.13.2

```rust
fn clone(&self) -> ObjectStoreScheme
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::parse::ObjectStoreScheme", "path": "ObjectStoreScheme"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [65, 32], "end": [65, 37], "filename": "src/parse.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/parse.rs:65`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-222173ae405ac5d7197bc8fb"></a>
## eq

`function` · `object_store::parse::ObjectStoreScheme::eq` · object_store 0.13.2

```rust
fn eq(&self, other: &ObjectStoreScheme) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::parse::ObjectStoreScheme", "path": "ObjectStoreScheme"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [65, 21], "end": [65, 30], "filename": "src/parse.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/parse.rs:65`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-aab56aa3951ec7794b66b38f"></a>
## fmt

`function` · `object_store::parse::ObjectStoreScheme::fmt` · object_store 0.13.2

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::parse::ObjectStoreScheme", "path": "ObjectStoreScheme"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [65, 10], "end": [65, 15], "filename": "src/parse.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/parse.rs:65`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-91f99d192a42f43dd68083df"></a>
## parse

`function` · `object_store::parse::ObjectStoreScheme::parse` · object_store 0.13.2

```rust
fn parse(url: &Url) -> Result<(Self, Path), Error>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::parse::ObjectStoreScheme", "path": "ObjectStoreScheme"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [81, 1], "end": [140, 2], "filename": "src/parse.rs"}, "trait": null, "trait_path": null}`

Source: `src/parse.rs:105`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Create an [`ObjectStoreScheme`](../operations/object_store.parse.ObjectStoreScheme.md#op-cfc57ebb4b0582b4fec3191b) from the provided [`Url`]

Returns the [`ObjectStoreScheme`](../operations/object_store.parse.ObjectStoreScheme.md#op-cfc57ebb4b0582b4fec3191b) and the remaining [`Path`](../operations/object_store.path.Path.md#op-387136a1d8baa9d740b7fc0b)

# Example
```
# use url::Url;
# use object_store::ObjectStoreScheme;
let url: Url = "file:///path/to/my/file".parse().unwrap();
let (scheme, path) = ObjectStoreScheme::parse(&url).unwrap();
assert_eq!(scheme, ObjectStoreScheme::Local);
assert_eq!(path.as_ref(), "path/to/my/file");

let url: Url = "https://blob.core.windows.net/container/path/to/my/file".parse().unwrap();
let (scheme, path) = ObjectStoreScheme::parse(&url).unwrap();
assert_eq!(scheme, ObjectStoreScheme::MicrosoftAzure);
assert_eq!(path.as_ref(), "path/to/my/file");

let url: Url = "https://example.com/path/to/my/file".parse().unwrap();
let (scheme, path) = ObjectStoreScheme::parse(&url).unwrap();
assert_eq!(scheme, ObjectStoreScheme::Http);
assert_eq!(path.as_ref(), "path/to/my/file");
```

Unresolved upstream links (retained, not inferred): ``Url``.
