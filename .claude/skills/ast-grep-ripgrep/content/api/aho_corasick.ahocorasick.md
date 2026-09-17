# `aho_corasick::ahocorasick`

Crate `aho-corasick` · 6 public items · structured records in [`model/aho_corasick.ahocorasick.json`](../model/aho_corasick.ahocorasick.json)

## AhoCorasickKind

`enum` · `aho_corasick::ahocorasick::AhoCorasickKind`

Also reachable as `aho_corasick::AhoCorasickKind`

```rust
enum AhoCorasickKind
```

**Variants**: `NoncontiguousNFA`, `ContiguousNFA`, `DFA`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

The type of Aho-Corasick implementation to use in an [`AhoCorasick`]
searcher.

This is principally used as an input to the
[`AhoCorasickBuilder::start_kind`] method. Its documentation goes into more
detail about each choice.

---

## AhoCorasick

`struct` · `aho_corasick::ahocorasick::AhoCorasick`

Also reachable as `aho_corasick::AhoCorasick`

```rust
struct AhoCorasick
```

**Derives**: Clone, Debug

**Methods** (30)

```rust
fn builder() -> AhoCorasickBuilder
fn find<'h, I: Into<Input<'h>>>(&self, input: I) -> Option<Match>
fn find_iter<'a, 'h, I: Into<Input<'h>>>(&'a self, input: I) -> FindIter<'a, 'h>
fn find_overlapping<'h, I: Into<Input<'h>>>(&self, input: I, state: &mut OverlappingState)
fn find_overlapping_iter<'a, 'h, I: Into<Input<'h>>>(&'a self, input: I) -> FindOverlappingIter<'a, 'h>
fn is_match<'h, I: Into<Input<'h>>>(&self, input: I) -> bool
fn kind(&self) -> AhoCorasickKind
fn match_kind(&self) -> MatchKind
fn max_pattern_len(&self) -> usize
fn memory_usage(&self) -> usize
fn min_pattern_len(&self) -> usize
fn new<I, P>(patterns: I) -> Result<AhoCorasick, BuildError> where I: IntoIterator<Item = P>, P: AsRef<[u8]>
fn patterns_len(&self) -> usize
fn replace_all<B>(&self, haystack: &str, replace_with: &[B]) -> String where B: AsRef<str>
fn replace_all_bytes<B>(&self, haystack: &[u8], replace_with: &[B]) -> Vec<u8> where B: AsRef<[u8]>
fn replace_all_with<F>(&self, haystack: &str, dst: &mut String, replace_with: F) where F: FnMut(&Match, &str, &mut String) -> bool
fn replace_all_with_bytes<F>(&self, haystack: &[u8], dst: &mut Vec<u8>, replace_with: F) where F: FnMut(&Match, &[u8], &mut Vec<u8>) -> bool
fn start_kind(&self) -> StartKind
fn stream_find_iter<'a, R: std::io::Read>(&'a self, rdr: R) -> StreamFindIter<'a, R>
fn try_find<'h, I: Into<Input<'h>>>(&self, input: I) -> Result<Option<Match>, MatchError>
fn try_find_iter<'a, 'h, I: Into<Input<'h>>>(&'a self, input: I) -> Result<FindIter<'a, 'h>, MatchError>
fn try_find_overlapping<'h, I: Into<Input<'h>>>(&self, input: I, state: &mut OverlappingState) -> Result<(), MatchError>
fn try_find_overlapping_iter<'a, 'h, I: Into<Input<'h>>>(&'a self, input: I) -> Result<FindOverlappingIter<'a, 'h>, MatchError>
fn try_replace_all<B>(&self, haystack: &str, replace_with: &[B]) -> Result<String, MatchError> where B: AsRef<str>
fn try_replace_all_bytes<B>(&self, haystack: &[u8], replace_with: &[B]) -> Result<Vec<u8>, MatchError> where B: AsRef<[u8]>
fn try_replace_all_with<F>(&self, haystack: &str, dst: &mut String, replace_with: F) -> Result<(), MatchError> where F: FnMut(&Match, &str, &mut String) -> bool
fn try_replace_all_with_bytes<F>(&self, haystack: &[u8], dst: &mut Vec<u8>, replace_with: F) -> Result<(), MatchError> where F: FnMut(&Match, &[u8], &mut Vec<u8>) -> bool
fn try_stream_find_iter<'a, R: std::io::Read>(&'a self, rdr: R) -> Result<StreamFindIter<'a, R>, MatchError>
fn try_stream_replace_all<R, W, B>(&self, rdr: R, wtr: W, replace_with: &[B]) -> Result<(), std::io::Error> where R: std::io::Read, W: std::io::Write, B: AsRef<[u8]>
fn try_stream_replace_all_with<R, W, F>(&self, rdr: R, wtr: W, replace_with: F) -> Result<(), std::io::Error> where R: std::io::Read, W: std::io::Write, F: FnMut(&Match, &[u8], &mut W) -> Result<(), std::io::Error>
```

An automaton for searching multiple strings in linear time.

The `AhoCorasick` type supports a few basic ways of constructing an
automaton, with the default being [`AhoCorasick::new`]. However, there
are a fair number of configurable options that can be set by using
[`AhoCorasickBuilder`] instead. Such options include, but are not limited
to, how matches are determined, simple case insensitivity, whether to use a
DFA or not and various knobs for controlling the space-vs-time trade offs
taken when building the automaton.

# Resource usage

Aho-Corasick automatons are always constructed in `O(p)` time, where
`p` is the combined length of all patterns being searched. With that
said, building an automaton can be fairly costly because of high constant
factors, particularly when enabling the [DFA](AhoCorasickKind::DFA) option
with [`AhoCorasickBuilder::kind`]. For this reason, it's generally a good
idea to build an automaton once and reuse it as much as possible.

Aho-Corasick automatons can also use a fair bit of memory. To get
a concrete idea of how much memory is being used, try using the
[`AhoCorasick::memory_usage`] method.

To give a quick idea of the differences between Aho-Corasick
implementations and their resource usage, here's a sample of construction
times and heap memory used after building an automaton from 100,000
randomly selected titles from Wikipedia:

* 99MB for a [`noncontiguous::NFA`] in 240ms.
* 21MB for a [`contiguous::NFA`] in 275ms.
* 1.6GB for a [`dfa::DFA`] in 1.88s.

(Note that the memory usage above reflects the size of each automaton and
not peak memory usage. For example, building a contiguous NFA requires
first building a noncontiguous NFA. Once the contiguous NFA is built, the
noncontiguous NFA is freed.)

This experiment very strongly argues that a contiguous NFA is often the
best balance in terms of resource usage. It takes a little longer to build,
but its memory usage is quite small. Its search speed (not listed) is
also often faster than a noncontiguous NFA, but a little slower than a
DFA. Indeed, when no specific [`AhoCorasickKind`] is used (which is the
default), a contiguous NFA is used in most cases.

The only "catch" to using a contiguous NFA is that, because of its variety
of compression tricks, it may not be able to support automatons as large as
what the noncontiguous NFA supports. In which case, building a contiguous
NFA will fail and (by default) `AhoCorasick` will automatically fall
back to a noncontiguous NFA. (This typically only happens when building
automatons from millions of patterns.) Otherwise, the small additional time
for building a contiguous NFA is almost certainly worth it.

# Cloning

The `AhoCorasick` type uses thread safe reference counting internally. It
is guaranteed that it is cheap to clone.

# Search configuration

Most of the search routines accept anything that can be cheaply converted
to an [`Input`]. This includes `&[u8]`, `&str` and `Input` itself.

# Construction failure

It is generally possible for building an Aho-Corasick automaton to fail.
Construction can fail in generally one way: when the inputs provided are
too big. Whether that's a pattern that is too long, too many patterns
or some combination of both. A first approximation for the scale at which
construction can fail is somewhere around "millions of patterns."

For that reason, if you're building an Aho-Corasick automaton from
untrusted input (or input that doesn't have any reasonable bounds on its
size), then it is strongly recommended to handle the possibility of an
error.

If you're constructing an Aho-Corasick automaton from static or trusted
data, then it is likely acceptable to panic (by calling `unwrap()` or
`expect()`) if construction fails.

# Fallibility

The `AhoCorasick` type provides a number of methods for searching, as one
might expect. Depending on how the Aho-Corasick automaton was built and
depending on the search configuration, it is possible for a search to
return an error. Since an error is _never_ dependent on the actual contents
of the haystack, this type provides both infallible and fallible methods
for searching. The infallible methods panic if an error occurs, and can be
used for convenience and when you know the search will never return an
error.

For example, the [`AhoCorasick::find_iter`] method is the infallible
version of the [`AhoCorasick::try_find_iter`] method.

Examples of errors that can occur:

* Running a search that requires [`MatchKind::Standard`] semantics (such
as a stream or overlapping search) with an automaton that was built with
[`MatchKind::LeftmostFirst`] or [`MatchKind::LeftmostLongest`] semantics.
* Running an anchored search with an automaton that only supports
unanchored searches. (By default, `AhoCorasick` only supports unanchored
searches. But this can be toggled with [`AhoCorasickBuilder::start_kind`].)
* Running an unanchored search with an automaton that only supports
anchored searches.

The common thread between the different types of errors is that they are
all rooted in the automaton construction and search configurations. If
those configurations are a static property of your program, then it is
reasonable to call infallible routines since you know an error will never
occur. And if one _does_ occur, then it's a bug in your program.

To re-iterate, if the patterns, build or search configuration come from
user or untrusted data, then you should handle errors at build or search
time. If only the haystack comes from user or untrusted data, then there
should be no need to handle errors anywhere and it is generally encouraged
to `unwrap()` (or `expect()`) both build and search time calls.

# Examples

This example shows how to search for occurrences of multiple patterns
simultaneously in a case insensitive fashion. Each match includes the
pattern that matched along with the byte offsets of the match.

```
use aho_corasick::{AhoCorasick, PatternID};

let patterns = &["apple", "maple", "snapple"];
let haystack = "Nobody likes maple in their apple flavored Snapple.";

let ac = AhoCorasick::builder()
    .ascii_case_insensitive(true)
    .build(patterns)
    .unwrap();
let mut matches = vec![];
for mat in ac.find_iter(haystack) {
    matches.push((mat.pattern(), mat.start(), mat.end()));
}
assert_eq!(matches, vec![
    (PatternID::must(1), 13, 18),
    (PatternID::must(0), 28, 33),
    (PatternID::must(2), 43, 50),
]);
```

This example shows how to replace matches with some other string:

```
use aho_corasick::AhoCorasick;

let patterns = &["fox", "brown", "quick"];
let haystack = "The quick brown fox.";
let replace_with = &["sloth", "grey", "slow"];

let ac = AhoCorasick::new(patterns).unwrap();
let result = ac.replace_all(haystack, replace_with);
assert_eq!(result, "The slow grey sloth.");
```

---

## AhoCorasickBuilder

`struct` · `aho_corasick::ahocorasick::AhoCorasickBuilder`

Also reachable as `aho_corasick::AhoCorasickBuilder`

```rust
struct AhoCorasickBuilder
```

**Derives**: Clone, Debug, Default

**Methods** (9)

```rust
fn ascii_case_insensitive(&mut self, yes: bool) -> &mut AhoCorasickBuilder
fn build<I, P>(&self, patterns: I) -> Result<AhoCorasick, BuildError> where I: IntoIterator<Item = P>, P: AsRef<[u8]>
fn byte_classes(&mut self, yes: bool) -> &mut AhoCorasickBuilder
fn dense_depth(&mut self, depth: usize) -> &mut AhoCorasickBuilder
fn kind(&mut self, kind: Option<AhoCorasickKind>) -> &mut AhoCorasickBuilder
fn match_kind(&mut self, kind: MatchKind) -> &mut AhoCorasickBuilder
fn new() -> AhoCorasickBuilder
fn prefilter(&mut self, yes: bool) -> &mut AhoCorasickBuilder
fn start_kind(&mut self, kind: StartKind) -> &mut AhoCorasickBuilder
```

A builder for configuring an Aho-Corasick automaton.

# Quick advice

* Use [`AhoCorasickBuilder::match_kind`] to configure your searcher
with [`MatchKind::LeftmostFirst`] if you want to match how backtracking
regex engines execute searches for `pat1|pat2|..|patN`. Use
[`MatchKind::LeftmostLongest`] if you want to match how POSIX regex engines
do it.
* If you need an anchored search, use [`AhoCorasickBuilder::start_kind`] to
set the [`StartKind::Anchored`] mode since [`StartKind::Unanchored`] is the
default. Or just use [`StartKind::Both`] to support both types of searches.
* You might want to use [`AhoCorasickBuilder::kind`] to set your searcher
to always use a [`AhoCorasickKind::DFA`] if search speed is critical and
memory usage isn't a concern. Otherwise, not setting a kind will probably
make the right choice for you. Beware that if you use [`StartKind::Both`]
to build a searcher that supports both unanchored and anchored searches
_and_ you set [`AhoCorasickKind::DFA`], then the DFA will essentially be
duplicated to support both simultaneously. This results in very high memory
usage.
* For all other options, their defaults are almost certainly what you want.

---

## FindIter

`struct` · `aho_corasick::ahocorasick::FindIter`

Also reachable as `aho_corasick::FindIter`

```rust
struct FindIter<'a, 'h>
```

**Implements**: `core::iter::traits::iterator::Iterator`

**Derives**: Debug

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<Match>
```

An iterator of non-overlapping matches in a particular haystack.

This iterator yields matches according to the [`MatchKind`] used by this
automaton.

This iterator is constructed via the [`AhoCorasick::find_iter`] and
[`AhoCorasick::try_find_iter`] methods.

The lifetime `'a` refers to the lifetime of the `AhoCorasick` automaton.

The lifetime `'h` refers to the lifetime of the haystack being searched.

---

## FindOverlappingIter

`struct` · `aho_corasick::ahocorasick::FindOverlappingIter`

Also reachable as `aho_corasick::FindOverlappingIter`

```rust
struct FindOverlappingIter<'a, 'h>
```

**Implements**: `core::iter::traits::iterator::Iterator`

**Derives**: Debug

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<Match>
```

An iterator of overlapping matches in a particular haystack.

This iterator will report all possible matches in a particular haystack,
even when the matches overlap.

This iterator is constructed via the [`AhoCorasick::find_overlapping_iter`]
and [`AhoCorasick::try_find_overlapping_iter`] methods.

The lifetime `'a` refers to the lifetime of the `AhoCorasick` automaton.

The lifetime `'h` refers to the lifetime of the haystack being searched.

---

## StreamFindIter

`struct` · `aho_corasick::ahocorasick::StreamFindIter`

Also reachable as `aho_corasick::StreamFindIter`

```rust
struct StreamFindIter<'a, R>
```

**Implements**: `core::iter::traits::iterator::Iterator`

**Derives**: Debug

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<Result<Match, std::io::Error>>
```

An iterator that reports Aho-Corasick matches in a stream.

This iterator yields elements of type `Result<Match, std::io::Error>`,
where an error is reported if there was a problem reading from the
underlying stream. The iterator terminates only when the underlying stream
reaches `EOF`.

This iterator is constructed via the [`AhoCorasick::stream_find_iter`] and
[`AhoCorasick::try_stream_find_iter`] methods.

The type variable `R` refers to the `io::Read` stream that is being read
from.

The lifetime `'a` refers to the lifetime of the corresponding
[`AhoCorasick`] searcher.

---
