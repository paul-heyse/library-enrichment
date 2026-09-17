# `aho_corasick::util::error`

Crate `aho-corasick` · 3 public items · structured records in [`model/aho_corasick.util.error.json`](../model/aho_corasick.util.error.json)

## MatchErrorKind

`enum` · `aho_corasick::util::error::MatchErrorKind`

Also reachable as `aho_corasick::MatchErrorKind`

```rust
enum MatchErrorKind
```

**Variants**: `InvalidInputAnchored`, `InvalidInputUnanchored`, `UnsupportedStream`, `UnsupportedOverlapping`, `UnsupportedEmpty`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

The underlying kind of a [`MatchError`].

This is a **non-exhaustive** enum. That means new variants may be added in
a semver-compatible release.

---

## BuildError

`struct` · `aho_corasick::util::error::BuildError`

Also reachable as `aho_corasick::BuildError`

```rust
struct BuildError
```

**Implements**: `core::error::Error`, `core::fmt::Display`

**Derives**: Clone, Debug

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
```

An error that occurred during the construction of an Aho-Corasick
automaton.

Build errors occur when some kind of limit has been exceeded, either in the
number of states, the number of patterns of the length of a pattern. These
limits aren't part of the public API, but they should generally be large
enough to handle most use cases.

When the `std` feature is enabled, this implements the `std::error::Error`
trait.

---

## MatchError

`struct` · `aho_corasick::util::error::MatchError`

Also reachable as `aho_corasick::MatchError`

```rust
struct MatchError
```

**Implements**: `core::error::Error`, `core::fmt::Display`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (7)

```rust
fn invalid_input_anchored() -> MatchError
fn invalid_input_unanchored() -> MatchError
fn kind(&self) -> &MatchErrorKind
fn new(kind: MatchErrorKind) -> MatchError
fn unsupported_empty() -> MatchError
fn unsupported_overlapping(got: MatchKind) -> MatchError
fn unsupported_stream(got: MatchKind) -> MatchError
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
```

An error that occurred during an Aho-Corasick search.

An error that occurs during a search is limited to some kind of
misconfiguration that resulted in an illegal call. Stated differently,
whether an error occurs is not dependent on the specific bytes in the
haystack.

Examples of misconfiguration:

* Executing a stream or overlapping search on a searcher that was built was
something other than [`MatchKind::Standard`](crate::MatchKind::Standard)
semantics.
* Requested an anchored or an unanchored search on a searcher that doesn't
support unanchored or anchored searches, respectively.

When the `std` feature is enabled, this implements the `std::error::Error`
trait.

---
