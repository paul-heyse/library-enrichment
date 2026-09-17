# `regex_automata::nfa::thompson::pikevm`

Crate `regex-automata` · 6 public items · structured records in [`model/regex_automata.nfa.thompson.pikevm.json`](../model/regex_automata.nfa.thompson.pikevm.json)

## Builder

`struct` · `regex_automata::nfa::thompson::pikevm::Builder`

```rust
struct Builder
```

**Derives**: Clone, Debug

**Methods** (7)

```rust
fn build(&self, pattern: &str) -> Result<PikeVM, BuildError>
fn build_from_nfa(&self, nfa: NFA) -> Result<PikeVM, BuildError>
fn build_many<P: AsRef<str>>(&self, patterns: &[P]) -> Result<PikeVM, BuildError>
fn configure(&mut self, config: Config) -> &mut Builder
fn new() -> Builder
fn syntax(&mut self, config: util::syntax::Config) -> &mut Builder
fn thompson(&mut self, config: thompson::Config) -> &mut Builder
```

A builder for a `PikeVM`.

This builder permits configuring options for the syntax of a pattern,
the NFA construction and the `PikeVM` construction. This builder is
different from a general purpose regex builder in that it permits fine
grain configuration of the construction process. The trade off for this is
complexity, and the possibility of setting a configuration that might not
make sense. For example, there are two different UTF-8 modes:

* [`util::syntax::Config::utf8`](crate::util::syntax::Config::utf8)
controls whether the pattern itself can contain sub-expressions that match
invalid UTF-8.
* [`thompson::Config::utf8`] controls whether empty matches that split a
Unicode codepoint are reported or not.

Generally speaking, callers will want to either enable all of these or
disable all of these.

# Example

This example shows how to disable UTF-8 mode in the syntax and the regex
itself. This is generally what you want for matching on arbitrary bytes.

```
use regex_automata::{
    nfa::thompson::{self, pikevm::PikeVM},
    util::syntax,
    Match,
};

let re = PikeVM::builder()
    .syntax(syntax::Config::new().utf8(false))
    .thompson(thompson::Config::new().utf8(false))
    .build(r"foo(?-u:[^b])ar.*")?;
let mut cache = re.create_cache();

let haystack = b"\xFEfoo\xFFarzz\xE2\x98\xFF\n";
let expected = Some(Match::must(0, 1..9));
let got = re.find_iter(&mut cache, haystack).next();
assert_eq!(expected, got);
// Notice that `(?-u:[^b])` matches invalid UTF-8,
// but the subsequent `.*` does not! Disabling UTF-8
// on the syntax permits this.
//
// N.B. This example does not show the impact of
// disabling UTF-8 mode on a PikeVM Config, since that
// only impacts regexes that can produce matches of
// length 0.
assert_eq!(b"foo\xFFarzz", &haystack[got.unwrap().range()]);

# Ok::<(), Box<dyn std::error::Error>>(())
```

---

## Cache

`struct` · `regex_automata::nfa::thompson::pikevm::Cache`

```rust
struct Cache
```

**Derives**: Clone, Debug

**Methods** (3)

```rust
fn memory_usage(&self) -> usize
fn new(re: &PikeVM) -> Cache
fn reset(&mut self, re: &PikeVM)
```

A cache represents mutable state that a [`PikeVM`] requires during a
search.

For a given [`PikeVM`], its corresponding cache may be created either via
[`PikeVM::create_cache`], or via [`Cache::new`]. They are equivalent in
every way, except the former does not require explicitly importing `Cache`.

A particular `Cache` is coupled with the [`PikeVM`] from which it
was created. It may only be used with that `PikeVM`. A cache and its
allocations may be re-purposed via [`Cache::reset`], in which case, it can
only be used with the new `PikeVM` (and not the old one).

---

## CapturesMatches

`struct` · `regex_automata::nfa::thompson::pikevm::CapturesMatches`

```rust
struct CapturesMatches<'r, 'c, 'h>
```

**Implements**: `core::iter::traits::iterator::Iterator`

**Derives**: Debug

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<Captures>
```

An iterator over all non-overlapping leftmost matches, with their capturing
groups, for a particular search.

The iterator yields a [`Captures`] value until no more matches could be
found.

The lifetime parameters are as follows:

* `'r` represents the lifetime of the PikeVM.
* `'c` represents the lifetime of the PikeVM's cache.
* `'h` represents the lifetime of the haystack being searched.

This iterator can be created with the [`PikeVM::captures_iter`] method.

---

## Config

`struct` · `regex_automata::nfa::thompson::pikevm::Config`

```rust
struct Config
```

**Derives**: Clone, Debug, Default

**Methods** (5)

```rust
fn get_match_kind(&self) -> MatchKind
fn get_prefilter(&self) -> Option<&Prefilter>
fn match_kind(self, kind: MatchKind) -> Config
fn new() -> Config
fn prefilter(self, pre: Option<Prefilter>) -> Config
```

The configuration used for building a [`PikeVM`].

A PikeVM configuration is a simple data object that is typically used with
[`Builder::configure`]. It can be cheaply cloned.

A default configuration can be created either with `Config::new`, or
perhaps more conveniently, with [`PikeVM::config`].

---

## FindMatches

`struct` · `regex_automata::nfa::thompson::pikevm::FindMatches`

```rust
struct FindMatches<'r, 'c, 'h>
```

**Implements**: `core::iter::traits::iterator::Iterator`

**Derives**: Debug

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<Match>
```

An iterator over all non-overlapping matches for a particular search.

The iterator yields a [`Match`] value until no more matches could be found.

The lifetime parameters are as follows:

* `'r` represents the lifetime of the PikeVM.
* `'c` represents the lifetime of the PikeVM's cache.
* `'h` represents the lifetime of the haystack being searched.

This iterator can be created with the [`PikeVM::find_iter`] method.

---

## PikeVM

`struct` · `regex_automata::nfa::thompson::pikevm::PikeVM`

```rust
struct PikeVM
```

**Derives**: Clone, Debug

**Methods** (21)

```rust
fn always_match() -> Result<PikeVM, BuildError>
fn builder() -> Builder
fn captures<'h, I: Into<Input<'h>>>(&self, cache: &mut Cache, input: I, caps: &mut Captures)
fn captures_iter<'r, 'c, 'h, I: Into<Input<'h>>>(&'r self, cache: &'c mut Cache, input: I) -> CapturesMatches<'r, 'c, 'h>
fn config() -> Config
fn create_cache(&self) -> Cache
fn create_captures(&self) -> Captures
fn find<'h, I: Into<Input<'h>>>(&self, cache: &mut Cache, input: I) -> Option<Match>
fn find_iter<'r, 'c, 'h, I: Into<Input<'h>>>(&'r self, cache: &'c mut Cache, input: I) -> FindMatches<'r, 'c, 'h>
fn get_config(&self) -> &Config
fn get_nfa(&self) -> &NFA
fn is_match<'h, I: Into<Input<'h>>>(&self, cache: &mut Cache, input: I) -> bool
fn never_match() -> Result<PikeVM, BuildError>
fn new(pattern: &str) -> Result<PikeVM, BuildError>
fn new_from_nfa(nfa: NFA) -> Result<PikeVM, BuildError>
fn new_many<P: AsRef<str>>(patterns: &[P]) -> Result<PikeVM, BuildError>
fn pattern_len(&self) -> usize
fn reset_cache(&self, cache: &mut Cache)
fn search(&self, cache: &mut Cache, input: &Input<'_>, caps: &mut Captures)
fn search_slots(&self, cache: &mut Cache, input: &Input<'_>, slots: &mut [Option<NonMaxUsize>]) -> Option<PatternID>
fn which_overlapping_matches(&self, cache: &mut Cache, input: &Input<'_>, patset: &mut PatternSet)
```

A virtual machine for executing regex searches with capturing groups.

# Infallible APIs

Unlike most other regex engines in this crate, a `PikeVM` never returns an
error at search time. It supports all [`Anchored`] configurations, never
quits and works on haystacks of arbitrary length.

There are two caveats to mention though:

* If an invalid pattern ID is given to a search via [`Anchored::Pattern`],
then the PikeVM will report "no match." This is consistent with all other
regex engines in this crate.
* When using [`PikeVM::which_overlapping_matches`] with a [`PatternSet`]
that has insufficient capacity to store all valid pattern IDs, then if a
match occurs for a `PatternID` that cannot be inserted, it is silently
dropped as if it did not match.

# Advice

The `PikeVM` is generally the most "powerful" regex engine in this crate.
"Powerful" in this context means that it can handle any regular expression
that is parseable by `regex-syntax` and any size haystack. Regrettably,
the `PikeVM` is also simultaneously often the _slowest_ regex engine in
practice. This results in an annoying situation where one generally tries
to pick any other regex engine (or perhaps none at all) before being
forced to fall back to a `PikeVM`.

For example, a common strategy for dealing with capturing groups is to
actually look for the overall match of the regex using a faster regex
engine, like a [lazy DFA](crate::hybrid::regex::Regex). Once the overall
match is found, one can then run the `PikeVM` on just the match span to
find the spans of the capturing groups. In this way, the faster regex
engine does the majority of the work, while the `PikeVM` only lends its
power in a more limited role.

Unfortunately, this isn't always possible because the faster regex engines
don't support all of the regex features in `regex-syntax`. This notably
includes (and is currently limited to) Unicode word boundaries. So if
your pattern has Unicode word boundaries, you typically can't use a
DFA-based regex engine at all (unless you [enable heuristic support for
it](crate::hybrid::dfa::Config::unicode_word_boundary)). (The [one-pass
DFA](crate::dfa::onepass::DFA) can handle Unicode word boundaries for
anchored searches only, but in a cruel sort of joke, many Unicode features
tend to result in making the regex _not_ one-pass.)

# Example

This example shows that the `PikeVM` implements Unicode word boundaries
correctly by default.

```
# if cfg!(miri) { return Ok(()); } // miri takes too long
use regex_automata::{nfa::thompson::pikevm::PikeVM, Match};

let re = PikeVM::new(r"\b\w+\b")?;
let mut cache = re.create_cache();

let mut it = re.find_iter(&mut cache, "Шерлок Холмс");
assert_eq!(Some(Match::must(0, 0..12)), it.next());
assert_eq!(Some(Match::must(0, 13..23)), it.next());
assert_eq!(None, it.next());
# Ok::<(), Box<dyn std::error::Error>>(())
```

---
