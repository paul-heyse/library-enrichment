# `ra_ap_syntax`

Crate `ra_ap_syntax` · 2 public items · structured records in [`model/ra_ap_syntax.json`](../model/ra_ap_syntax.json)

## match_ast

`macro` · `ra_ap_syntax::match_ast`

```rust
macro_rules! match_ast
```

Matches a `SyntaxNode` against an `ast` type.

# Example:

```ignore
match_ast! {
    match node {
        ast::CallExpr(it) => { ... },
        ast::MethodCallExpr(it) => { ... },
        ast::MacroCall(it) => { ... },
        _ => None,
    }
}
```

---

## Parse

`struct` · `ra_ap_syntax::Parse`

```rust
struct Parse<T>
```

**Implements**: `core::ops::drop::Drop`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (8)

```rust
fn cast<N: AstNode>(self) -> Option<Parse<N>>
fn debug_dump(&self) -> String
fn errors(&self) -> Vec<SyntaxError>
fn ok(self) -> Result<T, Vec<SyntaxError>>
fn reparse(&self, delete: TextRange, insert: &str, edition: Edition) -> Parse<SourceFile>
fn syntax_node(&self) -> SyntaxNode
fn to_syntax(self) -> Parse<SyntaxNode>
fn tree(&self) -> T
```

**via `core::ops::drop::Drop`**

```rust
fn drop(&mut self)
```

`Parse` is the result of the parsing: a syntax tree and a collection of
errors.

Note that we always produce a syntax tree, even for completely invalid
files.

---
