# `object_store::parse::parse_url_opts`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.parse.parse_url_opts.json).

<a id="op-196a22ab979ecdcbef0d1d32"></a>
## parse_url_opts

`function` · `object_store::parse::parse_url_opts` · object_store 0.13.2

```rust
fn parse_url_opts<I, K, V>(url: &url::Url, options: I) -> Result<(Box<dyn ObjectStore>, path::Path), super::Error> where I: IntoIterator<Item = (K, V)>, K: AsRef<str>, V: Into<String>
```

Source: `src/parse.rs:187`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Create an [`ObjectStore`](../operations/object_store.ObjectStore.md#op-94894eaf9e5f6b785baca8ca) based on the provided `url` and options

This method can be used to create an instance of one of the provided
`ObjectStore` implementations based on the URL scheme (see
[`ObjectStoreScheme`](../operations/object_store.parse.ObjectStoreScheme.md#op-cfc57ebb4b0582b4fec3191b) for more details).

For example
* `file:///path/to/my/file` will return a [`LocalFileSystem`](../operations/object_store.local.LocalFileSystem.md#op-972e6ecc4b3db74073f32c0b) instance
* `s3://bucket/path` will return an [`AmazonS3`] instance if the `aws` feature is enabled.

Arguments:
* `url`: The URL to parse
* `options`: A list of key-value pairs to pass to the [`ObjectStore`](../operations/object_store.ObjectStore.md#op-94894eaf9e5f6b785baca8ca) builder.
  Note different object stores accept different configuration options, so
  the options that are read depends on the `url` value. One common pattern
  is to pass configuration information via process variables using [`std::env::vars`].

Returns
- An [`ObjectStore`](../operations/object_store.ObjectStore.md#op-94894eaf9e5f6b785baca8ca) of the corresponding type
- The [`Path`](../operations/object_store.path.Path.md#op-387136a1d8baa9d740b7fc0b) into the [`ObjectStore`](../operations/object_store.ObjectStore.md#op-94894eaf9e5f6b785baca8ca) of the addressed resource

[`AmazonS3`]: https://docs.rs/object_store/0.12.0/object_store/aws/struct.AmazonS3.html

Unresolved upstream links (retained, not inferred): ``std::env::vars``.
