# `pyrefly_python::module_path`

Crate `pyrefly_python` · 3 public items · structured records in [`model/pyrefly_python.module_path.json`](../model/pyrefly_python.module_path.json)

## ModulePathDetails

`enum` · `pyrefly_python::module_path::ModulePathDetails`

```rust
enum ModulePathDetails
```

**Variants**: `FileSystem`, `Namespace`, `Memory`, `BundledTypeshed`, `BundledTypeshedThirdParty`, `BundledThirdParty`

**Implements**: `dupe::Dupe`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

---

## ModuleStyle

`enum` · `pyrefly_python::module_path::ModuleStyle`

```rust
enum ModuleStyle
```

**Variants**: `Executable`, `Interface`

**Implements**: `dupe::Dupe`, `pyrefly_types::equality::TypeEq`, `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (1)

```rust
fn of_path(path: &Path) -> Self
```

**via `pyrefly_util::visit::Visit`**

```rust
fn recurse<'a>(&'a self, _: &mut dyn FnMut(&'a To))
```

**via `pyrefly_util::visit::VisitMut`**

```rust
fn recurse_mut(&mut self, _: &mut dyn FnMut(&mut To))
```

---

## ModulePath

`struct` · `pyrefly_python::module_path::ModulePath`

```rust
struct ModulePath
```

**Implements**: `core::fmt::Display`, `dupe::Dupe`, `pyrefly_types::equality::TypeEq`, `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (17)

```rust
fn as_path(&self) -> &Path
fn bundled_third_party(relative_path: PathBuf) -> Self
fn bundled_typeshed(relative_path: PathBuf) -> Self
fn bundled_typeshed_third_party(relative_path: PathBuf) -> Self
fn details(&self) -> &ModulePathDetails
fn filesystem(path: PathBuf) -> Self
fn is_bundled(&self) -> bool
fn is_first_party_for_indexing(&self) -> bool
fn is_init(&self) -> bool
fn is_interface(&self) -> bool
fn is_memory(&self) -> bool
fn memory(path: PathBuf) -> Self
fn module_path_buf(&self) -> InternedPath
fn namespace(path: PathBuf) -> Self
fn root_of(&self, name: ModuleName) -> Option<PathBuf>
fn style(&self) -> ModuleStyle
fn to_key_eq(&self) -> ModulePath
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

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error>
```

Store information about where a module is sourced from.

---
