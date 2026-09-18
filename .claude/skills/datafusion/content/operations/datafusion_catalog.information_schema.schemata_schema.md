# `datafusion_catalog::information_schema::schemata_schema`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_catalog.information_schema.schemata_schema.json).

<a id="op-313993bf073277021adf5e64"></a>
## schemata_schema

`function` · `datafusion_catalog::information_schema::schemata_schema` · datafusion-catalog 55.1.0

```rust
fn schemata_schema() -> arrow::datatypes::SchemaRef
```

Source: `src/information_schema.rs:1023`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

The Arrow schema of [`information_schema.schemata`] rows.

Useful for downstream catalog implementations that want to declare a
`TableProvider` for `schemata` before populating any rows via
[`InformationSchemataBuilder`](../operations/datafusion_catalog.information_schema.InformationSchemataBuilder.md#op-396b7174acc6ef795c9624db).

Columns and nullability match
<https://www.postgresql.org/docs/current/infoschema-schemata.html>.

[`information_schema.schemata`]: https://www.postgresql.org/docs/current/infoschema-schemata.html
