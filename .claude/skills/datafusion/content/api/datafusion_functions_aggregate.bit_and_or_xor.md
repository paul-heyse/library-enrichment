# `datafusion_functions_aggregate::bit_and_or_xor`

Crate `datafusion-functions-aggregate` · 6 public items · structured records in [`model/datafusion_functions_aggregate.bit_and_or_xor.json`](../model/datafusion_functions_aggregate.bit_and_or_xor.json)

## bit_and

`function` · `datafusion_functions_aggregate::bit_and_or_xor::bit_and`

Also reachable as `datafusion_functions_aggregate::expr_fn::bit_and`

```rust
fn bit_and(expr_x: datafusion_expr::Expr) -> datafusion_expr::Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate.bit_and_or_xor.bit_and.md).


Returns the bitwiseBitwiseOperationType::Andof a group of values

---

## bit_and_udaf

`function` · `datafusion_functions_aggregate::bit_and_or_xor::bit_and_udaf`

```rust
fn bit_and_udaf() -> std::sync::Arc<datafusion_expr::AggregateUDF>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate.bit_and_or_xor.bit_and_udaf.md).


AggregateFunction that returns a [`AggregateUDF`](datafusion_expr::AggregateUDF) for [`bit_and`]

---

## bit_or

`function` · `datafusion_functions_aggregate::bit_and_or_xor::bit_or`

Also reachable as `datafusion_functions_aggregate::expr_fn::bit_or`

```rust
fn bit_or(expr_x: datafusion_expr::Expr) -> datafusion_expr::Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate.bit_and_or_xor.bit_or.md).


Returns the bitwiseBitwiseOperationType::Orof a group of values

---

## bit_or_udaf

`function` · `datafusion_functions_aggregate::bit_and_or_xor::bit_or_udaf`

```rust
fn bit_or_udaf() -> std::sync::Arc<datafusion_expr::AggregateUDF>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate.bit_and_or_xor.bit_or_udaf.md).


AggregateFunction that returns a [`AggregateUDF`](datafusion_expr::AggregateUDF) for [`bit_or`]

---

## bit_xor

`function` · `datafusion_functions_aggregate::bit_and_or_xor::bit_xor`

Also reachable as `datafusion_functions_aggregate::expr_fn::bit_xor`

```rust
fn bit_xor(expr_x: datafusion_expr::Expr) -> datafusion_expr::Expr
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate.bit_and_or_xor.bit_xor.md).


Returns the bitwiseBitwiseOperationType::Xorof a group of values

---

## bit_xor_udaf

`function` · `datafusion_functions_aggregate::bit_and_or_xor::bit_xor_udaf`

```rust
fn bit_xor_udaf() -> std::sync::Arc<datafusion_expr::AggregateUDF>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions_aggregate.bit_and_or_xor.bit_xor_udaf.md).


AggregateFunction that returns a [`AggregateUDF`](datafusion_expr::AggregateUDF) for [`bit_xor`]

---
