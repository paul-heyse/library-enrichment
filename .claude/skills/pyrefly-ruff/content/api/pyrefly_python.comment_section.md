# `pyrefly_python::comment_section`

Crate `pyrefly_python` · 1 public items · structured records in [`model/pyrefly_python.comment_section.json`](../model/pyrefly_python.comment_section.json)

## CommentSection

`struct` · `pyrefly_python::comment_section::CommentSection`

```rust
struct CommentSection
```

**Fields**: `level`, `title`, `range`, `line_number`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn extract_from_module(module: &Module) -> Vec<CommentSection>
fn parse(line: &str, line_number: u32, line_start: TextSize) -> Option<Self>
```

Represents a parsed comment section.

---
