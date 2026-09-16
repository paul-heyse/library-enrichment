# `ruff_python_ast::str`

Crate `ruff_python_ast` · 6 public items · structured records in [`model/ruff_python_ast.str.json`](../model/ruff_python_ast.str.json)

## Quote

`enum` · `ruff_python_ast::str::Quote`

```rust
enum Quote
```

**Variants**: `Single`, `Double`

**Implements**: `core::convert::TryFrom`, `core::fmt::Display`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (6)

```rust
const fn as_byte(self) -> u8
const fn as_char(self) -> char
const fn as_str(self) -> &'static str
const fn is_double(&self) -> bool
const fn is_single(&self) -> bool
const fn opposite(self) -> Self
```

**via `core::convert::TryFrom`**

```rust
fn try_from(value: char) -> Result<Self, Self::Error>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Enumeration of the two kinds of quotes that can be used
for Python string/f/t-string/bytestring literals

---

## TripleQuotes

`enum` · `ruff_python_ast::str::TripleQuotes`

```rust
enum TripleQuotes
```

**Variants**: `Yes`, `No`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
const fn is_no(self) -> bool
const fn is_yes(self) -> bool
```

---

## is_triple_quote

`function` · `ruff_python_ast::str::is_triple_quote`

```rust
fn is_triple_quote(content: &str) -> bool
```

Return `true` if the string is a triple-quote string or byte prefix.

---

## leading_quote

`function` · `ruff_python_ast::str::leading_quote`

```rust
fn leading_quote(content: &str) -> Option<&str>
```

Return the leading quote for a string, template, or bytes literal (e.g., `"""`).

---

## raw_contents

`function` · `ruff_python_ast::str::raw_contents`

```rust
fn raw_contents(contents: &str) -> Option<&str>
```

Strip the leading and trailing quotes from a string.
Assumes that the string is a valid string literal, but does not verify that the string
is a "simple" string literal (i.e., that it does not contain any implicit concatenations).

---

## trailing_quote

`function` · `ruff_python_ast::str::trailing_quote`

```rust
fn trailing_quote(content: &str) -> Option<&str>
```

Return the trailing quote string for a string, template, or bytes literal (e.g., `"""`).

---
