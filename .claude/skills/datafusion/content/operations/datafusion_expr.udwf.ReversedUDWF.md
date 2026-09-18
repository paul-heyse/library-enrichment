# `datafusion_expr::udwf::ReversedUDWF`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.udwf.ReversedUDWF.json).

<a id="op-0b9d31da343590b9f775ae6a"></a>
## ReversedUDWF

`enum` · `datafusion_expr::udwf::ReversedUDWF` · datafusion-expr 55.1.0

```rust
enum ReversedUDWF
```

Source: `src/udwf.rs:465`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-682b88747edceaf14272d930"></a>
## Identical

`variant` · `datafusion_expr::udwf::ReversedUDWF::Identical` · datafusion-expr 55.1.0

```rust
Identical
```

Source: `src/udwf.rs:468`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The result of evaluating the user-defined window function
remains identical when reversed.

<a id="op-15964c4e48776c5771d2dd2b"></a>
## NotSupported

`variant` · `datafusion_expr::udwf::ReversedUDWF::NotSupported` · datafusion-expr 55.1.0

```rust
NotSupported
```

Source: `src/udwf.rs:471`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

A window function which does not support evaluating the result
in reverse order.

<a id="op-2e8f1185638514149601de4c"></a>
## Reversed

`variant` · `datafusion_expr::udwf::ReversedUDWF::Reversed` · datafusion-expr 55.1.0

```rust
Reversed
```

Source: `src/udwf.rs:474`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Customize the user-defined window function for evaluating the
result in reverse order.
