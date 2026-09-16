# `ruff_python_trivia::tokenizer`

Crate `ruff_python_trivia` · 10 public items · structured records in [`model/ruff_python_trivia.tokenizer.json`](../model/ruff_python_trivia.tokenizer.json)

## SimpleTokenKind

`enum` · `ruff_python_trivia::tokenizer::SimpleTokenKind`

Also reachable as `ruff_python_trivia::SimpleTokenKind`

```rust
enum SimpleTokenKind
```

**Variants**: `Comment`, `Whitespace`, `EndOfFile`, `Continuation`, `Newline`, `LParen`, `RParen`, `LBrace`, `RBrace`, `LBracket`, `RBracket`, `Comma`, `Colon`, `Semi`, `Slash`, `Star`, `Dot`, `Plus`, `Minus`, `Equals`, `Greater`, `Less`, `Percent`, `Ampersand`, `Circumflex`, `Vbar`, `At`, `Tilde`, `EqEqual`, `NotEqual`, `LessEqual`, `GreaterEqual`, `LeftShift`, `RightShift`, `DoubleStar`, `DoubleStarEqual`, `PlusEqual`, `MinusEqual`, `StarEqual`, `SlashEqual`, `PercentEqual`, `AmperEqual`, `VbarEqual`, `CircumflexEqual`, `LeftShiftEqual`, `RightShiftEqual`, `DoubleSlash`, `DoubleSlashEqual`, `ColonEqual`, `Ellipsis`, `AtEqual`, `RArrow`, `And`, `As`, `Assert`, `Async`, `Await`, `Break`, `Class`, `Continue`, `Def`, `Del`, `Elif`, `Else`, `Except`, `Finally`, `For`, `From`, `Global`, `If`, `Import`, `In`, `Is`, `Lambda`, `Nonlocal`, `Not`, `Or`, `Pass`, `Raise`, `Return`, `Try`, `While`, `Lazy`, `Match`, `Type`, `Case`, `With`, `Yield`, `Name`, `Other`, `Bogus`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
const fn is_comment(self) -> bool
const fn is_trivia(self) -> bool
```

---

## find_only_token_in_range

`function` · `ruff_python_trivia::tokenizer::find_only_token_in_range`

Also reachable as `ruff_python_trivia::find_only_token_in_range`

```rust
fn find_only_token_in_range(range: ruff_text_size::TextRange, token_kind: SimpleTokenKind, code: &str) -> SimpleToken
```

Returns the only non-trivia, non-closing parenthesis token in `range`.

Includes debug assertions that the range only contains that single token.

---

## first_non_trivia_token

`function` · `ruff_python_trivia::tokenizer::first_non_trivia_token`

Also reachable as `ruff_python_trivia::first_non_trivia_token`

```rust
fn first_non_trivia_token(offset: ruff_text_size::TextSize, code: &str) -> Option<SimpleToken>
```

Searches for the first non-trivia character after `offset`.

The search skips over any whitespace and comments.

Returns `Some` if the source code after `offset` contains any non-trivia character.///
Returns `None` if the text after `offset` is empty or only contains trivia (whitespace or comments).

---

## lines_after

`function` · `ruff_python_trivia::tokenizer::lines_after`

Also reachable as `ruff_python_trivia::lines_after`

```rust
fn lines_after(offset: ruff_text_size::TextSize, code: &str) -> u32
```

Counts the empty lines between `offset` and the first non-whitespace character.

---

## lines_after_ignoring_end_of_line_trivia

`function` · `ruff_python_trivia::tokenizer::lines_after_ignoring_end_of_line_trivia`

Also reachable as `ruff_python_trivia::lines_after_ignoring_end_of_line_trivia`

```rust
fn lines_after_ignoring_end_of_line_trivia(offset: ruff_text_size::TextSize, code: &str) -> u32
```

Counts the empty lines after `offset`, ignoring any trailing trivia on the same line as
`offset`.

---

## lines_after_ignoring_trivia

`function` · `ruff_python_trivia::tokenizer::lines_after_ignoring_trivia`

Also reachable as `ruff_python_trivia::lines_after_ignoring_trivia`

```rust
fn lines_after_ignoring_trivia(offset: ruff_text_size::TextSize, code: &str) -> u32
```

Counts the empty lines after `offset`, ignoring any trailing trivia: end-of-line comments,
own-line comments, and any intermediary newlines.

---

## lines_before

`function` · `ruff_python_trivia::tokenizer::lines_before`

Also reachable as `ruff_python_trivia::lines_before`

```rust
fn lines_before(offset: ruff_text_size::TextSize, code: &str) -> u32
```

Returns the number of newlines between `offset` and the first non whitespace character in the source code.

---

## BackwardsTokenizer

`struct` · `ruff_python_trivia::tokenizer::BackwardsTokenizer`

Also reachable as `ruff_python_trivia::BackwardsTokenizer`

```rust
struct BackwardsTokenizer<'a>
```

**Implements**: `core::iter::traits::iterator::Iterator`

**Methods** (3)

```rust
fn new(source: &'a str, range: TextRange, comment_range: &'a [TextRange]) -> Self
fn skip_trivia(self) -> impl Iterator<Item = SimpleToken> + 'a
fn up_to(offset: TextSize, source: &'a str, comment_range: &'a [TextRange]) -> Self
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<Self::Item>
```

Simple zero allocation backwards tokenizer for finding preceding tokens.

The tokenizer must start at an offset that is trivia (e.g. not inside of a multiline string).
It will fail when reaching a string.

In case it finds something it can't parse, the tokenizer will return a
[`SimpleTokenKind::Other`] and then only a final [`SimpleTokenKind::Bogus`] afterwards.

---

## SimpleToken

`struct` · `ruff_python_trivia::tokenizer::SimpleToken`

Also reachable as `ruff_python_trivia::SimpleToken`

```rust
struct SimpleToken
```

**Fields**: `kind`, `range`

**Implements**: `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
const fn kind(&self) -> SimpleTokenKind
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> TextRange
```

---

## SimpleTokenizer

`struct` · `ruff_python_trivia::tokenizer::SimpleTokenizer`

Also reachable as `ruff_python_trivia::SimpleTokenizer`

```rust
struct SimpleTokenizer<'a>
```

**Implements**: `core::iter::traits::iterator::Iterator`

**Methods** (3)

```rust
fn new(source: &'a str, range: TextRange) -> Self
fn skip_trivia(self) -> impl Iterator<Item = SimpleToken> + 'a
fn starts_at(offset: TextSize, source: &'a str) -> Self
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<Self::Item>
```

Simple zero allocation tokenizer handling most tokens.

The tokenizer must start at an offset that is trivia (e.g. not inside of a multiline string).

In case it finds something it can't parse, the tokenizer will return a
[`SimpleTokenKind::Other`] and then only a final [`SimpleTokenKind::Bogus`] afterwards.

---
