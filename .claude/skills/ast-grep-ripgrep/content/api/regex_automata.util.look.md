# `regex_automata::util::look`

Crate `regex-automata` · 5 public items · structured records in [`model/regex_automata.util.look.json`](../model/regex_automata.util.look.json)

## Look

`enum` · `regex_automata::util::look::Look`

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

A look-around assertion.

An assertion matches at a position between characters in a haystack.
Namely, it does not actually "consume" any input as most parts of a regular
expression do. Assertions are a way of stating that some property must be
true at a particular point during matching.

For example, `(?m)^[a-z]+$` is a pattern that:

* Scans the haystack for a position at which `(?m:^)` is satisfied. That
occurs at either the beginning of the haystack, or immediately following
a `\n` character.
* Looks for one or more occurrences of `[a-z]`.
* Once `[a-z]+` has matched as much as it can, an overall match is only
reported when `[a-z]+` stops just before a `\n`.

So in this case, `abc` and `\nabc\n` match, but `\nabc1\n` does not.

Assertions are also called "look-around," "look-behind" and "look-ahead."
Specifically, some assertions are look-behind (like `^`), other assertions
are look-ahead (like `$`) and yet other assertions are both look-ahead and
look-behind (like `\b`).

# Assertions in an NFA

An assertion in a [`thompson::NFA`](crate::nfa::thompson::NFA) can be
thought of as a conditional epsilon transition. That is, a matching engine
like the [`PikeVM`](crate::nfa::thompson::pikevm::PikeVM) only permits
moving through conditional epsilon transitions when their condition
is satisfied at whatever position the `PikeVM` is currently at in the
haystack.

How assertions are handled in a `DFA` is trickier, since a DFA does not
have epsilon transitions at all. In this case, they are compiled into the
automaton itself, at the expense of more states than what would be required
without an assertion.

---

## LookMatcher

`struct` · `regex_automata::util::look::LookMatcher`

```rust
struct LookMatcher
```

**Derives**: Clone, Debug, Default

**Methods** (23)

```rust
fn get_line_terminator(&self) -> u8
fn is_end(&self, haystack: &[u8], at: usize) -> bool
fn is_end_crlf(&self, haystack: &[u8], at: usize) -> bool
fn is_end_lf(&self, haystack: &[u8], at: usize) -> bool
fn is_start(&self, _haystack: &[u8], at: usize) -> bool
fn is_start_crlf(&self, haystack: &[u8], at: usize) -> bool
fn is_start_lf(&self, haystack: &[u8], at: usize) -> bool
fn is_word_ascii(&self, haystack: &[u8], at: usize) -> bool
fn is_word_ascii_negate(&self, haystack: &[u8], at: usize) -> bool
fn is_word_end_ascii(&self, haystack: &[u8], at: usize) -> bool
fn is_word_end_half_ascii(&self, haystack: &[u8], at: usize) -> bool
fn is_word_end_half_unicode(&self, haystack: &[u8], at: usize) -> Result<bool, UnicodeWordBoundaryError>
fn is_word_end_unicode(&self, haystack: &[u8], at: usize) -> Result<bool, UnicodeWordBoundaryError>
fn is_word_start_ascii(&self, haystack: &[u8], at: usize) -> bool
fn is_word_start_half_ascii(&self, haystack: &[u8], at: usize) -> bool
fn is_word_start_half_unicode(&self, haystack: &[u8], at: usize) -> Result<bool, UnicodeWordBoundaryError>
fn is_word_start_unicode(&self, haystack: &[u8], at: usize) -> Result<bool, UnicodeWordBoundaryError>
fn is_word_unicode(&self, haystack: &[u8], at: usize) -> Result<bool, UnicodeWordBoundaryError>
fn is_word_unicode_negate(&self, haystack: &[u8], at: usize) -> Result<bool, UnicodeWordBoundaryError>
fn matches(&self, look: Look, haystack: &[u8], at: usize) -> bool
fn matches_set(&self, set: LookSet, haystack: &[u8], at: usize) -> bool
fn new() -> LookMatcher
fn set_line_terminator(&mut self, byte: u8) -> &mut LookMatcher
```

A matcher for look-around assertions.

This matcher permits configuring aspects of how look-around assertions are
matched.

# Example

A `LookMatcher` can change the line terminator used for matching multi-line
anchors such as `(?m:^)` and `(?m:$)`.

```
use regex_automata::{
    nfa::thompson::{self, pikevm::PikeVM},
    util::look::LookMatcher,
    Match, Input,
};

let mut lookm = LookMatcher::new();
lookm.set_line_terminator(b'\x00');

let re = PikeVM::builder()
    .thompson(thompson::Config::new().look_matcher(lookm))
    .build(r"(?m)^[a-z]+$")?;
let mut cache = re.create_cache();

// Multi-line assertions now use NUL as a terminator.
assert_eq!(
    Some(Match::must(0, 1..4)),
    re.find(&mut cache, b"\x00abc\x00"),
);
// ... and \n is no longer recognized as a terminator.
assert_eq!(
    None,
    re.find(&mut cache, b"\nabc\n"),
);

# Ok::<(), Box<dyn std::error::Error>>(())
```

---

## LookSet

`struct` · `regex_automata::util::look::LookSet`

```rust
struct LookSet
```

**Fields**: `bits`

**Derives**: Clone, Copy, Debug, Default, Eq, PartialEq, StructuralPartialEq

**Methods** (28)

```rust
fn available(self) -> Result<(), UnicodeWordBoundaryError>
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

LookSet is a memory-efficient set of look-around assertions.

This is useful for efficiently tracking look-around assertions. For
example, a [`thompson::NFA`](crate::nfa::thompson::NFA) provides properties
that return `LookSet`s.

---

## LookSetIter

`struct` · `regex_automata::util::look::LookSetIter`

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

## UnicodeWordBoundaryError

`struct` · `regex_automata::util::look::UnicodeWordBoundaryError`

```rust
struct UnicodeWordBoundaryError
```

**Implements**: `core::error::Error`, `core::fmt::Display`

**Derives**: Clone, Debug

**Methods** (1)

```rust
fn check() -> Result<(), UnicodeWordBoundaryError>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
```

An error that occurs when the Unicode-aware `\w` class is unavailable.

This error can occur when the data tables necessary for the Unicode aware
Perl character class `\w` are unavailable. The `\w` class is used to
determine whether a codepoint is considered a word character or not when
determining whether a Unicode aware `\b` (or `\B`) matches at a particular
position.

This error can only occur when the `unicode-word-boundary` feature is
disabled.

---
