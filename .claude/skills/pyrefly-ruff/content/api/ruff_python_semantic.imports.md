# `ruff_python_semantic::imports`

Crate `ruff_python_semantic` · 6 public items · structured records in [`model/ruff_python_semantic.imports.json`](../model/ruff_python_semantic.imports.json)

## NameImport

`enum` · `ruff_python_semantic::imports::NameImport`

Also reachable as `ruff_python_semantic::NameImport`

```rust
enum NameImport
```

**Variants**: `Import`, `ImportFrom`

**Implements**: `core::fmt::Display`, `ruff_cache::cache_key::CacheKey`, `ruff_python_semantic::imports::FutureImport`

**Derives**: Clone, Debug, Eq, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (3)

```rust
fn bound_name(&self) -> &str
fn matches(&self, name: &str, binding: &AnyImport<'_, '_>) -> bool
fn qualified_name(&self) -> QualifiedName<'_>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ruff_cache::cache_key::CacheKey`**

```rust
fn cache_key(&self, key: &mut ruff_cache::CacheKeyHasher)
```

**via `ruff_python_semantic::imports::FutureImport`**

```rust
fn is_future_import(&self) -> bool
```

A representation of an individual name imported via any import statement.

---

## Alias

`struct` · `ruff_python_semantic::imports::Alias`

Also reachable as `ruff_python_semantic::Alias`

```rust
struct Alias
```

**Fields**: `name`, `as_name`

**Implements**: `ruff_cache::cache_key::CacheKey`

**Derives**: Clone, Debug, Eq, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `ruff_cache::cache_key::CacheKey`**

```rust
fn cache_key(&self, key: &mut ruff_cache::CacheKeyHasher)
```

---

## MemberNameImport

`struct` · `ruff_python_semantic::imports::MemberNameImport`

Also reachable as `ruff_python_semantic::MemberNameImport`

```rust
struct MemberNameImport
```

**Fields**: `module`, `name`, `level`

**Implements**: `core::fmt::Display`, `ruff_cache::cache_key::CacheKey`, `ruff_python_semantic::imports::FutureImport`

**Derives**: Clone, Debug, Eq, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (2)

```rust
fn alias(module: String, name: String, as_name: String) -> Self
fn member(module: String, name: String) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ruff_cache::cache_key::CacheKey`**

```rust
fn cache_key(&self, key: &mut ruff_cache::CacheKeyHasher)
```

**via `ruff_python_semantic::imports::FutureImport`**

```rust
fn is_future_import(&self) -> bool
```

A representation of an individual name imported via a `from ... import` statement.

---

## ModuleNameImport

`struct` · `ruff_python_semantic::imports::ModuleNameImport`

Also reachable as `ruff_python_semantic::ModuleNameImport`

```rust
struct ModuleNameImport
```

**Fields**: `name`

**Implements**: `core::fmt::Display`, `ruff_cache::cache_key::CacheKey`, `ruff_python_semantic::imports::FutureImport`

**Derives**: Clone, Debug, Eq, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (2)

```rust
fn alias(name: String, as_name: String) -> Self
fn module(name: String) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `ruff_cache::cache_key::CacheKey`**

```rust
fn cache_key(&self, key: &mut ruff_cache::CacheKeyHasher)
```

**via `ruff_python_semantic::imports::FutureImport`**

```rust
fn is_future_import(&self) -> bool
```

A representation of an individual name imported via an `import` statement.

---

## NameImports

`struct` · `ruff_python_semantic::imports::NameImports`

Also reachable as `ruff_python_semantic::NameImports`

```rust
struct NameImports
```

**Implements**: `ruff_cache::cache_key::CacheKey`

**Derives**: Clone, Debug, Eq, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (1)

```rust
fn into_imports(self) -> Vec<NameImport>
```

**via `ruff_cache::cache_key::CacheKey`**

```rust
fn cache_key(&self, key: &mut ruff_cache::CacheKeyHasher)
```

A list of names imported via any import statement.

---

## FutureImport

`trait` · `ruff_python_semantic::imports::FutureImport`

Also reachable as `ruff_python_semantic::FutureImport`

```rust
trait FutureImport
```

**Implementors** (3)

- `ruff_python_semantic::imports::MemberNameImport`
- `ruff_python_semantic::imports::ModuleNameImport`
- `ruff_python_semantic::imports::NameImport`

**Methods** (1)

```rust
fn is_future_import(&self) -> bool
```

---
