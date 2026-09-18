# `datafusion_proto_models::from_proto`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto_models.from_proto.json).

<a id="op-8f4e5fc96bae413787554f62"></a>
## from_proto

`module` · `datafusion_proto_models::from_proto` · datafusion-proto-models 55.1.0

```rust
mod from_proto
```

Source: `src/from_proto.rs:18`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Conversions from the protobuf messages in this crate to their
`datafusion-common` counterparts.

The DataFusion side of these conversions lives *below* this crate in the
dependency graph, so it cannot host the impls itself. They live here
instead, on the local proto type — the same arrangement
`datafusion-proto-common` uses for `ScalarValue` and `Statistics`.
