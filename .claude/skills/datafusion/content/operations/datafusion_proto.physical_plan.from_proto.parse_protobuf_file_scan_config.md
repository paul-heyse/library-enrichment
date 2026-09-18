# `datafusion_proto::physical_plan::from_proto::parse_protobuf_file_scan_config`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto.physical_plan.from_proto.parse_protobuf_file_scan_config.json).

<a id="op-ab1e65bea91362ec93e3a2c7"></a>
## parse_protobuf_file_scan_config

`function` · `datafusion_proto::physical_plan::from_proto::parse_protobuf_file_scan_config` · datafusion-proto 55.1.0

```rust
fn parse_protobuf_file_scan_config(proto: &protobuf::FileScanExecConf, ctx: &super::PhysicalPlanDecodeContext<'_>, proto_converter: &dyn PhysicalProtoConverterExtension, file_source: std::sync::Arc<dyn FileSource>) -> datafusion_common::Result<datafusion_datasource::file_scan_config::FileScanConfig>
```

Source: `src/physical_plan/from_proto.rs:440`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
