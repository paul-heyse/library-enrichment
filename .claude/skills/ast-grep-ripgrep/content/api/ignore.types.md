# `ignore::types`

Crate `ignore` · 4 public items · structured records in [`model/ignore.types.json`](../model/ignore.types.json)

## FileTypeDef

`struct` · `ignore::types::FileTypeDef`

```rust
struct FileTypeDef
```

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn globs(&self) -> &[String]
fn name(&self) -> &str
```

A single file type definition.

File type definitions can be retrieved in aggregate from a file type
matcher. File type definitions are also reported when its responsible
for a match.

---

## Glob

`struct` · `ignore::types::Glob`

```rust
struct Glob<'a>
```

**Derives**: Clone, Debug

**Methods** (1)

```rust
fn file_type_def(&self) -> Option<&FileTypeDef>
```

Glob represents a single glob in a set of file type definitions.

There may be more than one glob for a particular file type.

This is used to report information about the highest precedent glob
that matched.

Note that not all matches necessarily correspond to a specific glob.
For example, if there are one or more selections and a file path doesn't
match any of those selections, then the file path is considered to be
ignored.

The lifetime `'a` refers to the lifetime of the underlying file type
definition, which corresponds to the lifetime of the file type matcher.

---

## Types

`struct` · `ignore::types::Types`

```rust
struct Types
```

**Derives**: Clone, Debug

**Methods** (5)

```rust
fn definitions(&self) -> &[FileTypeDef]
fn empty() -> Types
fn is_empty(&self) -> bool
fn len(&self) -> usize
fn matched<'a, P: AsRef<Path>>(&'a self, path: P, is_dir: bool) -> Match<Glob<'a>>
```

Types is a file type matcher.

---

## TypesBuilder

`struct` · `ignore::types::TypesBuilder`

```rust
struct TypesBuilder
```

**Methods** (9)

```rust
fn add(&mut self, name: &str, glob: &str) -> Result<(), Error>
fn add_def(&mut self, def: &str) -> Result<(), Error>
fn add_defaults(&mut self) -> &mut TypesBuilder
fn build(&self) -> Result<Types, Error>
fn clear(&mut self, name: &str) -> &mut TypesBuilder
fn definitions(&self) -> Vec<FileTypeDef>
fn negate(&mut self, name: &str) -> &mut TypesBuilder
fn new() -> TypesBuilder
fn select(&mut self, name: &str) -> &mut TypesBuilder
```

TypesBuilder builds a type matcher from a set of file type definitions and
a set of file type selections.

---
