# `datafusion_expr_common::casts`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr_common.casts.json).

<a id="op-3671c93dc7d2a614ab8cede0"></a>
## casts

`module` · `datafusion_expr_common::casts` · datafusion-expr-common 55.1.0

```rust
mod casts
```

Source: `src/casts.rs:18`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Utilities for casting scalar literals to different data types

This module contains functions for casting ScalarValue literals
to different data types, originally extracted from the optimizer's
unwrap_cast module to be shared between logical and physical layers.
