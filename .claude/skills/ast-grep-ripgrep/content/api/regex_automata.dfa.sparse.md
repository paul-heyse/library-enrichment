# `regex_automata::dfa::sparse`

Crate `regex-automata` · 1 public items · structured records in [`model/regex_automata.dfa.sparse.json`](../model/regex_automata.dfa.sparse.json)

## DFA

`struct` · `regex_automata::dfa::sparse::DFA`

```rust
struct DFA<T>
```

**Implements**: `regex_automata::dfa::automaton::Automaton`

**Derives**: Clone, Debug

**Methods** (20)

```rust
fn always_match() -> Result<DFA<Vec<u8>>, BuildError>
fn as_ref<'a>(&'a self) -> DFA<&'a [u8]>
fn byte_classes(&self) -> &ByteClasses
fn from_bytes(slice: &'a [u8]) -> Result<(DFA<&'a [u8]>, usize), DeserializeError>
unsafe fn from_bytes_unchecked(slice: &'a [u8]) -> Result<(DFA<&'a [u8]>, usize), DeserializeError>
fn memory_usage(&self) -> usize
fn never_match() -> Result<DFA<Vec<u8>>, BuildError>
fn new(pattern: &str) -> Result<DFA<Vec<u8>>, BuildError>
fn new_many<P: AsRef<str>>(patterns: &[P]) -> Result<DFA<Vec<u8>>, BuildError>
fn set_prefilter(&mut self, prefilter: Option<Prefilter>)
fn start_kind(&self) -> StartKind
fn starts_for_each_pattern(&self) -> bool
fn to_bytes_big_endian(&self) -> Vec<u8>
fn to_bytes_little_endian(&self) -> Vec<u8>
fn to_bytes_native_endian(&self) -> Vec<u8>
fn to_owned(&self) -> DFA<alloc::vec::Vec<u8>>
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
unsafe fn next_state_unchecked(&self, current: StateID, input: u8) -> StateID
fn pattern_len(&self) -> usize
fn start_state(&self, config: &start::Config) -> Result<StateID, StartError>
fn universal_start_state(&self, mode: Anchored) -> Option<StateID>
```

A sparse deterministic finite automaton (DFA) with variable sized states.

In contrast to a [dense::DFA], a sparse DFA uses a more space efficient
representation for its transitions. Consequently, sparse DFAs may use much
less memory than dense DFAs, but this comes at a price. In particular,
reading the more space efficient transitions takes more work, and
consequently, searching using a sparse DFA is typically slower than a dense
DFA.

A sparse DFA can be built using the default configuration via the
[`DFA::new`] constructor. Otherwise, one can configure various aspects of a
dense DFA via [`dense::Builder`], and then convert a dense DFA to a sparse
DFA using [`dense::DFA::to_sparse`].

In general, a sparse DFA supports all the same search operations as a dense
DFA.

Making the choice between a dense and sparse DFA depends on your specific
work load. If you can sacrifice a bit of search time performance, then a
sparse DFA might be the best choice. In particular, while sparse DFAs are
probably always slower than dense DFAs, you may find that they are easily
fast enough for your purposes!

# Type parameters

A `DFA` has one type parameter, `T`, which is used to represent the parts
of a sparse DFA. `T` is typically a `Vec<u8>` or a `&[u8]`.

# The `Automaton` trait

This type implements the [`Automaton`] trait, which means it can be used
for searching. For example:

```
use regex_automata::{dfa::{Automaton, sparse::DFA}, HalfMatch, Input};

let dfa = DFA::new("foo[0-9]+")?;
let expected = Some(HalfMatch::must(0, 8));
assert_eq!(expected, dfa.try_search_fwd(&Input::new("foo12345"))?);
# Ok::<(), Box<dyn std::error::Error>>(())
```

---
