# `ra_ap_syntax::ast::token_ext`

Crate `ra_ap_syntax` · 7 public items · structured records in [`model/ra_ap_syntax.ast.token_ext.json`](../model/ra_ap_syntax.ast.token_ext.json)

## AnyString

`enum` · `ra_ap_syntax::ast::token_ext::AnyString`

Also reachable as `ra_ap_syntax::ast::AnyString`

```rust
enum AnyString
```

**Variants**: `ByteString`, `CString`, `String`

**Implements**: `ra_ap_syntax::ast::AstToken`, `ra_ap_syntax::ast::token_ext::IsString`

**Methods** (1)

```rust
fn value(&self) -> Result<Cow<'_, str>, EscapeError>
```

**via `ra_ap_syntax::ast::AstToken`**

```rust
fn can_cast(kind: SyntaxKind) -> bool
fn cast(syntax: SyntaxToken) -> Option<Self>
fn syntax(&self) -> &SyntaxToken
```

**via `ra_ap_syntax::ast::token_ext::IsString`**

```rust
fn raw_prefix(&self) -> &'static str
fn unescape(&self, s: &str, callback: impl FnMut(Range<usize>, Result<char, EscapeError>))
```

---

## CommentShape

`enum` · `ra_ap_syntax::ast::token_ext::CommentShape`

Also reachable as `ra_ap_syntax::ast::CommentShape`

```rust
enum CommentShape
```

**Variants**: `Line`, `Block`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn from_text(text: &str) -> CommentShape
fn is_block(self) -> bool
fn is_line(self) -> bool
```

---

## Radix

`enum` · `ra_ap_syntax::ast::token_ext::Radix`

Also reachable as `ra_ap_syntax::ast::Radix`

```rust
enum Radix
```

**Variants**: `Binary`, `Octal`, `Decimal`, `Hexadecimal`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

---

## AnyComment

`struct` · `ra_ap_syntax::ast::token_ext::AnyComment`

Also reachable as `ra_ap_syntax::ast::AnyComment`

```rust
struct AnyComment
```

**Implements**: `ra_ap_syntax::ast::AstToken`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (8)

```rust
fn doc_kind(&self) -> Option<AttrKind>
fn is_inner(&self) -> bool
fn is_outer(&self) -> bool
fn kind(&self) -> CommentKind
fn prefix(&self) -> &'static str
fn shape(&self) -> CommentShape
fn text(&self) -> &str
fn text_with_markers(&self) -> &str
```

**via `ra_ap_syntax::ast::AstToken`**

```rust
fn can_cast(kind: SyntaxKind) -> bool where Self: Sized
fn cast(syntax: SyntaxToken) -> Option<Self> where Self: Sized
fn syntax(&self) -> &SyntaxToken
```

---

## CommentKind

`struct` · `ra_ap_syntax::ast::token_ext::CommentKind`

Also reachable as `ra_ap_syntax::ast::CommentKind`

```rust
struct CommentKind
```

**Fields**: `shape`, `doc`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn prefix(&self) -> &'static str
```

---

## QuoteOffsets

`struct` · `ra_ap_syntax::ast::token_ext::QuoteOffsets`

Also reachable as `ra_ap_syntax::ast::QuoteOffsets`

```rust
struct QuoteOffsets
```

**Fields**: `quotes`, `contents`

**Derives**: Debug

---

## IsString

`trait` · `ra_ap_syntax::ast::token_ext::IsString`

Also reachable as `ra_ap_syntax::ast::IsString`

```rust
trait IsString: AstToken
```

**Implementors** (4)

- `ra_ap_syntax::ast::generated::tokens::ByteString`
- `ra_ap_syntax::ast::generated::tokens::CString`
- `ra_ap_syntax::ast::generated::tokens::String`
- `ra_ap_syntax::ast::token_ext::AnyString`

**Methods** (11)

```rust
fn close_quote_text_range(&self) -> Option<TextRange>
fn escaped_char_ranges(&self, cb: &mut dyn FnMut(TextRange, Result<char, EscapeError>))
fn is_raw(&self) -> bool
fn map_offset_down(&self, offset: TextSize) -> Option<TextSize>
fn map_range_up(&self, range: TextRange) -> Option<TextRange>
fn open_quote_text_range(&self) -> Option<TextRange>
fn quote_offsets(&self) -> Option<QuoteOffsets>
fn raw_prefix(&self) -> &'static str
fn text_range_between_quotes(&self) -> Option<TextRange>
fn text_without_quotes(&self) -> &str
fn unescape(&self, s: &str, callback: impl FnMut(Range<usize>, Result<char, EscapeError>))
```

---
