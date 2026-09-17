# `globset`

Crate `globset` · 6 public items · structured records in [`model/globset.json`](../model/globset.json)

## ErrorKind

`enum` · `globset::ErrorKind`

```rust
enum ErrorKind
```

**Variants**: `InvalidRecursive`, `UnclosedClass`, `InvalidRange`, `UnopenedAlternates`, `UnclosedAlternates`, `NestedAlternates`, `DanglingEscape`, `Regex`

**Implements**: `core::fmt::Display`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

The kind of error that can occur when parsing a glob pattern.

---

## escape

`function` · `globset::escape`

```rust
fn escape(s: &str) -> String
```

Escape meta-characters within the given glob pattern.

The escaping works by surrounding meta-characters with brackets. For
example, `*` becomes `[*]`.

# Example

```
use globset::escape;

assert_eq!(escape("foo*bar"), "foo[*]bar");
assert_eq!(escape("foo?bar"), "foo[?]bar");
assert_eq!(escape("foo[bar"), "foo[[]bar");
assert_eq!(escape("foo]bar"), "foo[]]bar");
assert_eq!(escape("foo{bar"), "foo[{]bar");
assert_eq!(escape("foo}bar"), "foo[}]bar");
```

---

## Candidate

`struct` · `globset::Candidate`

```rust
struct Candidate<'a>
```

**Derives**: Clone, Debug

**Methods** (2)

```rust
fn from_bytes<P: AsRef<[u8]> + ?Sized>(path: &'a P) -> Candidate<'a>
fn new<P: AsRef<Path> + ?Sized>(path: &'a P) -> Candidate<'a>
```

A candidate path for matching.

All glob matching in this crate operates on `Candidate` values.
Constructing candidates has a very small cost associated with it, so
callers may find it beneficial to amortize that cost when matching a single
path against multiple globs or sets of globs.

---

## Error

`struct` · `globset::Error`

```rust
struct Error
```

**Implements**: `core::error::Error`, `core::fmt::Display`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn glob(&self) -> Option<&str>
fn kind(&self) -> &ErrorKind
```

**via `core::error::Error`**

```rust
fn description(&self) -> &str
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Represents an error that can occur when parsing a glob pattern.

---

## GlobSet

`struct` · `globset::GlobSet`

```rust
struct GlobSet
```

**Derives**: Clone, Debug, Default

**Methods** (13)

```rust
fn builder() -> GlobSetBuilder
const fn empty() -> GlobSet
fn is_empty(&self) -> bool
fn is_match<P: AsRef<Path>>(&self, path: P) -> bool
fn is_match_candidate(&self, path: &Candidate<'_>) -> bool
fn len(&self) -> usize
fn matches<P: AsRef<Path>>(&self, path: P) -> Vec<usize>
fn matches_all<P: AsRef<Path>>(&self, path: P) -> bool
fn matches_all_candidate(&self, path: &Candidate<'_>) -> bool
fn matches_candidate(&self, path: &Candidate<'_>) -> Vec<usize>
fn matches_candidate_into(&self, path: &Candidate<'_>, into: &mut Vec<usize>)
fn matches_into<P: AsRef<Path>>(&self, path: P, into: &mut Vec<usize>)
fn new<I, G>(globs: I) -> Result<GlobSet, Error> where I: IntoIterator<Item = G>, G: AsRef<Glob>
```

GlobSet represents a group of globs that can be matched together in a
single pass.

---

## GlobSetBuilder

`struct` · `globset::GlobSetBuilder`

```rust
struct GlobSetBuilder
```

**Derives**: Clone, Debug

**Methods** (3)

```rust
fn add(&mut self, pat: Glob) -> &mut GlobSetBuilder
fn build(&self) -> Result<GlobSet, Error>
fn new() -> GlobSetBuilder
```

GlobSetBuilder builds a group of patterns that can be used to
simultaneously match a file path.

---
