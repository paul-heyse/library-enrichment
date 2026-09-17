# `regex_automata::nfa::thompson::backtrack`

Crate `regex-automata` · 7 public items · structured records in [`model/regex_automata.nfa.thompson.backtrack.json`](../model/regex_automata.nfa.thompson.backtrack.json)

## min_visited_capacity

`function` · `regex_automata::nfa::thompson::backtrack::min_visited_capacity`

```rust
fn min_visited_capacity(nfa: &nfa::thompson::NFA, input: &util::search::Input<'_>) -> usize
```

Returns the minimum visited capacity for the given haystack.

This function can be used as the argument to [`Config::visited_capacity`]
in order to guarantee that a backtracking search for the given `input`
won't return an error when using a [`BoundedBacktracker`] built from the
given `NFA`.

This routine exists primarily as a way to test that the bounded backtracker
works correctly when its capacity is set to the smallest possible amount.
Still, it may be useful in cases where you know you want to use the bounded
backtracker for a specific input, and just need to know what visited
capacity to provide to make it work.

Be warned that this number could be quite large as it is multiplicative in
the size the given NFA and haystack.

---

## BoundedBacktracker

`struct` · `regex_automata::nfa::thompson::backtrack::BoundedBacktracker`

```rust
struct BoundedBacktracker
```

**Derives**: Clone, Debug

**Methods** (21)

```rust
fn always_match() -> Result<BoundedBacktracker, BuildError>
fn builder() -> Builder
fn config() -> Config
fn create_cache(&self) -> Cache
fn create_captures(&self) -> Captures
fn get_config(&self) -> &Config
fn get_nfa(&self) -> &NFA
fn max_haystack_len(&self) -> usize
fn never_match() -> Result<BoundedBacktracker, BuildError>
fn new(pattern: &str) -> Result<BoundedBacktracker, BuildError>
fn new_from_nfa(nfa: NFA) -> Result<BoundedBacktracker, BuildError>
fn new_many<P: AsRef<str>>(patterns: &[P]) -> Result<BoundedBacktracker, BuildError>
fn pattern_len(&self) -> usize
fn reset_cache(&self, cache: &mut Cache)
fn try_captures<'h, I: Into<Input<'h>>>(&self, cache: &mut Cache, input: I, caps: &mut Captures) -> Result<(), MatchError>
fn try_captures_iter<'r, 'c, 'h, I: Into<Input<'h>>>(&'r self, cache: &'c mut Cache, input: I) -> TryCapturesMatches<'r, 'c, 'h>
fn try_find<'h, I: Into<Input<'h>>>(&self, cache: &mut Cache, input: I) -> Result<Option<Match>, MatchError>
fn try_find_iter<'r, 'c, 'h, I: Into<Input<'h>>>(&'r self, cache: &'c mut Cache, input: I) -> TryFindMatches<'r, 'c, 'h>
fn try_is_match<'h, I: Into<Input<'h>>>(&self, cache: &mut Cache, input: I) -> Result<bool, MatchError>
fn try_search(&self, cache: &mut Cache, input: &Input<'_>, caps: &mut Captures) -> Result<(), MatchError>
fn try_search_slots(&self, cache: &mut Cache, input: &Input<'_>, slots: &mut [Option<NonMaxUsize>]) -> Result<Option<PatternID>, MatchError>
```

A backtracking regex engine that bounds its execution to avoid exponential
blow-up.

This regex engine only implements leftmost-first match semantics and
only supports leftmost searches. It effectively does the same thing as a
[`PikeVM`](thompson::pikevm::PikeVM), but typically does it faster because
it doesn't have to worry about copying capturing group spans for most NFA
states. Instead, the backtracker can maintain one set of captures (provided
by the caller) and never needs to copy them. In exchange, the backtracker
bounds itself to ensure it doesn't exhibit worst case exponential time.
This results in the backtracker only being able to handle short haystacks
given reasonable memory usage.

# Searches may return an error!

By design, this backtracking regex engine is bounded. This bound is
implemented by not visiting any combination of NFA state ID and position
in a haystack more than once. Thus, the total memory required to bound
backtracking is proportional to `haystack.len() * nfa.states().len()`.
This can obviously get quite large, since large haystacks aren't terribly
uncommon. To avoid using exorbitant memory, the capacity is bounded by
a fixed limit set via [`Config::visited_capacity`]. Thus, if the total
capacity required for a particular regex and a haystack exceeds this
capacity, then the search routine will return an error.

Unlike other regex engines that may return an error at search time (like
the DFA or the hybrid NFA/DFA), there is no way to guarantee that a bounded
backtracker will work for every haystack. Therefore, this regex engine
_only_ exposes fallible search routines to avoid the footgun of panicking
when running a search on a haystack that is too big.

If one wants to use the fallible search APIs without handling the
error, the only way to guarantee an error won't occur from the
haystack length is to ensure the haystack length does not exceed
[`BoundedBacktracker::max_haystack_len`].

# Example: Unicode word boundaries

This example shows that the bounded backtracker implements Unicode word
boundaries correctly by default.

```
# if cfg!(miri) { return Ok(()); } // miri takes too long
use regex_automata::{nfa::thompson::backtrack::BoundedBacktracker, Match};

let re = BoundedBacktracker::new(r"\b\w+\b")?;
let mut cache = re.create_cache();

let mut it = re.try_find_iter(&mut cache, "Шерлок Холмс");
assert_eq!(Some(Ok(Match::must(0, 0..12))), it.next());
assert_eq!(Some(Ok(Match::must(0, 13..23))), it.next());
assert_eq!(None, it.next());
# Ok::<(), Box<dyn std::error::Error>>(())
```

# Example: multiple regex patterns

The bounded backtracker supports searching for multiple patterns
simultaneously, just like other regex engines. Note though that because it
uses a backtracking strategy, this regex engine is unlikely to scale well
as more patterns are added. But then again, as more patterns are added, the
maximum haystack length allowed will also shorten (assuming the visited
capacity remains invariant).

```
use regex_automata::{nfa::thompson::backtrack::BoundedBacktracker, Match};

let re = BoundedBacktracker::new_many(&["[a-z]+", "[0-9]+"])?;
let mut cache = re.create_cache();

let mut it = re.try_find_iter(&mut cache, "abc 1 foo 4567 0 quux");
assert_eq!(Some(Ok(Match::must(0, 0..3))), it.next());
assert_eq!(Some(Ok(Match::must(1, 4..5))), it.next());
assert_eq!(Some(Ok(Match::must(0, 6..9))), it.next());
assert_eq!(Some(Ok(Match::must(1, 10..14))), it.next());
assert_eq!(Some(Ok(Match::must(1, 15..16))), it.next());
assert_eq!(Some(Ok(Match::must(0, 17..21))), it.next());
assert_eq!(None, it.next());
# Ok::<(), Box<dyn std::error::Error>>(())
```

---

## Builder

`struct` · `regex_automata::nfa::thompson::backtrack::Builder`

```rust
struct Builder
```

**Derives**: Clone, Debug

**Methods** (7)

```rust
fn build(&self, pattern: &str) -> Result<BoundedBacktracker, BuildError>
fn build_from_nfa(&self, nfa: NFA) -> Result<BoundedBacktracker, BuildError>
fn build_many<P: AsRef<str>>(&self, patterns: &[P]) -> Result<BoundedBacktracker, BuildError>
fn configure(&mut self, config: Config) -> &mut Builder
fn new() -> Builder
fn syntax(&mut self, config: util::syntax::Config) -> &mut Builder
fn thompson(&mut self, config: thompson::Config) -> &mut Builder
```

A builder for a bounded backtracker.

This builder permits configuring options for the syntax of a pattern, the
NFA construction and the `BoundedBacktracker` construction. This builder
is different from a general purpose regex builder in that it permits fine
grain configuration of the construction process. The trade off for this is
complexity, and the possibility of setting a configuration that might not
make sense. For example, there are two different UTF-8 modes:

* [`syntax::Config::utf8`](crate::util::syntax::Config::utf8) controls
whether the pattern itself can contain sub-expressions that match invalid
UTF-8.
* [`thompson::Config::utf8`] controls how the regex iterators themselves
advance the starting position of the next search when a match with zero
length is found.

Generally speaking, callers will want to either enable all of these or
disable all of these.

# Example

This example shows how to disable UTF-8 mode in the syntax and the regex
itself. This is generally what you want for matching on arbitrary bytes.

```
use regex_automata::{
    nfa::thompson::{self, backtrack::BoundedBacktracker},
    util::syntax,
    Match,
};

let re = BoundedBacktracker::builder()
    .syntax(syntax::Config::new().utf8(false))
    .thompson(thompson::Config::new().utf8(false))
    .build(r"foo(?-u:[^b])ar.*")?;
let mut cache = re.create_cache();

let haystack = b"\xFEfoo\xFFarzz\xE2\x98\xFF\n";
let expected = Some(Ok(Match::must(0, 1..9)));
let got = re.try_find_iter(&mut cache, haystack).next();
assert_eq!(expected, got);
// Notice that `(?-u:[^b])` matches invalid UTF-8,
// but the subsequent `.*` does not! Disabling UTF-8
// on the syntax permits this.
//
// N.B. This example does not show the impact of
// disabling UTF-8 mode on a BoundedBacktracker Config, since that
// only impacts regexes that can produce matches of
// length 0.
assert_eq!(b"foo\xFFarzz", &haystack[got.unwrap()?.range()]);

# Ok::<(), Box<dyn std::error::Error>>(())
```

---

## Cache

`struct` · `regex_automata::nfa::thompson::backtrack::Cache`

```rust
struct Cache
```

**Derives**: Clone, Debug

**Methods** (3)

```rust
fn memory_usage(&self) -> usize
fn new(re: &BoundedBacktracker) -> Cache
fn reset(&mut self, re: &BoundedBacktracker)
```

A cache represents mutable state that a [`BoundedBacktracker`] requires
during a search.

For a given [`BoundedBacktracker`], its corresponding cache may be created
either via [`BoundedBacktracker::create_cache`], or via [`Cache::new`].
They are equivalent in every way, except the former does not require
explicitly importing `Cache`.

A particular `Cache` is coupled with the [`BoundedBacktracker`] from which
it was created. It may only be used with that `BoundedBacktracker`. A cache
and its allocations may be re-purposed via [`Cache::reset`], in which case,
it can only be used with the new `BoundedBacktracker` (and not the old
one).

---

## Config

`struct` · `regex_automata::nfa::thompson::backtrack::Config`

```rust
struct Config
```

**Derives**: Clone, Debug, Default

**Methods** (5)

```rust
fn get_prefilter(&self) -> Option<&Prefilter>
fn get_visited_capacity(&self) -> usize
fn new() -> Config
fn prefilter(self, pre: Option<Prefilter>) -> Config
fn visited_capacity(self, capacity: usize) -> Config
```

The configuration used for building a bounded backtracker.

A bounded backtracker configuration is a simple data object that is
typically used with [`Builder::configure`].

---

## TryCapturesMatches

`struct` · `regex_automata::nfa::thompson::backtrack::TryCapturesMatches`

```rust
struct TryCapturesMatches<'r, 'c, 'h>
```

**Implements**: `core::iter::traits::iterator::Iterator`

**Derives**: Debug

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<Result<Captures, MatchError>>
```

An iterator over all non-overlapping leftmost matches, with their capturing
groups, for a fallible search.

The iterator yields a `Result<Captures, MatchError>` value until no more
matches could be found.

The lifetime parameters are as follows:

* `'r` represents the lifetime of the BoundedBacktracker.
* `'c` represents the lifetime of the BoundedBacktracker's cache.
* `'h` represents the lifetime of the haystack being searched.

This iterator can be created with the
[`BoundedBacktracker::try_captures_iter`] method.

---

## TryFindMatches

`struct` · `regex_automata::nfa::thompson::backtrack::TryFindMatches`

```rust
struct TryFindMatches<'r, 'c, 'h>
```

**Implements**: `core::iter::traits::iterator::Iterator`

**Derives**: Debug

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<Result<Match, MatchError>>
```

An iterator over all non-overlapping matches for a fallible search.

The iterator yields a `Result<Match, MatchError` value until no more
matches could be found.

The lifetime parameters are as follows:

* `'r` represents the lifetime of the BoundedBacktracker.
* `'c` represents the lifetime of the BoundedBacktracker's cache.
* `'h` represents the lifetime of the haystack being searched.

This iterator can be created with the [`BoundedBacktracker::try_find_iter`]
method.

---
