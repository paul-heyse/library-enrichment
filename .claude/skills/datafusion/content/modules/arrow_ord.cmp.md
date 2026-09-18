# `arrow_ord::cmp`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_ord.cmp.json).

<a id="op-4a319ff8353ad1130d47722a"></a>
## cmp

`module` · `arrow_ord::cmp` · arrow-ord 59.3.0

```rust
mod cmp
```

Source: `src/cmp.rs:18`. [Exact documentation build](https://docs.rs/crate/arrow-ord/59.3.0/json).

Comparison kernels for `Array`s.

These kernels can leverage SIMD if available on your system.  Currently no runtime
detection is provided, you should enable the specific SIMD intrinsics using
`RUSTFLAGS="-C target-feature=+avx2"` for example.  See the documentation
[here](https://doc.rust-lang.org/stable/core/arch/) for more information.

