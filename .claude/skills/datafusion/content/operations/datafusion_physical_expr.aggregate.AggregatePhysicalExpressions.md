# `datafusion_physical_expr::aggregate::AggregatePhysicalExpressions`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.aggregate.AggregatePhysicalExpressions.json).

<a id="op-96122e9615cdf99bdcbd5319"></a>
## AggregatePhysicalExpressions

`struct` · `datafusion_physical_expr::aggregate::AggregatePhysicalExpressions` · datafusion-physical-expr 55.1.0

```rust
struct AggregatePhysicalExpressions
```

Source: `src/aggregate.rs:1098`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Stores the physical expressions used inside the `AggregateExpr`.

<a id="op-34a69cf7244d8a44b67edc51"></a>
## args

`struct_field` · `datafusion_physical_expr::aggregate::AggregatePhysicalExpressions::args` · datafusion-physical-expr 55.1.0

```rust
args: Vec<std::sync::Arc<dyn PhysicalExpr>>
```

Source: `src/aggregate.rs:1100`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Aggregate function arguments

<a id="op-c17029671415b271da3b8aa8"></a>
## order_by_exprs

`struct_field` · `datafusion_physical_expr::aggregate::AggregatePhysicalExpressions::order_by_exprs` · datafusion-physical-expr 55.1.0

```rust
order_by_exprs: Vec<std::sync::Arc<dyn PhysicalExpr>>
```

Source: `src/aggregate.rs:1102`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Order by expressions
