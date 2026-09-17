# `regex_automata::dfa::start`

Crate `regex-automata` · 1 public items · structured records in [`model/regex_automata.dfa.start.json`](../model/regex_automata.dfa.start.json)

## StartKind

`enum` · `regex_automata::dfa::start::StartKind`

Also reachable as `regex_automata::dfa::StartKind`

```rust
enum StartKind
```

**Variants**: `Both`, `Unanchored`, `Anchored`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

The kind of anchored starting configurations to support in a DFA.

Fully compiled DFAs need to be explicitly configured as to which anchored
starting configurations to support. The reason for not just supporting
everything unconditionally is that it can use more resources (such as
memory and build time). The downside of this is that if you try to execute
a search using an [`Anchored`](crate::Anchored) mode that is not supported
by the DFA, then the search will return an error.

---
