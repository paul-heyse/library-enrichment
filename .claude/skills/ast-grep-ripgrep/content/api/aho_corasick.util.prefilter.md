# `aho_corasick::util::prefilter`

Crate `aho-corasick` · 2 public items · structured records in [`model/aho_corasick.util.prefilter.json`](../model/aho_corasick.util.prefilter.json)

## Candidate

`enum` · `aho_corasick::util::prefilter::Candidate`

Also reachable as `aho_corasick::automaton::Candidate`

```rust
enum Candidate
```

**Variants**: `None`, `Match`, `PossibleStartOfMatch`

**Derives**: Clone, Debug

**Methods** (1)

```rust
fn into_option(self) -> Option<usize>
```

A candidate is the result of running a prefilter on a haystack at a
particular position.

The result is either no match, a confirmed match or a possible match.

When no match is returned, the prefilter is guaranteeing that no possible
match can be found in the haystack, and the caller may trust this. That is,
all correct prefilters must never report false negatives.

In some cases, a prefilter can confirm a match very quickly, in which case,
the caller may use this to stop what it's doing and report the match. In
this case, prefilter implementations must never report a false positive.
In other cases, the prefilter can only report a potential match, in which
case the callers must attempt to confirm the match. In this case, prefilter
implementations are permitted to return false positives.

---

## Prefilter

`struct` · `aho_corasick::util::prefilter::Prefilter`

Also reachable as `aho_corasick::automaton::Prefilter`

```rust
struct Prefilter
```

**Derives**: Clone, Debug

**Methods** (1)

```rust
fn find_in(&self, haystack: &[u8], span: Span) -> Candidate
```

A prefilter for accelerating a search.

This crate uses prefilters in the core search implementations to accelerate
common cases. They typically only apply to cases where there are a small
number of patterns (less than 100 or so), but when they do, thoughput can
be boosted considerably, perhaps by an order of magnitude. When a prefilter
is active, it is used whenever a search enters an automaton's start state.

Currently, prefilters cannot be constructed by
callers. A `Prefilter` can only be accessed via the
[`Automaton::prefilter`](crate::automaton::Automaton::prefilter)
method and used to execute a search. In other words, a prefilter can be
used to optimize your own search implementation if necessary, but cannot do
much else. If you have a use case for more APIs, please submit an issue.

---
