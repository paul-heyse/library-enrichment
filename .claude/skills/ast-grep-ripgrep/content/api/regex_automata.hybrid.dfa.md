# `regex_automata::hybrid::dfa`

Crate `regex-automata` · 5 public items · structured records in [`model/regex_automata.hybrid.dfa.json`](../model/regex_automata.hybrid.dfa.json)

## Builder

`struct` · `regex_automata::hybrid::dfa::Builder`

```rust
struct Builder
```

**Derives**: Clone, Debug

**Methods** (7)

```rust
fn build(&self, pattern: &str) -> Result<DFA, BuildError>
fn build_from_nfa(&self, nfa: thompson::NFA) -> Result<DFA, BuildError>
fn build_many<P: AsRef<str>>(&self, patterns: &[P]) -> Result<DFA, BuildError>
fn configure(&mut self, config: Config) -> &mut Builder
fn new() -> Builder
fn syntax(&mut self, config: util::syntax::Config) -> &mut Builder
fn thompson(&mut self, config: thompson::Config) -> &mut Builder
```

A builder for constructing a lazy deterministic finite automaton from
regular expressions.

As a convenience, [`DFA::builder`] is an alias for [`Builder::new`]. The
advantage of the former is that it often lets you avoid importing the
`Builder` type directly.

This builder provides two main things:

1. It provides a few different `build` routines for actually constructing
a DFA from different kinds of inputs. The most convenient is
[`Builder::build`], which builds a DFA directly from a pattern string. The
most flexible is [`Builder::build_from_nfa`], which builds a DFA straight
from an NFA.
2. The builder permits configuring a number of things.
[`Builder::configure`] is used with [`Config`] to configure aspects of
the DFA and the construction process itself. [`Builder::syntax`] and
[`Builder::thompson`] permit configuring the regex parser and Thompson NFA
construction, respectively. The syntax and thompson configurations only
apply when building from a pattern string.

This builder always constructs a *single* lazy DFA. As such, this builder
can only be used to construct regexes that either detect the presence
of a match or find the end location of a match. A single DFA cannot
produce both the start and end of a match. For that information, use a
[`Regex`](crate::hybrid::regex::Regex), which can be similarly configured
using [`regex::Builder`](crate::hybrid::regex::Builder). The main reason
to use a DFA directly is if the end location of a match is enough for your
use case. Namely, a `Regex` will construct two lazy DFAs instead of one,
since a second reverse DFA is needed to find the start of a match.

# Example

This example shows how to build a lazy DFA that uses a tiny cache capacity
and completely disables Unicode. That is:

* Things such as `\w`, `.` and `\b` are no longer Unicode-aware. `\w`
  and `\b` are ASCII-only while `.` matches any byte except for `\n`
  (instead of any UTF-8 encoding of a Unicode scalar value except for
  `\n`). Things that are Unicode only, such as `\pL`, are not allowed.
* The pattern itself is permitted to match invalid UTF-8. For example,
  things like `[^a]` that match any byte except for `a` are permitted.

```
use regex_automata::{
    hybrid::dfa::DFA,
    nfa::thompson,
    util::syntax,
    HalfMatch, Input,
};

let dfa = DFA::builder()
    .configure(DFA::config().cache_capacity(5_000))
    .thompson(thompson::Config::new().utf8(false))
    .syntax(syntax::Config::new().unicode(false).utf8(false))
    .build(r"foo[^b]ar.*")?;
let mut cache = dfa.create_cache();

let haystack = b"\xFEfoo\xFFar\xE2\x98\xFF\n";
let expected = Some(HalfMatch::must(0, 10));
let got = dfa.try_search_fwd(&mut cache, &Input::new(haystack))?;
assert_eq!(expected, got);

# Ok::<(), Box<dyn std::error::Error>>(())
```

---

## Cache

`struct` · `regex_automata::hybrid::dfa::Cache`

```rust
struct Cache
```

**Derives**: Clone, Debug

**Methods** (8)

```rust
fn clear_count(&self) -> usize
fn memory_usage(&self) -> usize
fn new(dfa: &DFA) -> Cache
fn reset(&mut self, dfa: &DFA)
fn search_finish(&mut self, at: usize)
fn search_start(&mut self, at: usize)
fn search_total_len(&self) -> usize
fn search_update(&mut self, at: usize)
```

A cache represents a partially computed DFA.

A cache is the key component that differentiates a classical DFA and a
hybrid NFA/DFA (also called a "lazy DFA"). Where a classical DFA builds a
complete transition table that can handle all possible inputs, a hybrid
NFA/DFA starts with an empty transition table and builds only the parts
required during search. The parts that are built are stored in a cache. For
this reason, a cache is a required parameter for nearly every operation on
a [`DFA`].

Caches can be created from their corresponding DFA via
[`DFA::create_cache`]. A cache can only be used with either the DFA that
created it, or the DFA that was most recently used to reset it with
[`Cache::reset`]. Using a cache with any other DFA may result in panics
or incorrect results.

---

## Config

`struct` · `regex_automata::hybrid::dfa::Config`

```rust
struct Config
```

**Derives**: Clone, Debug, Default

**Methods** (24)

```rust
fn byte_classes(self, yes: bool) -> Config
fn cache_capacity(self, bytes: usize) -> Config
fn get_byte_classes(&self) -> bool
fn get_cache_capacity(&self) -> usize
fn get_match_kind(&self) -> MatchKind
fn get_minimum_bytes_per_state(&self) -> Option<usize>
fn get_minimum_cache_capacity(&self, nfa: &thompson::NFA) -> Result<usize, BuildError>
fn get_minimum_cache_clear_count(&self) -> Option<usize>
fn get_prefilter(&self) -> Option<&Prefilter>
fn get_quit(&self, byte: u8) -> bool
fn get_skip_cache_capacity_check(&self) -> bool
fn get_specialize_start_states(&self) -> bool
fn get_starts_for_each_pattern(&self) -> bool
fn get_unicode_word_boundary(&self) -> bool
fn match_kind(self, kind: MatchKind) -> Config
fn minimum_bytes_per_state(self, min: Option<usize>) -> Config
fn minimum_cache_clear_count(self, min: Option<usize>) -> Config
fn new() -> Config
fn prefilter(self, pre: Option<Prefilter>) -> Config
fn quit(self, byte: u8, yes: bool) -> Config
fn skip_cache_capacity_check(self, yes: bool) -> Config
fn specialize_start_states(self, yes: bool) -> Config
fn starts_for_each_pattern(self, yes: bool) -> Config
fn unicode_word_boundary(self, yes: bool) -> Config
```

The configuration used for building a lazy DFA.

As a convenience, [`DFA::config`] is an alias for [`Config::new`]. The
advantage of the former is that it often lets you avoid importing the
`Config` type directly.

A lazy DFA configuration is a simple data object that is typically used
with [`Builder::configure`].

The default configuration guarantees that a search will never return a
"gave up" or "quit" error, although it is possible for a search to fail
if [`Config::starts_for_each_pattern`] wasn't enabled (which it is not by
default) and an [`Anchored::Pattern`] mode is requested via [`Input`].

---

## DFA

`struct` · `regex_automata::hybrid::dfa::DFA`

```rust
struct DFA
```

**Derives**: Clone, Debug

**Methods** (27)

```rust
fn always_match() -> Result<DFA, BuildError>
fn builder() -> Builder
fn byte_classes(&self) -> &ByteClasses
fn config() -> Config
fn create_cache(&self) -> Cache
fn get_config(&self) -> &Config
fn get_nfa(&self) -> &thompson::NFA
fn match_len(&self, cache: &Cache, id: LazyStateID) -> usize
fn match_pattern(&self, cache: &Cache, id: LazyStateID, match_index: usize) -> PatternID
fn memory_usage(&self) -> usize
fn never_match() -> Result<DFA, BuildError>
fn new(pattern: &str) -> Result<DFA, BuildError>
fn new_many<P: AsRef<str>>(patterns: &[P]) -> Result<DFA, BuildError>
fn next_eoi_state(&self, cache: &mut Cache, current: LazyStateID) -> Result<LazyStateID, CacheError>
fn next_state(&self, cache: &mut Cache, current: LazyStateID, input: u8) -> Result<LazyStateID, CacheError>
fn next_state_untagged(&self, cache: &Cache, current: LazyStateID, input: u8) -> LazyStateID
unsafe fn next_state_untagged_unchecked(&self, cache: &Cache, current: LazyStateID, input: u8) -> LazyStateID
fn pattern_len(&self) -> usize
fn reset_cache(&self, cache: &mut Cache)
fn start_state(&self, cache: &mut Cache, config: &start::Config) -> Result<LazyStateID, StartError>
fn start_state_forward(&self, cache: &mut Cache, input: &Input<'_>) -> Result<LazyStateID, MatchError>
fn start_state_reverse(&self, cache: &mut Cache, input: &Input<'_>) -> Result<LazyStateID, MatchError>
fn try_search_fwd(&self, cache: &mut Cache, input: &Input<'_>) -> Result<Option<HalfMatch>, MatchError>
fn try_search_overlapping_fwd(&self, cache: &mut Cache, input: &Input<'_>, state: &mut OverlappingState) -> Result<(), MatchError>
fn try_search_overlapping_rev(&self, cache: &mut Cache, input: &Input<'_>, state: &mut OverlappingState) -> Result<(), MatchError>
fn try_search_rev(&self, cache: &mut Cache, input: &Input<'_>) -> Result<Option<HalfMatch>, MatchError>
fn try_which_overlapping_matches(&self, cache: &mut Cache, input: &Input<'_>, patset: &mut PatternSet) -> Result<(), MatchError>
```

A hybrid NFA/DFA (also called a "lazy DFA") for regex searching.

A lazy DFA is a DFA that builds itself at search time. It otherwise has
very similar characteristics as a [`dense::DFA`](crate::dfa::dense::DFA).
Indeed, both support precisely the same regex features with precisely the
same semantics.

Where as a `dense::DFA` must be completely built to handle any input before
it may be used for search, a lazy DFA starts off effectively empty. During
a search, a lazy DFA will build itself depending on whether it has already
computed the next transition or not. If it has, then it looks a lot like
a `dense::DFA` internally: it does a very fast table based access to find
the next transition. Otherwise, if the state hasn't been computed, then it
does determinization _for that specific transition_ to compute the next DFA
state.

The main selling point of a lazy DFA is that, in practice, it has
the performance profile of a `dense::DFA` without the weakness of it
taking worst case exponential time to build. Indeed, for each byte of
input, the lazy DFA will construct as most one new DFA state. Thus, a
lazy DFA achieves worst case `O(mn)` time for regex search (where `m ~
pattern.len()` and `n ~ haystack.len()`).

The main downsides of a lazy DFA are:

1. It requires mutable "cache" space during search. This is where the
transition table, among other things, is stored.
2. In pathological cases (e.g., if the cache is too small), it will run
out of room and either require a bigger cache capacity or will repeatedly
clear the cache and thus repeatedly regenerate DFA states. Overall, this
will tend to be slower than a typical NFA simulation.

# Capabilities

Like a `dense::DFA`, a single lazy DFA fundamentally supports the following
operations:

1. Detection of a match.
2. Location of the end of a match.
3. In the case of a lazy DFA with multiple patterns, which pattern matched
is reported as well.

A notable absence from the above list of capabilities is the location of
the *start* of a match. In order to provide both the start and end of
a match, *two* lazy DFAs are required. This functionality is provided by a
[`Regex`](crate::hybrid::regex::Regex).

# Example

This shows how to build a lazy DFA with the default configuration and
execute a search. Notice how, in contrast to a `dense::DFA`, we must create
a cache and pass it to our search routine.

```
use regex_automata::{hybrid::dfa::DFA, HalfMatch, Input};

let dfa = DFA::new("foo[0-9]+")?;
let mut cache = dfa.create_cache();

let expected = Some(HalfMatch::must(0, 8));
assert_eq!(expected, dfa.try_search_fwd(
    &mut cache, &Input::new("foo12345"))?,
);
# Ok::<(), Box<dyn std::error::Error>>(())
```

---

## OverlappingState

`struct` · `regex_automata::hybrid::dfa::OverlappingState`

```rust
struct OverlappingState
```

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn get_match(&self) -> Option<HalfMatch>
fn start() -> OverlappingState
```

Represents the current state of an overlapping search.

This is used for overlapping searches since they need to know something
about the previous search. For example, when multiple patterns match at the
same position, this state tracks the last reported pattern so that the next
search knows whether to report another matching pattern or continue with
the search at the next position. Additionally, it also tracks which state
the last search call terminated in.

This type provides little introspection capabilities. The only thing a
caller can do is construct it and pass it around to permit search routines
to use it to track state, and also ask whether a match has been found.

Callers should always provide a fresh state constructed via
[`OverlappingState::start`] when starting a new search. Reusing state from
a previous search may result in incorrect results.

---
