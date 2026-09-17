# `regex_automata::dfa::regex`

Crate `regex-automata` · 3 public items · structured records in [`model/regex_automata.dfa.regex.json`](../model/regex_automata.dfa.regex.json)

## Builder

`struct` · `regex_automata::dfa::regex::Builder`

```rust
struct Builder
```

**Derives**: Clone, Debug, Default

**Methods** (9)

```rust
fn build(&self, pattern: &str) -> Result<Regex, BuildError>
fn build_from_dfas<A: Automaton>(&self, forward: A, reverse: A) -> Regex<A>
fn build_many<P: AsRef<str>>(&self, patterns: &[P]) -> Result<Regex, BuildError>
fn build_many_sparse<P: AsRef<str>>(&self, patterns: &[P]) -> Result<Regex<sparse::DFA<Vec<u8>>>, BuildError>
fn build_sparse(&self, pattern: &str) -> Result<Regex<sparse::DFA<Vec<u8>>>, BuildError>
fn dense(&mut self, config: dense::Config) -> &mut Builder
fn new() -> Builder
fn syntax(&mut self, config: util::syntax::Config) -> &mut Builder
fn thompson(&mut self, config: nfa::thompson::Config) -> &mut Builder
```

A builder for a regex based on deterministic finite automatons.

This builder permits configuring options for the syntax of a pattern, the
NFA construction, the DFA construction and finally the regex searching
itself. This builder is different from a general purpose regex builder in
that it permits fine grain configuration of the construction process. The
trade off for this is complexity, and the possibility of setting a
configuration that might not make sense. For example, there are two
different UTF-8 modes:

* [`syntax::Config::utf8`](crate::util::syntax::Config::utf8) controls
whether the pattern itself can contain sub-expressions that match invalid
UTF-8.
* [`thompson::Config::utf8`](crate::nfa::thompson::Config::utf8) controls
how the regex iterators themselves advance the starting position of the
next search when a match with zero length is found.

Generally speaking, callers will want to either enable all of these or
disable all of these.

Internally, building a regex requires building two DFAs, where one is
responsible for finding the end of a match and the other is responsible
for finding the start of a match. If you only need to detect whether
something matched, or only the end of a match, then you should use a
[`dense::Builder`] to construct a single DFA, which is cheaper than
building two DFAs.

# Build methods

This builder has a few "build" methods. In general, it's the result of
combining the following parameters:

* Building one or many regexes.
* Building a regex with dense or sparse DFAs.

The simplest "build" method is [`Builder::build`]. It accepts a single
pattern and builds a dense DFA using `usize` for the state identifier
representation.

The most general "build" method is [`Builder::build_many`], which permits
building a regex that searches for multiple patterns simultaneously while
using a specific state identifier representation.

The most flexible "build" method, but hardest to use, is
[`Builder::build_from_dfas`]. This exposes the fact that a [`Regex`] is
just a pair of DFAs, and this method allows you to specify those DFAs
exactly.

# Example

This example shows how to disable UTF-8 mode in the syntax and the regex
itself. This is generally what you want for matching on arbitrary bytes.

```
# if cfg!(miri) { return Ok(()); } // miri takes too long
use regex_automata::{
    dfa::regex::Regex, nfa::thompson, util::syntax, Match,
};

let re = Regex::builder()
    .syntax(syntax::Config::new().utf8(false))
    .thompson(thompson::Config::new().utf8(false))
    .build(r"foo(?-u:[^b])ar.*")?;
let haystack = b"\xFEfoo\xFFarzz\xE2\x98\xFF\n";
let expected = Some(Match::must(0, 1..9));
let got = re.find(haystack);
assert_eq!(expected, got);
// Notice that `(?-u:[^b])` matches invalid UTF-8,
// but the subsequent `.*` does not! Disabling UTF-8
// on the syntax permits this.
assert_eq!(b"foo\xFFarzz", &haystack[got.unwrap().range()]);

# Ok::<(), Box<dyn std::error::Error>>(())
```

---

## FindMatches

`struct` · `regex_automata::dfa::regex::FindMatches`

```rust
struct FindMatches<'r, 'h, A>
```

**Implements**: `core::iter::traits::iterator::Iterator`

**Derives**: Debug

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<Match>
```

An iterator over all non-overlapping matches for an infallible search.

The iterator yields a [`Match`] value until no more matches could be found.
If the underlying regex engine returns an error, then a panic occurs.

The type parameters are as follows:

* `A` represents the type of the underlying DFA that implements the
[`Automaton`] trait.

The lifetime parameters are as follows:

* `'h` represents the lifetime of the haystack being searched.
* `'r` represents the lifetime of the regex object itself.

This iterator can be created with the [`Regex::find_iter`] method.

---

## Regex

`struct` · `regex_automata::dfa::regex::Regex`

```rust
struct Regex<A = DFA<alloc::vec::Vec<u32>>>
```

**Derives**: Clone, Debug

**Methods** (12)

```rust
fn builder() -> Builder
fn find<'h, I: Into<Input<'h>>>(&self, input: I) -> Option<Match>
fn find_iter<'r, 'h, I: Into<Input<'h>>>(&'r self, input: I) -> FindMatches<'r, 'h, A>
fn forward(&self) -> &A
fn is_match<'h, I: Into<Input<'h>>>(&self, input: I) -> bool
fn new(pattern: &str) -> Result<Regex, BuildError>
fn new_many<P: AsRef<str>>(patterns: &[P]) -> Result<Regex, BuildError>
fn new_many_sparse<P: AsRef<str>>(patterns: &[P]) -> Result<Regex<sparse::DFA<Vec<u8>>>, BuildError>
fn new_sparse(pattern: &str) -> Result<Regex<sparse::DFA<Vec<u8>>>, BuildError>
fn pattern_len(&self) -> usize
fn reverse(&self) -> &A
fn try_search(&self, input: &Input<'_>) -> Result<Option<Match>, MatchError>
```

A regular expression that uses deterministic finite automata for fast
searching.

A regular expression is comprised of two DFAs, a "forward" DFA and a
"reverse" DFA. The forward DFA is responsible for detecting the end of
a match while the reverse DFA is responsible for detecting the start
of a match. Thus, in order to find the bounds of any given match, a
forward search must first be run followed by a reverse search. A match
found by the forward DFA guarantees that the reverse DFA will also find
a match.

The type of the DFA used by a `Regex` corresponds to the `A` type
parameter, which must satisfy the [`Automaton`] trait. Typically, `A`
is either a [`dense::DFA`] or a [`sparse::DFA`], where dense DFAs use
more memory but search faster, while sparse DFAs use less memory but
search more slowly.

# Crate features

Note that despite what the documentation auto-generates, the _only_
crate feature needed to use this type is `dfa-search`. You do _not_
need to enable the `alloc` feature.

By default, a regex's automaton type parameter is set to
`dense::DFA<Vec<u32>>` when the `alloc` feature is enabled. For most
in-memory work loads, this is the most convenient type that gives the
best search performance. When the `alloc` feature is disabled, no
default type is used.

# When should I use this?

Generally speaking, if you can afford the overhead of building a full
DFA for your regex, and you don't need things like capturing groups,
then this is a good choice if you're looking to optimize for matching
speed. Note however that its speed may be worse than a general purpose
regex engine if you don't provide a [`dense::Config::prefilter`] to the
underlying DFA.

# Sparse DFAs

Since a `Regex` is generic over the [`Automaton`] trait, it can be
used with any kind of DFA. While this crate constructs dense DFAs by
default, it is easy enough to build corresponding sparse DFAs, and then
build a regex from them:

```
use regex_automata::dfa::regex::Regex;

// First, build a regex that uses dense DFAs.
let dense_re = Regex::new("foo[0-9]+")?;

// Second, build sparse DFAs from the forward and reverse dense DFAs.
let fwd = dense_re.forward().to_sparse()?;
let rev = dense_re.reverse().to_sparse()?;

// Third, build a new regex from the constituent sparse DFAs.
let sparse_re = Regex::builder().build_from_dfas(fwd, rev);

// A regex that uses sparse DFAs can be used just like with dense DFAs.
assert_eq!(true, sparse_re.is_match(b"foo123"));

# Ok::<(), Box<dyn std::error::Error>>(())
```

Alternatively, one can use a [`Builder`] to construct a sparse DFA
more succinctly. (Note though that dense DFAs are still constructed
first internally, and then converted to sparse DFAs, as in the example
above.)

```
use regex_automata::dfa::regex::Regex;

let sparse_re = Regex::builder().build_sparse(r"foo[0-9]+")?;
// A regex that uses sparse DFAs can be used just like with dense DFAs.
assert!(sparse_re.is_match(b"foo123"));

# Ok::<(), Box<dyn std::error::Error>>(())
```

# Fallibility

Most of the search routines defined on this type will _panic_ when the
underlying search fails. This might be because the DFA gave up because
it saw a quit byte, whether configured explicitly or via heuristic
Unicode word boundary support, although neither are enabled by default.
Or it might fail because an invalid `Input` configuration is given,
for example, with an unsupported [`Anchored`] mode.

If you need to handle these error cases instead of allowing them to
trigger a panic, then the lower level [`Regex::try_search`] provides
a fallible API that never panics.

# Example

This example shows how to cause a search to terminate if it sees a
`\n` byte, and handle the error returned. This could be useful if, for
example, you wanted to prevent a user supplied pattern from matching
across a line boundary.

```
# if cfg!(miri) { return Ok(()); } // miri takes too long
use regex_automata::{dfa::{self, regex::Regex}, Input, MatchError};

let re = Regex::builder()
    .dense(dfa::dense::Config::new().quit(b'\n', true))
    .build(r"foo\p{any}+bar")?;

let input = Input::new("foo\nbar");
// Normally this would produce a match, since \p{any} contains '\n'.
// But since we instructed the automaton to enter a quit state if a
// '\n' is observed, this produces a match error instead.
let expected = MatchError::quit(b'\n', 3);
let got = re.try_search(&input).unwrap_err();
assert_eq!(expected, got);

# Ok::<(), Box<dyn std::error::Error>>(())
```

---
