# `regex_automata::nfa::thompson::builder`

Crate `regex-automata` · 1 public items · structured records in [`model/regex_automata.nfa.thompson.builder.json`](../model/regex_automata.nfa.thompson.builder.json)

## Builder

`struct` · `regex_automata::nfa::thompson::builder::Builder`

Also reachable as `regex_automata::nfa::thompson::Builder`

```rust
struct Builder
```

**Derives**: Clone, Debug, Default

**Methods** (27)

```rust
fn add_capture_end(&mut self, next: StateID, group_index: u32) -> Result<StateID, BuildError>
fn add_capture_start(&mut self, next: StateID, group_index: u32, name: Option<Arc<str>>) -> Result<StateID, BuildError>
fn add_empty(&mut self) -> Result<StateID, BuildError>
fn add_fail(&mut self) -> Result<StateID, BuildError>
fn add_look(&mut self, next: StateID, look: Look) -> Result<StateID, BuildError>
fn add_match(&mut self) -> Result<StateID, BuildError>
fn add_range(&mut self, trans: Transition) -> Result<StateID, BuildError>
fn add_sparse(&mut self, transitions: Vec<Transition>) -> Result<StateID, BuildError>
fn add_union(&mut self, alternates: Vec<StateID>) -> Result<StateID, BuildError>
fn add_union_reverse(&mut self, alternates: Vec<StateID>) -> Result<StateID, BuildError>
fn build(&self, start_anchored: StateID, start_unanchored: StateID) -> Result<NFA, BuildError>
fn clear(&mut self)
fn current_pattern_id(&self) -> PatternID
fn finish_pattern(&mut self, start_id: StateID) -> Result<PatternID, BuildError>
fn get_look_matcher(&self) -> &LookMatcher
fn get_reverse(&self) -> bool
fn get_size_limit(&self) -> Option<usize>
fn get_utf8(&self) -> bool
fn memory_usage(&self) -> usize
fn new() -> Builder
fn patch(&mut self, from: StateID, to: StateID) -> Result<(), BuildError>
fn pattern_len(&self) -> usize
fn set_look_matcher(&mut self, m: LookMatcher)
fn set_reverse(&mut self, yes: bool)
fn set_size_limit(&mut self, limit: Option<usize>) -> Result<(), BuildError>
fn set_utf8(&mut self, yes: bool)
fn start_pattern(&mut self) -> Result<PatternID, BuildError>
```

An abstraction for building Thompson NFAs by hand.

A builder is what a [`thompson::Compiler`](crate::nfa::thompson::Compiler)
uses internally to translate a regex's high-level intermediate
representation into an [`NFA`].

The primary function of this builder is to abstract away the internal
representation of an NFA and make it difficult to produce NFAs are that
internally invalid or inconsistent. This builder also provides a way to
add "empty" states (which can be thought of as unconditional epsilon
transitions), despite the fact that [`thompson::State`](nfa::State) does
not have any "empty" representation. The advantage of "empty" states is
that they make the code for constructing a Thompson NFA logically simpler.

Many of the routines on this builder may panic or return errors. Generally
speaking, panics occur when an invalid sequence of method calls were made,
where as an error occurs if things get too big. (Where "too big" might mean
exhausting identifier space or using up too much heap memory in accordance
with the configured [`size_limit`](Builder::set_size_limit).)

# Overview

## Adding multiple patterns

Each pattern you add to an NFA should correspond to a pair of
[`Builder::start_pattern`] and [`Builder::finish_pattern`] calls, with
calls inbetween that add NFA states for that pattern. NFA states may be
added without first calling `start_pattern`, with the exception of adding
capturing states.

## Adding NFA states

Here is a very brief overview of each of the methods that add NFA states.
Every method adds a single state.

* [`add_empty`](Builder::add_empty): Add a state with a single
unconditional epsilon transition to another state.
* [`add_union`](Builder::add_union): Adds a state with unconditional
epsilon transitions to two or more states, with earlier transitions
preferred over later ones.
* [`add_union_reverse`](Builder::add_union_reverse): Adds a state with
unconditional epsilon transitions to two or more states, with later
transitions preferred over earlier ones.
* [`add_range`](Builder::add_range): Adds a state with a single transition
to another state that can only be followed if the current input byte is
within the range given.
* [`add_sparse`](Builder::add_sparse): Adds a state with two or more
range transitions to other states, where a transition is only followed
if the current input byte is within one of the ranges. All transitions
in this state have equal priority, and the corresponding ranges must be
non-overlapping.
* [`add_look`](Builder::add_look): Adds a state with a single *conditional*
epsilon transition to another state, where the condition depends on a
limited look-around property.
* [`add_capture_start`](Builder::add_capture_start): Adds a state with
a single unconditional epsilon transition that also instructs an NFA
simulation to record the current input position to a specific location in
memory. This is intended to represent the starting location of a capturing
group.
* [`add_capture_end`](Builder::add_capture_end): Adds a state with
a single unconditional epsilon transition that also instructs an NFA
simulation to record the current input position to a specific location in
memory. This is intended to represent the ending location of a capturing
group.
* [`add_fail`](Builder::add_fail): Adds a state that never transitions to
another state.
* [`add_match`](Builder::add_match): Add a state that indicates a match has
been found for a particular pattern. A match state is a final state with
no outgoing transitions.

## Setting transitions between NFA states

The [`Builder::patch`] method creates a transition from one state to the
next. If the `from` state corresponds to a state that supports multiple
outgoing transitions (such as "union"), then this adds the corresponding
transition. Otherwise, it sets the single transition. (This routine panics
if `from` corresponds to a state added by `add_sparse`, since sparse states
need more specialized handling.)

# Example

This annotated example shows how to hand construct the regex `[a-z]+`
(without an unanchored prefix).

```
use regex_automata::{
    nfa::thompson::{pikevm::PikeVM, Builder, Transition},
    util::primitives::StateID,
    Match,
};

let mut builder = Builder::new();
// Before adding NFA states for our pattern, we need to tell the builder
// that we are starting the pattern.
builder.start_pattern()?;
// Since we use the Pike VM below for searching, we need to add capturing
// states. If you're just going to build a DFA from the NFA, then capturing
// states do not need to be added.
let start = builder.add_capture_start(StateID::ZERO, 0, None)?;
let range = builder.add_range(Transition {
    // We don't know the state ID of the 'next' state yet, so we just fill
    // in a dummy 'ZERO' value.
    start: b'a', end: b'z', next: StateID::ZERO,
})?;
// This state will point back to 'range', but also enable us to move ahead.
// That is, this implements the '+' repetition operator. We add 'range' and
// then 'end' below to this alternation.
let alt = builder.add_union(vec![])?;
// The final state before the match state, which serves to capture the
// end location of the match.
let end = builder.add_capture_end(StateID::ZERO, 0)?;
// The match state for our pattern.
let mat = builder.add_match()?;
// Now we fill in the transitions between states.
builder.patch(start, range)?;
builder.patch(range, alt)?;
// If we added 'end' before 'range', then we'd implement non-greedy
// matching, i.e., '+?'.
builder.patch(alt, range)?;
builder.patch(alt, end)?;
builder.patch(end, mat)?;
// We must explicitly finish pattern and provide the starting state ID for
// this particular pattern.
builder.finish_pattern(start)?;
// Finally, when we build the NFA, we provide the anchored and unanchored
// starting state IDs. Since we didn't bother with an unanchored prefix
// here, we only support anchored searching. Thus, both starting states are
// the same.
let nfa = builder.build(start, start)?;

// Now build a Pike VM from our NFA, and use it for searching. This shows
// how we can use a regex engine without ever worrying about syntax!
let re = PikeVM::new_from_nfa(nfa)?;
let mut cache = re.create_cache();
let mut caps = re.create_captures();
let expected = Some(Match::must(0, 0..3));
re.captures(&mut cache, "foo0", &mut caps);
assert_eq!(expected, caps.get_match());

# Ok::<(), Box<dyn std::error::Error>>(())
```

---
