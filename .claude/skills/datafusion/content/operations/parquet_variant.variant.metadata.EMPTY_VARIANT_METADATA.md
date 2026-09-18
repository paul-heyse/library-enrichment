# `parquet_variant::variant::metadata::EMPTY_VARIANT_METADATA`

Full upstream contracts; raw type trees and source locators in [structured records](parquet_variant.variant.metadata.EMPTY_VARIANT_METADATA.json).

<a id="op-83536fd6af51d6fce9d38c07"></a>
## EMPTY_VARIANT_METADATA

`constant` · `parquet_variant::variant::metadata::EMPTY_VARIANT_METADATA` · parquet-variant 59.3.0

```rust
const EMPTY_VARIANT_METADATA: VariantMetadata<'_> = _
```

Source: `src/variant/metadata.rs:170`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

The empty metadata dictionary.

```
# use parquet_variant::{EMPTY_VARIANT_METADATA, VariantMetadata, WritableMetadataBuilder};
let mut metadata_builder = WritableMetadataBuilder::default();
metadata_builder.finish();
let metadata_bytes = metadata_builder.into_inner();
let empty_metadata = VariantMetadata::try_new(&metadata_bytes).unwrap();
assert_eq!(empty_metadata, EMPTY_VARIANT_METADATA);
```
