# `regex_syntax::hir`

Crate `regex-syntax` · 19 public items · structured records in [`model/regex_syntax.hir.json`](../model/regex_syntax.hir.json)

## Class

`enum` · `regex_syntax::hir::Class`

```rust
enum Class
```

**Variants**: `Unicode`, `Bytes`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (8)

```rust
fn case_fold_simple(&mut self)
fn is_empty(&self) -> bool
fn is_utf8(&self) -> bool
fn literal(&self) -> Option<Vec<u8>>
fn maximum_len(&self) -> Option<usize>
fn minimum_len(&self) -> Option<usize>
fn negate(&mut self)
fn try_case_fold_simple(&mut self) -> core::result::Result<(), CaseFoldError>
```

The high-level intermediate representation of a character class.

A character class corresponds to a set of characters. A character is either
defined by a Unicode scalar value or a byte.

A character class, regardless of its character type, is represented by a
sequence of non-overlapping non-adjacent ranges of characters.

There are no guarantees about which class variant is used. Generally
speaking, the Unicode variant is used whenever a class needs to contain
non-ASCII Unicode scalar values. But the Unicode variant can be used even
when Unicode mode is disabled. For example, at the time of writing, the
regex `(?-u:a|\xc2\xa0)` will compile down to HIR for the Unicode class
`[a\u00A0]` due to optimizations.

Note that `Bytes` variant may be produced even when it exclusively matches
valid UTF-8. This is because a `Bytes` variant represents an intention by
the author of the regular expression to disable Unicode mode, which in turn
impacts the semantics of case insensitive matching. For example, `(?i)k`
and `(?i-u)k` will not match the same set of strings.

---

## Dot

`enum` · `regex_syntax::hir::Dot`

```rust
enum Dot
```

**Variants**: `AnyChar`, `AnyByte`, `AnyCharExcept`, `AnyCharExceptLF`, `AnyCharExceptCRLF`, `AnyByteExcept`, `AnyByteExceptLF`, `AnyByteExceptCRLF`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

A type describing the different flavors of `.`.

This type is meant to be used with [`Hir::dot`], which is a convenience
routine for building HIR values derived from the `.` regex.

---

## ErrorKind

`enum` · `regex_syntax::hir::ErrorKind`

```rust
enum ErrorKind
```

**Variants**: `UnicodeNotAllowed`, `InvalidUtf8`, `InvalidLineTerminator`, `UnicodePropertyNotFound`, `UnicodePropertyValueNotFound`, `UnicodePerlClassNotFound`, `UnicodeCaseUnavailable`

**Implements**: `core::fmt::Display`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
```

The type of an error that occurred while building an `Hir`.

This error type is marked as `non_exhaustive`. This means that adding a
new variant is not considered a breaking change.

---

## HirKind

`enum` · `regex_syntax::hir::HirKind`

```rust
enum HirKind
```

**Variants**: `Empty`, `Literal`, `Class`, `Look`, `Repetition`, `Capture`, `Concat`, `Alternation`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn subs(&self) -> &[Hir]
```

The underlying kind of an arbitrary [`Hir`] expression.

An `HirKind` is principally useful for doing case analysis on the type
of a regular expression. If you're looking to build new `Hir` values,
then you _must_ use the smart constructors defined on `Hir`, like
[`Hir::repetition`], to build new `Hir` values. The API intentionally does
not expose any way of building an `Hir` directly from an `HirKind`.

---

## Look

`enum` · `regex_syntax::hir::Look`

```rust
enum Look
```

**Variants**: `Start`, `End`, `StartLF`, `EndLF`, `StartCRLF`, `EndCRLF`, `WordAscii`, `WordAsciiNegate`, `WordUnicode`, `WordUnicodeNegate`, `WordStartAscii`, `WordEndAscii`, `WordStartUnicode`, `WordEndUnicode`, `WordStartHalfAscii`, `WordEndHalfAscii`, `WordStartHalfUnicode`, `WordEndHalfUnicode`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (4)

```rust
const fn as_char(self) -> char
const fn as_repr(self) -> u32
const fn from_repr(repr: u32) -> Option<Look>
const fn reversed(self) -> Look
```

The high-level intermediate representation for a look-around assertion.

An assertion match is always zero-length. Also called an "empty match."

---

## Capture

`struct` · `regex_syntax::hir::Capture`

```rust
struct Capture
```

**Fields**: `index`, `name`, `sub`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

The high-level intermediate representation for a capturing group.

A capturing group always has an index and a child expression. It may
also have a name associated with it (e.g., `(?P<foo>\w)`), but it's not
necessary.

Note that there is no explicit representation of a non-capturing group
in a `Hir`. Instead, non-capturing grouping is handled automatically by
the recursive structure of the `Hir` itself.

---

## ClassBytes

`struct` · `regex_syntax::hir::ClassBytes`

```rust
struct ClassBytes
```

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (16)

```rust
fn case_fold_simple(&mut self)
fn difference(&mut self, other: &ClassBytes)
fn empty() -> ClassBytes
fn intersect(&mut self, other: &ClassBytes)
fn is_ascii(&self) -> bool
fn iter(&self) -> ClassBytesIter<'_>
fn literal(&self) -> Option<Vec<u8>>
fn maximum_len(&self) -> Option<usize>
fn minimum_len(&self) -> Option<usize>
fn negate(&mut self)
fn new<I>(ranges: I) -> ClassBytes where I: IntoIterator<Item = ClassBytesRange>
fn push(&mut self, range: ClassBytesRange)
fn ranges(&self) -> &[ClassBytesRange]
fn symmetric_difference(&mut self, other: &ClassBytes)
fn to_unicode_class(&self) -> Option<ClassUnicode>
fn union(&mut self, other: &ClassBytes)
```

A set of characters represented by arbitrary bytes.

Each byte corresponds to one character.

---

## ClassBytesIter

`struct` · `regex_syntax::hir::ClassBytesIter`

```rust
struct ClassBytesIter<'a>
```

**Implements**: `core::iter::traits::iterator::Iterator`

**Derives**: Debug

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<&'a ClassBytesRange>
```

An iterator over all ranges in a byte character class.

The lifetime `'a` refers to the lifetime of the underlying class.

---

## ClassBytesRange

`struct` · `regex_syntax::hir::ClassBytesRange`

```rust
struct ClassBytesRange
```

**Derives**: Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (4)

```rust
fn end(&self) -> u8
fn len(&self) -> usize
fn new(start: u8, end: u8) -> ClassBytesRange
fn start(&self) -> u8
```

A single range of characters represented by arbitrary bytes.

The range is closed. That is, the start and end of the range are included
in the range.

---

## ClassUnicode

`struct` · `regex_syntax::hir::ClassUnicode`

```rust
struct ClassUnicode
```

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (17)

```rust
fn case_fold_simple(&mut self)
fn difference(&mut self, other: &ClassUnicode)
fn empty() -> ClassUnicode
fn intersect(&mut self, other: &ClassUnicode)
fn is_ascii(&self) -> bool
fn iter(&self) -> ClassUnicodeIter<'_>
fn literal(&self) -> Option<Vec<u8>>
fn maximum_len(&self) -> Option<usize>
fn minimum_len(&self) -> Option<usize>
fn negate(&mut self)
fn new<I>(ranges: I) -> ClassUnicode where I: IntoIterator<Item = ClassUnicodeRange>
fn push(&mut self, range: ClassUnicodeRange)
fn ranges(&self) -> &[ClassUnicodeRange]
fn symmetric_difference(&mut self, other: &ClassUnicode)
fn to_byte_class(&self) -> Option<ClassBytes>
fn try_case_fold_simple(&mut self) -> core::result::Result<(), CaseFoldError>
fn union(&mut self, other: &ClassUnicode)
```

A set of characters represented by Unicode scalar values.

---

## ClassUnicodeIter

`struct` · `regex_syntax::hir::ClassUnicodeIter`

```rust
struct ClassUnicodeIter<'a>
```

**Implements**: `core::iter::traits::iterator::Iterator`

**Derives**: Debug

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<&'a ClassUnicodeRange>
```

An iterator over all ranges in a Unicode character class.

The lifetime `'a` refers to the lifetime of the underlying class.

---

## ClassUnicodeRange

`struct` · `regex_syntax::hir::ClassUnicodeRange`

```rust
struct ClassUnicodeRange
```

**Derives**: Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (4)

```rust
fn end(&self) -> char
fn len(&self) -> usize
fn new(start: char, end: char) -> ClassUnicodeRange
fn start(&self) -> char
```

A single range of characters represented by Unicode scalar values.

The range is closed. That is, the start and end of the range are included
in the range.

---

## Error

`struct` · `regex_syntax::hir::Error`

```rust
struct Error
```

**Implements**: `core::error::Error`, `core::fmt::Display`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn kind(&self) -> &ErrorKind
fn pattern(&self) -> &str
fn span(&self) -> &Span
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
```

An error that can occur while translating an `Ast` to a `Hir`.

---

## Hir

`struct` · `regex_syntax::hir::Hir`

```rust
struct Hir
```

**Implements**: `core::fmt::Display`, `core::ops::drop::Drop`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (13)

```rust
fn alternation(subs: Vec<Hir>) -> Hir
fn capture(capture: Capture) -> Hir
fn class(class: Class) -> Hir
fn concat(subs: Vec<Hir>) -> Hir
fn dot(dot: Dot) -> Hir
fn empty() -> Hir
fn fail() -> Hir
fn into_kind(self) -> HirKind
fn kind(&self) -> &HirKind
fn literal<B: Into<Box<[u8]>>>(lit: B) -> Hir
fn look(look: Look) -> Hir
fn properties(&self) -> &Properties
fn repetition(rep: Repetition) -> Hir
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
```

**via `core::ops::drop::Drop`**

```rust
fn drop(&mut self)
```

A high-level intermediate representation (HIR) for a regular expression.

An HIR value is a combination of a [`HirKind`] and a set of [`Properties`].
An `HirKind` indicates what kind of regular expression it is (a literal,
a repetition, a look-around assertion, etc.), where as a `Properties`
describes various facts about the regular expression. For example, whether
it matches UTF-8 or if it matches the empty string.

The HIR of a regular expression represents an intermediate step between
its abstract syntax (a structured description of the concrete syntax) and
an actual regex matcher. The purpose of HIR is to make regular expressions
easier to analyze. In particular, the AST is much more complex than the
HIR. For example, while an AST supports arbitrarily nested character
classes, the HIR will flatten all nested classes into a single set. The HIR
will also "compile away" every flag present in the concrete syntax. For
example, users of HIR expressions never need to worry about case folding;
it is handled automatically by the translator (e.g., by translating
`(?i:A)` to `[aA]`).

The specific type of an HIR expression can be accessed via its `kind`
or `into_kind` methods. This extra level of indirection exists for two
reasons:

1. Construction of an HIR expression *must* use the constructor methods on
this `Hir` type instead of building the `HirKind` values directly. This
permits construction to enforce invariants like "concatenations always
consist of two or more sub-expressions."
2. Every HIR expression contains attributes that are defined inductively,
and can be computed cheaply during the construction process. For example,
one such attribute is whether the expression must match at the beginning of
the haystack.

In particular, if you have an `HirKind` value, then there is intentionally
no way to build an `Hir` value from it. You instead need to do case
analysis on the `HirKind` value and build the `Hir` value using its smart
constructors.

# UTF-8

If the HIR was produced by a translator with
[`TranslatorBuilder::utf8`](translate::TranslatorBuilder::utf8) enabled,
then the HIR is guaranteed to match UTF-8 exclusively for all non-empty
matches.

For empty matches, those can occur at any position. It is the
responsibility of the regex engine to determine whether empty matches are
permitted between the code units of a single codepoint.

# Stack space

This type defines its own destructor that uses constant stack space and
heap space proportional to the size of the HIR.

Also, an `Hir`'s `fmt::Display` implementation prints an HIR as a regular
expression pattern string, and uses constant stack space and heap space
proportional to the size of the `Hir`. The regex it prints is guaranteed to
be _semantically_ equivalent to the original concrete syntax, but it may
look very different. (And potentially not practically readable by a human.)

An `Hir`'s `fmt::Debug` implementation currently does not use constant
stack space. The implementation will also suppress some details (such as
the `Properties` inlined into every `Hir` value to make it less noisy).

---

## Literal

`struct` · `regex_syntax::hir::Literal`

```rust
struct Literal
```

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

The high-level intermediate representation of a literal.

A literal corresponds to `0` or more bytes that should be matched
literally. The smart constructors defined on `Hir` will automatically
concatenate adjacent literals into one literal, and will even automatically
replace empty literals with `Hir::empty()`.

Note that despite a literal being represented by a sequence of bytes, its
`Debug` implementation will attempt to print it as a normal string. (That
is, not a sequence of decimal numbers.)

---

## LookSet

`struct` · `regex_syntax::hir::LookSet`

```rust
struct LookSet
```

**Fields**: `bits`

**Derives**: Clone, Copy, Debug, Default, Eq, PartialEq, StructuralPartialEq

**Methods** (27)

```rust
fn contains(self, look: Look) -> bool
fn contains_anchor(&self) -> bool
fn contains_anchor_crlf(&self) -> bool
fn contains_anchor_haystack(&self) -> bool
fn contains_anchor_lf(&self) -> bool
fn contains_anchor_line(&self) -> bool
fn contains_word(self) -> bool
fn contains_word_ascii(self) -> bool
fn contains_word_unicode(self) -> bool
fn empty() -> LookSet
fn full() -> LookSet
fn insert(self, look: Look) -> LookSet
fn intersect(self, other: LookSet) -> LookSet
fn is_empty(self) -> bool
fn iter(self) -> LookSetIter
fn len(self) -> usize
fn read_repr(slice: &[u8]) -> LookSet
fn remove(self, look: Look) -> LookSet
fn set_insert(&mut self, look: Look)
fn set_intersect(&mut self, other: LookSet)
fn set_remove(&mut self, look: Look)
fn set_subtract(&mut self, other: LookSet)
fn set_union(&mut self, other: LookSet)
fn singleton(look: Look) -> LookSet
fn subtract(self, other: LookSet) -> LookSet
fn union(self, other: LookSet) -> LookSet
fn write_repr(self, slice: &mut [u8])
```

A set of look-around assertions.

This is useful for efficiently tracking look-around assertions. For
example, an [`Hir`] provides properties that return `LookSet`s.

---

## LookSetIter

`struct` · `regex_syntax::hir::LookSetIter`

```rust
struct LookSetIter
```

**Implements**: `core::iter::traits::iterator::Iterator`

**Derives**: Clone, Debug

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<Look>
```

An iterator over all look-around assertions in a [`LookSet`].

This iterator is created by [`LookSet::iter`].

---

## Properties

`struct` · `regex_syntax::hir::Properties`

```rust
struct Properties
```

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (14)

```rust
fn explicit_captures_len(&self) -> usize
fn is_alternation_literal(&self) -> bool
fn is_literal(&self) -> bool
fn is_utf8(&self) -> bool
fn look_set(&self) -> LookSet
fn look_set_prefix(&self) -> LookSet
fn look_set_prefix_any(&self) -> LookSet
fn look_set_suffix(&self) -> LookSet
fn look_set_suffix_any(&self) -> LookSet
fn maximum_len(&self) -> Option<usize>
fn memory_usage(&self) -> usize
fn minimum_len(&self) -> Option<usize>
fn static_explicit_captures_len(&self) -> Option<usize>
fn union<I, P>(props: I) -> Properties where I: IntoIterator<Item = P>, P: core::borrow::Borrow<Properties>
```

A type that collects various properties of an HIR value.

Properties are always scalar values and represent meta data that is
computed inductively on an HIR value. Properties are defined for all
HIR values.

All methods on a `Properties` value take constant time and are meant to
be cheap to call.

---

## Repetition

`struct` · `regex_syntax::hir::Repetition`

```rust
struct Repetition
```

**Fields**: `min`, `max`, `greedy`, `sub`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn with(&self, sub: Hir) -> Repetition
```

The high-level intermediate representation of a repetition operator.

A repetition operator permits the repetition of an arbitrary
sub-expression.

---
