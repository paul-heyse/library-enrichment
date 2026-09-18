# `parquet::file::properties::ReaderProperties`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.file.properties.ReaderProperties.json).

<a id="op-10bf0c6bc812b59aa077ae83"></a>
## ReaderProperties

`struct` · `parquet::file::properties::ReaderProperties` · parquet 59.3.0

```rust
struct ReaderProperties
```

Source: `src/file/properties.rs:1790`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Configuration settings for reading parquet files.

All properties are immutable and `Send` + `Sync`.
Use [`ReaderPropertiesBuilder`](../operations/parquet.file.properties.ReaderPropertiesBuilder.md#op-cc208ee9b7fb0ebb0381c87f) to assemble these properties.

# Example

```rust
use parquet::file::properties::ReaderProperties;

// Create properties with default configuration.
let props = ReaderProperties::builder().build();

// Use properties builder to set certain options and assemble the configuration.
let props = ReaderProperties::builder()
    .set_backward_compatible_lz4(false)
    .build();
```

<a id="op-36af0313f1eec2b2fae811d9"></a>
## builder

`function` · `parquet::file::properties::ReaderProperties::builder` · parquet 59.3.0

```rust
fn builder() -> ReaderPropertiesBuilder
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::file::properties::ReaderProperties", "path": "ReaderProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1796, 1], "end": [1816, 2], "filename": "src/file/properties.rs"}, "trait": null, "trait_path": null}`

Source: `src/file/properties.rs:1798`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns builder for reader properties with default values.
