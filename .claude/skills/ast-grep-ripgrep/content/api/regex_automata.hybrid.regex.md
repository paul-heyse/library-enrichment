# `regex_automata::hybrid::regex`

Crate `regex-automata` · 4 public items · structured records in [`model/regex_automata.hybrid.regex.json`](../model/regex_automata.hybrid.regex.json)

## Builder

`struct` · `regex_automata::hybrid::regex::Builder`

```rust
struct Builder
```

**Derives**: Clone, Debug, Default

**Methods** (7)

```rust
fn build(&self, pattern: &str) -> Result<Regex, BuildError>
fn build_from_dfas(&self, forward: DFA, reverse: DFA) -> Regex
fn build_many<P: AsRef<str>>(&self, patterns: &[P]) -> Result<Regex, BuildError>
fn dfa(&mut self, config: dfa::Config) -> &mut Builder
fn new() -> Builder
fn syntax(&mut self, config: util::syntax::Config) -> &mut Builder
fn thompson(&mut self, config: thompson::Config) -> &mut Builder
```

A builder for a regex based on a hybrid NFA/DFA.

This builder permits configuring options for the syntax of a pattern, the
NFA construction, the lazy DFA construction and finally the regex searching
itself. This builder is different from a general purpose regex builder
in that it permits fine grain configuration of the construction process.
The trade off for this is complexity, and the possibility of setting a
configuration that might not make sense. For example, there are two
different UTF-8 modes:

* [`syntax::Config::utf8`](crate::util::syntax::Config::utf8) controls
whether the pattern itself can contain sub-expressions that match invalid
UTF-8.
* [`thompson::Config::utf8`] controls how the regex iterators themselves
advance the starting position of the next search when a match with zero
length is found.

Generally speaking, callers will want to either enable all of these or
disable all of these.

Internally, building a regex requires building two hybrid NFA/DFAs,
where one is responsible for finding the end of a match and the other is
responsible for finding the start of a match. If you only need to detect
whether something matched, or only the end of a match, then you should use
a [`dfa::Builder`] to construct a single hybrid NFA/DFA, which is cheaper
than building two of them.

# Example

This example shows how to disable UTF-8 mode in the syntax and the regex
itself. This is generally what you want for matching on arbitrary bytes.

```
# if cfg!(miri) { return Ok(()); } // miri takes too long
use regex_automata::{
    hybrid::regex::Regex, nfa::thompson, util::syntax, Match,
};

let re = Regex::builder()
    .syntax(syntax::Config::new().utf8(false))
    .thompson(thompson::Config::new().utf8(false))
    .build(r"foo(?-u:[^b])ar.*")?;
let mut cache = re.create_cache();

let haystack = b"\xFEfoo\xFFarzz\xE2\x98\xFF\n";
let expected = Some(Match::must(0, 1..9));
let got = re.find(&mut cache, haystack);
assert_eq!(expected, got);
// Notice that `(?-u:[^b])` matches invalid UTF-8,
// but the subsequent `.*` does not! Disabling UTF-8
// on the syntax permits this.
assert_eq!(b"foo\xFFarzz", &haystack[got.unwrap().range()]);

# Ok::<(), Box<dyn std::error::Error>>(())
```

---

## Cache

`struct` · `regex_automata::hybrid::regex::Cache`

```rust
struct Cache
```

**Derives**: Clone, Debug

**Methods** (9)

```rust
fn as_parts(&self) -> (&dfa::Cache, &dfa::Cache)
fn as_parts_mut(&mut self) -> (&mut dfa::Cache, &mut dfa::Cache)
fn forward(&mut self) -> &dfa::Cache
fn forward_mut(&mut self) -> &mut dfa::Cache
fn memory_usage(&self) -> usize
fn new(re: &Regex) -> Cache
fn reset(&mut self, re: &Regex)
fn reverse(&mut self) -> &dfa::Cache
fn reverse_mut(&mut self) -> &mut dfa::Cache
```

A cache represents a partially computed forward and reverse DFA.

A cache is the key component that differentiates a classical DFA and a
hybrid NFA/DFA (also called a "lazy DFA"). Where a classical DFA builds a
complete transition table that can handle all possible inputs, a hybrid
NFA/DFA starts with an empty transition table and builds only the parts
required during search. The parts that are built are stored in a cache. For
this reason, a cache is a required parameter for nearly every operation on
a [`Regex`].

Caches can be created from their corresponding `Regex` via
[`Regex::create_cache`]. A cache can only be used with either the `Regex`
that created it, or the `Regex` that was most recently used to reset it
with [`Cache::reset`]. Using a cache with any other `Regex` may result in
panics or incorrect results.

---

## FindMatches

`struct` · `regex_automata::hybrid::regex::FindMatches`

```rust
struct FindMatches<'r, 'c, 'h>
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

The lifetime parameters are as follows:

* `'r` represents the lifetime of the regex object.
* `'h` represents the lifetime of the haystack being searched.
* `'c` represents the lifetime of the regex cache.

This iterator can be created with the [`Regex::find_iter`] method.

---

## Regex

`struct` · `regex_automata::hybrid::regex::Regex`

```rust
struct Regex
```

**Derives**: Debug

**Methods** (12)

```rust
fn builder() -> Builder
fn create_cache(&self) -> Cache
fn find<'h, I: Into<Input<'h>>>(&self, cache: &mut Cache, input: I) -> Option<Match>
fn find_iter<'r, 'c, 'h, I: Into<Input<'h>>>(&'r self, cache: &'c mut Cache, input: I) -> FindMatches<'r, 'c, 'h>
fn forward(&self) -> &DFA
fn is_match<'h, I: Into<Input<'h>>>(&self, cache: &mut Cache, input: I) -> bool
fn new(pattern: &str) -> Result<Regex, BuildError>
fn new_many<P: AsRef<str>>(patterns: &[P]) -> Result<Regex, BuildError>
fn pattern_len(&self) -> usize
fn reset_cache(&self, cache: &mut Cache)
fn reverse(&self) -> &DFA
fn try_search(&self, cache: &mut Cache, input: &Input<'_>) -> Result<Option<Match>, MatchError>
```

A regular expression that uses hybrid NFA/DFAs (also called "lazy DFAs")
for searching.

A regular expression is comprised of two lazy DFAs, a "forward" DFA and a
"reverse" DFA. The forward DFA is responsible for detecting the end of
a match while the reverse DFA is responsible for detecting the start
of a match. Thus, in order to find the bounds of any given match, a
forward search must first be run followed by a reverse search. A match
found by the forward DFA guarantees that the reverse DFA will also find
a match.

# Fallibility

Most of the search routines defined on this type will _panic_ when the
underlying search fails. This might be because the DFA gave up because it
saw a quit byte, whether configured explicitly or via heuristic Unicode
word boundary support, although neither are enabled by default. It might
also fail if the underlying DFA determines it isn't making effective use of
the cache (which also never happens by default). Or it might fail because
an invalid `Input` configuration is given, for example, with an unsupported
[`Anchored`] mode.

If you need to handle these error cases instead of allowing them to trigger
a panic, then the lower level [`Regex::try_search`] provides a fallible API
that never panics.

# Example

This example shows how to cause a search to terminate if it sees a
`\n` byte, and handle the error returned. This could be useful if, for
example, you wanted to prevent a user supplied pattern from matching
across a line boundary.

```
# if cfg!(miri) { return Ok(()); } // miri takes too long
use regex_automata::{hybrid::{dfa, regex::Regex}, Input, MatchError};

let re = Regex::builder()
    .dfa(dfa::Config::new().quit(b'\n', true))
    .build(r"foo\p{any}+bar")?;
let mut cache = re.create_cache();

let input = Input::new("foo\nbar");
// Normally this would produce a match, since \p{any} contains '\n'.
// But since we instructed the automaton to enter a quit state if a
// '\n' is observed, this produces a match error instead.
let expected = MatchError::quit(b'\n', 3);
let got = re.try_search(&mut cache, &input).unwrap_err();
assert_eq!(expected, got);

# Ok::<(), Box<dyn std::error::Error>>(())
```

---
