# `regex_syntax::ast`

Crate `regex-syntax` · 41 public items · structured records in [`model/regex_syntax.ast.json`](../model/regex_syntax.ast.json)

## AssertionKind

`enum` · `regex_syntax::ast::AssertionKind`

```rust
enum AssertionKind
```

**Variants**: `StartLine`, `EndLine`, `StartText`, `EndText`, `WordBoundary`, `NotWordBoundary`, `WordBoundaryStart`, `WordBoundaryEnd`, `WordBoundaryStartAngle`, `WordBoundaryEndAngle`, `WordBoundaryStartHalf`, `WordBoundaryEndHalf`

**Implements**: `arbitrary::Arbitrary`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `arbitrary::Arbitrary`**

```rust
fn arbitrary(u: &mut arbitrary::Unstructured<'arbitrary>) -> arbitrary::Result<Self>
fn arbitrary_take_rest(u: arbitrary::Unstructured<'arbitrary>) -> arbitrary::Result<Self>
fn size_hint(depth: usize) -> (usize, ::core::option::Option<usize>)
```

An assertion kind.

---

## Ast

`enum` · `regex_syntax::ast::Ast`

```rust
enum Ast
```

**Variants**: `Empty`, `Flags`, `Literal`, `Dot`, `Assertion`, `ClassUnicode`, `ClassPerl`, `ClassBracketed`, `Repetition`, `Group`, `Alternation`, `Concat`

**Implements**: `arbitrary::Arbitrary`, `core::fmt::Display`, `core::ops::drop::Drop`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (14)

```rust
fn alternation(e: Alternation) -> Ast
fn assertion(e: Assertion) -> Ast
fn class_bracketed(e: ClassBracketed) -> Ast
fn class_perl(e: ClassPerl) -> Ast
fn class_unicode(e: ClassUnicode) -> Ast
fn concat(e: Concat) -> Ast
fn dot(span: Span) -> Ast
fn empty(span: Span) -> Ast
fn flags(e: SetFlags) -> Ast
fn group(e: Group) -> Ast
fn is_empty(&self) -> bool
fn literal(e: Literal) -> Ast
fn repetition(e: Repetition) -> Ast
fn span(&self) -> &Span
```

**via `arbitrary::Arbitrary`**

```rust
fn arbitrary(u: &mut arbitrary::Unstructured<'arbitrary>) -> arbitrary::Result<Self>
fn arbitrary_take_rest(u: arbitrary::Unstructured<'arbitrary>) -> arbitrary::Result<Self>
fn size_hint(depth: usize) -> (usize, ::core::option::Option<usize>)
fn try_size_hint(depth: usize) -> ::core::result::Result<(usize, ::core::option::Option<usize>), arbitrary::MaxRecursionReached>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
```

**via `core::ops::drop::Drop`**

```rust
fn drop(&mut self)
```

An abstract syntax tree for a single regular expression.

An `Ast`'s `fmt::Display` implementation uses constant stack space and heap
space proportional to the size of the `Ast`.

This type defines its own destructor that uses constant stack space and
heap space proportional to the size of the `Ast`.

---

## ClassAsciiKind

`enum` · `regex_syntax::ast::ClassAsciiKind`

```rust
enum ClassAsciiKind
```

**Variants**: `Alnum`, `Alpha`, `Ascii`, `Blank`, `Cntrl`, `Digit`, `Graph`, `Lower`, `Print`, `Punct`, `Space`, `Upper`, `Word`, `Xdigit`

**Implements**: `arbitrary::Arbitrary`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn from_name(name: &str) -> Option<ClassAsciiKind>
```

**via `arbitrary::Arbitrary`**

```rust
fn arbitrary(u: &mut arbitrary::Unstructured<'arbitrary>) -> arbitrary::Result<Self>
fn arbitrary_take_rest(u: arbitrary::Unstructured<'arbitrary>) -> arbitrary::Result<Self>
fn size_hint(depth: usize) -> (usize, ::core::option::Option<usize>)
```

The available ASCII character classes.

---

## ClassPerlKind

`enum` · `regex_syntax::ast::ClassPerlKind`

```rust
enum ClassPerlKind
```

**Variants**: `Digit`, `Space`, `Word`

**Implements**: `arbitrary::Arbitrary`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `arbitrary::Arbitrary`**

```rust
fn arbitrary(u: &mut arbitrary::Unstructured<'arbitrary>) -> arbitrary::Result<Self>
fn arbitrary_take_rest(u: arbitrary::Unstructured<'arbitrary>) -> arbitrary::Result<Self>
fn size_hint(depth: usize) -> (usize, ::core::option::Option<usize>)
```

The available Perl character classes.

---

## ClassSet

`enum` · `regex_syntax::ast::ClassSet`

```rust
enum ClassSet
```

**Variants**: `Item`, `BinaryOp`

**Implements**: `arbitrary::Arbitrary`, `core::ops::drop::Drop`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn span(&self) -> &Span
fn union(ast: ClassSetUnion) -> ClassSet
```

**via `arbitrary::Arbitrary`**

```rust
fn arbitrary(u: &mut arbitrary::Unstructured<'arbitrary>) -> arbitrary::Result<Self>
fn arbitrary_take_rest(u: arbitrary::Unstructured<'arbitrary>) -> arbitrary::Result<Self>
fn size_hint(depth: usize) -> (usize, ::core::option::Option<usize>)
fn try_size_hint(depth: usize) -> ::core::result::Result<(usize, ::core::option::Option<usize>), arbitrary::MaxRecursionReached>
```

**via `core::ops::drop::Drop`**

```rust
fn drop(&mut self)
```

A character class set.

This type corresponds to the internal structure of a bracketed character
class. That is, every bracketed character is one of two types: a union of
items (literals, ranges, other bracketed classes) or a tree of binary set
operations.

---

## ClassSetBinaryOpKind

`enum` · `regex_syntax::ast::ClassSetBinaryOpKind`

```rust
enum ClassSetBinaryOpKind
```

**Variants**: `Intersection`, `Difference`, `SymmetricDifference`

**Implements**: `arbitrary::Arbitrary`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

**via `arbitrary::Arbitrary`**

```rust
fn arbitrary(u: &mut arbitrary::Unstructured<'arbitrary>) -> arbitrary::Result<Self>
fn arbitrary_take_rest(u: arbitrary::Unstructured<'arbitrary>) -> arbitrary::Result<Self>
fn size_hint(depth: usize) -> (usize, ::core::option::Option<usize>)
```

The type of a Unicode character class set operation.

Note that this doesn't explicitly represent union since there is no
explicit union operator. Concatenation inside a character class corresponds
to the union operation.

---

## ClassSetItem

`enum` · `regex_syntax::ast::ClassSetItem`

```rust
enum ClassSetItem
```

**Variants**: `Empty`, `Literal`, `Range`, `Ascii`, `Unicode`, `Perl`, `Bracketed`, `Union`

**Implements**: `arbitrary::Arbitrary`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn span(&self) -> &Span
```

**via `arbitrary::Arbitrary`**

```rust
fn arbitrary(u: &mut arbitrary::Unstructured<'arbitrary>) -> arbitrary::Result<Self>
fn arbitrary_take_rest(u: arbitrary::Unstructured<'arbitrary>) -> arbitrary::Result<Self>
fn size_hint(depth: usize) -> (usize, ::core::option::Option<usize>)
fn try_size_hint(depth: usize) -> ::core::result::Result<(usize, ::core::option::Option<usize>), arbitrary::MaxRecursionReached>
```

A single component of a character class set.

---

## ClassUnicodeKind

`enum` · `regex_syntax::ast::ClassUnicodeKind`

```rust
enum ClassUnicodeKind
```

**Variants**: `OneLetter`, `Named`, `NamedValue`

**Implements**: `arbitrary::Arbitrary`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `arbitrary::Arbitrary`**

```rust
fn arbitrary(u: &mut arbitrary::Unstructured<'_>) -> arbitrary::Result<ClassUnicodeKind>
fn size_hint(depth: usize) -> (usize, Option<usize>)
```

The available forms of Unicode character classes.

---

## ClassUnicodeOpKind

`enum` · `regex_syntax::ast::ClassUnicodeOpKind`

```rust
enum ClassUnicodeOpKind
```

**Variants**: `Equal`, `Colon`, `NotEqual`

**Implements**: `arbitrary::Arbitrary`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn is_equal(&self) -> bool
```

**via `arbitrary::Arbitrary`**

```rust
fn arbitrary(u: &mut arbitrary::Unstructured<'arbitrary>) -> arbitrary::Result<Self>
fn arbitrary_take_rest(u: arbitrary::Unstructured<'arbitrary>) -> arbitrary::Result<Self>
fn size_hint(depth: usize) -> (usize, ::core::option::Option<usize>)
```

The type of op used in a Unicode character class.

---

## ErrorKind

`enum` · `regex_syntax::ast::ErrorKind`

```rust
enum ErrorKind
```

**Variants**: `CaptureLimitExceeded`, `ClassEscapeInvalid`, `ClassRangeInvalid`, `ClassRangeLiteral`, `ClassUnclosed`, `DecimalEmpty`, `DecimalInvalid`, `EscapeHexEmpty`, `EscapeHexInvalid`, `EscapeHexInvalidDigit`, `EscapeUnexpectedEof`, `EscapeUnrecognized`, `FlagDanglingNegation`, `FlagDuplicate`, `FlagRepeatedNegation`, `FlagUnexpectedEof`, `FlagUnrecognized`, `GroupNameDuplicate`, `GroupNameEmpty`, `GroupNameInvalid`, `GroupNameUnexpectedEof`, `GroupUnclosed`, `GroupUnopened`, `NestLimitExceeded`, `RepetitionCountInvalid`, `RepetitionCountDecimalEmpty`, `RepetitionCountUnclosed`, `RepetitionMissing`, `SpecialWordBoundaryUnclosed`, `SpecialWordBoundaryUnrecognized`, `SpecialWordOrRepetitionUnexpectedEof`, `UnicodeClassInvalid`, `UnsupportedBackreference`, `UnsupportedLookAround`

**Implements**: `arbitrary::Arbitrary`, `core::fmt::Display`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `arbitrary::Arbitrary`**

```rust
fn arbitrary(u: &mut arbitrary::Unstructured<'arbitrary>) -> arbitrary::Result<Self>
fn arbitrary_take_rest(u: arbitrary::Unstructured<'arbitrary>) -> arbitrary::Result<Self>
fn size_hint(depth: usize) -> (usize, ::core::option::Option<usize>)
fn try_size_hint(depth: usize) -> ::core::result::Result<(usize, ::core::option::Option<usize>), arbitrary::MaxRecursionReached>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
```

The type of an error that occurred while building an AST.

This error type is marked as `non_exhaustive`. This means that adding a
new variant is not considered a breaking change.

---

## Flag

`enum` · `regex_syntax::ast::Flag`

```rust
enum Flag
```

**Variants**: `CaseInsensitive`, `MultiLine`, `DotMatchesNewLine`, `SwapGreed`, `Unicode`, `CRLF`, `IgnoreWhitespace`

**Implements**: `arbitrary::Arbitrary`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

**via `arbitrary::Arbitrary`**

```rust
fn arbitrary(u: &mut arbitrary::Unstructured<'arbitrary>) -> arbitrary::Result<Self>
fn arbitrary_take_rest(u: arbitrary::Unstructured<'arbitrary>) -> arbitrary::Result<Self>
fn size_hint(depth: usize) -> (usize, ::core::option::Option<usize>)
```

A single flag.

---

## FlagsItemKind

`enum` · `regex_syntax::ast::FlagsItemKind`

```rust
enum FlagsItemKind
```

**Variants**: `Negation`, `Flag`

**Implements**: `arbitrary::Arbitrary`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn is_negation(&self) -> bool
```

**via `arbitrary::Arbitrary`**

```rust
fn arbitrary(u: &mut arbitrary::Unstructured<'arbitrary>) -> arbitrary::Result<Self>
fn arbitrary_take_rest(u: arbitrary::Unstructured<'arbitrary>) -> arbitrary::Result<Self>
fn size_hint(depth: usize) -> (usize, ::core::option::Option<usize>)
fn try_size_hint(depth: usize) -> ::core::result::Result<(usize, ::core::option::Option<usize>), arbitrary::MaxRecursionReached>
```

The kind of an item in a group of flags.

---

## GroupKind

`enum` · `regex_syntax::ast::GroupKind`

```rust
enum GroupKind
```

**Variants**: `CaptureIndex`, `CaptureName`, `NonCapturing`

**Implements**: `arbitrary::Arbitrary`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `arbitrary::Arbitrary`**

```rust
fn arbitrary(u: &mut arbitrary::Unstructured<'arbitrary>) -> arbitrary::Result<Self>
fn arbitrary_take_rest(u: arbitrary::Unstructured<'arbitrary>) -> arbitrary::Result<Self>
fn size_hint(depth: usize) -> (usize, ::core::option::Option<usize>)
fn try_size_hint(depth: usize) -> ::core::result::Result<(usize, ::core::option::Option<usize>), arbitrary::MaxRecursionReached>
```

The kind of a group.

---

## HexLiteralKind

`enum` · `regex_syntax::ast::HexLiteralKind`

```rust
enum HexLiteralKind
```

**Variants**: `X`, `UnicodeShort`, `UnicodeLong`

**Implements**: `arbitrary::Arbitrary`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn digits(&self) -> u32
```

**via `arbitrary::Arbitrary`**

```rust
fn arbitrary(u: &mut arbitrary::Unstructured<'arbitrary>) -> arbitrary::Result<Self>
fn arbitrary_take_rest(u: arbitrary::Unstructured<'arbitrary>) -> arbitrary::Result<Self>
fn size_hint(depth: usize) -> (usize, ::core::option::Option<usize>)
```

The type of a Unicode hex literal.

Note that all variants behave the same when used with brackets. They only
differ when used without brackets in the number of hex digits that must
follow.

---

## LiteralKind

`enum` · `regex_syntax::ast::LiteralKind`

```rust
enum LiteralKind
```

**Variants**: `Verbatim`, `Meta`, `Superfluous`, `Octal`, `HexFixed`, `HexBrace`, `Special`

**Implements**: `arbitrary::Arbitrary`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `arbitrary::Arbitrary`**

```rust
fn arbitrary(u: &mut arbitrary::Unstructured<'arbitrary>) -> arbitrary::Result<Self>
fn arbitrary_take_rest(u: arbitrary::Unstructured<'arbitrary>) -> arbitrary::Result<Self>
fn size_hint(depth: usize) -> (usize, ::core::option::Option<usize>)
fn try_size_hint(depth: usize) -> ::core::result::Result<(usize, ::core::option::Option<usize>), arbitrary::MaxRecursionReached>
```

The kind of a single literal expression.

---

## RepetitionKind

`enum` · `regex_syntax::ast::RepetitionKind`

```rust
enum RepetitionKind
```

**Variants**: `ZeroOrOne`, `ZeroOrMore`, `OneOrMore`, `Range`

**Implements**: `arbitrary::Arbitrary`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `arbitrary::Arbitrary`**

```rust
fn arbitrary(u: &mut arbitrary::Unstructured<'arbitrary>) -> arbitrary::Result<Self>
fn arbitrary_take_rest(u: arbitrary::Unstructured<'arbitrary>) -> arbitrary::Result<Self>
fn size_hint(depth: usize) -> (usize, ::core::option::Option<usize>)
fn try_size_hint(depth: usize) -> ::core::result::Result<(usize, ::core::option::Option<usize>), arbitrary::MaxRecursionReached>
```

The kind of a repetition operator.

---

## RepetitionRange

`enum` · `regex_syntax::ast::RepetitionRange`

```rust
enum RepetitionRange
```

**Variants**: `Exactly`, `AtLeast`, `Bounded`

**Implements**: `arbitrary::Arbitrary`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn is_valid(&self) -> bool
```

**via `arbitrary::Arbitrary`**

```rust
fn arbitrary(u: &mut arbitrary::Unstructured<'arbitrary>) -> arbitrary::Result<Self>
fn arbitrary_take_rest(u: arbitrary::Unstructured<'arbitrary>) -> arbitrary::Result<Self>
fn size_hint(depth: usize) -> (usize, ::core::option::Option<usize>)
fn try_size_hint(depth: usize) -> ::core::result::Result<(usize, ::core::option::Option<usize>), arbitrary::MaxRecursionReached>
```

A range repetition operator.

---

## SpecialLiteralKind

`enum` · `regex_syntax::ast::SpecialLiteralKind`

```rust
enum SpecialLiteralKind
```

**Variants**: `Bell`, `FormFeed`, `Tab`, `LineFeed`, `CarriageReturn`, `VerticalTab`, `Space`

**Implements**: `arbitrary::Arbitrary`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `arbitrary::Arbitrary`**

```rust
fn arbitrary(u: &mut arbitrary::Unstructured<'arbitrary>) -> arbitrary::Result<Self>
fn arbitrary_take_rest(u: arbitrary::Unstructured<'arbitrary>) -> arbitrary::Result<Self>
fn size_hint(depth: usize) -> (usize, ::core::option::Option<usize>)
```

The type of a special literal.

A special literal is a special escape sequence recognized by the regex
parser, e.g., `\f` or `\n`.

---

## Alternation

`struct` · `regex_syntax::ast::Alternation`

```rust
struct Alternation
```

**Fields**: `span`, `asts`

**Implements**: `arbitrary::Arbitrary`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn into_ast(self) -> Ast
```

**via `arbitrary::Arbitrary`**

```rust
fn arbitrary(u: &mut arbitrary::Unstructured<'arbitrary>) -> arbitrary::Result<Self>
fn arbitrary_take_rest(u: arbitrary::Unstructured<'arbitrary>) -> arbitrary::Result<Self>
fn size_hint(depth: usize) -> (usize, ::core::option::Option<usize>)
fn try_size_hint(depth: usize) -> ::core::result::Result<(usize, ::core::option::Option<usize>), arbitrary::MaxRecursionReached>
```

An alternation of regular expressions.

---

## Assertion

`struct` · `regex_syntax::ast::Assertion`

```rust
struct Assertion
```

**Fields**: `span`, `kind`

**Implements**: `arbitrary::Arbitrary`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `arbitrary::Arbitrary`**

```rust
fn arbitrary(u: &mut arbitrary::Unstructured<'arbitrary>) -> arbitrary::Result<Self>
fn arbitrary_take_rest(u: arbitrary::Unstructured<'arbitrary>) -> arbitrary::Result<Self>
fn size_hint(depth: usize) -> (usize, ::core::option::Option<usize>)
fn try_size_hint(depth: usize) -> ::core::result::Result<(usize, ::core::option::Option<usize>), arbitrary::MaxRecursionReached>
```

A single zero-width assertion.

---

## CaptureName

`struct` · `regex_syntax::ast::CaptureName`

```rust
struct CaptureName
```

**Fields**: `span`, `name`, `index`

**Implements**: `arbitrary::Arbitrary`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `arbitrary::Arbitrary`**

```rust
fn arbitrary(u: &mut arbitrary::Unstructured<'_>) -> arbitrary::Result<CaptureName>
fn size_hint(depth: usize) -> (usize, Option<usize>)
```

A capture name.

This corresponds to the name itself between the angle brackets in, e.g.,
`(?P<foo>expr)`.

---

## ClassAscii

`struct` · `regex_syntax::ast::ClassAscii`

```rust
struct ClassAscii
```

**Fields**: `span`, `kind`, `negated`

**Implements**: `arbitrary::Arbitrary`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `arbitrary::Arbitrary`**

```rust
fn arbitrary(u: &mut arbitrary::Unstructured<'arbitrary>) -> arbitrary::Result<Self>
fn arbitrary_take_rest(u: arbitrary::Unstructured<'arbitrary>) -> arbitrary::Result<Self>
fn size_hint(depth: usize) -> (usize, ::core::option::Option<usize>)
fn try_size_hint(depth: usize) -> ::core::result::Result<(usize, ::core::option::Option<usize>), arbitrary::MaxRecursionReached>
```

An ASCII character class.

---

## ClassBracketed

`struct` · `regex_syntax::ast::ClassBracketed`

```rust
struct ClassBracketed
```

**Fields**: `span`, `negated`, `kind`

**Implements**: `arbitrary::Arbitrary`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `arbitrary::Arbitrary`**

```rust
fn arbitrary(u: &mut arbitrary::Unstructured<'arbitrary>) -> arbitrary::Result<Self>
fn arbitrary_take_rest(u: arbitrary::Unstructured<'arbitrary>) -> arbitrary::Result<Self>
fn size_hint(depth: usize) -> (usize, ::core::option::Option<usize>)
fn try_size_hint(depth: usize) -> ::core::result::Result<(usize, ::core::option::Option<usize>), arbitrary::MaxRecursionReached>
```

A bracketed character class, e.g., `[a-z0-9]`.

---

## ClassPerl

`struct` · `regex_syntax::ast::ClassPerl`

```rust
struct ClassPerl
```

**Fields**: `span`, `kind`, `negated`

**Implements**: `arbitrary::Arbitrary`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `arbitrary::Arbitrary`**

```rust
fn arbitrary(u: &mut arbitrary::Unstructured<'arbitrary>) -> arbitrary::Result<Self>
fn arbitrary_take_rest(u: arbitrary::Unstructured<'arbitrary>) -> arbitrary::Result<Self>
fn size_hint(depth: usize) -> (usize, ::core::option::Option<usize>)
fn try_size_hint(depth: usize) -> ::core::result::Result<(usize, ::core::option::Option<usize>), arbitrary::MaxRecursionReached>
```

A Perl character class.

---

## ClassSetBinaryOp

`struct` · `regex_syntax::ast::ClassSetBinaryOp`

```rust
struct ClassSetBinaryOp
```

**Fields**: `span`, `kind`, `lhs`, `rhs`

**Implements**: `arbitrary::Arbitrary`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `arbitrary::Arbitrary`**

```rust
fn arbitrary(u: &mut arbitrary::Unstructured<'arbitrary>) -> arbitrary::Result<Self>
fn arbitrary_take_rest(u: arbitrary::Unstructured<'arbitrary>) -> arbitrary::Result<Self>
fn size_hint(depth: usize) -> (usize, ::core::option::Option<usize>)
fn try_size_hint(depth: usize) -> ::core::result::Result<(usize, ::core::option::Option<usize>), arbitrary::MaxRecursionReached>
```

A Unicode character class set operation.

---

## ClassSetRange

`struct` · `regex_syntax::ast::ClassSetRange`

```rust
struct ClassSetRange
```

**Fields**: `span`, `start`, `end`

**Implements**: `arbitrary::Arbitrary`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn is_valid(&self) -> bool
```

**via `arbitrary::Arbitrary`**

```rust
fn arbitrary(u: &mut arbitrary::Unstructured<'arbitrary>) -> arbitrary::Result<Self>
fn arbitrary_take_rest(u: arbitrary::Unstructured<'arbitrary>) -> arbitrary::Result<Self>
fn size_hint(depth: usize) -> (usize, ::core::option::Option<usize>)
fn try_size_hint(depth: usize) -> ::core::result::Result<(usize, ::core::option::Option<usize>), arbitrary::MaxRecursionReached>
```

A single character class range in a set.

---

## ClassSetUnion

`struct` · `regex_syntax::ast::ClassSetUnion`

```rust
struct ClassSetUnion
```

**Fields**: `span`, `items`

**Implements**: `arbitrary::Arbitrary`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn into_item(self) -> ClassSetItem
fn push(&mut self, item: ClassSetItem)
```

**via `arbitrary::Arbitrary`**

```rust
fn arbitrary(u: &mut arbitrary::Unstructured<'arbitrary>) -> arbitrary::Result<Self>
fn arbitrary_take_rest(u: arbitrary::Unstructured<'arbitrary>) -> arbitrary::Result<Self>
fn size_hint(depth: usize) -> (usize, ::core::option::Option<usize>)
fn try_size_hint(depth: usize) -> ::core::result::Result<(usize, ::core::option::Option<usize>), arbitrary::MaxRecursionReached>
```

A union of items inside a character class set.

---

## ClassUnicode

`struct` · `regex_syntax::ast::ClassUnicode`

```rust
struct ClassUnicode
```

**Fields**: `span`, `negated`, `kind`

**Implements**: `arbitrary::Arbitrary`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn is_negated(&self) -> bool
```

**via `arbitrary::Arbitrary`**

```rust
fn arbitrary(u: &mut arbitrary::Unstructured<'arbitrary>) -> arbitrary::Result<Self>
fn arbitrary_take_rest(u: arbitrary::Unstructured<'arbitrary>) -> arbitrary::Result<Self>
fn size_hint(depth: usize) -> (usize, ::core::option::Option<usize>)
fn try_size_hint(depth: usize) -> ::core::result::Result<(usize, ::core::option::Option<usize>), arbitrary::MaxRecursionReached>
```

A Unicode character class.

---

## Comment

`struct` · `regex_syntax::ast::Comment`

```rust
struct Comment
```

**Fields**: `span`, `comment`

**Implements**: `arbitrary::Arbitrary`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `arbitrary::Arbitrary`**

```rust
fn arbitrary(u: &mut arbitrary::Unstructured<'arbitrary>) -> arbitrary::Result<Self>
fn arbitrary_take_rest(u: arbitrary::Unstructured<'arbitrary>) -> arbitrary::Result<Self>
fn size_hint(depth: usize) -> (usize, ::core::option::Option<usize>)
fn try_size_hint(depth: usize) -> ::core::result::Result<(usize, ::core::option::Option<usize>), arbitrary::MaxRecursionReached>
```

A comment from a regular expression with an associated span.

A regular expression can only contain comments when the `x` flag is
enabled.

---

## Concat

`struct` · `regex_syntax::ast::Concat`

```rust
struct Concat
```

**Fields**: `span`, `asts`

**Implements**: `arbitrary::Arbitrary`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn into_ast(self) -> Ast
```

**via `arbitrary::Arbitrary`**

```rust
fn arbitrary(u: &mut arbitrary::Unstructured<'arbitrary>) -> arbitrary::Result<Self>
fn arbitrary_take_rest(u: arbitrary::Unstructured<'arbitrary>) -> arbitrary::Result<Self>
fn size_hint(depth: usize) -> (usize, ::core::option::Option<usize>)
fn try_size_hint(depth: usize) -> ::core::result::Result<(usize, ::core::option::Option<usize>), arbitrary::MaxRecursionReached>
```

A concatenation of regular expressions.

---

## Error

`struct` · `regex_syntax::ast::Error`

```rust
struct Error
```

**Implements**: `arbitrary::Arbitrary`, `core::error::Error`, `core::fmt::Display`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (4)

```rust
fn auxiliary_span(&self) -> Option<&Span>
fn kind(&self) -> &ErrorKind
fn pattern(&self) -> &str
fn span(&self) -> &Span
```

**via `arbitrary::Arbitrary`**

```rust
fn arbitrary(u: &mut arbitrary::Unstructured<'arbitrary>) -> arbitrary::Result<Self>
fn arbitrary_take_rest(u: arbitrary::Unstructured<'arbitrary>) -> arbitrary::Result<Self>
fn size_hint(depth: usize) -> (usize, ::core::option::Option<usize>)
fn try_size_hint(depth: usize) -> ::core::result::Result<(usize, ::core::option::Option<usize>), arbitrary::MaxRecursionReached>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
```

An error that occurred while parsing a regular expression into an abstract
syntax tree.

Note that not all ASTs represents a valid regular expression. For example,
an AST is constructed without error for `\p{Quux}`, but `Quux` is not a
valid Unicode property name. That particular error is reported when
translating an AST to the high-level intermediate representation (`HIR`).

---

## Flags

`struct` · `regex_syntax::ast::Flags`

```rust
struct Flags
```

**Fields**: `span`, `items`

**Implements**: `arbitrary::Arbitrary`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn add_item(&mut self, item: FlagsItem) -> Option<usize>
fn flag_state(&self, flag: Flag) -> Option<bool>
```

**via `arbitrary::Arbitrary`**

```rust
fn arbitrary(u: &mut arbitrary::Unstructured<'arbitrary>) -> arbitrary::Result<Self>
fn arbitrary_take_rest(u: arbitrary::Unstructured<'arbitrary>) -> arbitrary::Result<Self>
fn size_hint(depth: usize) -> (usize, ::core::option::Option<usize>)
fn try_size_hint(depth: usize) -> ::core::result::Result<(usize, ::core::option::Option<usize>), arbitrary::MaxRecursionReached>
```

A group of flags.

This corresponds only to the sequence of flags themselves, e.g., `is-u`.

---

## FlagsItem

`struct` · `regex_syntax::ast::FlagsItem`

```rust
struct FlagsItem
```

**Fields**: `span`, `kind`

**Implements**: `arbitrary::Arbitrary`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `arbitrary::Arbitrary`**

```rust
fn arbitrary(u: &mut arbitrary::Unstructured<'arbitrary>) -> arbitrary::Result<Self>
fn arbitrary_take_rest(u: arbitrary::Unstructured<'arbitrary>) -> arbitrary::Result<Self>
fn size_hint(depth: usize) -> (usize, ::core::option::Option<usize>)
fn try_size_hint(depth: usize) -> ::core::result::Result<(usize, ::core::option::Option<usize>), arbitrary::MaxRecursionReached>
```

A single item in a group of flags.

---

## Group

`struct` · `regex_syntax::ast::Group`

```rust
struct Group
```

**Fields**: `span`, `kind`, `ast`

**Implements**: `arbitrary::Arbitrary`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn capture_index(&self) -> Option<u32>
fn flags(&self) -> Option<&Flags>
fn is_capturing(&self) -> bool
```

**via `arbitrary::Arbitrary`**

```rust
fn arbitrary(u: &mut arbitrary::Unstructured<'arbitrary>) -> arbitrary::Result<Self>
fn arbitrary_take_rest(u: arbitrary::Unstructured<'arbitrary>) -> arbitrary::Result<Self>
fn size_hint(depth: usize) -> (usize, ::core::option::Option<usize>)
fn try_size_hint(depth: usize) -> ::core::result::Result<(usize, ::core::option::Option<usize>), arbitrary::MaxRecursionReached>
```

A grouped regular expression.

This includes both capturing and non-capturing groups. This does **not**
include flag-only groups like `(?is)`, but does contain any group that
contains a sub-expression, e.g., `(a)`, `(?P<name>a)`, `(?:a)` and
`(?is:a)`.

---

## Literal

`struct` · `regex_syntax::ast::Literal`

```rust
struct Literal
```

**Fields**: `span`, `kind`, `c`

**Implements**: `arbitrary::Arbitrary`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn byte(&self) -> Option<u8>
```

**via `arbitrary::Arbitrary`**

```rust
fn arbitrary(u: &mut arbitrary::Unstructured<'arbitrary>) -> arbitrary::Result<Self>
fn arbitrary_take_rest(u: arbitrary::Unstructured<'arbitrary>) -> arbitrary::Result<Self>
fn size_hint(depth: usize) -> (usize, ::core::option::Option<usize>)
fn try_size_hint(depth: usize) -> ::core::result::Result<(usize, ::core::option::Option<usize>), arbitrary::MaxRecursionReached>
```

A single literal expression.

A literal corresponds to a single Unicode scalar value. Literals may be
represented in their literal form, e.g., `a` or in their escaped form,
e.g., `\x61`.

---

## Position

`struct` · `regex_syntax::ast::Position`

```rust
struct Position
```

**Fields**: `offset`, `line`, `column`

**Implements**: `arbitrary::Arbitrary`

**Derives**: Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (1)

```rust
fn new(offset: usize, line: usize, column: usize) -> Position
```

**via `arbitrary::Arbitrary`**

```rust
fn arbitrary(u: &mut arbitrary::Unstructured<'arbitrary>) -> arbitrary::Result<Self>
fn arbitrary_take_rest(u: arbitrary::Unstructured<'arbitrary>) -> arbitrary::Result<Self>
fn size_hint(depth: usize) -> (usize, ::core::option::Option<usize>)
fn try_size_hint(depth: usize) -> ::core::result::Result<(usize, ::core::option::Option<usize>), arbitrary::MaxRecursionReached>
```

A single position in a regular expression.

A position encodes one half of a span, and include the byte offset, line
number and column number.

---

## Repetition

`struct` · `regex_syntax::ast::Repetition`

```rust
struct Repetition
```

**Fields**: `span`, `op`, `greedy`, `ast`

**Implements**: `arbitrary::Arbitrary`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `arbitrary::Arbitrary`**

```rust
fn arbitrary(u: &mut arbitrary::Unstructured<'arbitrary>) -> arbitrary::Result<Self>
fn arbitrary_take_rest(u: arbitrary::Unstructured<'arbitrary>) -> arbitrary::Result<Self>
fn size_hint(depth: usize) -> (usize, ::core::option::Option<usize>)
fn try_size_hint(depth: usize) -> ::core::result::Result<(usize, ::core::option::Option<usize>), arbitrary::MaxRecursionReached>
```

A repetition operation applied to a regular expression.

---

## RepetitionOp

`struct` · `regex_syntax::ast::RepetitionOp`

```rust
struct RepetitionOp
```

**Fields**: `span`, `kind`

**Implements**: `arbitrary::Arbitrary`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `arbitrary::Arbitrary`**

```rust
fn arbitrary(u: &mut arbitrary::Unstructured<'arbitrary>) -> arbitrary::Result<Self>
fn arbitrary_take_rest(u: arbitrary::Unstructured<'arbitrary>) -> arbitrary::Result<Self>
fn size_hint(depth: usize) -> (usize, ::core::option::Option<usize>)
fn try_size_hint(depth: usize) -> ::core::result::Result<(usize, ::core::option::Option<usize>), arbitrary::MaxRecursionReached>
```

The repetition operator itself.

---

## SetFlags

`struct` · `regex_syntax::ast::SetFlags`

```rust
struct SetFlags
```

**Fields**: `span`, `flags`

**Implements**: `arbitrary::Arbitrary`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `arbitrary::Arbitrary`**

```rust
fn arbitrary(u: &mut arbitrary::Unstructured<'arbitrary>) -> arbitrary::Result<Self>
fn arbitrary_take_rest(u: arbitrary::Unstructured<'arbitrary>) -> arbitrary::Result<Self>
fn size_hint(depth: usize) -> (usize, ::core::option::Option<usize>)
fn try_size_hint(depth: usize) -> ::core::result::Result<(usize, ::core::option::Option<usize>), arbitrary::MaxRecursionReached>
```

A group of flags that is not applied to a particular regular expression.

---

## Span

`struct` · `regex_syntax::ast::Span`

```rust
struct Span
```

**Fields**: `start`, `end`

**Implements**: `arbitrary::Arbitrary`

**Derives**: Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (6)

```rust
fn is_empty(&self) -> bool
fn is_one_line(&self) -> bool
fn new(start: Position, end: Position) -> Span
fn splat(pos: Position) -> Span
fn with_end(self, pos: Position) -> Span
fn with_start(self, pos: Position) -> Span
```

**via `arbitrary::Arbitrary`**

```rust
fn arbitrary(u: &mut arbitrary::Unstructured<'arbitrary>) -> arbitrary::Result<Self>
fn arbitrary_take_rest(u: arbitrary::Unstructured<'arbitrary>) -> arbitrary::Result<Self>
fn size_hint(depth: usize) -> (usize, ::core::option::Option<usize>)
fn try_size_hint(depth: usize) -> ::core::result::Result<(usize, ::core::option::Option<usize>), arbitrary::MaxRecursionReached>
```

Span represents the position information of a single AST item.

All span positions are absolute byte offsets that can be used on the
original regular expression that was parsed.

---

## WithComments

`struct` · `regex_syntax::ast::WithComments`

```rust
struct WithComments
```

**Fields**: `ast`, `comments`

**Implements**: `arbitrary::Arbitrary`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `arbitrary::Arbitrary`**

```rust
fn arbitrary(u: &mut arbitrary::Unstructured<'arbitrary>) -> arbitrary::Result<Self>
fn arbitrary_take_rest(u: arbitrary::Unstructured<'arbitrary>) -> arbitrary::Result<Self>
fn size_hint(depth: usize) -> (usize, ::core::option::Option<usize>)
fn try_size_hint(depth: usize) -> ::core::result::Result<(usize, ::core::option::Option<usize>), arbitrary::MaxRecursionReached>
```

An abstract syntax tree for a singular expression along with comments
found.

Comments are not stored in the tree itself to avoid complexity. Each
comment contains a span of precisely where it occurred in the original
regular expression.

---
