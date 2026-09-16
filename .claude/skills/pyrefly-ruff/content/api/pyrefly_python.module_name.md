# `pyrefly_python::module_name`

Crate `pyrefly_python` · 4 public items · structured records in [`model/pyrefly_python.module_name.json`](../model/pyrefly_python.module_name.json)

## ModuleNameKind

`enum` · `pyrefly_python::module_name::ModuleNameKind`

```rust
enum ModuleNameKind
```

**Variants**: `Guaranteed`, `Fallback`

**Implements**: `dupe::Dupe`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

Indicates whether a module name was found on the guaranteed search path
or via fallback heuristics.

---

## is_python_identifier

`function` · `pyrefly_python::module_name::is_python_identifier`

```rust
fn is_python_identifier(s: &str) -> bool
```

Whether `str.isidentifier()` would return true (Python 3 rules, no keyword check).

---

## ModuleName

`struct` · `pyrefly_python::module_name::ModuleName`

```rust
struct ModuleName
```

**Implements**: `core::fmt::Display`, `dupe::Dupe`, `pyrefly_types::equality::TypeEq`, `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (48)

```rust
fn abc() -> Self
fn append(self, name: &Name) -> Self
fn as_str(&self) -> &str
fn attr() -> Self
fn attrs() -> Self
fn builtins() -> Self
fn collections() -> Self
fn collections_abc() -> Self
fn components(self) -> Vec<Name>
fn dataclasses() -> Self
fn django_models() -> Self
fn django_models_enums() -> Self
fn django_models_fields() -> Self
fn django_models_fields_related() -> Self
fn django_models_fields_related_descriptors() -> Self
fn django_utils_functional() -> Self
fn enum_() -> Self
fn extra_builtins() -> Self
fn factory_base() -> Self
fn first_component(self) -> Name
fn from_name(x: &Name) -> Self
fn from_parts(parts: impl IntoIterator<Item = impl Display + AsRef<str>>) -> Self
fn from_path<'a>(path: &Path, includes: impl Iterator<Item = &'a PathBuf>, extra_extensions: &[String]) -> Option<ModuleName>
fn from_path_with_fallback<'a>(path: &Path, normal_includes: impl Iterator<Item = &'a PathBuf>, fallback_includes: impl Iterator<Item = &'a PathBuf>, extra_extensions: &[String]) -> Option<ModuleNameWithKind>
fn from_relative_path(path: &Path) -> anyhow::Result<Self>
fn from_str(x: &str) -> Self
fn from_string(x: String) -> Self
fn functools() -> Self
fn future() -> Self
fn marshmallow_schema() -> Self
fn new_maybe_relative(self, is_init: bool, dots: u32, suffix: Option<&Name>) -> Option<Self>
fn parent(&self) -> Option<Self>
fn pydantic() -> Self
fn pydantic_alias_generators() -> Self
fn pydantic_dataclasses() -> Self
fn pydantic_root_model() -> Self
fn pydantic_settings() -> Self
fn pydantic_types() -> Self
fn relative_module_name_between(from: &Path, to: &Path) -> Option<ModuleName>
fn rest_framework_fields() -> Self
fn rest_framework_serializers() -> Self
fn string_templatelib() -> Self
fn type_checker_internals() -> Self
fn types() -> Self
fn typing() -> Self
fn typing_extensions() -> Self
fn unknown() -> Self
fn warnings() -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `pyrefly_util::visit::Visit`**

```rust
fn recurse<'a>(&'a self, _: &mut dyn FnMut(&'a To))
```

**via `pyrefly_util::visit::VisitMut`**

```rust
fn recurse_mut(&mut self, _: &mut dyn FnMut(&mut To))
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error>
```

The name of a python module. Examples: `foo.bar.baz`, `.foo.bar`.

---

## ModuleNameWithKind

`struct` · `pyrefly_python::module_name::ModuleNameWithKind`

```rust
struct ModuleNameWithKind
```

**Implements**: `dupe::Dupe`

**Derives**: Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd

**Methods** (5)

```rust
fn fallback(name: ModuleName) -> Self
fn guaranteed(name: ModuleName) -> Self
fn is_fallback(&self) -> bool
fn kind(&self) -> ModuleNameKind
fn name(&self) -> ModuleName
```

A module name that tracks whether it was found on the guaranteed search path
or via fallback heuristics.

Fallback module names may not be reliable for determining project structure,
since they were inferred using heuristic search paths rather than explicit configuration.

Note: Equality, hashing, and ordering only consider the underlying ModuleName,
not the kind. This ensures that modules with the same name are treated identically
regardless of how their name was discovered.

---
