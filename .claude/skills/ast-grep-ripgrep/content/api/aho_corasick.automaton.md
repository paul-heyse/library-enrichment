# `aho_corasick::automaton`

Crate `aho-corasick` · 5 public items · structured records in [`model/aho_corasick.automaton.json`](../model/aho_corasick.automaton.json)

## FindIter

`struct` · `aho_corasick::automaton::FindIter`

```rust
struct FindIter<'a, 'h, A>
```

**Implements**: `core::iter::traits::iterator::Iterator`

**Derives**: Debug

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<Match>
```

An iterator of non-overlapping matches in a particular haystack.

This iterator yields matches according to the [`MatchKind`] used by this
automaton.

This iterator is constructed via the [`Automaton::try_find_iter`] method.

The type variable `A` refers to the implementation of the [`Automaton`]
trait used to execute the search.

The lifetime `'a` refers to the lifetime of the [`Automaton`]
implementation.

The lifetime `'h` refers to the lifetime of the haystack being searched.

---

## FindOverlappingIter

`struct` · `aho_corasick::automaton::FindOverlappingIter`

```rust
struct FindOverlappingIter<'a, 'h, A>
```

**Implements**: `core::iter::traits::iterator::Iterator`

**Derives**: Debug

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<Match>
```

An iterator of overlapping matches in a particular haystack.

This iterator will report all possible matches in a particular haystack,
even when the matches overlap.

This iterator is constructed via the
[`Automaton::try_find_overlapping_iter`] method.

The type variable `A` refers to the implementation of the [`Automaton`]
trait used to execute the search.

The lifetime `'a` refers to the lifetime of the [`Automaton`]
implementation.

The lifetime `'h` refers to the lifetime of the haystack being searched.

---

## OverlappingState

`struct` · `aho_corasick::automaton::OverlappingState`

```rust
struct OverlappingState
```

**Derives**: Clone, Debug

**Methods** (2)

```rust
fn get_match(&self) -> Option<Match>
fn start() -> OverlappingState
```

Represents the current state of an overlapping search.

This is used for overlapping searches since they need to know something
about the previous search. For example, when multiple patterns match at the
same position, this state tracks the last reported pattern so that the next
search knows whether to report another matching pattern or continue with
the search at the next position. Additionally, it also tracks which state
the last search call terminated in and the current offset of the search
in the haystack.

This type provides limited introspection capabilities. The only thing a
caller can do is construct it and pass it around to permit search routines
to use it to track state, and to ask whether a match has been found.

Callers should always provide a fresh state constructed via
[`OverlappingState::start`] when starting a new search. That same state
should be reused for subsequent searches on the same `Input`. The state
given will advance through the haystack itself. Callers can detect the end
of a search when neither an error nor a match is returned.

# Example

This example shows how to manually iterate over all overlapping matches. If
you need this, you might consider using
[`AhoCorasick::find_overlapping_iter`](crate::AhoCorasick::find_overlapping_iter)
instead, but this shows how to correctly use an `OverlappingState`.

```
use aho_corasick::{
    automaton::OverlappingState,
    AhoCorasick, Input, Match,
};

let patterns = &["append", "appendage", "app"];
let haystack = "append the app to the appendage";

let ac = AhoCorasick::new(patterns).unwrap();
let mut state = OverlappingState::start();
let mut matches = vec![];

loop {
    ac.find_overlapping(haystack, &mut state);
    let mat = match state.get_match() {
        None => break,
        Some(mat) => mat,
    };
    matches.push(mat);
}
let expected = vec![
    Match::must(2, 0..3),
    Match::must(0, 0..6),
    Match::must(2, 11..14),
    Match::must(2, 22..25),
    Match::must(0, 22..28),
    Match::must(1, 22..31),
];
assert_eq!(expected, matches);
```

---

## StreamFindIter

`struct` · `aho_corasick::automaton::StreamFindIter`

```rust
struct StreamFindIter<'a, A, R>
```

**Implements**: `core::iter::traits::iterator::Iterator`

**Derives**: Debug

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<std::io::Result<Match>>
```

An iterator that reports matches in a stream.

This iterator yields elements of type `io::Result<Match>`, where an error
is reported if there was a problem reading from the underlying stream.
The iterator terminates only when the underlying stream reaches `EOF`.

This iterator is constructed via the [`Automaton::try_stream_find_iter`]
method.

The type variable `A` refers to the implementation of the [`Automaton`]
trait used to execute the search.

The type variable `R` refers to the `io::Read` stream that is being read
from.

The lifetime `'a` refers to the lifetime of the [`Automaton`]
implementation.

---

## Automaton

`trait` · `aho_corasick::automaton::Automaton`

```rust
unsafe trait Automaton: private::Sealed
```

**Implementors** (3)

- `aho_corasick::dfa::DFA`
- `aho_corasick::nfa::contiguous::NFA`
- `aho_corasick::nfa::noncontiguous::NFA`

**Methods** (26)

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
fn try_find(&self, input: &Input<'_>) -> Result<Option<Match>, MatchError>
fn try_find_iter<'a, 'h>(&'a self, input: Input<'h>) -> Result<FindIter<'a, 'h, Self>, MatchError> where Self: Sized
fn try_find_overlapping(&self, input: &Input<'_>, state: &mut OverlappingState) -> Result<(), MatchError>
fn try_find_overlapping_iter<'a, 'h>(&'a self, input: Input<'h>) -> Result<FindOverlappingIter<'a, 'h, Self>, MatchError> where Self: Sized
fn try_replace_all<B>(&self, haystack: &str, replace_with: &[B]) -> Result<String, MatchError> where Self: Sized, B: AsRef<str>
fn try_replace_all_bytes<B>(&self, haystack: &[u8], replace_with: &[B]) -> Result<Vec<u8>, MatchError> where Self: Sized, B: AsRef<[u8]>
fn try_replace_all_with<F>(&self, haystack: &str, dst: &mut String, replace_with: F) -> Result<(), MatchError> where Self: Sized, F: FnMut(&Match, &str, &mut String) -> bool
fn try_replace_all_with_bytes<F>(&self, haystack: &[u8], dst: &mut Vec<u8>, replace_with: F) -> Result<(), MatchError> where Self: Sized, F: FnMut(&Match, &[u8], &mut Vec<u8>) -> bool
fn try_stream_find_iter<'a, R: std::io::Read>(&'a self, rdr: R) -> Result<StreamFindIter<'a, Self, R>, MatchError> where Self: Sized
fn try_stream_replace_all<R, W, B>(&self, rdr: R, wtr: W, replace_with: &[B]) -> std::io::Result<()> where Self: Sized, R: std::io::Read, W: std::io::Write, B: AsRef<[u8]>
fn try_stream_replace_all_with<R, W, F>(&self, rdr: R, wtr: W, replace_with: F) -> std::io::Result<()> where Self: Sized, R: std::io::Read, W: std::io::Write, F: FnMut(&Match, &[u8], &mut W) -> std::io::Result<()>
```

A trait that abstracts over Aho-Corasick automata.

This trait primarily exists for niche use cases such as:

* Using an NFA or DFA directly, bypassing the top-level
[`AhoCorasick`](crate::AhoCorasick) searcher. Currently, these include
[`noncontiguous::NFA`](crate::nfa::noncontiguous::NFA),
[`contiguous::NFA`](crate::nfa::contiguous::NFA) and
[`dfa::DFA`](crate::dfa::DFA).
* Implementing your own custom search routine by walking the automaton
yourself. This might be useful for implementing search on non-contiguous
strings or streams.

For most use cases, it is not expected that users will need
to use or even know about this trait. Indeed, the top level
[`AhoCorasick`](crate::AhoCorasick) searcher does not expose any details
about this trait, nor does it implement it itself.

Note that this trait defines a number of default methods, such as
[`Automaton::try_find`] and [`Automaton::try_find_iter`], which implement
higher level search routines in terms of the lower level automata API.

# Sealed

Currently, this trait is sealed. That means users of this crate can write
generic routines over this trait but cannot implement it themselves. This
restriction may be lifted in the future, but sealing the trait permits
adding new required methods in a backwards compatible fashion.

# Special states

This trait encodes a notion of "special" states in an automaton. Namely,
a state is treated as special if it is a dead, match or start state:

* A dead state is a state that cannot be left once entered. All transitions
on a dead state lead back to itself. The dead state is meant to be treated
as a sentinel indicating that the search should stop and return a match if
one has been found, and nothing otherwise.
* A match state is a state that indicates one or more patterns have
matched. Depending on the [`MatchKind`] of the automaton, a search may
stop once a match is seen, or it may continue looking for matches until
it enters a dead state or sees the end of the haystack.
* A start state is a state that a search begins in. It is useful to know
when a search enters a start state because it may mean that a prefilter can
be used to skip ahead and quickly look for candidate matches. Unlike dead
and match states, it is never necessary to explicitly handle start states
for correctness. Indeed, in this crate, implementations of `Automaton`
will only treat start states as "special" when a prefilter is enabled and
active. Otherwise, treating it as special has no purpose and winds up
slowing down the overall search because it results in ping-ponging between
the main state transition and the "special" state logic.

Since checking whether a state is special by doing three different
checks would be too expensive inside a fast search loop, the
[`Automaton::is_special`] method is provided for quickly checking whether
the state is special. The `Automaton::is_dead`, `Automaton::is_match` and
`Automaton::is_start` predicates can then be used to determine which kind
of special state it is.

# Panics

Most of the APIs on this trait should panic or give incorrect results
if invalid inputs are given to it. For example, `Automaton::next_state`
has unspecified behavior if the state ID given to it is not a valid
state ID for the underlying automaton. Valid state IDs can only be
retrieved in one of two ways: calling `Automaton::start_state` or calling
`Automaton::next_state` with a valid state ID.

# Safety

This trait is not safe to implement so that code may rely on the
correctness of implementations of this trait to avoid undefined behavior.
The primary correctness guarantees are:

* `Automaton::start_state` always returns a valid state ID or an error or
panics.
* `Automaton::next_state`, when given a valid state ID, always returns
a valid state ID for all values of `anchored` and `byte`, or otherwise
panics.

In general, the rest of the methods on `Automaton` need to uphold their
contracts as well. For example, `Automaton::is_dead` should only returns
true if the given state ID is actually a dead state.

Note that currently this crate does not rely on the safety property defined
here to avoid undefined behavior. Instead, this was done to make it
_possible_ to do in the future.

# Example

This example shows how one might implement a basic but correct search
routine. We keep things simple by not using prefilters or worrying about
anchored searches, but do make sure our search is correct for all possible
[`MatchKind`] semantics. (The comments in the code below note the parts
that are needed to support certain `MatchKind` semantics.)

```
use aho_corasick::{
    automaton::Automaton,
    nfa::noncontiguous::NFA,
    Anchored, Match, MatchError, MatchKind,
};

// Run an unanchored search for 'aut' in 'haystack'. Return the first match
// seen according to the automaton's match semantics. This returns an error
// if the given automaton does not support unanchored searches.
fn find<A: Automaton>(
    aut: A,
    haystack: &[u8],
) -> Result<Option<Match>, MatchError> {
    let mut sid = aut.start_state(Anchored::No)?;
    let mut at = 0;
    let mut mat = None;
    let get_match = |sid, at| {
        let pid = aut.match_pattern(sid, 0);
        let len = aut.pattern_len(pid);
        Match::new(pid, (at - len)..at)
    };
    // Start states can be match states!
    if aut.is_match(sid) {
        mat = Some(get_match(sid, at));
        // Standard semantics require matches to be reported as soon as
        // they're seen. Otherwise, we continue until we see a dead state
        // or the end of the haystack.
        if matches!(aut.match_kind(), MatchKind::Standard) {
            return Ok(mat);
        }
    }
    while at < haystack.len() {
        sid = aut.next_state(Anchored::No, sid, haystack[at]);
        if aut.is_special(sid) {
            if aut.is_dead(sid) {
                return Ok(mat);
            } else if aut.is_match(sid) {
                mat = Some(get_match(sid, at + 1));
                // As above, standard semantics require that we return
                // immediately once a match is found.
                if matches!(aut.match_kind(), MatchKind::Standard) {
                    return Ok(mat);
                }
            }
        }
        at += 1;
    }
    Ok(mat)
}

// Show that it works for standard searches.
let nfa = NFA::new(&["samwise", "sam"]).unwrap();
assert_eq!(Some(Match::must(1, 0..3)), find(&nfa, b"samwise")?);

// But also works when using leftmost-first. Notice how the match result
// has changed!
let nfa = NFA::builder()
    .match_kind(MatchKind::LeftmostFirst)
    .build(&["samwise", "sam"])
    .unwrap();
assert_eq!(Some(Match::must(0, 0..7)), find(&nfa, b"samwise")?);

# Ok::<(), Box<dyn std::error::Error>>(())
```

---
