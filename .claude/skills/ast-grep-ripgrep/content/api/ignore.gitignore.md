# `ignore::gitignore`

Crate `ignore` · 4 public items · structured records in [`model/ignore.gitignore.json`](../model/ignore.gitignore.json)

## gitconfig_excludes_path

`function` · `ignore::gitignore::gitconfig_excludes_path`

```rust
fn gitconfig_excludes_path() -> Option<std::path::PathBuf>
```

Return the file path of the current environment's global gitignore file.

Note that the file path returned may not exist.

---

## Gitignore

`struct` · `ignore::gitignore::Gitignore`

```rust
struct Gitignore
```

**Derives**: Clone, Debug

**Methods** (10)

```rust
fn empty() -> Gitignore
fn global() -> (Gitignore, Option<Error>)
fn is_empty(&self) -> bool
fn len(&self) -> usize
fn matched<P: AsRef<Path>>(&self, path: P, is_dir: bool) -> Match<&Glob>
fn matched_path_or_any_parents<P: AsRef<Path>>(&self, path: P, is_dir: bool) -> Match<&Glob>
fn new<P: AsRef<Path>>(gitignore_path: P) -> (Gitignore, Option<Error>)
fn num_ignores(&self) -> u64
fn num_whitelists(&self) -> u64
fn path(&self) -> &Path
```

Gitignore is a matcher for the globs in one or more gitignore files
in the same directory.

---

## GitignoreBuilder

`struct` · `ignore::gitignore::GitignoreBuilder`

```rust
struct GitignoreBuilder
```

**Derives**: Clone, Debug

**Methods** (7)

```rust
fn add<P: AsRef<Path>>(&mut self, path: P) -> Option<Error>
fn add_line(&mut self, from: Option<PathBuf>, line: &str) -> Result<&mut GitignoreBuilder, Error>
fn allow_unclosed_class(&mut self, yes: bool) -> &mut GitignoreBuilder
fn build(&self) -> Result<Gitignore, Error>
fn build_global(self) -> (Gitignore, Option<Error>)
fn case_insensitive(&mut self, yes: bool) -> Result<&mut GitignoreBuilder, Error>
fn new<P: AsRef<Path>>(root: P) -> GitignoreBuilder
```

Builds a matcher for a single set of globs from a .gitignore file.

---

## Glob

`struct` · `ignore::gitignore::Glob`

```rust
struct Glob
```

**Derives**: Clone, Debug

**Methods** (5)

```rust
fn actual(&self) -> &str
fn from(&self) -> Option<&Path>
fn is_only_dir(&self) -> bool
fn is_whitelist(&self) -> bool
fn original(&self) -> &str
```

Glob represents a single glob in a gitignore file.

This is used to report information about the highest precedent glob that
matched in one or more gitignore files.

---
