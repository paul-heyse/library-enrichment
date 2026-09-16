# `ruff_linter::message`

Crate `ruff_linter` · 3 public items · structured records in [`model/ruff_linter.message.json`](../model/ruff_linter.message.json)

## create_panic_diagnostic

`function` · `ruff_linter::message::create_panic_diagnostic`

```rust
fn create_panic_diagnostic(error: &ruff_db::panic::PanicError, path: Option<&std::path::Path>) -> ruff_db::diagnostic::Diagnostic
```

Create a `Diagnostic` from a panic.

---

## render_diagnostics

`function` · `ruff_linter::message::render_diagnostics`

```rust
fn render_diagnostics(writer: &mut dyn Write, format: settings::types::OutputFormat, config: ruff_db::diagnostic::DisplayDiagnosticConfig, context: &EmitterContext<'_>, diagnostics: &[ruff_db::diagnostic::Diagnostic]) -> std::io::Result<()>
```

---

## EmitterContext

`struct` · `ruff_linter::message::EmitterContext`

```rust
struct EmitterContext<'a>
```

**Implements**: `ruff_db::diagnostic::render::FileResolver`

**Methods** (1)

```rust
fn new(notebook_indexes: &'a FxHashMap<String, NotebookIndex>) -> Self
```

**via `ruff_db::diagnostic::render::FileResolver`**

```rust
fn current_directory(&self) -> &std::path::Path
fn input(&self, _file: File) -> Input
fn is_notebook(&self, file: &UnifiedFile) -> bool
fn notebook_index(&self, file: &UnifiedFile) -> Option<NotebookIndex>
fn path(&self, _file: File) -> &str
```

Context used while rendering diagnostics.

---
