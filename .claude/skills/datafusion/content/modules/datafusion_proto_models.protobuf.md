# `datafusion_proto_models::protobuf`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto_models.protobuf.json).

<a id="op-6aa92fae71de00fed9e1f341"></a>
## protobuf

`module` · `datafusion_proto_models::protobuf` · datafusion-proto-models 55.1.0

```rust
mod protobuf
```

Source: `src/lib.rs:53`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

All DataFusion protobuf model types.

Includes both the types declared in `datafusion.proto` and the
`datafusion_proto_common` types it imports, in a single flat namespace
so consumers can `use datafusion_proto_models::protobuf::*;`.
