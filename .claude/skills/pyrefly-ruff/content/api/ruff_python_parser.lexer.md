# `ruff_python_parser::lexer`

Crate `ruff_python_parser` · 1 public items · structured records in [`model/ruff_python_parser.lexer.json`](../model/ruff_python_parser.lexer.json)

## Lexer

`struct` · `ruff_python_parser::lexer::Lexer`

```rust
struct Lexer<'src>
```

**Derives**: Debug

**Methods** (1)

```rust
fn next_token(&mut self) -> TokenKind
```

A lexer for Python source code.

---
