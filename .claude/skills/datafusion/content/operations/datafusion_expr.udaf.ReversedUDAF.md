# `datafusion_expr::udaf::ReversedUDAF`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.udaf.ReversedUDAF.json).

<a id="op-2b0e7bd31435f6d14b27908f"></a>
## ReversedUDAF

`enum` · `datafusion_expr::udaf::ReversedUDAF` · datafusion-expr 55.1.0

```rust
enum ReversedUDAF
```

Source: `src/udaf.rs:1215`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9691b18e6bdfd81a8b803d3d"></a>
## Identical

`variant` · `datafusion_expr::udaf::ReversedUDAF::Identical` · datafusion-expr 55.1.0

```rust
Identical
```

Source: `src/udaf.rs:1217`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The expression is the same as the original expression, like SUM, COUNT

<a id="op-653cbeaa070b25d046b123bc"></a>
## NotSupported

`variant` · `datafusion_expr::udaf::ReversedUDAF::NotSupported` · datafusion-expr 55.1.0

```rust
NotSupported
```

Source: `src/udaf.rs:1219`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The expression does not support reverse calculation

<a id="op-83cabc5630bc055d4bf76a4d"></a>
## Reversed

`variant` · `datafusion_expr::udaf::ReversedUDAF::Reversed` · datafusion-expr 55.1.0

```rust
Reversed
```

Source: `src/udaf.rs:1221`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The expression is different from the original expression
