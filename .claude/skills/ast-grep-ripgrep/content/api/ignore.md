# `ignore`

Crate `ignore` · 2 public items · structured records in [`model/ignore.json`](../model/ignore.json)

## Error

`enum` · `ignore::Error`

```rust
enum Error
```

**Variants**: `Partial`, `WithLineNumber`, `WithPath`, `WithDepth`, `Loop`, `Io`, `Glob`, `UnrecognizedFileType`, `InvalidDefinition`

**Implements**: `core::convert::From`, `core::error::Error`, `core::fmt::Display`

**Derives**: Clone, Debug

**Methods** (5)

```rust
fn depth(&self) -> Option<usize>
fn into_io_error(self) -> Option<std::io::Error>
fn io_error(&self) -> Option<&std::io::Error>
fn is_io(&self) -> bool
fn is_partial(&self) -> bool
```

**via `core::convert::From`**

```rust
fn from(err: std::io::Error) -> Error
```

**via `core::error::Error`**

```rust
fn description(&self) -> &str
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Represents an error that can occur when parsing a gitignore file.

---

## Match

`enum` · `ignore::Match`

```rust
enum Match<T>
```

**Variants**: `None`, `Ignore`, `Whitelist`

**Derives**: Clone, Debug

**Methods** (7)

```rust
fn inner(&self) -> Option<&T>
fn invert(self) -> Match<T>
fn is_ignore(&self) -> bool
fn is_none(&self) -> bool
fn is_whitelist(&self) -> bool
fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Match<U>
fn or(self, other: Self) -> Self
```

The result of a glob match.

The type parameter `T` typically refers to a type that provides more
information about a particular match. For example, it might identify
the specific gitignore file and the specific glob pattern that caused
the match.

---
