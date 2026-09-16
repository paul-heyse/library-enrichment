# `pyrefly_python::nesting_context`

Crate `pyrefly_python` · 1 public items · structured records in [`model/pyrefly_python.nesting_context.json`](../model/pyrefly_python.nesting_context.json)

## NestingContext

`struct` · `pyrefly_python::nesting_context::NestingContext`

```rust
struct NestingContext
```

**Implements**: `dupe::Dupe`, `pyrefly_util::display::DisplayWith`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (11)

```rust
fn ancestor_function_path(&self, module: &Module) -> Option<ruff_python_ast::name::Name>
fn class(identifier: ShortIdentifier, parent: NestingContext) -> Self
fn function(identifier: ShortIdentifier, parent: NestingContext) -> Self
fn has_function_ancestor(&self) -> bool
fn identifier(&self) -> Option<&ShortIdentifier>
fn is_class(&self) -> bool
fn is_function(&self) -> bool
fn is_toplevel(&self) -> bool
fn owner_path(&self, module: &Module, name: &str) -> Option<ruff_python_ast::name::Name>
fn parent(&self) -> Option<&NestingContext>
fn toplevel() -> Self
```

**via `pyrefly_util::display::DisplayWith`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>, ctx: &Module) -> fmt::Result
```

Represents the syntactical nesting context of a given class or function.

---
