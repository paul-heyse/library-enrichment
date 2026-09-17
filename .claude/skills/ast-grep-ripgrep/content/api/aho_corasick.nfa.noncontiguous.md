# `aho_corasick::nfa::noncontiguous`

Crate `aho-corasick` · 2 public items · structured records in [`model/aho_corasick.nfa.noncontiguous.json`](../model/aho_corasick.nfa.noncontiguous.json)

## Builder

`struct` · `aho_corasick::nfa::noncontiguous::Builder`

```rust
struct Builder
```

**Derives**: Clone, Debug, Default

**Methods** (6)

```rust
fn ascii_case_insensitive(&mut self, yes: bool) -> &mut Builder
fn build<I, P>(&self, patterns: I) -> Result<NFA, BuildError> where I: IntoIterator<Item = P>, P: AsRef<[u8]>
fn dense_depth(&mut self, depth: usize) -> &mut Builder
fn match_kind(&mut self, kind: MatchKind) -> &mut Builder
fn new() -> Builder
fn prefilter(&mut self, yes: bool) -> &mut Builder
```

A builder for configuring an Aho-Corasick noncontiguous NFA.

This builder has a subset of the options available to a
[`AhoCorasickBuilder`](crate::AhoCorasickBuilder). Of the shared options,
their behavior is identical.

---

## NFA

`struct` · `aho_corasick::nfa::noncontiguous::NFA`

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

A noncontiguous NFA implementation of Aho-Corasick.

When possible, prefer using [`AhoCorasick`](crate::AhoCorasick) instead of
this type directly. Using an `NFA` directly is typically only necessary
when one needs access to the [`Automaton`] trait implementation.

This NFA represents the "core" implementation of Aho-Corasick in this
crate. Namely, constructing this NFA involving building a trie and then
filling in the failure transitions between states, similar to what is
described in any standard textbook description of Aho-Corasick.

In order to minimize heap usage and to avoid additional construction costs,
this implementation represents the transitions of all states as distinct
sparse memory allocations. This is where it gets its name from. That is,
this NFA has no contiguous memory allocation for its transition table. Each
state gets its own allocation.

While the sparse representation keeps memory usage to somewhat reasonable
levels, it is still quite large and also results in somewhat mediocre
search performance. For this reason, it is almost always a good idea to
use a [`contiguous::NFA`](crate::nfa::contiguous::NFA) instead. It is
marginally slower to build, but has higher throughput and can sometimes use
an order of magnitude less memory. The main reason to use a noncontiguous
NFA is when you need the fastest possible construction time, or when a
contiguous NFA does not have the desired capacity. (The total number of NFA
states it can have is fewer than a noncontiguous NFA.)

# Example

This example shows how to build an `NFA` directly and use it to execute
[`Automaton::try_find`]:

```
use aho_corasick::{
    automaton::Automaton,
    nfa::noncontiguous::NFA,
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
