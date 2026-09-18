# `datafusion_common::cast`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.cast.json).

<a id="op-827852c41ed3c33599bdd20d"></a>
## cast

`module` · `datafusion_common::cast` · datafusion-common 55.1.0

```rust
mod cast
```

Source: `src/cast.rs:18`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

This module provides DataFusion specific casting functions
that provide error handling. They are intended to "never fail"
but provide an error message rather than a panic, as the corresponding
kernels in arrow-rs such as `as_boolean_array` do.
