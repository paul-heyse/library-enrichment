# `regex_automata::util::start`

Crate `regex-automata` · 1 public items · structured records in [`model/regex_automata.util.start.json`](../model/regex_automata.util.start.json)

## Config

`struct` · `regex_automata::util::start::Config`

```rust
struct Config
```

**Derives**: Clone, Debug

**Methods** (7)

```rust
fn anchored(self, mode: Anchored) -> Config
fn from_input_forward(input: &Input<'_>) -> Config
fn from_input_reverse(input: &Input<'_>) -> Config
fn get_anchored(&self) -> Anchored
fn get_look_behind(&self) -> Option<u8>
fn look_behind(self, byte: Option<u8>) -> Config
fn new() -> Config
```

The configuration used to determine a DFA's start state for a search.

A DFA has a single starting state in the typical textbook description. That
is, it corresponds to the set of all starting states for the NFA that built
it, along with their epsilon closures. In this crate, however, DFAs have
many possible start states due to a few factors:

* DFAs support the ability to run either anchored or unanchored searches.
Each type of search needs its own start state. For example, an unanchored
search requires starting at a state corresponding to a regex with a
`(?s-u:.)*?` prefix, which will match through anything.
* DFAs also optionally support starting an anchored search for any one
specific pattern. Each such pattern requires its own start state.
* If a look-behind assertion like `^` or `\b` is used in the regex, then
the DFA will need to inspect a single byte immediately before the start of
the search to choose the correct start state.

Indeed, this configuration precisely encapsulates all of the above factors.
The [`Config::anchored`] method sets which kind of anchored search to
perform while the [`Config::look_behind`] method provides a way to set
the byte that occurs immediately before the start of the search.

Generally speaking, this type is only useful when you want to run searches
without using an [`Input`]. In particular, an `Input` wants a haystack
slice, but callers may not have a contiguous sequence of bytes as a
haystack in all cases. This type provides a lower level of control such
that callers can provide their own anchored configuration and look-behind
byte explicitly.

# Example

This shows basic usage that permits running a search with a DFA without
using the `Input` abstraction.

```
use regex_automata::{
    dfa::{Automaton, dense},
    util::start,
    Anchored,
};

let dfa = dense::DFA::new(r"(?-u)\b\w+\b")?;
let haystack = "quartz";

let config = start::Config::new().anchored(Anchored::Yes);
let mut state = dfa.start_state(&config)?;
for &b in haystack.as_bytes().iter() {
    state = dfa.next_state(state, b);
}
state = dfa.next_eoi_state(state);
assert!(dfa.is_match_state(state));

# Ok::<(), Box<dyn std::error::Error>>(())
```

This example shows how to correctly run a search that doesn't begin at
the start of a haystack. Notice how we set the look-behind byte, and as
a result, the `\b` assertion does not match.

```
use regex_automata::{
    dfa::{Automaton, dense},
    util::start,
    Anchored,
};

let dfa = dense::DFA::new(r"(?-u)\b\w+\b")?;
let haystack = "quartz";

let config = start::Config::new()
    .anchored(Anchored::Yes)
    .look_behind(Some(b'q'));
let mut state = dfa.start_state(&config)?;
for &b in haystack.as_bytes().iter().skip(1) {
    state = dfa.next_state(state, b);
}
state = dfa.next_eoi_state(state);
// No match!
assert!(!dfa.is_match_state(state));

# Ok::<(), Box<dyn std::error::Error>>(())
```

If we had instead not set a look-behind byte, then the DFA would assume
that it was starting at the beginning of the haystack, and thus `\b` should
match. This in turn would result in erroneously reporting a match:

```
use regex_automata::{
    dfa::{Automaton, dense},
    util::start,
    Anchored,
};

let dfa = dense::DFA::new(r"(?-u)\b\w+\b")?;
let haystack = "quartz";

// Whoops, forgot the look-behind byte...
let config = start::Config::new().anchored(Anchored::Yes);
let mut state = dfa.start_state(&config)?;
for &b in haystack.as_bytes().iter().skip(1) {
    state = dfa.next_state(state, b);
}
state = dfa.next_eoi_state(state);
// And now we get a match unexpectedly.
assert!(dfa.is_match_state(state));

# Ok::<(), Box<dyn std::error::Error>>(())
```

---
