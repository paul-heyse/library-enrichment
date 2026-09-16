# `ruff_db::parsed`

Crate `ruff_db` · 4 public items · structured records in [`model/ruff_db.parsed.json`](../model/ruff_db.parsed.json)

## parsed_module

`function` · `ruff_db::parsed::parsed_module`

```rust
fn parsed_module<'db>(db: &'db dyn Db, file: PythonFile<'_>) -> &'db ParsedModule
```

Returns the parsed AST of `file`, including its token stream.

The query uses Ruff's error-resilient parser. That means that the parser always succeeds to produce an
AST even if the file contains syntax errors. The parse errors
are then accessible through [`Parsed::errors`].

The query is only cached when the [`source_text()`] hasn't changed. This is because
comparing two ASTs is a non-trivial operation and every offset change is directly
reflected in the changed AST offsets.
The other reason is that Ruff's AST doesn't implement `Eq` which Salsa requires
for determining if a query result is unchanged.

The LRU capacity of 200 was picked without any empirical evidence that it's optimal,
instead it's a wild guess that it should be unlikely that incremental changes involve
more than 200 modules. Parsed ASTs within the same revision are never evicted by Salsa.

---

## parsed_string_annotation

`function` · `ruff_db::parsed::parsed_string_annotation`

```rust
fn parsed_string_annotation(source: &str, string: &ruff_python_ast::StringLiteral) -> Result<ruff_python_parser::Parsed<ruff_python_ast::ModExpression>, ruff_python_parser::ParseError>
```

---

## ParsedModule

`struct` · `ruff_db::parsed::ParsedModule`

```rust
struct ParsedModule
```

**Implements**: `get_size2::GetSize`

**Derives**: Clone, Debug, Eq, PartialEq

**Methods** (4)

```rust
fn clear(&self)
fn file(&self) -> File
fn load(&self, db: &dyn Db) -> ParsedModuleRef
fn python_version(&self) -> PythonVersion
```

**via `get_size2::GetSize`**

```rust
fn get_heap_size(&self) -> usize
fn get_heap_size_with_tracker<TRACKER: ::get_size2::GetSizeTracker>(&self, tracker: TRACKER) -> (usize, TRACKER)
```

A wrapper around a parsed module.

This type manages instances of the module AST. A particular instance of the AST
is represented with the [`ParsedModuleRef`] type.

---

## ParsedModuleRef

`struct` · `ruff_db::parsed::ParsedModuleRef`

```rust
struct ParsedModuleRef
```

**Implements**: `core::ops::deref::Deref`

**Derives**: Clone

**Methods** (2)

```rust
fn get_by_index<'ast>(&'ast self, index: NodeIndex) -> AnyRootNodeRef<'ast>
fn module(&self) -> &ParsedModule
```

**via `core::ops::deref::Deref`**

```rust
fn deref(&self) -> &Self::Target
```

Cheap cloneable wrapper around an instance of a module AST.

---
