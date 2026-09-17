# `regex_syntax::hir::print`

Crate `regex-syntax` · 1 public items · structured records in [`model/regex_syntax.hir.print.json`](../model/regex_syntax.hir.print.json)

## Printer

`struct` · `regex_syntax::hir::print::Printer`

```rust
struct Printer
```

**Derives**: Debug

**Methods** (2)

```rust
fn new() -> Printer
fn print<W: fmt::Write>(&mut self, hir: &Hir, wtr: W) -> fmt::Result
```

A printer for a regular expression's high-level intermediate
representation.

A printer converts a high-level intermediate representation (HIR) to a
regular expression pattern string. This particular printer uses constant
stack space and heap space proportional to the size of the HIR.

Since this printer is only using the HIR, the pattern it prints will likely
not resemble the original pattern at all. For example, a pattern like
`\pL` will have its entire class written out.

The purpose of this printer is to provide a means to mutate an HIR and then
build a regular expression from the result of that mutation. (A regex
library could provide a constructor from this HIR explicitly, but that
creates an unnecessary public coupling between the regex library and this
specific HIR representation.)

---
