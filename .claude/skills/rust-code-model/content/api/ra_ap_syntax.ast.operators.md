# `ra_ap_syntax::ast::operators`

Crate `ra_ap_syntax` · 7 public items · structured records in [`model/ra_ap_syntax.ast.operators.json`](../model/ra_ap_syntax.ast.operators.json)

## ArithOp

`enum` · `ra_ap_syntax::ast::operators::ArithOp`

Also reachable as `ra_ap_syntax::ast::ArithOp`

```rust
enum ArithOp
```

**Variants**: `Add`, `Mul`, `Sub`, `Div`, `Rem`, `Shl`, `Shr`, `BitXor`, `BitOr`, `BitAnd`

**Implements**: `core::fmt::Display`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

---

## BinaryOp

`enum` · `ra_ap_syntax::ast::operators::BinaryOp`

Also reachable as `ra_ap_syntax::ast::BinaryOp`

```rust
enum BinaryOp
```

**Variants**: `LogicOp`, `ArithOp`, `CmpOp`, `Assignment`

**Implements**: `core::fmt::Display`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

---

## CmpOp

`enum` · `ra_ap_syntax::ast::operators::CmpOp`

Also reachable as `ra_ap_syntax::ast::CmpOp`

```rust
enum CmpOp
```

**Variants**: `Eq`, `Ord`

**Implements**: `core::fmt::Display`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

---

## LogicOp

`enum` · `ra_ap_syntax::ast::operators::LogicOp`

Also reachable as `ra_ap_syntax::ast::LogicOp`

```rust
enum LogicOp
```

**Variants**: `And`, `Or`

**Implements**: `core::fmt::Display`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

---

## Ordering

`enum` · `ra_ap_syntax::ast::operators::Ordering`

Also reachable as `ra_ap_syntax::ast::Ordering`

```rust
enum Ordering
```

**Variants**: `Less`, `Greater`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

---

## RangeOp

`enum` · `ra_ap_syntax::ast::operators::RangeOp`

Also reachable as `ra_ap_syntax::ast::RangeOp`

```rust
enum RangeOp
```

**Variants**: `Exclusive`, `Inclusive`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

---

## UnaryOp

`enum` · `ra_ap_syntax::ast::operators::UnaryOp`

Also reachable as `ra_ap_syntax::ast::UnaryOp`

```rust
enum UnaryOp
```

**Variants**: `Deref`, `Not`, `Neg`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

---
