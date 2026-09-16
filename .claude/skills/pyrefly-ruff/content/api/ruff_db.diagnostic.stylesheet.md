# `ruff_db::diagnostic::stylesheet`

Crate `ruff_db` · 2 public items · structured records in [`model/ruff_db.diagnostic.stylesheet.json`](../model/ruff_db.diagnostic.stylesheet.json)

## fmt_with_hyperlink

`function` · `ruff_db::diagnostic::stylesheet::fmt_with_hyperlink`

Also reachable as `ruff_db::diagnostic::fmt_with_hyperlink`

```rust
fn fmt_with_hyperlink<'a, T>(content: T, url: Option<&'a str>, stylesheet: &DiagnosticStylesheet) -> impl std::fmt::Display + 'a where T: std::fmt::Display + 'a
```

---

## DiagnosticStylesheet

`struct` · `ruff_db::diagnostic::stylesheet::DiagnosticStylesheet`

Also reachable as `ruff_db::diagnostic::DiagnosticStylesheet`

```rust
struct DiagnosticStylesheet
```

**Derives**: Clone, Debug, Default

**Methods** (2)

```rust
fn plain() -> Self
fn styled() -> Self
```

---
