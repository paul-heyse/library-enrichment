# `parquet_variant::variant::metadata::EMPTY_VARIANT_METADATA_BYTES`

Full upstream contracts; raw type trees and source locators in [structured records](parquet_variant.variant.metadata.EMPTY_VARIANT_METADATA_BYTES.json).

<a id="op-00d796e16461ea32776ea39c"></a>
## EMPTY_VARIANT_METADATA_BYTES

`constant` · `parquet_variant::variant::metadata::EMPTY_VARIANT_METADATA_BYTES` · parquet-variant 59.3.0

```rust
const EMPTY_VARIANT_METADATA_BYTES: &[u8] = _
```

Source: `src/variant/metadata.rs:158`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

The canonical byte slice corresponding to an empty metadata dictionary.

```
# use parquet_variant::{EMPTY_VARIANT_METADATA_BYTES, VariantMetadata, WritableMetadataBuilder};
let mut metadata_builder = WritableMetadataBuilder::default();
metadata_builder.finish();
let metadata_bytes = metadata_builder.into_inner();
assert_eq!(&metadata_bytes, EMPTY_VARIANT_METADATA_BYTES);
```
