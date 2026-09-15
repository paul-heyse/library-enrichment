# `sqlparser::ast::operator`

Crate `sqlparser` · 2 public items · structured records in [`model/sqlparser.ast.operator.json`](../model/sqlparser.ast.operator.json)

## BinaryOperator

`enum` · `sqlparser::ast::operator::BinaryOperator`

Also reachable as `sqlparser::ast::BinaryOperator`

```rust
enum BinaryOperator
```

**Variants**: `Plus`, `Minus`, `Multiply`, `Divide`, `Modulo`, `StringConcat`, `Gt`, `Lt`, `GtEq`, `LtEq`, `Spaceship`, `Eq`, `NotEq`, `And`, `Or`, `Xor`, `BitwiseOr`, `BitwiseAnd`, `BitwiseXor`, `DuckIntegerDivide`, `MyIntegerDivide`, `Match`, `Regexp`, `Custom`, `PGBitwiseXor`, `PGBitwiseShiftLeft`, `PGBitwiseShiftRight`, `PGExp`, `PGOverlap`, `PGRegexMatch`, `PGRegexIMatch`, `PGRegexNotMatch`, `PGRegexNotIMatch`, `PGLikeMatch`, `PGILikeMatch`, `PGNotLikeMatch`, `PGNotILikeMatch`, `PGStartsWith`, `Arrow`, `LongArrow`, `HashArrow`, `HashLongArrow`, `AtAt`, `AtArrow`, `ArrowAt`, `HashMinus`, `AtQuestion`, `Question`, `QuestionAnd`, `QuestionPipe`, `PGCustomBinaryOperator`, `Overlaps`, `DoubleHash`, `LtDashGt`, `AndLt`, `AndGt`, `LtLtPipe`, `PipeGtGt`, `AndLtPipe`, `PipeAndGt`, `LtCaret`, `GtCaret`, `QuestionHash`, `QuestionDash`, `QuestionDashPipe`, `QuestionDoublePipe`, `At`, `TildeEq`, `Assignment`

**Implements**: `core::fmt::Display`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`, `sqlparser::ast::visitor::Visit`, `sqlparser::ast::visitor::VisitMut`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

**via `sqlparser::ast::visitor::Visit`**

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

**via `sqlparser::ast::visitor::VisitMut`**

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Binary operators

---

## UnaryOperator

`enum` · `sqlparser::ast::operator::UnaryOperator`

Also reachable as `sqlparser::ast::UnaryOperator`

```rust
enum UnaryOperator
```

**Variants**: `AtDashAt`, `BangNot`, `BitwiseNot`, `DoubleAt`, `Hash`, `Plus`, `Minus`, `Not`, `PGAbs`, `PGCubeRoot`, `PGPostfixFactorial`, `PGPrefixFactorial`, `PGSquareRoot`, `QuestionDash`, `QuestionPipe`

**Implements**: `core::fmt::Display`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`, `sqlparser::ast::visitor::Visit`, `sqlparser::ast::visitor::VisitMut`

**Derives**: Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

**via `sqlparser::ast::visitor::Visit`**

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

**via `sqlparser::ast::visitor::VisitMut`**

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Unary operators

---
