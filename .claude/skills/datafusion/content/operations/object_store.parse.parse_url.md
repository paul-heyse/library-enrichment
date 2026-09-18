# `object_store::parse::parse_url`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.parse.parse_url.json).

<a id="op-e7654980e962e003895cf511"></a>
## parse_url

`function` · `object_store::parse::parse_url` · object_store 0.13.2

```rust
fn parse_url(url: &url::Url) -> Result<(Box<dyn ObjectStore>, path::Path), super::Error>
```

Source: `src/parse.rs:161`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Create an [`ObjectStore`](../operations/object_store.ObjectStore.md#op-94894eaf9e5f6b785baca8ca) based on the provided `url`

Returns
- An [`ObjectStore`](../operations/object_store.ObjectStore.md#op-94894eaf9e5f6b785baca8ca) of the corresponding type
- The [`Path`](../operations/object_store.path.Path.md#op-387136a1d8baa9d740b7fc0b) into the [`ObjectStore`](../operations/object_store.ObjectStore.md#op-94894eaf9e5f6b785baca8ca) of the addressed resource
