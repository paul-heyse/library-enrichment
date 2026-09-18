# `datafusion_proto_common::common`

Crate `datafusion-proto-common` · 1 public items · structured records in [`model/datafusion_proto_common.common.json`](../model/datafusion_proto_common.common.json)

## proto_error

`function` · `datafusion_proto_common::common::proto_error`

Also reachable as `datafusion_proto::protobuf::proto_error`

```rust
fn proto_error<S: Into<String>>(message: S) -> datafusion_common::DataFusionError
```

[Full member, field, variant and typed contracts](../operations/datafusion_proto_common.common.proto_error.md).


Return a `DataFusionError::Internal` with the given message

---
