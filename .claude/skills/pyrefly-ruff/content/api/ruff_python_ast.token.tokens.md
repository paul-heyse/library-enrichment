# `ruff_python_ast::token::tokens`

Crate `ruff_python_ast` · 3 public items · structured records in [`model/ruff_python_ast.token.tokens.json`](../model/ruff_python_ast.token.tokens.json)

## TokenAt

`enum` · `ruff_python_ast::token::tokens::TokenAt`

Also reachable as `ruff_python_ast::token::TokenAt`

```rust
enum TokenAt
```

**Variants**: `None`, `Single`, `Between`

**Implements**: `core::iter::traits::iterator::Iterator`, `core::iter::traits::marker::FusedIterator`

**Derives**: Clone, Debug

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<Self::Item>
```

A token that encloses a given offset or ends exactly at it.

---

## TokenIterWithContext

`struct` · `ruff_python_ast::token::tokens::TokenIterWithContext`

Also reachable as `ruff_python_ast::token::TokenIterWithContext`

```rust
struct TokenIterWithContext<'a>
```

**Implements**: `core::iter::traits::iterator::Iterator`, `core::iter::traits::marker::FusedIterator`

**Derives**: Clone, Debug

**Methods** (4)

```rust
const fn in_parenthesized_context(&self) -> bool
const fn nesting(&self) -> u32
fn new(tokens: &'a [Token]) -> TokenIterWithContext<'a>
fn peek(&self) -> Option<&'a Token>
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<Self::Item>
```

An iterator over the [`Token`]s with context.

Use [`Tokens::iter_with_context`] to iterate over all tokens, or [`Self::new`] to iterate over a
token slice.

---

## Tokens

`struct` · `ruff_python_ast::token::tokens::Tokens`

Also reachable as `ruff_python_ast::token::Tokens`

```rust
struct Tokens
```

**Implements**: `core::iter::traits::collect::IntoIterator`, `core::ops::deref::Deref`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (8)

```rust
fn after(&self, offset: TextSize) -> &[Token]
fn at_offset(&self, offset: TextSize) -> TokenAt
fn before(&self, offset: TextSize) -> &[Token]
fn in_range(&self, range: TextRange) -> &[Token]
fn iter_with_context(&self) -> TokenIterWithContext<'_>
fn new(tokens: Vec<Token>) -> Tokens
fn split_at(&self, offset: TextSize) -> (&[Token], &[Token])
fn token_range(&self, offset: TextSize) -> TextRange
```

**via `core::iter::traits::collect::IntoIterator`**

```rust
fn into_iter(self) -> Self::IntoIter
```

**via `core::ops::deref::Deref`**

```rust
fn deref(&self) -> &Self::Target
```

Tokens represents a vector of lexed [`Token`].

---
