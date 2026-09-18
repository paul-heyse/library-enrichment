# `datafusion_datasource_avro::read_avro_schema_from_reader`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource_avro.read_avro_schema_from_reader.json).

<a id="op-9cb6fbce688f06cfcf757c49"></a>
## read_avro_schema_from_reader

`function` · `datafusion_datasource_avro::read_avro_schema_from_reader` · datafusion-datasource-avro 55.1.0

```rust
fn read_avro_schema_from_reader<R: Read>(reader: &mut R) -> datafusion_common::Result<arrow::datatypes::Schema>
```

Source: `src/mod.rs:41`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-avro/55.1.0/json).

Read Avro schema given a reader
