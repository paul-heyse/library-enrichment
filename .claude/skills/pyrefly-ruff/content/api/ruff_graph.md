# `ruff_graph`

Crate `ruff_graph` · 2 public items · structured records in [`model/ruff_graph.json`](../model/ruff_graph.json)

## ImportMap

`struct` · `ruff_graph::ImportMap`

```rust
struct ImportMap
```

**Derives**: Debug, Default

**Methods** (2)

```rust
fn dependencies(imports: impl IntoIterator<Item = (SystemPathBuf, ModuleImports)>) -> Self
fn dependents(imports: impl IntoIterator<Item = (SystemPathBuf, ModuleImports)>) -> Self
```

---

## ModuleImports

`struct` · `ruff_graph::ModuleImports`

```rust
struct ModuleImports
```

**Derives**: Debug, Default

**Methods** (5)

```rust
fn detect<'db>(db: &'db ModuleDb, environment: ResolverEnvironment<'db>, source: &SourceKind, path: &SystemPath, package: Option<&SystemPath>, string_imports: StringImports, type_checking_imports: bool) -> Result<Self>
fn extend(&mut self, paths: impl IntoIterator<Item = SystemPathBuf>)
fn is_empty(&self) -> bool
fn len(&self) -> usize
fn relative_to(self, path: &SystemPath) -> Self
```

---
