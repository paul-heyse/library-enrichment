# `pyrefly_python::qname`

Crate `pyrefly_python` · 1 public items · structured records in [`model/pyrefly_python.qname.json`](../model/pyrefly_python.qname.json)

## QName

`struct` · `pyrefly_python::qname::QName`

```rust
struct QName
```

**Implements**: `core::fmt::Display`, `pyrefly_types::equality::TypeEq`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd

**Methods** (12)

```rust
fn fmt_name(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
fn fmt_with_location(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
fn fmt_with_module(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
fn id(&self) -> &Name
fn module(&self) -> &Module
fn module_name(&self) -> ModuleName
fn module_path(&self) -> &ModulePath
fn module_qualified_name(&self) -> String
fn name_relative_to_module(&self) -> String
fn new(name: Identifier, parent: NestingContext, module: Module) -> Self
fn parent(&self) -> &NestingContext
fn range(&self) -> TextRange
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

A name, plus where it is defined.

---
