# `datafusion_datasource_avro`

Crate `datafusion-datasource-avro` · 1 public items · structured records in [`model/datafusion_datasource_avro.json`](../model/datafusion_datasource_avro.json)

## read_avro_schema_from_reader

`function` · `datafusion_datasource_avro::read_avro_schema_from_reader`

```rust
fn read_avro_schema_from_reader<R: Read>(reader: &mut R) -> datafusion_common::Result<arrow::datatypes::Schema>
```

Read Avro schema given a reader

---
