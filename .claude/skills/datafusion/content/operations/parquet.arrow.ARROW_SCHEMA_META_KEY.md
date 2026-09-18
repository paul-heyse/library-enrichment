# `parquet::arrow::ARROW_SCHEMA_META_KEY`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.arrow.ARROW_SCHEMA_META_KEY.json).

<a id="op-40f8e942a81681d7b74aaa33"></a>
## ARROW_SCHEMA_META_KEY

`constant` · `parquet::arrow::ARROW_SCHEMA_META_KEY` · parquet 59.3.0

```rust
const ARROW_SCHEMA_META_KEY: &str = "ARROW:schema"
```

Source: `src/arrow/mod.rs:220`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Schema metadata key used to store serialized Arrow schema

The Arrow schema is encoded using the Arrow IPC format, and then base64
encoded. This is the same format used by arrow-cpp systems, such as pyarrow.
