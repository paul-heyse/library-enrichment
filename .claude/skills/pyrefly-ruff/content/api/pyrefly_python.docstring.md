# `pyrefly_python::docstring`

Crate `pyrefly_python` · 3 public items · structured records in [`model/pyrefly_python.docstring.json`](../model/pyrefly_python.docstring.json)

## dedent_block_preserving_layout

`function` · `pyrefly_python::docstring::dedent_block_preserving_layout`

```rust
fn dedent_block_preserving_layout(text: &str) -> Option<String>
```

Dedent a block of text while preserving blank lines, similar to how we handle docstrings.

---

## parse_parameter_documentation

`function` · `pyrefly_python::docstring::parse_parameter_documentation`

```rust
fn parse_parameter_documentation(docstring: &str) -> std::collections::HashMap<String, String>
```

Extract a map of `parameter -> markdown` documentation snippets from the
supplied docstring, supporting both Sphinx (`:param foo:`) and
Google-style (`Args:`) formats.

---

## Docstring

`struct` · `pyrefly_python::docstring::Docstring`

```rust
struct Docstring
```

**Derives**: Clone, Debug

**Methods** (3)

```rust
fn clean(docstring: &str) -> String
fn range_from_stmts(xs: &[Stmt]) -> Option<TextRange>
fn resolve(&self) -> String
```

---
