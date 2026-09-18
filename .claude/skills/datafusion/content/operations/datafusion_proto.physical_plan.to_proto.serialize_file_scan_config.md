# `datafusion_proto::physical_plan::to_proto::serialize_file_scan_config`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto.physical_plan.to_proto.serialize_file_scan_config.json).

<a id="op-e440e8b8631d84fa98e0ae0d"></a>
## serialize_file_scan_config

`function` · `datafusion_proto::physical_plan::to_proto::serialize_file_scan_config` · datafusion-proto 55.1.0

```rust
fn serialize_file_scan_config(conf: &datafusion_datasource::file_scan_config::FileScanConfig, codec: &dyn PhysicalExtensionCodec, proto_converter: &dyn PhysicalProtoConverterExtension) -> datafusion_common::Result<protobuf::FileScanExecConf>
```

Source: `src/physical_plan/to_proto.rs:360`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
