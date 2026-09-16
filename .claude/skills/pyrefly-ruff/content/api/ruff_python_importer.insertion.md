# `ruff_python_importer::insertion`

Crate `ruff_python_importer` · 1 public items · structured records in [`model/ruff_python_importer.insertion.json`](../model/ruff_python_importer.insertion.json)

## Insertion

`struct` · `ruff_python_importer::insertion::Insertion`

Also reachable as `ruff_python_importer::Insertion`

```rust
struct Insertion<'a>
```

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (6)

```rust
fn end_of_statement(stmt: &Stmt, contents: &str, stylist: &Stylist<'_>) -> Insertion<'static>
fn existing_import(stmt: &Stmt, tokens: &Tokens) -> Option<Insertion<'static>>
fn into_edit(self, content: &str) -> Edit
fn is_inline(&self) -> bool
fn start_of_block(location: TextSize, contents: &'a str, stylist: &Stylist<'_>, tokens: &Tokens) -> Insertion<'a>
fn start_of_file(body: &[Stmt], contents: &str, stylist: &Stylist<'_>, within_range: Option<TextRange>) -> Insertion<'static>
```

---
