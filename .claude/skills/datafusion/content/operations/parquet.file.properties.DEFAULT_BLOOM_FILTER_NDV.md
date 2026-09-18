# `parquet::file::properties::DEFAULT_BLOOM_FILTER_NDV`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.file.properties.DEFAULT_BLOOM_FILTER_NDV.json).

<a id="op-9c8e92e4f942656d39a72c51"></a>
## DEFAULT_BLOOM_FILTER_NDV

`constant` · `parquet::file::properties::DEFAULT_BLOOM_FILTER_NDV` · parquet 59.3.0

```rust
const DEFAULT_BLOOM_FILTER_NDV: u64 = _
```

Source: `src/file/properties.rs:64`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Default value for [`BloomFilterProperties::ndv()`](../operations/parquet.file.properties.BloomFilterProperties.md#op-39b748ff76f16fec3d6a46aa).

Note: this is only the fallback default used when constructing [`BloomFilterProperties`](../operations/parquet.file.properties.BloomFilterProperties.md#op-d8c6090fa6f2cfe781d4070c)
directly. When using [`WriterPropertiesBuilder`](../operations/parquet.file.properties.WriterPropertiesBuilder.md#op-bff56448e2488ad16504b642), columns with bloom filters enabled
but without an explicit NDV will have their NDV resolved at build time to
[`WriterProperties::max_row_group_row_count`](../operations/parquet.file.properties.WriterProperties.md#op-59c83d225f83bcf8d0c3dbd2), which may differ from this constant
if the user configured a custom row group size.
