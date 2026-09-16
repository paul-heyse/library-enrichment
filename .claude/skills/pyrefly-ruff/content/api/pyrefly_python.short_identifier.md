# `pyrefly_python::short_identifier`

Crate `pyrefly_python` · 1 public items · structured records in [`model/pyrefly_python.short_identifier.json`](../model/pyrefly_python.short_identifier.json)

## ShortIdentifier

`struct` · `pyrefly_python::short_identifier::ShortIdentifier`

```rust
struct ShortIdentifier
```

**Implements**: `pyrefly_types::equality::TypeEq`, `pyrefly_util::display::DisplayWith`, `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (3)

```rust
fn expr_name(x: &ExprName) -> Self
fn from_text_range(range: TextRange) -> Self
fn new(name: &Identifier) -> Self
```

**via `pyrefly_util::display::DisplayWith`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>, ctx: &Module) -> fmt::Result
```

**via `pyrefly_util::visit::Visit`**

```rust
fn recurse<'a>(&'a self, _: &mut dyn FnMut(&'a To))
```

**via `pyrefly_util::visit::VisitMut`**

```rust
fn recurse_mut(&mut self, _: &mut dyn FnMut(&mut To))
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> TextRange
```

An identifier, where we can drop the `Name` part because it came from a `Module`.

---
