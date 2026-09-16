# `ruff_python_ast::operator_precedence`

Crate `ruff_python_ast` · 1 public items · structured records in [`model/ruff_python_ast.operator_precedence.json`](../model/ruff_python_ast.operator_precedence.json)

## OperatorPrecedence

`enum` · `ruff_python_ast::operator_precedence::OperatorPrecedence`

Also reachable as `ruff_python_ast::OperatorPrecedence`

```rust
enum OperatorPrecedence
```

**Variants**: `None`, `Yield`, `Assign`, `Starred`, `Lambda`, `IfElse`, `Or`, `And`, `Not`, `ComparisonsMembershipIdentity`, `BitOr`, `BitXor`, `BitAnd`, `LeftRightShift`, `AddSub`, `MulDivRemain`, `PosNegBitNot`, `Exponent`, `Await`, `CallAttribute`, `Atomic`

**Implements**: `core::convert::From`

**Derives**: Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (2)

```rust
fn from_expr(expr: &Expr) -> Self
fn is_right_associative(self) -> bool
```

**via `core::convert::From`**

```rust
fn from(unary_op: UnaryOp) -> Self
fn from(expr_ref: ExprRef<'a>) -> Self
fn from(expr: &Expr) -> Self
fn from(operator: Operator) -> Self
fn from(operator: BoolOp) -> Self
```

Represents the precedence levels for Python expressions.
Variants at the top have lower precedence and variants at the bottom have
higher precedence.

See: <https://docs.python.org/3/reference/expressions.html#operator-precedence>

---
