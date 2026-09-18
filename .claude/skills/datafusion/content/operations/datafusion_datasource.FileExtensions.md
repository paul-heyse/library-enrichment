# `datafusion_datasource::FileExtensions`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource.FileExtensions.json).

<a id="op-f98598f3aec2efb2fae85138"></a>
## FileExtensions

`type_alias` · `datafusion_datasource::FileExtensions` · datafusion-datasource 55.1.0

```rust
type FileExtensions = datafusion_common::extensions::Extensions
```

Source: `src/mod.rs:81`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

User-defined per-file extension data, keyed by concrete Rust type.

Re-exported from [`datafusion_common::extensions::Extensions`](../operations/datafusion_common.extensions.Extensions.md#op-26fd22d4c6fccf540f6e2750); the same
type backs `SessionConfig::extensions`, `ExtendedStatistics::extensions`,
and other extension fields throughout DataFusion.
