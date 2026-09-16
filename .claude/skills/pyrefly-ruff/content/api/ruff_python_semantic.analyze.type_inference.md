# `ruff_python_semantic::analyze::type_inference`

Crate `ruff_python_semantic` · 3 public items · structured records in [`model/ruff_python_semantic.analyze.type_inference.json`](../model/ruff_python_semantic.analyze.type_inference.json)

## NumberLike

`enum` · `ruff_python_semantic::analyze::type_inference::NumberLike`

```rust
enum NumberLike
```

**Variants**: `Integer`, `Float`, `Complex`, `Bool`

**Derives**: Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

A numeric type, or a type that can be trivially coerced to a numeric type.

---

## PythonType

`enum` · `ruff_python_semantic::analyze::type_inference::PythonType`

```rust
enum PythonType
```

**Variants**: `String`, `Bytes`, `Number`, `None`, `Ellipsis`, `Dict`, `List`, `Set`, `Tuple`, `Generator`

**Derives**: Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

An extremely simple type inference system for individual expressions.

This system can only represent and infer the types of simple data types
such as strings, integers, floats, and containers. It cannot infer the
types of variables or expressions that are not statically known from
individual AST nodes alone.

---

## ResolvedPythonType

`enum` · `ruff_python_semantic::analyze::type_inference::ResolvedPythonType`

```rust
enum ResolvedPythonType
```

**Variants**: `Atom`, `Union`, `Unknown`, `TypeError`

**Implements**: `core::convert::From`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn union(self, other: Self) -> Self
```

**via `core::convert::From`**

```rust
fn from(expr: &Expr) -> Self
```

---
