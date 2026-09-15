# `sqlparser::ast::comments`

Crate `sqlparser` · 4 public items · structured records in [`model/sqlparser.ast.comments.json`](../model/sqlparser.ast.comments.json)

## Comment

`enum` · `sqlparser::ast::comments::Comment`

```rust
enum Comment
```

**Variants**: `SingleLine`, `MultiLine`

**Implements**: `core::ops::deref::Deref`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn as_str(&self) -> &str
```

**via `core::ops::deref::Deref`**

```rust
fn deref(&self) -> &Self::Target
```

A unified type of the different source code comment formats.

---

## CommentWithSpan

`struct` · `sqlparser::ast::comments::CommentWithSpan`

```rust
struct CommentWithSpan
```

**Fields**: `comment`, `span`

**Implements**: `core::ops::deref::Deref`, `sqlparser::ast::spans::Spanned`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `core::ops::deref::Deref`**

```rust
fn deref(&self) -> &Self::Target
```

**via `sqlparser::ast::spans::Spanned`**

```rust
fn span(&self) -> Span
```

A source code comment with information of its entire span.

---

## Comments

`struct` · `sqlparser::ast::comments::Comments`

```rust
struct Comments
```

**Derives**: Clone, Debug, Default

**Methods** (1)

```rust
fn find<R: RangeBounds<Location>>(&self, range: R) -> Iter<'_>
```

An opaque container for comments from a parse SQL source code.

---

## Iter

`struct` · `sqlparser::ast::comments::Iter`

```rust
struct Iter<'a>
```

**Implements**: `core::iter::traits::iterator::Iterator`

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<Self::Item>
```

An opaque iterator implementation over comments served by [Comments::find].

---
