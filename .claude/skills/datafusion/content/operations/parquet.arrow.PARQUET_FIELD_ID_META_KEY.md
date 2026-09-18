# `parquet::arrow::PARQUET_FIELD_ID_META_KEY`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.arrow.PARQUET_FIELD_ID_META_KEY.json).

<a id="op-749ca2a97ded741673b72e91"></a>
## PARQUET_FIELD_ID_META_KEY

`constant` · `parquet::arrow::PARQUET_FIELD_ID_META_KEY` · parquet 59.3.0

```rust
const PARQUET_FIELD_ID_META_KEY: &str = "PARQUET:field_id"
```

Source: `src/arrow/mod.rs:227`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

The value of this metadata key, if present on [`Field::metadata`], will be used
to populate [`BasicTypeInfo::id`]

[`Field::metadata`]: arrow_schema::Field::metadata
[`BasicTypeInfo::id`]: crate::schema::types::BasicTypeInfo::id

Unresolved upstream links (retained, not inferred): `arrow_schema::Field::metadata`.
