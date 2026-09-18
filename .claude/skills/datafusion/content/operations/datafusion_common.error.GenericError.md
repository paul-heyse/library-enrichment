# `datafusion_common::error::GenericError`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.error.GenericError.json).

<a id="op-dbe8ae53fe9de4f4decc9e5e"></a>
## GenericError

`type_alias` · `datafusion_common::error::GenericError` · datafusion-common 55.1.0

```rust
type GenericError = Box<dyn Error + Send + Sync>
```

Source: `src/error.rs:65`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Error type for generic operations that could result in DataFusionError::External
