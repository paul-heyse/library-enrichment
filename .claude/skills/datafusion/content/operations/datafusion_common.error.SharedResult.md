# `datafusion_common::error::SharedResult`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.error.SharedResult.json).

<a id="op-1bf4b4471b9598382eb47de8"></a>
## SharedResult

`type_alias` · `datafusion_common::error::SharedResult` · datafusion-common 55.1.0

```rust
type SharedResult<T> = result::Result<T, std::sync::Arc<DataFusionError>>
```

Source: `src/error.rs:62`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Result type for operations that could result in an [DataFusionError](../operations/datafusion_common.error.DataFusionError.md#op-d335eb18d3c98363748854bb) and needs to be shared (wrapped into `Arc`).
