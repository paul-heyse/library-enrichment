# `datafusion_common::utils::transpose`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.utils.transpose.json).

<a id="op-2194ea71a7d3a16cc7345b8c"></a>
## transpose

`function` · `datafusion_common::utils::transpose` · datafusion-common 55.1.0

```rust
fn transpose<T>(original: Vec<Vec<T>>) -> Vec<Vec<T>>
```

Source: `src/utils/mod.rs:1067`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Transposes the given vector of vectors.
