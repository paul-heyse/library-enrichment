# `ruff_linter::rules::pycodestyle::rules::errors`

Crate `ruff_linter` · 1 public items · structured records in [`model/ruff_linter.rules.pycodestyle.rules.errors.json`](../model/ruff_linter.rules.pycodestyle.rules.errors.json)

## IOError

`struct` · `ruff_linter::rules::pycodestyle::rules::errors::IOError`

Also reachable as `ruff_linter::IOError`

```rust
struct IOError
```

**Fields**: `message`

**Implements**: `ruff_linter::violation::Violation`, `ruff_linter::violation::ViolationMetadata`

**via `ruff_linter::violation::Violation`**

```rust
fn message(&self) -> String
fn message_formats() -> &'static [&'static str]
```

**via `ruff_linter::violation::ViolationMetadata`**

```rust
fn category() -> codes::Category
fn explain() -> Option<&'static str>
fn file() -> &'static str
fn line() -> u32
fn rule() -> registry::Rule
fn status() -> codes::RuleStatus
```

## What it does
This is not a regular diagnostic; instead, it's raised when a file cannot be read
from disk.

## Why is this bad?
An `IOError` indicates an error in the development setup. For example, the user may
not have permissions to read a given file, or the filesystem may contain a broken
symlink.

## Example
On Linux or macOS:
```shell
$ echo 'print("hello world!")' > a.py
$ chmod 000 a.py
$ ruff a.py
a.py:1:1: E902 Permission denied (os error 13)
Found 1 error.
```

## References
- [UNIX Permissions introduction](https://mason.gmu.edu/~montecin/UNIXpermiss.htm)
- [Command Line Basics: Symbolic Links](https://www.digitalocean.com/community/tutorials/workflow-symbolic-links)

---
