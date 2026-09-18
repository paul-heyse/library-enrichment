# `datafusion_datasource_parquet::file_format::ObjectStoreFetch`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource_parquet.file_format.ObjectStoreFetch.json).

<a id="op-c08f295819ee43372fd8b18f"></a>
## ObjectStoreFetch

`struct` · `datafusion_datasource_parquet::file_format::ObjectStoreFetch` · datafusion-datasource-parquet 55.1.0

```rust
struct ObjectStoreFetch<'a>
```

Source: `src/file_format.rs:607`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

[`MetadataFetch`](../operations/parquet.arrow.async_reader.metadata.MetadataFetch.md#op-2f1a6adade5c3e921474d943) adapter for reading bytes from an [`ObjectStore`](../operations/object_store.ObjectStore.md#op-94894eaf9e5f6b785baca8ca)

<a id="op-50bb1eb8aea713ca38939c03"></a>
## fetch

`function` · `datafusion_datasource_parquet::file_format::ObjectStoreFetch::fetch` · datafusion-datasource-parquet 55.1.0

```rust
fn fetch(&mut self, range: Range<u64>) -> BoxFuture<'_, Result<Bytes, ParquetError>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "datafusion_datasource_parquet::file_format::ObjectStoreFetch", "path": "ObjectStoreFetch"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [618, 1], "end": [628, 2], "filename": "src/file_format.rs"}, "trait": {"args": null, "id": "parquet::arrow::async_reader::metadata::MetadataFetch", "path": "MetadataFetch"}, "trait_path": "parquet::arrow::async_reader::metadata::MetadataFetch"}`

Source: `src/file_format.rs:619`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ae788716d32e0677146cf9da"></a>
## new

`function` · `datafusion_datasource_parquet::file_format::ObjectStoreFetch::new` · datafusion-datasource-parquet 55.1.0

```rust
fn new(store: &'a dyn ObjectStore, meta: &'a ObjectMeta) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_datasource_parquet::file_format::ObjectStoreFetch", "path": "ObjectStoreFetch"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [612, 1], "end": [616, 2], "filename": "src/file_format.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_format.rs:613`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-parquet/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
