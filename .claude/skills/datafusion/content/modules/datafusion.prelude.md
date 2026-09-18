# `datafusion::prelude`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion.prelude.json).

<a id="op-fc0d3e1eb9d33a0a2c4756df"></a>
## prelude

`module` · `datafusion::prelude` · datafusion 55.1.0

```rust
mod prelude
```

Source: `src/prelude.rs:18`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

DataFusion "prelude" to simplify importing common types.

Like the standard library's prelude, this module simplifies importing of
common items. Unlike the standard prelude, the contents of this module must
be imported manually:

```
use datafusion::prelude::*;
```
