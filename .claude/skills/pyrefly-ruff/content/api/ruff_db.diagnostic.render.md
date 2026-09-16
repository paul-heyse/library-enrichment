# `ruff_db::diagnostic::render`

Crate `ruff_db` · 5 public items · structured records in [`model/ruff_db.diagnostic.render.json`](../model/ruff_db.diagnostic.render.json)

## DisplayDiagnostic

`struct` · `ruff_db::diagnostic::render::DisplayDiagnostic`

Also reachable as `ruff_db::diagnostic::DisplayDiagnostic`

```rust
struct DisplayDiagnostic<'a>
```

**Implements**: `core::fmt::Display`

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

A type that implements `std::fmt::Display` for diagnostic rendering.

It is created via [`Diagnostic::display`].

The lifetime parameter, `'a`, refers to the shorter of:

* The lifetime of the rendering configuration.
* The lifetime of the resolver used to load the contents of `Span`
  values. When using Salsa, this most commonly corresponds to the lifetime
  of a Salsa `Db`.
* The lifetime of the diagnostic being rendered.

---

## DisplayDiagnostics

`struct` · `ruff_db::diagnostic::render::DisplayDiagnostics`

Also reachable as `ruff_db::diagnostic::DisplayDiagnostics`

```rust
struct DisplayDiagnostics<'a>
```

**Implements**: `core::fmt::Display`

**Methods** (1)

```rust
fn new(resolver: &'a dyn FileResolver, config: &'a DisplayDiagnosticConfig, diagnostics: &'a [Diagnostic]) -> DisplayDiagnostics<'a>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

A type that implements `std::fmt::Display` for rendering a collection of diagnostics.

It is intended for collections of diagnostics that need to be serialized together, as is the
case for JSON, for example.

See [`DisplayDiagnostic`] for rendering individual `Diagnostic`s and details about the lifetime
constraints.

---

## DummyFileResolver

`struct` · `ruff_db::diagnostic::render::DummyFileResolver`

Also reachable as `ruff_db::diagnostic::DummyFileResolver`

```rust
struct DummyFileResolver
```

**Implements**: `ruff_db::diagnostic::render::FileResolver`

**via `ruff_db::diagnostic::render::FileResolver`**

```rust
fn current_directory(&self) -> &Path
fn input(&self, _file: File) -> Input
fn is_notebook(&self, _file: &UnifiedFile) -> bool
fn notebook_index(&self, _file: &UnifiedFile) -> Option<NotebookIndex>
fn path(&self, _file: File) -> &str
```

A stub implementation of [`FileResolver`] intended for testing.

---

## Input

`struct` · `ruff_db::diagnostic::render::Input`

Also reachable as `ruff_db::diagnostic::Input`

```rust
struct Input
```

**Derives**: Clone, Debug

An abstraction over a unit of user input.

A single unit of user input usually corresponds to a `File`.
This contains the actual content of that input as well as a
line index for efficiently querying its contents.

---

## FileResolver

`trait` · `ruff_db::diagnostic::render::FileResolver`

Also reachable as `ruff_db::diagnostic::FileResolver`

```rust
trait FileResolver
```

**Implementors** (2)

- `ruff_db::diagnostic::render::DummyFileResolver`
- `ruff_linter::message::EmitterContext`

**Methods** (5)

```rust
fn current_directory(&self) -> &Path
fn input(&self, file: File) -> Input
fn is_notebook(&self, file: &UnifiedFile) -> bool
fn notebook_index(&self, file: &UnifiedFile) -> Option<NotebookIndex>
fn path(&self, file: File) -> &str
```

A trait that facilitates the retrieval of source code from a `Span`.

At present, this is tightly coupled with a Salsa database. In the future,
it is intended for this resolver to become an abstraction providing a
similar API. We define things this way for now to keep the Salsa coupling
at "arm's" length, and to make it easier to do the actual de-coupling in
the future.

For example, at time of writing (2025-03-07), the plan is (roughly) for
Ruff to grow its own interner of file paths so that a `Span` can store an
interned ID instead of a (roughly) `Arc<Path>`. This interner is planned
to be entirely separate from the Salsa interner used by ty, and so,
callers will need to pass in a different "resolver" for turning `Span`s
into actual file paths/contents. The infrastructure for this isn't fully in
place, but this type serves to demarcate the intended abstraction boundary.

---
