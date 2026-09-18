# `datafusion_common::rounding`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.rounding.json).

<a id="op-4fee35be9b56f17c1e94342e"></a>
## rounding

`module` · `datafusion_common::rounding` · datafusion-common 55.1.0

```rust
mod rounding
```

Source: `src/rounding.rs:18`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Floating point rounding mode utility library
TODO: Remove this custom implementation and the "libc" dependency when
      floating-point rounding mode manipulation functions become available
      in Rust.
