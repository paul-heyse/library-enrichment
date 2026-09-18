# `buoyant_kernel::expressions::lit`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.expressions.lit.json).

<a id="op-0d3f5d153aed18a81e0320b4"></a>
## lit

`function` · `buoyant_kernel::expressions::lit` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn lit(value: impl Into<Scalar>) -> Expression
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L47).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:47`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Build an [`Expression::Literal`](../operations/buoyant_kernel.expressions.Expression.md#op-27c1daaf577668ddbb85bbe0) from anything that converts into a [`Scalar`](../operations/buoyant_kernel.expressions.scalars.Scalar.md#op-8dd45baeed3da91aa357441a).

Concise alternative to [`Expression::literal`](../operations/buoyant_kernel.expressions.Expression.md#op-3f92c8cb2152d1236538d451) for plan builders. Accepts the same value
types [`Scalar`](../operations/buoyant_kernel.expressions.scalars.Scalar.md#op-8dd45baeed3da91aa357441a) does (`i32`, `i64`, `&str`, `bool`, ...).

```
# use buoyant_kernel as delta_kernel;
use delta_kernel::expressions::lit;
let _zero = lit(0i64);
```
