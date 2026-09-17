# `regex_syntax::hir::literal`

Crate `regex-syntax` · 5 public items · structured records in [`model/regex_syntax.hir.literal.json`](../model/regex_syntax.hir.literal.json)

## ExtractKind

`enum` · `regex_syntax::hir::literal::ExtractKind`

```rust
enum ExtractKind
```

**Variants**: `Prefix`, `Suffix`

**Derives**: Clone, Debug, Default

**Methods** (2)

```rust
fn is_prefix(&self) -> bool
fn is_suffix(&self) -> bool
```

The kind of literals to extract from an [`Hir`] expression.

The default extraction kind is `Prefix`.

---

## rank

`function` · `regex_syntax::hir::literal::rank`

```rust
fn rank(byte: u8) -> u8
```

Returns the "rank" of the given byte.

The minimum rank value is `0` and the maximum rank value is `255`.

The rank of a byte is derived from a heuristic background distribution of
relative frequencies of bytes. The heuristic says that lower the rank of a
byte, the less likely that byte is to appear in any arbitrary haystack.

---

## Extractor

`struct` · `regex_syntax::hir::literal::Extractor`

```rust
struct Extractor
```

**Derives**: Clone, Debug, Default

**Methods** (7)

```rust
fn extract(&self, hir: &Hir) -> Seq
fn kind(&mut self, kind: ExtractKind) -> &mut Extractor
fn limit_class(&mut self, limit: usize) -> &mut Extractor
fn limit_literal_len(&mut self, limit: usize) -> &mut Extractor
fn limit_repeat(&mut self, limit: usize) -> &mut Extractor
fn limit_total(&mut self, limit: usize) -> &mut Extractor
fn new() -> Extractor
```

Extracts prefix or suffix literal sequences from [`Hir`] expressions.

Literal extraction is based on the following observations:

* Many regexes start with one or a small number of literals.
* Substring search for literals is often much faster (sometimes by an order
of magnitude) than a regex search.

Thus, in many cases, one can search for literals to find candidate starting
locations of a match, and then only run the full regex engine at each such
location instead of over the full haystack.

The main downside of literal extraction is that it can wind up causing a
search to be slower overall. For example, if there are many matches or if
there are many candidates that don't ultimately lead to a match, then a
lot of overhead will be spent in shuffling back-and-forth between substring
search and the regex engine. This is the fundamental reason why literal
optimizations for regex patterns is sometimes considered a "black art."

# Look-around assertions

Literal extraction treats all look-around assertions as-if they match every
empty string. So for example, the regex `\bquux\b` will yield a sequence
containing a single exact literal `quux`. However, not all occurrences
of `quux` correspond to a match a of the regex. For example, `\bquux\b`
does not match `ZquuxZ` anywhere because `quux` does not fall on a word
boundary.

In effect, if your regex contains look-around assertions, then a match of
an exact literal does not necessarily mean the regex overall matches. So
you may still need to run the regex engine in such cases to confirm the
match.

The precise guarantee you get from a literal sequence is: if every literal
in the sequence is exact and the original regex contains zero look-around
assertions, then a preference-order multi-substring search of those
literals will precisely match a preference-order search of the original
regex.

# Example

This shows how to extract prefixes:

```
use regex_syntax::{hir::literal::{Extractor, Literal, Seq}, parse};

let hir = parse(r"(a|b|c)(x|y|z)[A-Z]+foo")?;
let got = Extractor::new().extract(&hir);
// All literals returned are "inexact" because none of them reach the
// match state.
let expected = Seq::from_iter([
    Literal::inexact("ax"),
    Literal::inexact("ay"),
    Literal::inexact("az"),
    Literal::inexact("bx"),
    Literal::inexact("by"),
    Literal::inexact("bz"),
    Literal::inexact("cx"),
    Literal::inexact("cy"),
    Literal::inexact("cz"),
]);
assert_eq!(expected, got);

# Ok::<(), Box<dyn std::error::Error>>(())
```

This shows how to extract suffixes:

```
use regex_syntax::{
    hir::literal::{Extractor, ExtractKind, Literal, Seq},
    parse,
};

let hir = parse(r"foo|[A-Z]+bar")?;
let got = Extractor::new().kind(ExtractKind::Suffix).extract(&hir);
// Since 'foo' gets to a match state, it is considered exact. But 'bar'
// does not because of the '[A-Z]+', and thus is marked inexact.
let expected = Seq::from_iter([
    Literal::exact("foo"),
    Literal::inexact("bar"),
]);
assert_eq!(expected, got);

# Ok::<(), Box<dyn std::error::Error>>(())
```

---

## Literal

`struct` · `regex_syntax::hir::literal::Literal`

```rust
struct Literal
```

**Implements**: `core::convert::AsRef`, `core::convert::From`

**Derives**: Clone, Debug, Eq, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (12)

```rust
fn as_bytes(&self) -> &[u8]
fn exact<B: Into<Vec<u8>>>(bytes: B) -> Literal
fn extend(&mut self, lit: &Literal)
fn inexact<B: Into<Vec<u8>>>(bytes: B) -> Literal
fn into_bytes(self) -> Vec<u8>
fn is_empty(&self) -> bool
fn is_exact(&self) -> bool
fn keep_first_bytes(&mut self, len: usize)
fn keep_last_bytes(&mut self, len: usize)
fn len(&self) -> usize
fn make_inexact(&mut self)
fn reverse(&mut self)
```

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &[u8]
```

**via `core::convert::From`**

```rust
fn from(byte: u8) -> Literal
fn from(ch: char) -> Literal
```

A single literal extracted from an [`Hir`] expression.

A literal is composed of two things:

* A sequence of bytes. No guarantees with respect to UTF-8 are provided.
In particular, even if the regex a literal is extracted from is UTF-8, the
literal extracted may not be valid UTF-8. (For example, if an [`Extractor`]
limit resulted in trimming a literal in a way that splits a codepoint.)
* Whether the literal is "exact" or not. An "exact" literal means that it
has not been trimmed, and may continue to be extended. If a literal is
"exact" after visiting the entire `Hir` expression, then this implies that
the literal leads to a match state. (Although it doesn't necessarily imply
all occurrences of the literal correspond to a match of the regex, since
literal extraction ignores look-around assertions.)

---

## Seq

`struct` · `regex_syntax::hir::literal::Seq`

```rust
struct Seq
```

**Implements**: `core::iter::traits::collect::FromIterator`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (31)

```rust
fn cross_forward(&mut self, other: &mut Seq)
fn cross_reverse(&mut self, other: &mut Seq)
fn dedup(&mut self)
fn empty() -> Seq
fn infinite() -> Seq
fn is_empty(&self) -> bool
fn is_exact(&self) -> bool
fn is_finite(&self) -> bool
fn is_inexact(&self) -> bool
fn keep_first_bytes(&mut self, len: usize)
fn keep_last_bytes(&mut self, len: usize)
fn len(&self) -> Option<usize>
fn literals(&self) -> Option<&[Literal]>
fn longest_common_prefix(&self) -> Option<&[u8]>
fn longest_common_suffix(&self) -> Option<&[u8]>
fn make_inexact(&mut self)
fn make_infinite(&mut self)
fn max_cross_len(&self, other: &Seq) -> Option<usize>
fn max_literal_len(&self) -> Option<usize>
fn max_union_len(&self, other: &Seq) -> Option<usize>
fn min_literal_len(&self) -> Option<usize>
fn minimize_by_preference(&mut self)
fn new<I, B>(it: I) -> Seq where I: IntoIterator<Item = B>, B: AsRef<[u8]>
fn optimize_for_prefix_by_preference(&mut self)
fn optimize_for_suffix_by_preference(&mut self)
fn push(&mut self, lit: Literal)
fn reverse_literals(&mut self)
fn singleton(lit: Literal) -> Seq
fn sort(&mut self)
fn union(&mut self, other: &mut Seq)
fn union_into_empty(&mut self, other: &mut Seq)
```

**via `core::iter::traits::collect::FromIterator`**

```rust
fn from_iter<T: IntoIterator<Item = Literal>>(it: T) -> Seq
```

A sequence of literals.

A `Seq` is very much like a set in that it represents a union of its
members. That is, it corresponds to a set of literals where at least one
must match in order for a particular [`Hir`] expression to match. (Whether
this corresponds to the entire `Hir` expression, a prefix of it or a suffix
of it depends on how the `Seq` was extracted from the `Hir`.)

It is also unlike a set in that multiple identical literals may appear,
and that the order of the literals in the `Seq` matters. For example, if
the sequence is `[sam, samwise]` and leftmost-first matching is used, then
`samwise` can never match and the sequence is equivalent to `[sam]`.

# States of a sequence

A `Seq` has a few different logical states to consider:

* The sequence can represent "any" literal. When this happens, the set does
not have a finite size. The purpose of this state is to inhibit callers
from making assumptions about what literals are required in order to match
a particular [`Hir`] expression. Generally speaking, when a set is in this
state, literal optimizations are inhibited. A good example of a regex that
will cause this sort of set to appear is `[A-Za-z]`. The character class
is just too big (and also too narrow) to be usefully expanded into 52
different literals. (Note that the decision for when a seq should become
infinite is determined by the caller. A seq itself has no hard-coded
limits.)
* The sequence can be empty, in which case, it is an affirmative statement
that there are no literals that can match the corresponding `Hir`.
Consequently, the `Hir` never matches any input. For example, `[a&&b]`.
* The sequence can be non-empty, in which case, at least one of the
literals must match in order for the corresponding `Hir` to match.

# Example

This example shows how literal sequences can be simplified by stripping
suffixes and minimizing while maintaining preference order.

```
use regex_syntax::hir::literal::{Literal, Seq};

let mut seq = Seq::new(&[
    "farm",
    "appliance",
    "faraway",
    "apple",
    "fare",
    "gap",
    "applicant",
    "applaud",
]);
seq.keep_first_bytes(3);
seq.minimize_by_preference();
// Notice that 'far' comes before 'app', which matches the order in the
// original sequence. This guarantees that leftmost-first semantics are
// not altered by simplifying the set.
let expected = Seq::from_iter([
    Literal::inexact("far"),
    Literal::inexact("app"),
    Literal::exact("gap"),
]);
assert_eq!(expected, seq);
```

---
