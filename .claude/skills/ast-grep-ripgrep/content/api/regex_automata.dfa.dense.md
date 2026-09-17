# `regex_automata::dfa::dense`

Crate `regex-automata` · 4 public items · structured records in [`model/regex_automata.dfa.dense.json`](../model/regex_automata.dfa.dense.json)

## BuildError

`struct` · `regex_automata::dfa::dense::BuildError`

```rust
struct BuildError
```

**Implements**: `core::error::Error`, `core::fmt::Display`

**Derives**: Clone, Debug

**Methods** (1)

```rust
fn is_size_limit_exceeded(&self) -> bool
```

**via `core::error::Error`**

```rust
fn source(&self) -> Option<&dyn std::error::Error + 'static>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
```

An error that occurred during the construction of a DFA.

This error does not provide many introspection capabilities. There are
generally only two things you can do with it:

* Obtain a human readable message via its `std::fmt::Display` impl.
* Access an underlying [`nfa::thompson::BuildError`](thompson::BuildError)
type from its `source` method via the `std::error::Error` trait. This error
only occurs when using convenience routines for building a DFA directly
from a pattern string.

When the `std` feature is enabled, this implements the `std::error::Error`
trait.

---

## Builder

`struct` · `regex_automata::dfa::dense::Builder`

```rust
struct Builder
```

**Derives**: Clone, Debug, Default

**Methods** (7)

```rust
fn build(&self, pattern: &str) -> Result<DFA<alloc::vec::Vec<u32>>, BuildError>
fn build_from_nfa(&self, nfa: &thompson::NFA) -> Result<DFA<alloc::vec::Vec<u32>>, BuildError>
fn build_many<P: AsRef<str>>(&self, patterns: &[P]) -> Result<DFA<alloc::vec::Vec<u32>>, BuildError>
fn configure(&mut self, config: Config) -> &mut Builder
fn new() -> Builder
fn syntax(&mut self, config: util::syntax::Config) -> &mut Builder
fn thompson(&mut self, config: thompson::Config) -> &mut Builder
```

A builder for constructing a deterministic finite automaton from regular
expressions.

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

This builder always constructs a *single* DFA. As such, this builder
can only be used to construct regexes that either detect the presence
of a match or find the end location of a match. A single DFA cannot
produce both the start and end of a match. For that information, use a
[`Regex`](crate::dfa::regex::Regex), which can be similarly configured
using [`regex::Builder`](crate::dfa::regex::Builder). The main reason to
use a DFA directly is if the end location of a match is enough for your use
case. Namely, a `Regex` will construct two DFAs instead of one, since a
second reverse DFA is needed to find the start of a match.

Note that if one wants to build a sparse DFA, you must first build a dense
DFA and convert that to a sparse DFA. There is no way to build a sparse
DFA without first building a dense DFA.

# Example

This example shows how to build a minimized DFA that completely disables
Unicode. That is:

* Things such as `\w`, `.` and `\b` are no longer Unicode-aware. `\w`
  and `\b` are ASCII-only while `.` matches any byte except for `\n`
  (instead of any UTF-8 encoding of a Unicode scalar value except for
  `\n`). Things that are Unicode only, such as `\pL`, are not allowed.
* The pattern itself is permitted to match invalid UTF-8. For example,
  things like `[^a]` that match any byte except for `a` are permitted.

```
use regex_automata::{
    dfa::{Automaton, dense},
    util::syntax,
    HalfMatch, Input,
};

let dfa = dense::Builder::new()
    .configure(dense::Config::new().minimize(false))
    .syntax(syntax::Config::new().unicode(false).utf8(false))
    .build(r"foo[^b]ar.*")?;

let haystack = b"\xFEfoo\xFFar\xE2\x98\xFF\n";
let expected = Some(HalfMatch::must(0, 10));
let got = dfa.try_search_fwd(&Input::new(haystack))?;
assert_eq!(expected, got);

# Ok::<(), Box<dyn std::error::Error>>(())
```

---

## Config

`struct` · `regex_automata::dfa::dense::Config`

```rust
struct Config
```

**Derives**: Clone, Debug, Default

**Methods** (25)

```rust
fn accelerate(self, yes: bool) -> Config
fn byte_classes(self, yes: bool) -> Config
fn determinize_size_limit(self, bytes: Option<usize>) -> Config
fn dfa_size_limit(self, bytes: Option<usize>) -> Config
fn get_accelerate(&self) -> bool
fn get_byte_classes(&self) -> bool
fn get_determinize_size_limit(&self) -> Option<usize>
fn get_dfa_size_limit(&self) -> Option<usize>
fn get_match_kind(&self) -> MatchKind
fn get_minimize(&self) -> bool
fn get_prefilter(&self) -> Option<&Prefilter>
fn get_quit(&self, byte: u8) -> bool
fn get_specialize_start_states(&self) -> bool
fn get_starts(&self) -> StartKind
fn get_starts_for_each_pattern(&self) -> bool
fn get_unicode_word_boundary(&self) -> bool
fn match_kind(self, kind: MatchKind) -> Config
fn minimize(self, yes: bool) -> Config
fn new() -> Config
fn prefilter(self, pre: Option<Prefilter>) -> Config
fn quit(self, byte: u8, yes: bool) -> Config
fn specialize_start_states(self, yes: bool) -> Config
fn start_kind(self, kind: StartKind) -> Config
fn starts_for_each_pattern(self, yes: bool) -> Config
fn unicode_word_boundary(self, yes: bool) -> Config
```

The configuration used for compiling a dense DFA.

As a convenience, [`DFA::config`] is an alias for [`Config::new`]. The
advantage of the former is that it often lets you avoid importing the
`Config` type directly.

A dense DFA configuration is a simple data object that is typically used
with [`dense::Builder::configure`](self::Builder::configure).

The default configuration guarantees that a search will never return
a "quit" error, although it is possible for a search to fail if
[`Config::starts_for_each_pattern`] wasn't enabled (which it is
not by default) and an [`Anchored::Pattern`] mode is requested via
[`Input`](crate::Input).

---

## DFA

`struct` · `regex_automata::dfa::dense::DFA`

```rust
struct DFA<T>
```

**Implements**: `regex_automata::dfa::automaton::Automaton`

**Derives**: Clone, Debug

**Methods** (26)

```rust
fn alphabet_len(&self) -> usize
fn always_match() -> Result<DFA<alloc::vec::Vec<u32>>, BuildError>
fn as_ref(&self) -> DFA<&[u32]>
fn builder() -> Builder
fn byte_classes(&self) -> &ByteClasses
fn config() -> Config
fn from_bytes(slice: &'a [u8]) -> Result<(DFA<&'a [u32]>, usize), DeserializeError>
unsafe fn from_bytes_unchecked(slice: &'a [u8]) -> Result<(DFA<&'a [u32]>, usize), DeserializeError>
fn memory_usage(&self) -> usize
fn never_match() -> Result<DFA<alloc::vec::Vec<u32>>, BuildError>
fn new(pattern: &str) -> Result<DFA<alloc::vec::Vec<u32>>, BuildError>
fn new_many<P: AsRef<str>>(patterns: &[P]) -> Result<DFA<alloc::vec::Vec<u32>>, BuildError>
fn set_prefilter(&mut self, prefilter: Option<Prefilter>)
fn start_kind(&self) -> StartKind
fn starts_for_each_pattern(&self) -> bool
fn stride(&self) -> usize
fn stride2(&self) -> usize
fn to_bytes_big_endian(&self) -> (Vec<u8>, usize)
fn to_bytes_little_endian(&self) -> (Vec<u8>, usize)
fn to_bytes_native_endian(&self) -> (Vec<u8>, usize)
fn to_owned(&self) -> DFA<alloc::vec::Vec<u32>>
fn to_sparse(&self) -> Result<sparse::DFA<Vec<u8>>, BuildError>
fn write_to_big_endian(&self, dst: &mut [u8]) -> Result<usize, SerializeError>
fn write_to_len(&self) -> usize
fn write_to_little_endian(&self, dst: &mut [u8]) -> Result<usize, SerializeError>
fn write_to_native_endian(&self, dst: &mut [u8]) -> Result<usize, SerializeError>
```

**via `regex_automata::dfa::automaton::Automaton`**

```rust
fn accelerator(&self, id: StateID) -> &[u8]
fn get_prefilter(&self) -> Option<&Prefilter>
fn has_empty(&self) -> bool
fn is_accel_state(&self, id: StateID) -> bool
fn is_always_start_anchored(&self) -> bool
fn is_dead_state(&self, id: StateID) -> bool
fn is_match_state(&self, id: StateID) -> bool
fn is_quit_state(&self, id: StateID) -> bool
fn is_special_state(&self, id: StateID) -> bool
fn is_start_state(&self, id: StateID) -> bool
fn is_utf8(&self) -> bool
fn match_len(&self, id: StateID) -> usize
fn match_pattern(&self, id: StateID, match_index: usize) -> PatternID
fn next_eoi_state(&self, current: StateID) -> StateID
fn next_state(&self, current: StateID, input: u8) -> StateID
unsafe fn next_state_unchecked(&self, current: StateID, byte: u8) -> StateID
fn pattern_len(&self) -> usize
fn start_state(&self, config: &start::Config) -> Result<StateID, StartError>
fn universal_start_state(&self, mode: Anchored) -> Option<StateID>
```

A dense table-based deterministic finite automaton (DFA).

All dense DFAs have one or more start states, zero or more match states
and a transition table that maps the current state and the current byte
of input to the next state. A DFA can use this information to implement
fast searching. In particular, the use of a dense DFA generally makes the
trade off that match speed is the most valuable characteristic, even if
building the DFA may take significant time *and* space. (More concretely,
building a DFA takes time and space that is exponential in the size of the
pattern in the worst case.) As such, the processing of every byte of input
is done with a small constant number of operations that does not vary with
the pattern, its size or the size of the alphabet. If your needs don't line
up with this trade off, then a dense DFA may not be an adequate solution to
your problem.

In contrast, a [`sparse::DFA`] makes the opposite
trade off: it uses less space but will execute a variable number of
instructions per byte at match time, which makes it slower for matching.
(Note that space usage is still exponential in the size of the pattern in
the worst case.)

A DFA can be built using the default configuration via the
[`DFA::new`] constructor. Otherwise, one can
configure various aspects via [`dense::Builder`](Builder).

A single DFA fundamentally supports the following operations:

1. Detection of a match.
2. Location of the end of a match.
3. In the case of a DFA with multiple patterns, which pattern matched is
   reported as well.

A notable absence from the above list of capabilities is the location of
the *start* of a match. In order to provide both the start and end of
a match, *two* DFAs are required. This functionality is provided by a
[`Regex`](crate::dfa::regex::Regex).

# Type parameters

A `DFA` has one type parameter, `T`, which is used to represent state IDs,
pattern IDs and accelerators. `T` is typically a `Vec<u32>` or a `&[u32]`.

# The `Automaton` trait

This type implements the [`Automaton`] trait, which means it can be used
for searching. For example:

```
use regex_automata::{dfa::{Automaton, dense::DFA}, HalfMatch, Input};

let dfa = DFA::new("foo[0-9]+")?;
let expected = HalfMatch::must(0, 8);
assert_eq!(Some(expected), dfa.try_search_fwd(&Input::new("foo12345"))?);
# Ok::<(), Box<dyn std::error::Error>>(())
```

---
