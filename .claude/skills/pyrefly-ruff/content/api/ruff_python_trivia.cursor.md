# `ruff_python_trivia::cursor`

Crate `ruff_python_trivia` · 2 public items · structured records in [`model/ruff_python_trivia.cursor.json`](../model/ruff_python_trivia.cursor.json)

## EOF_CHAR

`constant` · `ruff_python_trivia::cursor::EOF_CHAR`

Also reachable as `ruff_python_trivia::EOF_CHAR`

```rust
const EOF_CHAR: char = '\0'
```

---

## Cursor

`struct` · `ruff_python_trivia::cursor::Cursor`

Also reachable as `ruff_python_trivia::Cursor`

```rust
struct Cursor<'a>
```

**Derives**: Clone, Debug

**Methods** (18)

```rust
fn as_bytes(&self) -> &'a [u8]
fn as_str(&self) -> &'a str
fn bump(&mut self) -> Option<char>
fn chars(&self) -> Chars<'a>
fn eat_char(&mut self, c: char) -> bool
fn eat_char2(&mut self, c1: char, c2: char) -> bool
fn eat_char3(&mut self, c1: char, c2: char, c3: char) -> bool
fn eat_if(&mut self, predicate: impl FnMut(char) -> bool) -> bool
fn eat_while(&mut self, predicate: impl FnMut(char) -> bool)
fn first(&self) -> char
fn is_eof(&self) -> bool
fn new(source: &'a str) -> Self
fn offset(&self) -> TextSize
fn second(&self) -> char
fn skip_bytes(&mut self, count: usize)
fn start_token(&mut self)
fn text_len(&self) -> TextSize
fn token_len(&self) -> TextSize
```

A [`Cursor`] over a string.

Based on [`rustc`'s `Cursor`](https://github.com/rust-lang/rust/blob/d1b7355d3d7b4ead564dbecb1d240fcc74fff21b/compiler/rustc_lexer/src/cursor.rs)

---
