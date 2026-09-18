# `datafusion_proto_common::from_proto::parse_proto_fields_to_fields`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto_common.from_proto.parse_proto_fields_to_fields.json).

<a id="op-7ae11261868678eb8a0ad15a"></a>
## parse_proto_fields_to_fields

`function` · `datafusion_proto_common::from_proto::parse_proto_fields_to_fields` · datafusion-proto-common 55.1.0

```rust
fn parse_proto_fields_to_fields<'a, I>(fields: I) -> Result<Vec<arrow::datatypes::Field>, Error> where I: IntoIterator<Item = &'a protobuf::Field>
```

Source: `src/from_proto/mod.rs:1263`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

Converts a vector of `protobuf::Field`s to `Arc<arrow::Field>`s.
