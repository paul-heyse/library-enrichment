# `aho_corasick::dfa`

Crate `aho-corasick` · 2 public items · structured records in [`model/aho_corasick.dfa.json`](../model/aho_corasick.dfa.json)

## Builder

`struct` · `aho_corasick::dfa::Builder`

```rust
struct Builder
```

**Derives**: Clone, Debug, Default

**Methods** (8)

```rust
fn ascii_case_insensitive(&mut self, yes: bool) -> &mut Builder
fn build<I, P>(&self, patterns: I) -> Result<DFA, BuildError> where I: IntoIterator<Item = P>, P: AsRef<[u8]>
fn build_from_noncontiguous(&self, nnfa: &noncontiguous::NFA) -> Result<DFA, BuildError>
fn byte_classes(&mut self, yes: bool) -> &mut Builder
fn match_kind(&mut self, kind: MatchKind) -> &mut Builder
fn new() -> Builder
fn prefilter(&mut self, yes: bool) -> &mut Builder
fn start_kind(&mut self, kind: StartKind) -> &mut Builder
```

A builder for configuring an Aho-Corasick DFA.

This builder has a subset of the options available to a
[`AhoCorasickBuilder`](crate::AhoCorasickBuilder). Of the shared options,
their behavior is identical.

---

## DFA

`struct` · `aho_corasick::dfa::DFA`

```rust
struct DFA
```

**Implements**: `aho_corasick::automaton::Automaton`

**Derives**: Clone, Debug

**Methods** (2)

```rust
fn builder() -> Builder
fn new<I, P>(patterns: I) -> Result<DFA, BuildError> where I: IntoIterator<Item = P>, P: AsRef<[u8]>
```

**via `aho_corasick::automaton::Automaton`**

```rust
fn is_dead(&self, sid: StateID) -> bool
fn is_match(&self, sid: StateID) -> bool
fn is_special(&self, sid: StateID) -> bool
fn is_start(&self, sid: StateID) -> bool
fn match_kind(&self) -> MatchKind
fn match_len(&self, sid: StateID) -> usize
fn match_pattern(&self, sid: StateID, index: usize) -> PatternID
fn max_pattern_len(&self) -> usize
fn memory_usage(&self) -> usize
fn min_pattern_len(&self) -> usize
fn next_state(&self, _anchored: Anchored, sid: StateID, byte: u8) -> StateID
fn pattern_len(&self, pid: PatternID) -> usize
fn patterns_len(&self) -> usize
fn prefilter(&self) -> Option<&Prefilter>
fn start_state(&self, anchored: Anchored) -> Result<StateID, MatchError>
```

A DFA implementation of Aho-Corasick.

When possible, prefer using [`AhoCorasick`](crate::AhoCorasick) instead of
this type directly. Using a `DFA` directly is typically only necessary when
one needs access to the [`Automaton`] trait implementation.

This DFA can only be built by first constructing a [`noncontiguous::NFA`].
Both [`DFA::new`] and [`Builder::build`] do this for you automatically, but
[`Builder::build_from_noncontiguous`] permits doing it explicitly.

A DFA provides the best possible search performance (in this crate) via two
mechanisms:

* All states use a dense representation for their transitions.
* All failure transitions are pre-computed such that they are never
explicitly handled at search time.

These two facts combined mean that every state transition is performed
using a constant number of instructions. However, this comes at
great cost. The memory usage of a DFA can be quite exorbitant.
It is potentially multiple orders of magnitude greater than a
[`contiguous::NFA`](crate::nfa::contiguous::NFA) for example. In exchange,
a DFA will typically have better search speed than a `contiguous::NFA`, but
not by orders of magnitude.

Unless you have a small number of patterns or memory usage is not a concern
and search performance is critical, a DFA is usually not the best choice.

Moreover, unlike the NFAs in this crate, it is costly for a DFA to
support for anchored and unanchored search configurations. Namely,
since failure transitions are pre-computed, supporting both anchored
and unanchored searches requires a duplication of the transition table,
making the memory usage of such a DFA ever bigger. (The NFAs in this crate
unconditionally support both anchored and unanchored searches because there
is essentially no added cost for doing so.) It is for this reason that
a DFA's support for anchored and unanchored searches can be configured
via [`Builder::start_kind`]. By default, a DFA only supports unanchored
searches.

# Example

This example shows how to build an `DFA` directly and use it to execute
[`Automaton::try_find`]:

```
use aho_corasick::{
    automaton::Automaton,
    dfa::DFA,
    Input, Match,
};

let patterns = &["b", "abc", "abcd"];
let haystack = "abcd";

let nfa = DFA::new(patterns).unwrap();
assert_eq!(
    Some(Match::must(0, 1..2)),
    nfa.try_find(&Input::new(haystack))?,
);
# Ok::<(), Box<dyn std::error::Error>>(())
```

It is also possible to implement your own version of `try_find`. See the
[`Automaton`] documentation for an example.

---
