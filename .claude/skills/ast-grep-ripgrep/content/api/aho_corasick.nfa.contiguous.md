# `aho_corasick::nfa::contiguous`

Crate `aho-corasick` · 2 public items · structured records in [`model/aho_corasick.nfa.contiguous.json`](../model/aho_corasick.nfa.contiguous.json)

## Builder

`struct` · `aho_corasick::nfa::contiguous::Builder`

```rust
struct Builder
```

**Derives**: Clone, Debug, Default

**Methods** (8)

```rust
fn ascii_case_insensitive(&mut self, yes: bool) -> &mut Builder
fn build<I, P>(&self, patterns: I) -> Result<NFA, BuildError> where I: IntoIterator<Item = P>, P: AsRef<[u8]>
fn build_from_noncontiguous(&self, nnfa: &noncontiguous::NFA) -> Result<NFA, BuildError>
fn byte_classes(&mut self, yes: bool) -> &mut Builder
fn dense_depth(&mut self, depth: usize) -> &mut Builder
fn match_kind(&mut self, kind: MatchKind) -> &mut Builder
fn new() -> Builder
fn prefilter(&mut self, yes: bool) -> &mut Builder
```

A builder for configuring an Aho-Corasick contiguous NFA.

This builder has a subset of the options available to a
[`AhoCorasickBuilder`](crate::AhoCorasickBuilder). Of the shared options,
their behavior is identical.

---

## NFA

`struct` · `aho_corasick::nfa::contiguous::NFA`

```rust
struct NFA
```

**Implements**: `aho_corasick::automaton::Automaton`

**Derives**: Clone, Debug

**Methods** (2)

```rust
fn builder() -> Builder
fn new<I, P>(patterns: I) -> Result<NFA, BuildError> where I: IntoIterator<Item = P>, P: AsRef<[u8]>
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
fn next_state(&self, anchored: Anchored, sid: StateID, byte: u8) -> StateID
fn pattern_len(&self, pid: PatternID) -> usize
fn patterns_len(&self) -> usize
fn prefilter(&self) -> Option<&Prefilter>
fn start_state(&self, anchored: Anchored) -> Result<StateID, MatchError>
```

A contiguous NFA implementation of Aho-Corasick.

When possible, prefer using [`AhoCorasick`](crate::AhoCorasick) instead of
this type directly. Using an `NFA` directly is typically only necessary
when one needs access to the [`Automaton`] trait implementation.

This NFA can only be built by first constructing a [`noncontiguous::NFA`].
Both [`NFA::new`] and [`Builder::build`] do this for you automatically, but
[`Builder::build_from_noncontiguous`] permits doing it explicitly.

The main difference between a noncontiguous NFA and a contiguous NFA is
that the latter represents all of its states and transitions in a single
allocation, where as the former uses a separate allocation for each state.
Doing this at construction time while keeping a low memory footprint isn't
feasible, which is primarily why there are two different NFA types: one
that does the least amount of work possible to build itself, and another
that does a little extra work to compact itself and make state transitions
faster by making some states use a dense representation.

Because a contiguous NFA uses a single allocation, there is a lot more
opportunity for compression tricks to reduce the heap memory used. Indeed,
it is not uncommon for a contiguous NFA to use an order of magnitude less
heap memory than a noncontiguous NFA. Since building a contiguous NFA
usually only takes a fraction of the time it takes to build a noncontiguous
NFA, the overall build time is not much slower. Thus, in most cases, a
contiguous NFA is the best choice.

Since a contiguous NFA uses various tricks for compression and to achieve
faster state transitions, currently, its limit on the number of states
is somewhat smaller than what a noncontiguous NFA can achieve. Generally
speaking, you shouldn't expect to run into this limit if the number of
patterns is under 1 million. It is plausible that this limit will be
increased in the future. If the limit is reached, building a contiguous NFA
will return an error. Often, since building a contiguous NFA is relatively
cheap, it can make sense to always try it even if you aren't sure if it
will fail or not. If it does, you can always fall back to a noncontiguous
NFA. (Indeed, the main [`AhoCorasick`](crate::AhoCorasick) type employs a
strategy similar to this at construction time.)

# Example

This example shows how to build an `NFA` directly and use it to execute
[`Automaton::try_find`]:

```
use aho_corasick::{
    automaton::Automaton,
    nfa::contiguous::NFA,
    Input, Match,
};

let patterns = &["b", "abc", "abcd"];
let haystack = "abcd";

let nfa = NFA::new(patterns).unwrap();
assert_eq!(
    Some(Match::must(0, 1..2)),
    nfa.try_find(&Input::new(haystack))?,
);
# Ok::<(), Box<dyn std::error::Error>>(())
```

It is also possible to implement your own version of `try_find`. See the
[`Automaton`] documentation for an example.

---
