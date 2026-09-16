# `ruff_server::edit::text_document`

Crate `ruff_server` · 1 public items · structured records in [`model/ruff_server.edit.text_document.json`](../model/ruff_server.edit.text_document.json)

## TextDocument

`struct` · `ruff_server::edit::text_document::TextDocument`

Also reachable as `ruff_server::TextDocument`

```rust
struct TextDocument
```

**Derives**: Clone, Debug

**Methods** (3)

```rust
fn apply_changes(&mut self, changes: Vec<lsp_types::TextDocumentContentChangeEvent>, new_version: i32, encoding: PositionEncoding)
fn contents(&self) -> &str
fn new(contents: String, version: i32) -> Self
```

The state of an individual document in the server. Stays up-to-date
with changes made by the user, including unsaved changes.

---
