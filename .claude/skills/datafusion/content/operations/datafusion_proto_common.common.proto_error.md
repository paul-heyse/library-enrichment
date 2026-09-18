# `datafusion_proto_common::common::proto_error`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto_common.common.proto_error.json).

<a id="op-a946f801e8367b5b38983382"></a>
## proto_error

`function` · `datafusion_proto_common::common::proto_error` · datafusion-proto-common 55.1.0

```rust
fn proto_error<S: Into<String>>(message: S) -> datafusion_common::DataFusionError
```

Source: `src/common.rs:21`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-common/55.1.0/json).

Return a `DataFusionError::Internal` with the given message
