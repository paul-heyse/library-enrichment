# `ignore::overrides`

Crate `ignore` · 3 public items · structured records in [`model/ignore.overrides.json`](../model/ignore.overrides.json)

## Glob

`struct` · `ignore::overrides::Glob`

```rust
struct Glob<'a>
```

**Derives**: Clone, Debug

Glob represents a single glob in an override matcher.

This is used to report information about the highest precedent glob
that matched.

Note that not all matches necessarily correspond to a specific glob. For
example, if there are one or more whitelist globs and a file path doesn't
match any glob in the set, then the file path is considered to be ignored.

The lifetime `'a` refers to the lifetime of the matcher that produced
this glob.

---

## Override

`struct` · `ignore::overrides::Override`

```rust
struct Override
```

**Derives**: Clone, Debug

**Methods** (6)

```rust
fn empty() -> Override
fn is_empty(&self) -> bool
fn matched<'a, P: AsRef<Path>>(&'a self, path: P, is_dir: bool) -> Match<Glob<'a>>
fn num_ignores(&self) -> u64
fn num_whitelists(&self) -> u64
fn path(&self) -> &Path
```

Manages a set of overrides provided explicitly by the end user.

---

## OverrideBuilder

`struct` · `ignore::overrides::OverrideBuilder`

```rust
struct OverrideBuilder
```

**Derives**: Clone, Debug

**Methods** (5)

```rust
fn add(&mut self, glob: &str) -> Result<&mut OverrideBuilder, Error>
fn allow_unclosed_class(&mut self, yes: bool) -> &mut OverrideBuilder
fn build(&self) -> Result<Override, Error>
fn case_insensitive(&mut self, yes: bool) -> Result<&mut OverrideBuilder, Error>
fn new<P: AsRef<Path>>(path: P) -> OverrideBuilder
```

Builds a matcher for a set of glob overrides.

---
