# `pyrefly::report::pysa::pysa_report_capnp`

Crate `pyrefly` · 5 public items · structured records in [`model/pyrefly.report.pysa.pysa_report_capnp.json`](../model/pyrefly.report.pysa.pysa_report_capnp.json)

## ImplicitReceiver

`enum` · `pyrefly::report::pysa::pysa_report_capnp::ImplicitReceiver`

```rust
enum ImplicitReceiver
```

**Variants**: `TrueWithClassReceiver`, `TrueWithObjectReceiver`, `False`

**Implements**: `capnp::introspect::Introspect`, `capnp::traits::HasTypeId`, `core::convert::TryFrom`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

**via `capnp::introspect::Introspect`**

```rust
fn introspect() -> ::capnp::introspect::Type
```

**via `core::convert::TryFrom`**

```rust
fn try_from(value: u16) -> ::core::result::Result<Self, <ImplicitReceiver as ::core::convert::TryFrom>::Error>
```

---

## PysaClassFieldDeclaration

`enum` · `pyrefly::report::pysa::pysa_report_capnp::PysaClassFieldDeclaration`

```rust
enum PysaClassFieldDeclaration
```

**Variants**: `None`, `DeclaredByAnnotation`, `DeclaredWithoutAnnotation`, `AssignedInBody`, `DefinedWithoutAssign`, `DefinedInMethod`

**Implements**: `capnp::introspect::Introspect`, `capnp::traits::HasTypeId`, `core::convert::TryFrom`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

**via `capnp::introspect::Introspect`**

```rust
fn introspect() -> ::capnp::introspect::Type
```

**via `core::convert::TryFrom`**

```rust
fn try_from(value: u16) -> ::core::result::Result<Self, <PysaClassFieldDeclaration as ::core::convert::TryFrom>::Error>
```

---

## ReturnShimArgumentMapping

`enum` · `pyrefly::report::pysa::pysa_report_capnp::ReturnShimArgumentMapping`

```rust
enum ReturnShimArgumentMapping
```

**Variants**: `ReturnExpression`, `ReturnExpressionElement`

**Implements**: `capnp::introspect::Introspect`, `capnp::traits::HasTypeId`, `core::convert::TryFrom`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

**via `capnp::introspect::Introspect`**

```rust
fn introspect() -> ::capnp::introspect::Type
```

**via `core::convert::TryFrom`**

```rust
fn try_from(value: u16) -> ::core::result::Result<Self, <ReturnShimArgumentMapping as ::core::convert::TryFrom>::Error>
```

---

## TypeModifier

`enum` · `pyrefly::report::pysa::pysa_report_capnp::TypeModifier`

```rust
enum TypeModifier
```

**Variants**: `Optional`, `Coroutine`, `Awaitable`, `TypeVariableBound`, `TypeVariableConstraint`, `Type`

**Implements**: `capnp::introspect::Introspect`, `capnp::traits::HasTypeId`, `core::convert::TryFrom`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

**via `capnp::introspect::Introspect`**

```rust
fn introspect() -> ::capnp::introspect::Type
```

**via `core::convert::TryFrom`**

```rust
fn try_from(value: u16) -> ::core::result::Result<Self, <TypeModifier as ::core::convert::TryFrom>::Error>
```

---

## UnresolvedReason

`enum` · `pyrefly::report::pysa::pysa_report_capnp::UnresolvedReason`

```rust
enum UnresolvedReason
```

**Variants**: `LambdaArgument`, `UnexpectedPyreflyTarget`, `EmptyPyreflyCallTarget`, `UnknownClassField`, `ClassFieldOnlyExistInObject`, `UnsupportedFunctionTarget`, `UnexpectedDefiningClass`, `UnexpectedInitMethod`, `UnexpectedNewMethod`, `UnexpectedCalleeExpression`, `UnresolvedMagicDunderAttr`, `UnresolvedMagicDunderAttrDueToNoBase`, `UnresolvedMagicDunderAttrDueToNoAttribute`, `Mixed`

**Implements**: `capnp::introspect::Introspect`, `capnp::traits::HasTypeId`, `core::convert::TryFrom`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

**via `capnp::introspect::Introspect`**

```rust
fn introspect() -> ::capnp::introspect::Type
```

**via `core::convert::TryFrom`**

```rust
fn try_from(value: u16) -> ::core::result::Result<Self, <UnresolvedReason as ::core::convert::TryFrom>::Error>
```

---
