# `datafusion_proto_models`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto_models.json).

<a id="op-95437f3660926976bee3ce25"></a>
## datafusion_proto_models

`module` · `datafusion_proto_models` · datafusion-proto-models 55.1.0

```rust
mod datafusion_proto_models
```

Source: `src/lib.rs:18`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

`prost`-generated DataFusion protobuf model types.

This crate contains the generated structs for DataFusion's logical and
physical plan protobuf schemas (see `proto/datafusion.proto`), plus the
[`From`] / [`TryFrom`] conversions between them and the `datafusion-common`
types they mirror. Those conversions live here because their DataFusion side
sits *below* this crate in the dependency graph and so cannot host the impls
itself — see [`from_proto`](../modules/datafusion_proto_models.from_proto.md#op-8f4e5fc96bae413787554f62) and [`to_proto`](../modules/datafusion_proto_models.to_proto.md#op-bb5ed5cf119a297f0a1e22ef). It is the schema source of
truth for [`datafusion-proto`].

Most users should depend on [`datafusion-proto`] instead, which re-exports
these types under [`datafusion_proto::protobuf`].

[`datafusion-proto`]: https://crates.io/crates/datafusion-proto
[`datafusion-proto-common`]: https://crates.io/crates/datafusion-proto-common
[`datafusion_proto::protobuf`]: https://docs.rs/datafusion-proto/latest/datafusion_proto/protobuf/index.html

Unresolved upstream links (retained, not inferred): ``From``, ``TryFrom``.
