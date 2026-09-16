# `ruff_linter::linter`

Crate `ruff_linter` · 10 public items · structured records in [`model/ruff_linter.linter.json`](../model/ruff_linter.linter.json)

## ParseSource

`enum` · `ruff_linter::linter::ParseSource`

```rust
enum ParseSource
```

**Variants**: `None`, `Precomputed`

**Derives**: Clone, Debug

---

## add_suppressions_to_path

`function` · `ruff_linter::linter::add_suppressions_to_path`

```rust
fn add_suppressions_to_path(path: &std::path::Path, package: Option<package::PackageRoot<'_>>, source_kind: &source_kind::SourceKind, source_type: ruff_python_ast::PySourceType, settings: &settings::LinterSettings, reason: Option<&str>, suppression_kind: SuppressionKind) -> anyhow::Result<usize>
```

Add any missing suppression comments to the source code at the given `Path`.

---

## check_path

`function` · `ruff_linter::linter::check_path`

```rust
fn check_path(path: &std::path::Path, package: Option<package::PackageRoot<'_>>, locator: &Locator<'_>, stylist: &ruff_python_codegen::Stylist<'_>, indexer: &ruff_python_index::Indexer, directives: &directives::Directives, settings: &settings::LinterSettings, noqa: flags::Noqa, source_kind: &source_kind::SourceKind, source_type: ruff_python_ast::PySourceType, parsed: &ruff_python_parser::Parsed<ruff_python_ast::ModModule>, target_version: settings::TargetVersion, suppressions: &suppression::Suppressions) -> Vec<ruff_db::diagnostic::Diagnostic>
```

Generate [`Diagnostic`]s from the source code contents at the given `Path`.

---

## lint_fix

`function` · `ruff_linter::linter::lint_fix`

```rust
fn lint_fix<'a>(path: &std::path::Path, package: Option<package::PackageRoot<'_>>, noqa: flags::Noqa, unsafe_fixes: settings::types::UnsafeFixes, settings: &settings::LinterSettings, source_kind: &'a source_kind::SourceKind, source_type: ruff_python_ast::PySourceType) -> anyhow::Result<FixerResult<'a>>
```

Generate `Diagnostic`s from source code content, iteratively fixing
until stable.

---

## lint_only

`function` · `ruff_linter::linter::lint_only`

```rust
fn lint_only(path: &std::path::Path, package: Option<package::PackageRoot<'_>>, settings: &settings::LinterSettings, noqa: flags::Noqa, source_kind: &source_kind::SourceKind, source_type: ruff_python_ast::PySourceType, source: ParseSource) -> LinterResult
```

Generate a [`Diagnostic`] for each diagnostic triggered by the given source code.

---

## parse_unchecked_source

`function` · `ruff_linter::linter::parse_unchecked_source`

```rust
fn parse_unchecked_source(source_kind: &source_kind::SourceKind, source_type: ruff_python_ast::PySourceType, target_version: ruff_python_ast::PythonVersion) -> ruff_python_parser::Parsed<ruff_python_ast::ModModule>
```

Like [`ruff_python_parser::parse_unchecked_source`], but with an explicit [`PythonVersion`] and
per-cell notebook parsing.

Per-cell modules are merged so definitions remain visible across cells.

---

## FixTable

`struct` · `ruff_linter::linter::FixTable`

```rust
struct FixTable
```

**Derives**: Debug, Default, PartialEq, StructuralPartialEq

**Methods** (4)

```rust
fn counts(&self) -> impl Iterator<Item = usize>
fn entry(&mut self, id: DiagnosticId) -> FixTableEntry<'_>
fn is_empty(&self) -> bool
fn iter(&self) -> impl Iterator<Item = (DiagnosticId, Option<&SecondaryCode>, usize)>
```

A mapping from a diagnostic's identifier to its optional noqa code and fix count.

---

## FixTableEntry

`struct` · `ruff_linter::linter::FixTableEntry`

```rust
struct FixTableEntry<'a>
```

**Methods** (1)

```rust
fn or_default(self, code: Option<&SecondaryCode>) -> &'a mut usize
```

---

## FixerResult

`struct` · `ruff_linter::linter::FixerResult`

```rust
struct FixerResult<'a>
```

**Fields**: `result`, `transformed`, `fixed`

---

## LinterResult

`struct` · `ruff_linter::linter::LinterResult`

```rust
struct LinterResult
```

**Fields**: `diagnostics`

**Methods** (1)

```rust
fn has_invalid_syntax(&self) -> bool
```

---
