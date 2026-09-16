# `ruff_linter::package`

Crate `ruff_linter` · 1 public items · structured records in [`model/ruff_linter.package.json`](../model/ruff_linter.package.json)

## PackageRoot

`enum` · `ruff_linter::package::PackageRoot`

```rust
enum PackageRoot<'a>
```

**Variants**: `Root`, `Nested`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn nested(path: &'a Path) -> Self
fn path(self) -> &'a Path
fn root(path: &'a Path) -> Self
```

The root directory of a Python package.

---
