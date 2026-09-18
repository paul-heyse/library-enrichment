# `object_store::parse`

Crate `object_store` · 4 public items · structured records in [`model/object_store.parse.json`](../model/object_store.parse.json)

## Error

`enum` · `object_store::parse::Error`

```rust
enum Error
```

**Variants**: `Unrecognised`, `Path`

[Full member, field, variant and typed contracts](../operations/object_store.parse.Error.md).


---

## ObjectStoreScheme

`enum` · `object_store::parse::ObjectStoreScheme`

Also reachable as `datafusion::object_store::ObjectStoreScheme`, `object_store::ObjectStoreScheme`

```rust
enum ObjectStoreScheme
```

**Variants**: `Local`, `Memory`, `AmazonS3`, `GoogleCloudStorage`, `MicrosoftAzure`, `Http`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn parse(url: &Url) -> Result<(Self, Path), Error>
```

[Full member, field, variant and typed contracts](../operations/object_store.parse.ObjectStoreScheme.md).


Recognizes various URL formats, identifying the relevant [`ObjectStore`]

See [`ObjectStoreScheme::parse`] for more details

# Supported formats:
- `file:///path/to/my/file` -> [`LocalFileSystem`]
- `memory:///` -> [`InMemory`]
- `s3://bucket/path` -> [`AmazonS3`](crate::aws::AmazonS3) (also supports `s3a`)
- `gs://bucket/path` -> [`GoogleCloudStorage`](crate::gcp::GoogleCloudStorage)
- `[az|abfs[s]]://container[@<account>.<host>]/path` -> [`MicrosoftAzure`](crate::azure::MicrosoftAzure)
- `http://mydomain/path` -> [`HttpStore`](crate::http::HttpStore)
- `https://mydomain/path` -> [`HttpStore`](crate::http::HttpStore)

There are also special cases for AWS and Azure for `https://{host?}/path` paths:
- `dfs.core.windows.net`, `blob.core.windows.net`, `dfs.fabric.microsoft.com`, `blob.fabric.microsoft.com` -> [`MicrosoftAzure`](crate::azure::MicrosoftAzure)
- `amazonaws.com` -> [`AmazonS3`](crate::aws::AmazonS3)
- `r2.cloudflarestorage.com` -> [`AmazonS3`](crate::aws::AmazonS3)

---

## parse_url

`function` · `object_store::parse::parse_url`

Also reachable as `datafusion::object_store::parse_url`, `object_store::parse_url`

```rust
fn parse_url(url: &url::Url) -> Result<(Box<dyn ObjectStore>, path::Path), super::Error>
```

[Full member, field, variant and typed contracts](../operations/object_store.parse.parse_url.md).


Create an [`ObjectStore`] based on the provided `url`

Returns
- An [`ObjectStore`] of the corresponding type
- The [`Path`] into the [`ObjectStore`] of the addressed resource

---

## parse_url_opts

`function` · `object_store::parse::parse_url_opts`

Also reachable as `datafusion::object_store::parse_url_opts`, `object_store::parse_url_opts`

```rust
fn parse_url_opts<I, K, V>(url: &url::Url, options: I) -> Result<(Box<dyn ObjectStore>, path::Path), super::Error> where I: IntoIterator<Item = (K, V)>, K: AsRef<str>, V: Into<String>
```

[Full member, field, variant and typed contracts](../operations/object_store.parse.parse_url_opts.md).


Create an [`ObjectStore`] based on the provided `url` and options

This method can be used to create an instance of one of the provided
`ObjectStore` implementations based on the URL scheme (see
[`ObjectStoreScheme`] for more details).

For example
* `file:///path/to/my/file` will return a [`LocalFileSystem`] instance
* `s3://bucket/path` will return an [`AmazonS3`] instance if the `aws` feature is enabled.

Arguments:
* `url`: The URL to parse
* `options`: A list of key-value pairs to pass to the [`ObjectStore`] builder.
  Note different object stores accept different configuration options, so
  the options that are read depends on the `url` value. One common pattern
  is to pass configuration information via process variables using [`std::env::vars`].

Returns
- An [`ObjectStore`] of the corresponding type
- The [`Path`] into the [`ObjectStore`] of the addressed resource

[`AmazonS3`]: https://docs.rs/object_store/0.12.0/object_store/aws/struct.AmazonS3.html

---
