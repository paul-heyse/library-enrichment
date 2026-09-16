# `pyrefly_types::read_only`

Crate `pyrefly_types` · 2 public items · structured records in [`model/pyrefly_types.read_only.json`](../model/pyrefly_types.read_only.json)

## IsFinalVariableInitialized

`enum` · `pyrefly_types::read_only::IsFinalVariableInitialized`

```rust
enum IsFinalVariableInitialized
```

**Variants**: `Yes`, `No`

**Implements**: `pyrefly_types::equality::TypeEq`, `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `pyrefly_types::equality::TypeEq`**

```rust
fn type_eq(&self, other: &Self, ctx: &mut equality::TypeEqCtx) -> bool
```

**via `pyrefly_util::visit::Visit`**

```rust
fn recurse<'a>(&'a self, f: &mut dyn FnMut(&'a To))
```

**via `pyrefly_util::visit::VisitMut`**

```rust
fn recurse_mut(&mut self, f: &mut dyn FnMut(&mut To))
```

Indicates whether a `Final` variable was initialized at the point of declaration

---

## ReadOnlyReason

`enum` · `pyrefly_types::read_only::ReadOnlyReason`

```rust
enum ReadOnlyReason
```

**Variants**: `Final`, `ReadOnlyQualifier`, `FrozenDataclass`, `AttrsFrozen`, `NamedTuple`, `ClassVar`, `ClassObjectInitializedOnBody`, `Super`, `PydanticFrozen`, `PydanticFrozenField`, `EnumMemberValue`, `Getattr`

**Implements**: `pyrefly_types::equality::TypeEq`, `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (1)

```rust
fn error_message(&self) -> String
```

**via `pyrefly_types::equality::TypeEq`**

```rust
fn type_eq(&self, other: &Self, ctx: &mut equality::TypeEqCtx) -> bool
```

**via `pyrefly_util::visit::Visit`**

```rust
fn recurse<'a>(&'a self, f: &mut dyn FnMut(&'a To))
```

**via `pyrefly_util::visit::VisitMut`**

```rust
fn recurse_mut(&mut self, f: &mut dyn FnMut(&mut To))
```

Represents the specific reason why a field is read-only, to provide better error messages

---
