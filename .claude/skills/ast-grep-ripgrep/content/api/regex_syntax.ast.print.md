# `regex_syntax::ast::print`

Crate `regex-syntax` · 1 public items · structured records in [`model/regex_syntax.ast.print.json`](../model/regex_syntax.ast.print.json)

## Printer

`struct` · `regex_syntax::ast::print::Printer`

```rust
struct Printer
```

**Derives**: Debug

**Methods** (2)

```rust
fn new() -> Printer
fn print<W: fmt::Write>(&mut self, ast: &Ast, wtr: W) -> fmt::Result
```

A printer for a regular expression abstract syntax tree.

A printer converts an abstract syntax tree (AST) to a regular expression
pattern string. This particular printer uses constant stack space and heap
space proportional to the size of the AST.

This printer will not necessarily preserve the original formatting of the
regular expression pattern string. For example, all whitespace and comments
are ignored.

---
