# `ruff_python_ast::token`

Crate `ruff_python_ast` · 3 public items · structured records in [`model/ruff_python_ast.token.json`](../model/ruff_python_ast.token.json)

## TokenKind

`enum` · `ruff_python_ast::token::TokenKind`

```rust
enum TokenKind
```

**Variants**: `Name`, `Int`, `Float`, `Complex`, `String`, `FStringStart`, `FStringMiddle`, `FStringEnd`, `TStringStart`, `TStringMiddle`, `TStringEnd`, `IpyEscapeCommand`, `Comment`, `Newline`, `NonLogicalNewline`, `Indent`, `Dedent`, `EndOfFile`, `Question`, `Exclamation`, `Lpar`, `Rpar`, `Lsqb`, `Rsqb`, `Colon`, `Comma`, `Semi`, `Plus`, `Minus`, `Star`, `Slash`, `Vbar`, `Amper`, `Less`, `Greater`, `Equal`, `Dot`, `Percent`, `Lbrace`, `Rbrace`, `EqEqual`, `NotEqual`, `LessEqual`, `GreaterEqual`, `Tilde`, `CircumFlex`, `LeftShift`, `RightShift`, `DoubleStar`, `DoubleStarEqual`, `PlusEqual`, `MinusEqual`, `StarEqual`, `SlashEqual`, `PercentEqual`, `AmperEqual`, `VbarEqual`, `CircumflexEqual`, `LeftShiftEqual`, `RightShiftEqual`, `DoubleSlash`, `DoubleSlashEqual`, `ColonEqual`, `At`, `AtEqual`, `Rarrow`, `Ellipsis`, `And`, `As`, `Assert`, `Async`, `Await`, `Break`, `Class`, `Continue`, `Def`, `Del`, `Elif`, `Else`, `Except`, `False`, `Finally`, `For`, `From`, `Global`, `If`, `Import`, `In`, `Is`, `Lambda`, `None`, `Nonlocal`, `Not`, `Or`, `Pass`, `Raise`, `Return`, `True`, `Try`, `While`, `With`, `Yield`, `Case`, `Lazy`, `Match`, `Type`, `Unknown`

**Implements**: `core::convert::From`, `core::fmt::Display`

**Derives**: Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (20)

```rust
const fn as_augmented_assign_operator(self) -> Option<Operator>
const fn as_binary_operator(self) -> Option<Operator>
const fn as_bool_operator(self) -> Option<BoolOp>
const fn as_unary_arithmetic_operator(self) -> Option<UnaryOp>
const fn as_unary_operator(self) -> Option<UnaryOp>
const fn is_any_newline(self) -> bool
const fn is_arithmetic(self) -> bool
const fn is_bitwise_or_shift(self) -> bool
const fn is_comment(&self) -> bool
const fn is_dot(self) -> bool
const fn is_eof(self) -> bool
const fn is_interpolated_string_end(self) -> bool
fn is_keyword(self) -> bool
const fn is_lbrace(self) -> bool
fn is_non_soft_keyword(self) -> bool
const fn is_operator(self) -> bool
const fn is_singleton(self) -> bool
fn is_soft_keyword(self) -> bool
const fn is_trivia(&self) -> bool
const fn is_unary_arithmetic_operator(self) -> bool
```

**via `core::convert::From`**

```rust
fn from(op: Operator) -> Self
fn from(op: BoolOp) -> Self
fn from(op: UnaryOp) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

A kind of a token.

---

## Token

`struct` · `ruff_python_ast::token::Token`

```rust
struct Token
```

**Implements**: `ruff_text_size::traits::Ranged`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (7)

```rust
const fn as_tuple(&self) -> (TokenKind, TextRange)
fn is_triple_quoted_string(self) -> bool
const fn kind(&self) -> TokenKind
fn new(kind: TokenKind, range: TextRange, flags: TokenFlags) -> Token
fn string_flags(self) -> Option<AnyStringFlags>
fn string_quote_style(self) -> Quote
fn unwrap_string_flags(self) -> AnyStringFlags
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> TextRange
```

---

## TokenFlags

`struct` · `ruff_python_ast::token::TokenFlags`

```rust
struct TokenFlags
```

**Implements**: `bitflags::traits::Flags`, `bitflags::traits::PublicFlags`, `core::fmt::Binary`, `core::fmt::LowerHex`, `core::fmt::Octal`, `core::fmt::UpperHex`, `core::iter::traits::collect::Extend`, `core::iter::traits::collect::FromIterator`, `core::iter::traits::collect::IntoIterator`, `core::ops::arith::Sub`, `core::ops::arith::SubAssign`, `core::ops::bit::BitAnd`, `core::ops::bit::BitAndAssign`, `core::ops::bit::BitOr`, `core::ops::bit::BitOrAssign`, `core::ops::bit::BitXor`, `core::ops::bit::BitXorAssign`, `core::ops::bit::Not`, `ruff_python_ast::nodes::StringFlags`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (28)

```rust
const fn all() -> Self
const fn bits(&self) -> u16
const fn complement(self) -> Self
const fn contains(&self, other: Self) -> bool
const fn difference(self, other: Self) -> Self
const fn empty() -> Self
const fn from_bits(bits: u16) -> __private::core::option::Option<Self>
const fn from_bits_retain(bits: u16) -> Self
const fn from_bits_truncate(bits: u16) -> Self
fn from_name(name: &str) -> __private::core::option::Option<Self>
fn insert(&mut self, other: Self)
const fn intersection(self, other: Self) -> Self
const fn intersects(&self, other: Self) -> bool
const fn is_all(&self) -> bool
const fn is_empty(&self) -> bool
const fn is_f_string(self) -> bool
const fn is_interpolated_string(self) -> bool
const fn is_non_ascii_name(self) -> bool
const fn is_raw_string(self) -> bool
const fn is_t_string(self) -> bool
fn is_triple_quoted_interpolated_string(self) -> bool
const fn iter(&self) -> iter::Iter<TokenFlags>
const fn iter_names(&self) -> iter::IterNames<TokenFlags>
fn remove(&mut self, other: Self)
fn set(&mut self, other: Self, value: bool)
const fn symmetric_difference(self, other: Self) -> Self
fn toggle(&mut self, other: Self)
const fn union(self, other: Self) -> Self
```

**via `bitflags::traits::Flags`**

```rust
fn all_named() -> TokenFlags
fn bits(&self) -> u16
fn from_bits_retain(bits: u16) -> TokenFlags
```

**via `core::fmt::Binary`**

```rust
fn fmt(&self, f: &mut __private::core::fmt::Formatter<'_>) -> __private::core::fmt::Result
```

**via `core::fmt::LowerHex`**

```rust
fn fmt(&self, f: &mut __private::core::fmt::Formatter<'_>) -> __private::core::fmt::Result
```

**via `core::fmt::Octal`**

```rust
fn fmt(&self, f: &mut __private::core::fmt::Formatter<'_>) -> __private::core::fmt::Result
```

**via `core::fmt::UpperHex`**

```rust
fn fmt(&self, f: &mut __private::core::fmt::Formatter<'_>) -> __private::core::fmt::Result
```

**via `core::iter::traits::collect::Extend`**

```rust
fn extend<T: __private::core::iter::IntoIterator<Item = Self>>(&mut self, iterator: T)
```

**via `core::iter::traits::collect::FromIterator`**

```rust
fn from_iter<T: __private::core::iter::IntoIterator<Item = Self>>(iterator: T) -> Self
```

**via `core::iter::traits::collect::IntoIterator`**

```rust
fn into_iter(self) -> Self::IntoIter
```

**via `core::ops::arith::Sub`**

```rust
fn sub(self, other: Self) -> Self
```

**via `core::ops::arith::SubAssign`**

```rust
fn sub_assign(&mut self, other: Self)
```

**via `core::ops::bit::BitAnd`**

```rust
fn bitand(self, other: Self) -> Self
```

**via `core::ops::bit::BitAndAssign`**

```rust
fn bitand_assign(&mut self, other: Self)
```

**via `core::ops::bit::BitOr`**

```rust
fn bitor(self, other: TokenFlags) -> Self
```

**via `core::ops::bit::BitOrAssign`**

```rust
fn bitor_assign(&mut self, other: Self)
```

**via `core::ops::bit::BitXor`**

```rust
fn bitxor(self, other: Self) -> Self
```

**via `core::ops::bit::BitXorAssign`**

```rust
fn bitxor_assign(&mut self, other: Self)
```

**via `core::ops::bit::Not`**

```rust
fn not(self) -> Self
```

**via `ruff_python_ast::nodes::StringFlags`**

```rust
fn is_unclosed(self) -> bool
fn prefix(self) -> AnyStringPrefix
fn quote_style(self) -> Quote
fn triple_quotes(self) -> TripleQuotes
```

---
