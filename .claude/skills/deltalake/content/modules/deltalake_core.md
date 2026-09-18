# `deltalake_core`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.json).

<a id="op-8b6174e0942219488f9d16f9"></a>
## deltalake_core

`module` · `deltalake_core` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
mod deltalake_core
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/lib.rs#L1).

Source: `crates/core/src/lib.rs:1`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Native Delta Lake implementation in Rust

# Usage

Load a Delta Table by URL:

```rust
# use url::Url;
async {
  let table_url = Url::from_directory_path("/abs/test/tests/data/simple_table").unwrap();
  let table = deltalake_core::open_table(table_url).await.unwrap();
  let version = table.version();
};
```

Load a specific version of Delta Table by URL then filter files by partitions:

```rust
# use url::Url;
async {
  let table_url = Url::from_directory_path("/abs/test/tests/data/simple_table").unwrap();
  let table = deltalake_core::open_table_with_version(table_url, 0).await.unwrap();
  let filter = [(
      "month",
      deltalake_core::FilterOp::Eq,
      deltalake_core::FilterValue::Scalar("12"),
  )];
  let files = table.get_files_by_partitions(&filter).await.unwrap();
};
```

Load a specific version of Delta Table by URL and datetime:

```rust
# use url::Url;
async {
  let table_url = Url::from_directory_path("../test/tests/data/simple_table").unwrap();
  let table = deltalake_core::open_table_with_ds(
      table_url,
      "2020-05-02T23:47:31-07:00",
  ).await.unwrap();
  let version = table.version();
};
```

# Optional cargo package features

- `s3`, `gcs`, `azure` - enable the storage backends for AWS S3, Google Cloud Storage (GCS),
  or Azure Blob Storage / Azure Data Lake Storage Gen2 (ADLS2). Use `s3-native-tls` to use native TLS
  instead of Rust TLS implementation.
- `datafusion` - enable the `datafusion::datasource::TableProvider` trait implementation
  for Delta Tables, allowing them to be queried using [DataFusion](https://github.com/apache/arrow-datafusion).
- `datafusion-ext` - DEPRECATED: alias for `datafusion` feature.

# Querying Delta Tables with Datafusion

Querying from local filesystem:
```
use std::sync::Arc;
# use url::Url;

# #[cfg(feature="datafusion")]
async {
  use datafusion::prelude::SessionContext;
  let mut ctx = SessionContext::new();
  let table_url = Url::from_directory_path("/abs/test/tests/data/simple_table").unwrap();
  let table = deltalake_core::open_table(table_url)
      .await
      .unwrap();
  ctx.register_table("demo", table.table_provider().await.unwrap()).unwrap();

  let batches = ctx
      .sql("SELECT * FROM demo").await.unwrap()
      .collect()
      .await.unwrap();
};
```
