# `datafusion_physical_expr_common::physical_expr::proto_encode`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr_common.physical_expr.proto_encode.json).

<a id="op-6ede9797bc17981c9be277ae"></a>
## proto_encode

`module` · `datafusion_physical_expr_common::physical_expr::proto_encode` · datafusion-physical-expr-common 55.1.0

```rust
mod proto_encode
```

Source: `src/physical_expr.rs:527`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Encode-side context for [`PhysicalExpr::try_to_proto`](../operations/datafusion_physical_expr_common.physical_expr.PhysicalExpr.md#op-3afb516a11e28f2442ad2f38).

Expression authors only ever see [`proto_encode::PhysicalExprEncodeCtx`](../operations/datafusion_physical_expr_common.physical_expr.proto_encode.PhysicalExprEncodeCtx.md#op-a1072ae5c0335855f9ea80bc):
a concrete struct with stable methods. Internally it dispatches to a
[`proto_encode::PhysicalExprEncode`](../operations/datafusion_physical_expr_common.physical_expr.proto_encode.PhysicalExprEncode.md#op-7551ad99ab64fae576a7ac31) implementor that lives in
`datafusion-proto`, which is what lets `physical-expr-common` stay free
of `datafusion-proto` as a dep.

More specialized helpers (e.g. encoding UDFs/UDAFs/UDWFs through the
extension codec) can be added to the context as expressions migrate;
today they're not required because the encoder forwards to the existing
codec via the proto converter.
