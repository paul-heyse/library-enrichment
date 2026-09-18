# `datafusion_proto::physical_plan::from_proto::parse_table_schema_from_proto`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto.physical_plan.from_proto.parse_table_schema_from_proto.json).

<a id="op-6dfd7aeb67cd4a080f788ba9"></a>
## parse_table_schema_from_proto

`function` · `datafusion_proto::physical_plan::from_proto::parse_table_schema_from_proto` · datafusion-proto 55.1.0

```rust
fn parse_table_schema_from_proto(proto: &protobuf::FileScanExecConf) -> datafusion_common::Result<datafusion_datasource::TableSchema>
```

Source: `src/physical_plan/from_proto.rs:434`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

Parses a TableSchema from protobuf, extracting the file schema and partition columns
