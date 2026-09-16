# `pyrefly::alt::attr`

Crate `pyrefly` · 5 public items · structured records in [`model/pyrefly.alt.attr.json`](../model/pyrefly.alt.attr.json)

## AttrDefinition

`enum` · `pyrefly::alt::attr::AttrDefinition`

```rust
enum AttrDefinition
```

**Variants**: `FullyResolved`, `PartiallyResolvedImportedModuleAttribute`, `Submodule`

**Derives**: Clone, Debug

---

## AttrSubsetError

`enum` · `pyrefly::alt::attr::AttrSubsetError`

```rust
enum AttrSubsetError
```

**Variants**: `NoAccess`, `Property`, `ReadOnly`, `Descriptor`, `Getattr`, `ModuleFallback`, `ClassVarMismatch`, `Covariant`, `Invariant`, `Contravariant`

**Derives**: Clone, Debug

**Methods** (1)

```rust
fn to_error_msg(&self, child_class: &Name, parent_class: &Name, attr_name: &Name) -> String
```

---

## ClassBase

`enum` · `pyrefly::alt::attr::ClassBase`

```rust
enum ClassBase
```

**Variants**: `ClassDef`, `ClassType`, `Quantified`, `SelfType`, `Protocol`

**Derives**: Clone, Debug

**Methods** (5)

```rust
fn class_object(&self) -> &Class
fn class_type(&self) -> &ClassType
fn targs(&self) -> Option<&TArgs>
fn to_self_type(self, heap: &TypeHeap) -> Type
fn to_type(self, heap: &TypeHeap) -> Type
```

A normalized type for attribute lookup which has "class-like" lookup behavior. For example,
when we look up an instance method from a class base, we get the unbound function type.

---

## NoAccessReason

`enum` · `pyrefly::alt::attr::NoAccessReason`

```rust
enum NoAccessReason
```

**Variants**: `ClassUseOfInstanceAttribute`, `ClassAttributeIsGeneric`, `SettingReadOnlyProperty`, `SettingReadOnlyDescriptor`, `SuperMethodNeedsImplementation`, `ProxyMethodClassAccess`, `ProxyMethodTargetInvalid`

**Derives**: Clone, Debug

**Methods** (1)

```rust
fn to_error_msg(&self, attr_name: &Name) -> String
```

---

## AttrInfo

`struct` · `pyrefly::alt::attr::AttrInfo`

```rust
struct AttrInfo
```

**Fields**: `name`, `ty`, `is_deprecated`, `definition`, `is_reexport`

**Derives**: Debug

---
