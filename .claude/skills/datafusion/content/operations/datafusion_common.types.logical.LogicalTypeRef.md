# `datafusion_common::types::logical::LogicalTypeRef`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.types.logical.LogicalTypeRef.json).

<a id="op-809f92ae09b91bedd0070ff9"></a>
## LogicalTypeRef

`type_alias` · `datafusion_common::types::logical::LogicalTypeRef` · datafusion-common 55.1.0

```rust
type LogicalTypeRef = std::sync::Arc<dyn LogicalType>
```

Source: `src/types/logical.rs:46`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

A reference counted [`LogicalType`](../operations/datafusion_common.types.logical.LogicalType.md#op-a70362eb6689066b420dd5aa).
