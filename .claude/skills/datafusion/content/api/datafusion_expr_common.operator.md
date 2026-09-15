# `datafusion_expr_common::operator`

Crate `datafusion-expr-common` · 1 public items · structured records in [`model/datafusion_expr_common.operator.json`](../model/datafusion_expr_common.operator.json)

## Operator

`enum` · `datafusion_expr_common::operator::Operator`

Also reachable as `datafusion::logical_expr::Operator`, `datafusion_expr::Operator`

```rust
enum Operator
```

**Variants**: `Eq`, `NotEq`, `Lt`, `LtEq`, `Gt`, `GtEq`, `Plus`, `Minus`, `Multiply`, `Divide`, `Modulo`, `And`, `Or`, `IsDistinctFrom`, `IsNotDistinctFrom`, `RegexMatch`, `RegexIMatch`, `RegexNotMatch`, `RegexNotIMatch`, `LikeMatch`, `ILikeMatch`, `NotLikeMatch`, `NotILikeMatch`, `BitwiseAnd`, `BitwiseOr`, `BitwiseXor`, `BitwiseShiftRight`, `BitwiseShiftLeft`, `StringConcat`, `AtArrow`, `ArrowAt`, `Arrow`, `LongArrow`, `HashArrow`, `HashLongArrow`, `AtAt`, `IntegerDivide`, `HashMinus`, `AtQuestion`, `Question`, `QuestionAnd`, `QuestionPipe`, `Colon`

**Implements**: `core::fmt::Display`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (8)

```rust
fn from_proto_name(name: &str) -> Option<Operator>
fn is_logic_operator(&self) -> bool
fn is_numerical_operators(&self) -> bool
fn negate(&self) -> Option<Operator>
fn precedence(&self) -> u8
fn returns_null_on_null(&self) -> bool
fn supports_propagation(&self) -> bool
fn swap(&self) -> Option<Operator>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Operators applied to expressions

---
