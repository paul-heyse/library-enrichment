# `arrow_arith::boolean`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_arith.boolean.json).

<a id="op-c84d0611c140b4791eb749fe"></a>
## boolean

`module` · `arrow_arith::boolean` · arrow-arith 59.3.0

```rust
mod boolean
```

Source: `src/boolean.rs:18`. [Exact documentation build](https://docs.rs/crate/arrow-arith/59.3.0/json).

Defines boolean kernels on Arrow `BooleanArray`'s, e.g. `AND`, `OR` and `NOT`.

These kernels can leverage SIMD if available on your system.  Currently no runtime
detection is provided, you should enable the specific SIMD intrinsics using
`RUSTFLAGS="-C target-feature=+avx2"` for example.  See the documentation
[here](https://doc.rust-lang.org/stable/core/arch/) for more information.
