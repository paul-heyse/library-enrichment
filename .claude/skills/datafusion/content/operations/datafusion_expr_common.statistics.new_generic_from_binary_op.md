# `datafusion_expr_common::statistics::new_generic_from_binary_op`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr_common.statistics.new_generic_from_binary_op.json).

<a id="op-60f58c9a7f7e6129fb021edb"></a>
## new_generic_from_binary_op

`function` · `datafusion_expr_common::statistics::new_generic_from_binary_op` · datafusion-expr-common 55.1.0

```rust
fn new_generic_from_binary_op(op: &operator::Operator, left: &Distribution, right: &Distribution) -> datafusion_common::Result<Distribution>
```

Source: `src/statistics.rs:804`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Creates a new [`Generic`](../operations/datafusion_expr_common.statistics.Distribution.md#op-a2f187e0006172f198bef833) distribution that represents the result of the
given binary operation on two unknown quantities represented by their
[`Distribution`](../operations/datafusion_expr_common.statistics.Distribution.md#op-01128278dc758957f40620ba) objects. The function computes the mean, median and
variance if possible.
