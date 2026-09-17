# `aho_corasick::packed::api`

Crate `aho-corasick` · 5 public items · structured records in [`model/aho_corasick.packed.api.json`](../model/aho_corasick.packed.api.json)

## MatchKind

`enum` · `aho_corasick::packed::api::MatchKind`

Also reachable as `aho_corasick::packed::MatchKind`

```rust
enum MatchKind
```

**Variants**: `LeftmostFirst`, `LeftmostLongest`

**Derives**: Clone, Copy, Debug, Default, Eq, PartialEq, StructuralPartialEq

A knob for controlling the match semantics of a packed multiple string
searcher.

This differs from the [`MatchKind`](crate::MatchKind) type in the top-level
crate module in that it doesn't support "standard" match semantics,
and instead only supports leftmost-first or leftmost-longest. Namely,
"standard" semantics cannot be easily supported by packed searchers.

For more information on the distinction between leftmost-first and
leftmost-longest, see the docs on the top-level `MatchKind` type.

Unlike the top-level `MatchKind` type, the default match semantics for this
type are leftmost-first.

---

## Builder

`struct` · `aho_corasick::packed::api::Builder`

Also reachable as `aho_corasick::packed::Builder`

```rust
struct Builder
```

**Derives**: Clone, Debug, Default

**Methods** (6)

```rust
fn add<P: AsRef<[u8]>>(&mut self, pattern: P) -> &mut Builder
fn build(&self) -> Option<Searcher>
fn extend<I, P>(&mut self, patterns: I) -> &mut Builder where I: IntoIterator<Item = P>, P: AsRef<[u8]>
fn len(&self) -> usize
fn minimum_len(&self) -> usize
fn new() -> Builder
```

A builder for constructing a packed searcher from a collection of patterns.

# Example

This example shows how to use a builder to construct a searcher. By
default, leftmost-first match semantics are used.

```
use aho_corasick::{packed::{Builder, MatchKind}, PatternID};

# fn example() -> Option<()> {
let searcher = Builder::new()
    .add("foobar")
    .add("foo")
    .build()?;
let matches: Vec<PatternID> = searcher
    .find_iter("foobar")
    .map(|mat| mat.pattern())
    .collect();
assert_eq!(vec![PatternID::ZERO], matches);
# Some(()) }
# if cfg!(all(feature = "std", any(
#     target_arch = "x86_64", target_arch = "aarch64",
# ))) {
#     example().unwrap()
# } else {
#     assert!(example().is_none());
# }
```

---

## Config

`struct` · `aho_corasick::packed::api::Config`

Also reachable as `aho_corasick::packed::Config`

```rust
struct Config
```

**Derives**: Clone, Debug, Default

**Methods** (4)

```rust
fn builder(&self) -> Builder
fn heuristic_pattern_limits(&mut self, yes: bool) -> &mut Config
fn match_kind(&mut self, kind: MatchKind) -> &mut Config
fn new() -> Config
```

The configuration for a packed multiple pattern searcher.

The configuration is currently limited only to being able to select the
match semantics (leftmost-first or leftmost-longest) of a searcher. In the
future, more knobs may be made available.

A configuration produces a [`packed::Builder`](Builder), which in turn can
be used to construct a [`packed::Searcher`](Searcher) for searching.

# Example

This example shows how to use leftmost-longest semantics instead of the
default (leftmost-first).

```
use aho_corasick::{packed::{Config, MatchKind}, PatternID};

# fn example() -> Option<()> {
let searcher = Config::new()
    .match_kind(MatchKind::LeftmostLongest)
    .builder()
    .add("foo")
    .add("foobar")
    .build()?;
let matches: Vec<PatternID> = searcher
    .find_iter("foobar")
    .map(|mat| mat.pattern())
    .collect();
assert_eq!(vec![PatternID::must(1)], matches);
# Some(()) }
# if cfg!(all(feature = "std", any(
#     target_arch = "x86_64", target_arch = "aarch64",
# ))) {
#     example().unwrap()
# } else {
#     assert!(example().is_none());
# }
```

---

## FindIter

`struct` · `aho_corasick::packed::api::FindIter`

Also reachable as `aho_corasick::packed::FindIter`

```rust
struct FindIter<'s, 'h>
```

**Implements**: `core::iter::traits::iterator::Iterator`

**Derives**: Debug

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<Match>
```

An iterator over non-overlapping matches from a packed searcher.

The lifetime `'s` refers to the lifetime of the underlying [`Searcher`],
while the lifetime `'h` refers to the lifetime of the haystack being
searched.

---

## Searcher

`struct` · `aho_corasick::packed::api::Searcher`

Also reachable as `aho_corasick::packed::Searcher`

```rust
struct Searcher
```

**Derives**: Clone, Debug

**Methods** (9)

```rust
fn builder() -> Builder
fn config() -> Config
fn find<B: AsRef<[u8]>>(&self, haystack: B) -> Option<Match>
fn find_in<B: AsRef<[u8]>>(&self, haystack: B, span: Span) -> Option<Match>
fn find_iter<'a, 'b, B: ?Sized + AsRef<[u8]>>(&'a self, haystack: &'b B) -> FindIter<'a, 'b>
fn match_kind(&self) -> &MatchKind
fn memory_usage(&self) -> usize
fn minimum_len(&self) -> usize
fn new<I, P>(patterns: I) -> Option<Searcher> where I: IntoIterator<Item = P>, P: AsRef<[u8]>
```

A packed searcher for quickly finding occurrences of multiple patterns.

If callers need more flexible construction, or if one wants to change the
match semantics (either leftmost-first or leftmost-longest), then one can
use the [`Config`] and/or [`Builder`] types for more fine grained control.

# Example

This example shows how to create a searcher from an iterator of patterns.
By default, leftmost-first match semantics are used.

```
use aho_corasick::{packed::{MatchKind, Searcher}, PatternID};

# fn example() -> Option<()> {
let searcher = Searcher::new(["foobar", "foo"].iter().cloned())?;
let matches: Vec<PatternID> = searcher
    .find_iter("foobar")
    .map(|mat| mat.pattern())
    .collect();
assert_eq!(vec![PatternID::ZERO], matches);
# Some(()) }
# if cfg!(all(feature = "std", any(
#     target_arch = "x86_64", target_arch = "aarch64",
# ))) {
#     example().unwrap()
# } else {
#     assert!(example().is_none());
# }
```

---
