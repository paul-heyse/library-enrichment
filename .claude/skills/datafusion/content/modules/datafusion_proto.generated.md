# `datafusion_proto::generated`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto.generated.json).

<a id="op-0eac1292bbab1b153fb65984"></a>
## generated

`module` · `datafusion_proto::generated` · datafusion-proto 55.1.0

```rust
mod generated
```

Source: `src/lib.rs:156`. [Exact documentation build](https://docs.rs/crate/datafusion-proto/55.1.0/json).

Backwards-compatible re-export of the moved generated types.

The prost-generated structs now live in `datafusion-proto-models`;
this module preserves the legacy `datafusion_proto::generated::*` paths
for downstream callers. Prefer the [`protobuf`](../modules/datafusion_proto.protobuf.md#op-d2413ee72416d338db09392b) module (or
[`datafusion_proto_models`](../modules/datafusion_proto_models.md#op-95437f3660926976bee3ce25) directly) in new code.
