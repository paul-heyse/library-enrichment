# `pyrefly::export::definitions`

Crate `pyrefly` · 7 public items · structured records in [`model/pyrefly.export.definitions.json`](../model/pyrefly.export.definitions.json)

## DefinitionStyle

`enum` · `pyrefly::export::definitions::DefinitionStyle`

```rust
enum DefinitionStyle
```

**Variants**: `MutableCapture`, `Annotated`, `Unannotated`, `ImplicitGlobal`, `ImportAs`, `ImportAsEq`, `Import`, `ImportModule`, `ImportInvalidRelative`, `Delete`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (1)

```rust
fn is_import(&self) -> bool
```

How a name is defined. If a name is defined outside of this
module, we additionally store the module we got it from.

This type is ordered - if there are multiple statements defining
the name, then the minimal style is the one we track.

---

## DunderAllEntry

`enum` · `pyrefly::export::definitions::DunderAllEntry`

```rust
enum DunderAllEntry
```

**Variants**: `Name`, `Module`, `Remove`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

---

## DunderAllKind

`enum` · `pyrefly::export::definitions::DunderAllKind`

```rust
enum DunderAllKind
```

**Variants**: `Inferred`, `Specified`, `Unresolvable`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

Whether `__all__` was explicitly defined by the user or synthesized from module definitions.

---

## MutableCaptureKind

`enum` · `pyrefly::export::definitions::MutableCaptureKind`

```rust
enum MutableCaptureKind
```

**Variants**: `Global`, `Nonlocal`

**Derives**: Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

---

## Definition

`struct` · `pyrefly::export::definitions::Definition`

```rust
struct Definition
```

**Fields**: `style`, `range`, `needs_anywhere`, `docstring_range`, `last_range`, `main_guard_only`, `has_value_definition`

**Derives**: Clone, Debug

**Methods** (1)

```rust
fn annotation(&self) -> Option<ShortIdentifier>
```

---

## Definitions

`struct` · `pyrefly::export::definitions::Definitions`

```rust
struct Definitions
```

**Fields**: `definitions`, `import_all`, `dunder_all`, `implicitly_imported_submodules`, `deprecated`, `final_names`, `special_exports`, `name_reads`

**Derives**: Clone, Debug, Default

**Methods** (5)

```rust
fn ensure_dunder_all(&mut self, style: ModuleStyle)
fn extend_dunder_all(&mut self, extra: &[Name])
fn implicit_captures(&self) -> SmallSet<Name>
fn inject_implicit_globals(&mut self)
fn new(x: &[Stmt], module_name: ModuleName, is_init: bool, sys_info: SysInfo) -> Self
```

Find the definitions available in a scope. Does not traverse inside classes/functions,
since they are separate scopes.

---

## DunderAll

`struct` · `pyrefly::export::definitions::DunderAll`

```rust
struct DunderAll
```

**Fields**: `kind`, `entries`

**Derives**: Clone, Debug, Default

The `__all__` variable contents with tracking of whether it was user-specified.

---
