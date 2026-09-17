# `aho_corasick::util::search`

Crate `aho-corasick` · 6 public items · structured records in [`model/aho_corasick.util.search.json`](../model/aho_corasick.util.search.json)

## Anchored

`enum` · `aho_corasick::util::search::Anchored`

Also reachable as `aho_corasick::Anchored`

```rust
enum Anchored
```

**Variants**: `No`, `Yes`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn is_anchored(&self) -> bool
```

The type of anchored search to perform.

If an Aho-Corasick searcher does not support the anchored mode selected,
then the search will return an error or panic, depending on whether a
fallible or an infallible routine was called.

---

## MatchKind

`enum` · `aho_corasick::util::search::MatchKind`

Also reachable as `aho_corasick::MatchKind`

```rust
enum MatchKind
```

**Variants**: `Standard`, `LeftmostFirst`, `LeftmostLongest`

**Derives**: Clone, Copy, Debug, Default, Eq, PartialEq, StructuralPartialEq

A knob for controlling the match semantics of an Aho-Corasick automaton.

There are two generally different ways that Aho-Corasick automatons can
report matches. The first way is the "standard" approach that results from
implementing most textbook explanations of Aho-Corasick. The second way is
to report only the leftmost non-overlapping matches. The leftmost approach
is in turn split into two different ways of resolving ambiguous matches:
leftmost-first and leftmost-longest.

The `Standard` match kind is the default and is the only one that supports
overlapping matches and stream searching. (Trying to find overlapping or
streaming matches using leftmost match semantics will result in an error in
fallible APIs and a panic when using infallibe APIs.) The `Standard` match
kind will report matches as they are seen. When searching for overlapping
matches, then all possible matches are reported. When searching for
non-overlapping matches, the first match seen is reported. For example, for
non-overlapping matches, given the patterns `abcd` and `b` and the haystack
`abcdef`, only a match for `b` is reported since it is detected first. The
`abcd` match is never reported since it overlaps with the `b` match.

In contrast, the leftmost match kind always prefers the leftmost match
among all possible matches. Given the same example as above with `abcd` and
`b` as patterns and `abcdef` as the haystack, the leftmost match is `abcd`
since it begins before the `b` match, even though the `b` match is detected
before the `abcd` match. In this case, the `b` match is not reported at all
since it overlaps with the `abcd` match.

The difference between leftmost-first and leftmost-longest is in how they
resolve ambiguous matches when there are multiple leftmost matches to
choose from. Leftmost-first always chooses the pattern that was provided
earliest, where as leftmost-longest always chooses the longest matching
pattern. For example, given the patterns `a` and `ab` and the subject
string `ab`, the leftmost-first match is `a` but the leftmost-longest match
is `ab`. Conversely, if the patterns were given in reverse order, i.e.,
`ab` and `a`, then both the leftmost-first and leftmost-longest matches
would be `ab`. Stated differently, the leftmost-first match depends on the
order in which the patterns were given to the Aho-Corasick automaton.
Because of that, when leftmost-first matching is used, if a pattern `A`
that appears before a pattern `B` is a prefix of `B`, then it is impossible
to ever observe a match of `B`.

If you're not sure which match kind to pick, then stick with the standard
kind, which is the default. In particular, if you need overlapping or
streaming matches, then you _must_ use the standard kind. The leftmost
kinds are useful in specific circumstances. For example, leftmost-first can
be very useful as a way to implement match priority based on the order of
patterns given and leftmost-longest can be useful for dictionary searching
such that only the longest matching words are reported.

# Relationship with regular expression alternations

Understanding match semantics can be a little tricky, and one easy way
to conceptualize non-overlapping matches from an Aho-Corasick automaton
is to think about them as a simple alternation of literals in a regular
expression. For example, let's say we wanted to match the strings
`Sam` and `Samwise`, which would turn into the regex `Sam|Samwise`. It
turns out that regular expression engines have two different ways of
matching this alternation. The first way, leftmost-longest, is commonly
found in POSIX compatible implementations of regular expressions (such as
`grep`). The second way, leftmost-first, is commonly found in backtracking
implementations such as Perl. (Some regex engines, such as RE2 and Rust's
regex engine do not use backtracking, but still implement leftmost-first
semantics in an effort to match the behavior of dominant backtracking
regex engines such as those found in Perl, Ruby, Python, Javascript and
PHP.)

That is, when matching `Sam|Samwise` against `Samwise`, a POSIX regex
will match `Samwise` because it is the longest possible match, but a
Perl-like regex will match `Sam` since it appears earlier in the
alternation. Indeed, the regex `Sam|Samwise` in a Perl-like regex engine
will never match `Samwise` since `Sam` will always have higher priority.
Conversely, matching the regex `Samwise|Sam` against `Samwise` will lead to
a match of `Samwise` in both POSIX and Perl-like regexes since `Samwise` is
still longest match, but it also appears earlier than `Sam`.

The "standard" match semantics of Aho-Corasick generally don't correspond
to the match semantics of any large group of regex implementations, so
there's no direct analogy that can be made here. Standard match semantics
are generally useful for overlapping matches, or if you just want to see
matches as they are detected.

The main conclusion to draw from this section is that the match semantics
can be tweaked to precisely match either Perl-like regex alternations or
POSIX regex alternations.

---

## StartKind

`enum` · `aho_corasick::util::search::StartKind`

Also reachable as `aho_corasick::StartKind`

```rust
enum StartKind
```

**Variants**: `Both`, `Unanchored`, `Anchored`

**Derives**: Clone, Copy, Debug, Default, Eq, PartialEq, StructuralPartialEq

The kind of anchored starting configurations to support in an Aho-Corasick
searcher.

Depending on which searcher is used internally by
[`AhoCorasick`](crate::AhoCorasick), supporting both unanchored
and anchored searches can be quite costly. For this reason,
[`AhoCorasickBuilder::start_kind`](crate::AhoCorasickBuilder::start_kind)
can be used to configure whether your searcher supports unanchored,
anchored or both kinds of searches.

This searcher configuration knob works in concert with the search time
configuration [`Input::anchored`]. Namely, if one requests an unsupported
anchored mode, then the search will either panic or return an error,
depending on whether you're using infallible or fallibe APIs, respectively.

`AhoCorasick` by default only supports unanchored searches.

---

## Input

`struct` · `aho_corasick::util::search::Input`

Also reachable as `aho_corasick::Input`

```rust
struct Input<'h>
```

**Implements**: `core::convert::From`

**Derives**: Clone, Debug

**Methods** (19)

```rust
fn anchored(self, mode: Anchored) -> Input<'h>
fn earliest(self, yes: bool) -> Input<'h>
fn end(&self) -> usize
fn get_anchored(&self) -> Anchored
fn get_earliest(&self) -> bool
fn get_range(&self) -> Range<usize>
fn get_span(&self) -> Span
fn haystack(&self) -> &[u8]
fn is_done(&self) -> bool
fn new<H: ?Sized + AsRef<[u8]>>(haystack: &'h H) -> Input<'h>
fn range<R: RangeBounds<usize>>(self, range: R) -> Input<'h>
fn set_anchored(&mut self, mode: Anchored)
fn set_earliest(&mut self, yes: bool)
fn set_end(&mut self, end: usize)
fn set_range<R: RangeBounds<usize>>(&mut self, range: R)
fn set_span<S: Into<Span>>(&mut self, span: S)
fn set_start(&mut self, start: usize)
fn span<S: Into<Span>>(self, span: S) -> Input<'h>
fn start(&self) -> usize
```

**via `core::convert::From`**

```rust
fn from(haystack: &'h H) -> Input<'h>
```

The configuration and the haystack to use for an Aho-Corasick search.

When executing a search, there are a few parameters one might want to
configure:

* The haystack to search, provided to the [`Input::new`] constructor. This
is the only required parameter.
* The span _within_ the haystack to limit a search to. (The default
is the entire haystack.) This is configured via [`Input::span`] or
[`Input::range`].
* Whether to run an unanchored (matches can occur anywhere after the
start of the search) or anchored (matches can only occur beginning at
the start of the search) search. Unanchored search is the default. This is
configured via [`Input::anchored`].
* Whether to quit the search as soon as a match has been found, regardless
of the [`MatchKind`] that the searcher was built with. This is configured
via [`Input::earliest`].

For most cases, the defaults for all optional parameters are appropriate.
The utility of this type is that it keeps the default or common case simple
while permitting tweaking parameters in more niche use cases while reusing
the same search APIs.

# Valid bounds and search termination

An `Input` permits setting the bounds of a search via either
[`Input::span`] or [`Input::range`]. The bounds set must be valid, or
else a panic will occur. Bounds are valid if and only if:

* The bounds represent a valid range into the input's haystack.
* **or** the end bound is a valid ending bound for the haystack *and*
the start bound is exactly one greater than the end bound.

In the latter case, [`Input::is_done`] will return true and indicates any
search receiving such an input should immediately return with no match.

Other than representing "search is complete," the `Input::span` and
`Input::range` APIs are never necessary. Instead, callers can slice the
haystack instead, e.g., with `&haystack[start..end]`. With that said, they
can be more convenient than slicing because the match positions reported
when using `Input::span` or `Input::range` are in terms of the original
haystack. If you instead use `&haystack[start..end]`, then you'll need to
add `start` to any match position returned in order for it to be a correct
index into `haystack`.

# Example: `&str` and `&[u8]` automatically convert to an `Input`

There is a `From<&T> for Input` implementation for all `T: AsRef<[u8]>`.
Additionally, the [`AhoCorasick`](crate::AhoCorasick) search APIs accept
a `Into<Input>`. These two things combined together mean you can provide
things like `&str` and `&[u8]` to search APIs when the defaults are
suitable, but also an `Input` when they're not. For example:

```
use aho_corasick::{AhoCorasick, Anchored, Input, Match, StartKind};

// Build a searcher that supports both unanchored and anchored modes.
let ac = AhoCorasick::builder()
    .start_kind(StartKind::Both)
    .build(&["abcd", "b"])
    .unwrap();
let haystack = "abcd";

// A search using default parameters is unanchored. With standard
// semantics, this finds `b` first.
assert_eq!(
    Some(Match::must(1, 1..2)),
    ac.find(haystack),
);
// Using the same 'find' routine, we can provide an 'Input' explicitly
// that is configured to do an anchored search. Since 'b' doesn't start
// at the beginning of the search, it is not reported as a match.
assert_eq!(
    Some(Match::must(0, 0..4)),
    ac.find(Input::new(haystack).anchored(Anchored::Yes)),
);
```

---

## Match

`struct` · `aho_corasick::util::search::Match`

Also reachable as `aho_corasick::Match`

```rust
struct Match
```

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (10)

```rust
fn end(&self) -> usize
fn is_empty(&self) -> bool
fn len(&self) -> usize
fn must<S: Into<Span>>(pattern: usize, span: S) -> Match
fn new<S: Into<Span>>(pattern: PatternID, span: S) -> Match
fn offset(&self, offset: usize) -> Match
fn pattern(&self) -> PatternID
fn range(&self) -> core::ops::Range<usize>
fn span(&self) -> Span
fn start(&self) -> usize
```

A representation of a match reported by an Aho-Corasick searcher.

A match has two essential pieces of information: the [`PatternID`] that
matches, and the [`Span`] of the match in a haystack.

The pattern is identified by an ID, which corresponds to its position
(starting from `0`) relative to other patterns used to construct the
corresponding searcher. If only a single pattern is provided, then all
matches are guaranteed to have a pattern ID of `0`.

Every match reported by a searcher guarantees that its span has its start
offset as less than or equal to its end offset.

---

## Span

`struct` · `aho_corasick::util::search::Span`

Also reachable as `aho_corasick::Span`

```rust
struct Span
```

**Fields**: `start`, `end`

**Implements**: `core::convert::From`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (5)

```rust
fn contains(&self, offset: usize) -> bool
fn is_empty(&self) -> bool
fn len(&self) -> usize
fn offset(&self, offset: usize) -> Span
fn range(&self) -> Range<usize>
```

**via `core::convert::From`**

```rust
fn from(range: Range<usize>) -> Span
```

A representation of a range in a haystack.

A span corresponds to the starting and ending _byte offsets_ of a
contiguous region of bytes. The starting offset is inclusive while the
ending offset is exclusive. That is, a span is a half-open interval.

A span is used to report the offsets of a match, but it is also used to
convey which region of a haystack should be searched via routines like
[`Input::span`].

This is basically equivalent to a `std::ops::Range<usize>`, except this
type implements `Copy` which makes it more ergonomic to use in the context
of this crate. Indeed, `Span` exists only because `Range<usize>` does
not implement `Copy`. Like a range, this implements `Index` for `[u8]`
and `str`, and `IndexMut` for `[u8]`. For convenience, this also impls
`From<Range>`, which means things like `Span::from(5..10)` work.

There are no constraints on the values of a span. It is, for example, legal
to create a span where `start > end`.

---
