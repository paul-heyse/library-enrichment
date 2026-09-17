# `regex_automata::dfa::automaton`

Crate `regex-automata` · 3 public items · structured records in [`model/regex_automata.dfa.automaton.json`](../model/regex_automata.dfa.automaton.json)

## StartError

`enum` · `regex_automata::dfa::automaton::StartError`

Also reachable as `regex_automata::dfa::StartError`

```rust
enum StartError
```

**Variants**: `Quit`, `UnsupportedAnchored`

**Implements**: `core::error::Error`, `core::fmt::Display`

**Derives**: Clone, Debug

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
```

An error that can occur when computing the start state for a search.

Computing a start state can fail for a few reasons, either based on
incorrect configuration or even based on whether the look-behind byte
triggers a quit state. Typically one does not need to handle this error
if you're using [`Automaton::start_state_forward`] (or its reverse
counterpart), as that routine automatically converts `StartError` to a
[`MatchError`] for you.

This error may be returned by the [`Automaton::start_state`] routine.

This error implements the `std::error::Error` trait when the `std` feature
is enabled.

This error is marked as non-exhaustive. New variants may be added in a
semver compatible release.

---

## OverlappingState

`struct` · `regex_automata::dfa::automaton::OverlappingState`

Also reachable as `regex_automata::dfa::OverlappingState`

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

## Automaton

`trait` · `regex_automata::dfa::automaton::Automaton`

Also reachable as `regex_automata::dfa::Automaton`

```rust
unsafe trait Automaton
```

**Implementors** (2)

- `regex_automata::dfa::dense::DFA`
- `regex_automata::dfa::sparse::DFA`

**Methods** (26)

```rust
fn accelerator(&self, _id: StateID) -> &[u8]
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
fn match_pattern(&self, id: StateID, index: usize) -> PatternID
fn next_eoi_state(&self, current: StateID) -> StateID
fn next_state(&self, current: StateID, input: u8) -> StateID
unsafe fn next_state_unchecked(&self, current: StateID, input: u8) -> StateID
fn pattern_len(&self) -> usize
fn start_state(&self, config: &start::Config) -> Result<StateID, StartError>
fn start_state_forward(&self, input: &Input<'_>) -> Result<StateID, MatchError>
fn start_state_reverse(&self, input: &Input<'_>) -> Result<StateID, MatchError>
fn try_search_fwd(&self, input: &Input<'_>) -> Result<Option<HalfMatch>, MatchError>
fn try_search_overlapping_fwd(&self, input: &Input<'_>, state: &mut OverlappingState) -> Result<(), MatchError>
fn try_search_overlapping_rev(&self, input: &Input<'_>, state: &mut OverlappingState) -> Result<(), MatchError>
fn try_search_rev(&self, input: &Input<'_>) -> Result<Option<HalfMatch>, MatchError>
fn try_which_overlapping_matches(&self, input: &Input<'_>, patset: &mut PatternSet) -> Result<(), MatchError>
fn universal_start_state(&self, _mode: Anchored) -> Option<StateID>
```

A trait describing the interface of a deterministic finite automaton (DFA).

The complexity of this trait probably means that it's unlikely for others
to implement it. The primary purpose of the trait is to provide for a way
of abstracting over different types of DFAs. In this crate, that means
dense DFAs and sparse DFAs. (Dense DFAs are fast but memory hungry, where
as sparse DFAs are slower but come with a smaller memory footprint. But
they otherwise provide exactly equivalent expressive power.) For example, a
[`dfa::regex::Regex`](crate::dfa::regex::Regex) is generic over this trait.

Normally, a DFA's execution model is very simple. You might have a single
start state, zero or more final or "match" states and a function that
transitions from one state to the next given the next byte of input.
Unfortunately, the interface described by this trait is significantly
more complicated than this. The complexity has a number of different
reasons, mostly motivated by performance, functionality or space savings:

* A DFA can search for multiple patterns simultaneously. This
means extra information is returned when a match occurs. Namely,
a match is not just an offset, but an offset plus a pattern ID.
[`Automaton::pattern_len`] returns the number of patterns compiled into
the DFA, [`Automaton::match_len`] returns the total number of patterns
that match in a particular state and [`Automaton::match_pattern`] permits
iterating over the patterns that match in a particular state.
* A DFA can have multiple start states, and the choice of which start
state to use depends on the content of the string being searched and
position of the search, as well as whether the search is an anchored
search for a specific pattern in the DFA. Moreover, computing the start
state also depends on whether you're doing a forward or a reverse search.
[`Automaton::start_state_forward`] and [`Automaton::start_state_reverse`]
are used to compute the start state for forward and reverse searches,
respectively.
* All matches are delayed by one byte to support things like `$` and `\b`
at the end of a pattern. Therefore, every use of a DFA is required to use
[`Automaton::next_eoi_state`]
at the end of the search to compute the final transition.
* For optimization reasons, some states are treated specially. Every
state is either special or not, which can be determined via the
[`Automaton::is_special_state`] method. If it's special, then the state
must be at least one of a few possible types of states. (Note that some
types can overlap, for example, a match state can also be an accel state.
But some types can't. If a state is a dead state, then it can never be any
other type of state.) Those types are:
    * A dead state. A dead state means the DFA will never enter a match
    state. This can be queried via the [`Automaton::is_dead_state`] method.
    * A quit state. A quit state occurs if the DFA had to stop the search
    prematurely for some reason. This can be queried via the
    [`Automaton::is_quit_state`] method.
    * A match state. A match state occurs when a match is found. When a DFA
    enters a match state, the search may stop immediately (when looking
    for the earliest match), or it may continue to find the leftmost-first
    match. This can be queried via the [`Automaton::is_match_state`]
    method.
    * A start state. A start state is where a search begins. For every
    search, there is exactly one start state that is used, however, a
    DFA may contain many start states. When the search is in a start
    state, it may use a prefilter to quickly skip to candidate matches
    without executing the DFA on every byte. This can be queried via the
    [`Automaton::is_start_state`] method.
    * An accel state. An accel state is a state that is accelerated.
    That is, it is a state where _most_ of its transitions loop back to
    itself and only a small number of transitions lead to other states.
    This kind of state is said to be accelerated because a search routine
    can quickly look for the bytes leading out of the state instead of
    continuing to execute the DFA on each byte. This can be queried via the
    [`Automaton::is_accel_state`] method. And the bytes that lead out of
    the state can be queried via the [`Automaton::accelerator`] method.

There are a number of provided methods on this trait that implement
efficient searching (for forwards and backwards) with a DFA using
all of the above features of this trait. In particular, given the
complexity of all these features, implementing a search routine in
this trait can be a little subtle. With that said, it is possible to
somewhat simplify the search routine. For example, handling accelerated
states is strictly optional, since it is always correct to assume that
`Automaton::is_accel_state` returns false. However, one complex part of
writing a search routine using this trait is handling the 1-byte delay of a
match. That is not optional.

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

---
