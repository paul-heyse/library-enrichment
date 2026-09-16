# `pyrefly_types::module`

Crate `pyrefly_types` · 1 public items · structured records in [`model/pyrefly_types.module.json`](../model/pyrefly_types.module.json)

## ModuleType

`struct` · `pyrefly_types::module::ModuleType`

```rust
struct ModuleType
```

**Implements**: `core::fmt::Display`, `pyrefly_types::equality::TypeEq`, `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (8)

```rust
fn add_module(&self, m: ModuleName) -> Self
fn is_submodules_imported_directly(&self) -> bool
fn merge(&mut self, m: &ModuleType)
fn new(name: Name, modules: OrderedSet<ModuleName>) -> ModuleType
fn new_as(name: ModuleName) -> ModuleType
fn parts(&self) -> &[Name]
fn push_part(&self, component: Name) -> Self
fn to_type(self, heap: &TypeHeap) -> Type
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `pyrefly_types::equality::TypeEq`**

```rust
fn type_eq(&self, other: &Self, ctx: &mut equality::TypeEqCtx) -> bool
```

**via `pyrefly_util::visit::Visit`**

```rust
fn recurse<'a>(&'a self, _: &mut dyn FnMut(&'a Type))
```

**via `pyrefly_util::visit::VisitMut`**

```rust
fn recurse_mut(&mut self, _: &mut dyn FnMut(&mut Type))
```

In Python if you do `import foo.bar` and `import foo.baz` then what you are really
doing is importing a single symbol `foo` that contains the two modules accessible from it.

To represent that, we have a set of modules and a `path` of how far down we are.
Any module that does not start with a prefix of the `path` is no longer accessible,
but we keep them around (under an `Arc`) since it's more efficient not to recreate
the `SmallMap` on each access.

---
