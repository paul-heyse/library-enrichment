# `regex_automata::nfa::thompson::nfa`

Crate `regex-automata` · 6 public items · structured records in [`model/regex_automata.nfa.thompson.nfa.json`](../model/regex_automata.nfa.thompson.nfa.json)

## State

`enum` · `regex_automata::nfa::thompson::nfa::State`

Also reachable as `regex_automata::nfa::thompson::State`

```rust
enum State
```

**Variants**: `ByteRange`, `Sparse`, `Dense`, `Look`, `Union`, `BinaryUnion`, `Capture`, `Fail`, `Match`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn is_epsilon(&self) -> bool
```

A state in an NFA.

In theory, it can help to conceptualize an `NFA` as a graph consisting of
`State`s. Each `State` contains its complete set of outgoing transitions.

In practice, it can help to conceptualize an `NFA` as a sequence of
instructions for a virtual machine. Each `State` says what to do and where
to go next.

Strictly speaking, the practical interpretation is the most correct one,
because of the [`Capture`](State::Capture) state. Namely, a `Capture`
state always forwards execution to another state unconditionally. Its only
purpose is to cause a side effect: the recording of the current input
position at a particular location in memory. In this sense, an `NFA`
has more power than a theoretical non-deterministic finite automaton.

For most uses of this crate, it is likely that one may never even need to
be aware of this type at all. The main use cases for looking at `State`s
directly are if you need to write your own search implementation or if you
need to do some kind of analysis on the NFA.

---

## DenseTransitions

`struct` · `regex_automata::nfa::thompson::nfa::DenseTransitions`

Also reachable as `regex_automata::nfa::thompson::DenseTransitions`

```rust
struct DenseTransitions
```

**Fields**: `transitions`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn matches(&self, haystack: &[u8], at: usize) -> Option<StateID>
fn matches_byte(&self, byte: u8) -> Option<StateID>
```

A sequence of transitions used to represent a dense state.

This is the primary representation of a [`Dense`](State::Dense) state. It
provides constant time matching. That is, given a byte in a haystack and
a `DenseTransitions`, one can determine if the state matches in constant
time.

This is in contrast to `SparseTransitions`, whose time complexity is
necessarily bigger than constant time. Also in contrast, `DenseTransitions`
usually requires (much) more heap memory.

---

## NFA

`struct` · `regex_automata::nfa::thompson::nfa::NFA`

Also reachable as `regex_automata::nfa::thompson::NFA`

```rust
struct NFA
```

**Derives**: Clone, Debug

**Methods** (24)

```rust
fn always_match() -> NFA
fn byte_classes(&self) -> &ByteClasses
fn compiler() -> Compiler
fn config() -> Config
fn group_info(&self) -> &GroupInfo
fn has_capture(&self) -> bool
fn has_empty(&self) -> bool
fn is_always_start_anchored(&self) -> bool
fn is_reverse(&self) -> bool
fn is_utf8(&self) -> bool
fn look_matcher(&self) -> &LookMatcher
fn look_set_any(&self) -> LookSet
fn look_set_prefix_any(&self) -> LookSet
fn memory_usage(&self) -> usize
fn never_match() -> NFA
fn new(pattern: &str) -> Result<NFA, BuildError>
fn new_many<P: AsRef<str>>(patterns: &[P]) -> Result<NFA, BuildError>
fn pattern_len(&self) -> usize
fn patterns(&self) -> PatternIter<'_>
fn start_anchored(&self) -> StateID
fn start_pattern(&self, pid: PatternID) -> Option<StateID>
fn start_unanchored(&self) -> StateID
fn state(&self, id: StateID) -> &State
fn states(&self) -> &[State]
```

A byte oriented Thompson non-deterministic finite automaton (NFA).

A Thompson NFA is a finite state machine that permits unconditional epsilon
transitions, but guarantees that there exists at most one non-epsilon
transition for each element in the alphabet for each state.

An NFA may be used directly for searching, for analysis or to build
a deterministic finite automaton (DFA).

# Cheap clones

Since an NFA is a core data type in this crate that many other regex
engines are based on top of, it is convenient to give ownership of an NFA
to said regex engines. Because of this, an NFA uses reference counting
internally. Therefore, it is cheap to clone and it is encouraged to do so.

# Capabilities

Using an NFA for searching via the
[`PikeVM`](crate::nfa::thompson::pikevm::PikeVM) provides the most amount
of "power" of any regex engine in this crate. Namely, it supports the
following in all cases:

1. Detection of a match.
2. Location of a match, including both the start and end offset, in a
single pass of the haystack.
3. Location of matching capturing groups.
4. Handles multiple patterns, including (1)-(3) when multiple patterns are
present.

# Capturing Groups

Groups refer to parenthesized expressions inside a regex pattern. They look
like this, where `exp` is an arbitrary regex:

* `(exp)` - An unnamed capturing group.
* `(?P<name>exp)` or `(?<name>exp)` - A named capturing group.
* `(?:exp)` - A non-capturing group.
* `(?i:exp)` - A non-capturing group that sets flags.

Only the first two forms are said to be _capturing_. Capturing
means that the last position at which they match is reportable. The
[`Captures`](crate::util::captures::Captures) type provides convenient
access to the match positions of capturing groups, which includes looking
up capturing groups by their name.

# Byte oriented

This NFA is byte oriented, which means that all of its transitions are
defined on bytes. In other words, the alphabet of an NFA consists of the
256 different byte values.

While DFAs nearly demand that they be byte oriented for performance
reasons, an NFA could conceivably be *Unicode codepoint* oriented. Indeed,
a previous version of this NFA supported both byte and codepoint oriented
modes. A codepoint oriented mode can work because an NFA fundamentally uses
a sparse representation of transitions, which works well with the large
sparse space of Unicode codepoints.

Nevertheless, this NFA is only byte oriented. This choice is primarily
driven by implementation simplicity, and also in part memory usage. In
practice, performance between the two is roughly comparable. However,
building a DFA (including a hybrid DFA) really wants a byte oriented NFA.
So if we do have a codepoint oriented NFA, then we also need to generate
byte oriented NFA in order to build an hybrid NFA/DFA. Thus, by only
generating byte oriented NFAs, we can produce one less NFA. In other words,
if we made our NFA codepoint oriented, we'd need to *also* make it support
a byte oriented mode, which is more complicated. But a byte oriented mode
can support everything.

# Differences with DFAs

At the theoretical level, the precise difference between an NFA and a DFA
is that, in a DFA, for every state, an input symbol unambiguously refers
to a single transition _and_ that an input symbol is required for each
transition. At a practical level, this permits DFA implementations to be
implemented at their core with a small constant number of CPU instructions
for each byte of input searched. In practice, this makes them quite a bit
faster than NFAs _in general_. Namely, in order to execute a search for any
Thompson NFA, one needs to keep track of a _set_ of states, and execute
the possible transitions on all of those states for each input symbol.
Overall, this results in much more overhead. To a first approximation, one
can expect DFA searches to be about an order of magnitude faster.

So why use an NFA at all? The main advantage of an NFA is that it takes
linear time (in the size of the pattern string after repetitions have been
expanded) to build and linear memory usage. A DFA, on the other hand, may
take exponential time and/or space to build. Even in non-pathological
cases, DFAs often take quite a bit more memory than their NFA counterparts,
_especially_ if large Unicode character classes are involved. Of course,
an NFA also provides additional capabilities. For example, it can match
Unicode word boundaries on non-ASCII text and resolve the positions of
capturing groups.

Note that a [`hybrid::regex::Regex`](crate::hybrid::regex::Regex) strikes a
good balance between an NFA and a DFA. It avoids the exponential build time
of a DFA while maintaining its fast search time. The downside of a hybrid
NFA/DFA is that in some cases it can be slower at search time than the NFA.
(It also has less functionality than a pure NFA. It cannot handle Unicode
word boundaries on non-ASCII text and cannot resolve capturing groups.)

# Example

This shows how to build an NFA with the default configuration and execute a
search using the Pike VM.

```
use regex_automata::{nfa::thompson::pikevm::PikeVM, Match};

let re = PikeVM::new(r"foo[0-9]+")?;
let mut cache = re.create_cache();
let mut caps = re.create_captures();

let expected = Some(Match::must(0, 0..8));
re.captures(&mut cache, b"foo12345", &mut caps);
assert_eq!(expected, caps.get_match());

# Ok::<(), Box<dyn std::error::Error>>(())
```

# Example: resolving capturing groups

This example shows how to parse some simple dates and extract the
components of each date via capturing groups.

```
# if cfg!(miri) { return Ok(()); } // miri takes too long
use regex_automata::{
    nfa::thompson::pikevm::PikeVM,
    util::captures::Captures,
};

let vm = PikeVM::new(r"(?P<y>\d{4})-(?P<m>\d{2})-(?P<d>\d{2})")?;
let mut cache = vm.create_cache();

let haystack = "2012-03-14, 2013-01-01 and 2014-07-05";
let all: Vec<Captures> = vm.captures_iter(
    &mut cache, haystack.as_bytes()
).collect();
// There should be a total of 3 matches.
assert_eq!(3, all.len());
// The year from the second match is '2013'.
let span = all[1].get_group_by_name("y").unwrap();
assert_eq!("2013", &haystack[span]);

# Ok::<(), Box<dyn std::error::Error>>(())
```

This example shows that only the last match of a capturing group is
reported, even if it had to match multiple times for an overall match
to occur.

```
use regex_automata::{nfa::thompson::pikevm::PikeVM, Span};

let re = PikeVM::new(r"([a-z]){4}")?;
let mut cache = re.create_cache();
let mut caps = re.create_captures();

let haystack = b"quux";
re.captures(&mut cache, haystack, &mut caps);
assert!(caps.is_match());
assert_eq!(Some(Span::from(3..4)), caps.get_group(1));

# Ok::<(), Box<dyn std::error::Error>>(())
```

---

## PatternIter

`struct` · `regex_automata::nfa::thompson::nfa::PatternIter`

Also reachable as `regex_automata::nfa::thompson::PatternIter`

```rust
struct PatternIter<'a>
```

**Implements**: `core::iter::traits::iterator::Iterator`

**Derives**: Debug

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<PatternID>
```

An iterator over all pattern IDs in an NFA.

This iterator is created by [`NFA::patterns`].

The lifetime parameter `'a` refers to the lifetime of the NFA from which
this pattern iterator was created.

---

## SparseTransitions

`struct` · `regex_automata::nfa::thompson::nfa::SparseTransitions`

Also reachable as `regex_automata::nfa::thompson::SparseTransitions`

```rust
struct SparseTransitions
```

**Fields**: `transitions`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn matches(&self, haystack: &[u8], at: usize) -> Option<StateID>
fn matches_byte(&self, byte: u8) -> Option<StateID>
```

A sequence of transitions used to represent a sparse state.

This is the primary representation of a [`Sparse`](State::Sparse) state.
It corresponds to a sorted sequence of transitions with non-overlapping
byte ranges. If the byte at the current position in the haystack matches
one of the byte ranges, then the finite state machine should take the
corresponding transition.

---

## Transition

`struct` · `regex_automata::nfa::thompson::nfa::Transition`

Also reachable as `regex_automata::nfa::thompson::Transition`

```rust
struct Transition
```

**Fields**: `start`, `end`, `next`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn matches(&self, haystack: &[u8], at: usize) -> bool
fn matches_byte(&self, byte: u8) -> bool
fn matches_unit(&self, unit: alphabet::Unit) -> bool
```

A single transition to another state.

This transition may only be followed if the current byte in the haystack
falls in the inclusive range of bytes specified.

---
