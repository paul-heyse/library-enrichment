# `datafusion_expr::udwf::LimitEffect`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.udwf.LimitEffect.json).

<a id="op-267c5386a8a6f249083b10ad"></a>
## LimitEffect

`enum` · `datafusion_expr::udwf::LimitEffect` · datafusion-expr 55.1.0

```rust
enum LimitEffect
```

Source: `src/udwf.rs:454`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

the effect this function will have on the limit pushdown

<a id="op-ca680c5e5196860871138024"></a>
## Absolute

`variant` · `datafusion_expr::udwf::LimitEffect::Absolute` · datafusion-expr 55.1.0

```rust
Absolute
```

Source: `src/udwf.rs:462`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Limit needs to be at least N rows

<a id="op-b8c73d517f5c44b041f37d80"></a>
## None

`variant` · `datafusion_expr::udwf::LimitEffect::None` · datafusion-expr 55.1.0

```rust
None
```

Source: `src/udwf.rs:456`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Does not affect the limit (i.e. this is causal)

<a id="op-c47fd5e12af67226fdf3851c"></a>
## Relative

`variant` · `datafusion_expr::udwf::LimitEffect::Relative` · datafusion-expr 55.1.0

```rust
Relative
```

Source: `src/udwf.rs:460`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Grow the limit by N rows

<a id="op-2ac705aa5847ae8e41b368fd"></a>
## Unknown

`variant` · `datafusion_expr::udwf::LimitEffect::Unknown` · datafusion-expr 55.1.0

```rust
Unknown
```

Source: `src/udwf.rs:458`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Either undeclared, or dynamic (only evaluatable at run time)
