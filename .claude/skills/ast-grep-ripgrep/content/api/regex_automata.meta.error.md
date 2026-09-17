# `regex_automata::meta::error`

Crate `regex-automata` · 1 public items · structured records in [`model/regex_automata.meta.error.json`](../model/regex_automata.meta.error.json)

## BuildError

`struct` · `regex_automata::meta::error::BuildError`

Also reachable as `regex_automata::meta::BuildError`

```rust
struct BuildError
```

**Implements**: `core::error::Error`, `core::fmt::Display`

**Derives**: Clone, Debug

**Methods** (3)

```rust
fn pattern(&self) -> Option<PatternID>
fn size_limit(&self) -> Option<usize>
fn syntax_error(&self) -> Option<&regex_syntax::Error>
```

**via `core::error::Error`**

```rust
fn source(&self) -> Option<&dyn std::error::Error + 'static>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
```

An error that occurs when construction of a `Regex` fails.

A build error is generally a result of one of two possible failure
modes. First is a parse or syntax error in the concrete syntax of a
pattern. Second is that the construction of the underlying regex matcher
fails, usually because it gets too big with respect to limits like
[`Config::nfa_size_limit`](crate::meta::Config::nfa_size_limit).

This error provides very little introspection capabilities. You can:

* Ask for the [`PatternID`] of the pattern that caused an error, if one
is available. This is available for things like syntax errors, but not for
cases where build limits are exceeded.
* Ask for the underlying syntax error, but only if the error is a syntax
error.
* Ask for a human readable message corresponding to the underlying error.
* The `BuildError::source` method (from the `std::error::Error`
trait implementation) may be used to query for an underlying error if one
exists. There are no API guarantees about which error is returned.

When the `std` feature is enabled, this implements `std::error::Error`.

---
