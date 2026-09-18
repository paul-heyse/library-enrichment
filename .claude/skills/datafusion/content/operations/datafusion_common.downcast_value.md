# `datafusion_common::downcast_value`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.downcast_value.json).

<a id="op-3f2ad3c8c4f028c66d8716c5"></a>
## downcast_value

`macro` · `datafusion_common::downcast_value` · datafusion-common 55.1.0

```rust
macro_rules! downcast_value
```

Source: `src/lib.rs:133`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Downcast an Arrow Array to a concrete type, return an `DataFusionError::Internal` if the cast is
not possible. In normal usage of DataFusion the downcast should always succeed.

Example: `let array = downcast_value!(values, Int32Array)`
