# `regex_automata::nfa::thompson::error`

Crate `regex-automata` · 1 public items · structured records in [`model/regex_automata.nfa.thompson.error.json`](../model/regex_automata.nfa.thompson.error.json)

## BuildError

`struct` · `regex_automata::nfa::thompson::error::BuildError`

Also reachable as `regex_automata::nfa::thompson::BuildError`

```rust
struct BuildError
```

**Implements**: `core::error::Error`, `core::fmt::Display`

**Derives**: Clone, Debug

**Methods** (1)

```rust
fn size_limit(&self) -> Option<usize>
```

**via `core::error::Error`**

```rust
fn source(&self) -> Option<&dyn std::error::Error + 'static>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
```

An error that can occurred during the construction of a thompson NFA.

This error does not provide many introspection capabilities. There are
generally only two things you can do with it:

* Obtain a human readable message via its `std::fmt::Display` impl.
* Access an underlying [`regex_syntax::Error`] type from its `source`
method via the `std::error::Error` trait. This error only occurs when using
convenience routines for building an NFA directly from a pattern string.

Otherwise, errors typically occur when a limit has been breached. For
example, if the total heap usage of the compiled NFA exceeds the limit
set by [`Config::nfa_size_limit`](crate::nfa::thompson::Config), then
building the NFA will fail.

---
