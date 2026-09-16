# `pyrefly::alt::call`

Crate `pyrefly` · 5 public items · structured records in [`model/pyrefly.alt.call.json`](../model/pyrefly.alt.call.json)

## CallStyle

`enum` · `pyrefly::alt::call::CallStyle`

```rust
enum CallStyle<'a>
```

**Variants**: `Method`, `FreeForm`

---

## CallTarget

`enum` · `pyrefly::alt::call::CallTarget`

```rust
enum CallTarget
```

**Variants**: `Callable`, `Function`, `BoundMethod`, `Class`, `TypedDict`, `FunctionOverload`, `BoundMethodOverload`, `Union`, `Any`

**Derives**: Clone, Debug

A thing that can be called (see as_call_target and call_infer).
Note that a single "call" may invoke multiple functions under the hood,
e.g., `__new__` followed by `__init__` for Class.

---

## CallTargetLookup

`enum` · `pyrefly::alt::call::CallTargetLookup`

```rust
enum CallTargetLookup
```

**Variants**: `Ok`, `Error`, `CircularCall`

**Derives**: Clone, Debug

**Methods** (1)

```rust
fn is_error(&self) -> bool
```

---

## ConstructorKind

`enum` · `pyrefly::alt::call::ConstructorKind`

```rust
enum ConstructorKind
```

**Variants**: `BareClassName`, `TypeOfClass`, `TypeOfSelf`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

---

## TargetWithTParams

`struct` · `pyrefly::alt::call::TargetWithTParams`

```rust
struct TargetWithTParams<T>
```

**Derives**: Clone, Debug

---
