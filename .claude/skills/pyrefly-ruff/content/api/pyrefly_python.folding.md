# `pyrefly_python::folding`

Crate `pyrefly_python` · 2 public items · structured records in [`model/pyrefly_python.folding.json`](../model/pyrefly_python.folding.json)

## FoldKind

`enum` · `pyrefly_python::folding::FoldKind`

```rust
enum FoldKind
```

**Variants**: `Code`, `Comment`, `CommentSection`, `Region`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

Semantic category of a folding range before conversion to LSP kinds.

---

## folding_ranges

`function` · `pyrefly_python::folding::folding_ranges`

```rust
fn folding_ranges(module: &module::Module, body: &[ruff_python_ast::Stmt]) -> Vec<(ruff_text_size::TextRange, FoldKind)>
```

Find the folding ranges (where you can collapse the code) in a module, given the AST.

---
