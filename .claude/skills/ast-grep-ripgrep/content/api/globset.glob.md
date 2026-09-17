# `globset::glob`

Crate `globset` · 3 public items · structured records in [`model/globset.glob.json`](../model/globset.glob.json)

## Glob

`struct` · `globset::glob::Glob`

Also reachable as `globset::Glob`

```rust
struct Glob
```

**Implements**: `core::convert::AsRef`, `core::fmt::Display`, `core::str::traits::FromStr`

**Derives**: Clone, Debug, Eq, Hash, PartialEq

**Methods** (4)

```rust
fn compile_matcher(&self) -> GlobMatcher
fn glob(&self) -> &str
fn new(glob: &str) -> Result<Glob, Error>
fn regex(&self) -> &str
```

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &Glob
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(glob: &str) -> Result<Self, Self::Err>
```

Glob represents a successfully parsed shell glob pattern.

It cannot be used directly to match file paths, but it can be converted
to a regular expression string or a matcher.

---

## GlobBuilder

`struct` · `globset::glob::GlobBuilder`

Also reachable as `globset::GlobBuilder`

```rust
struct GlobBuilder<'a>
```

**Derives**: Clone, Debug

**Methods** (7)

```rust
fn allow_unclosed_class(&mut self, yes: bool) -> &mut GlobBuilder<'a>
fn backslash_escape(&mut self, yes: bool) -> &mut GlobBuilder<'a>
fn build(&self) -> Result<Glob, Error>
fn case_insensitive(&mut self, yes: bool) -> &mut GlobBuilder<'a>
fn empty_alternates(&mut self, yes: bool) -> &mut GlobBuilder<'a>
fn literal_separator(&mut self, yes: bool) -> &mut GlobBuilder<'a>
fn new(glob: &'a str) -> GlobBuilder<'a>
```

A builder for a pattern.

This builder enables configuring the match semantics of a pattern. For
example, one can make matching case insensitive.

The lifetime `'a` refers to the lifetime of the pattern string.

---

## GlobMatcher

`struct` · `globset::glob::GlobMatcher`

Also reachable as `globset::GlobMatcher`

```rust
struct GlobMatcher
```

**Derives**: Clone, Debug

**Methods** (3)

```rust
fn glob(&self) -> &Glob
fn is_match<P: AsRef<Path>>(&self, path: P) -> bool
fn is_match_candidate(&self, path: &Candidate<'_>) -> bool
```

A matcher for a single pattern.

---
