# Using these as libraries

**I want this behaviour inside my own program.**

Both tools are thin shells over library crates that can be used directly, and both ship bindings for other languages.

ripgrep's search pipeline is `grep-matcher` (the matcher interface), `grep-regex` and `grep-pcre2` (the two engines), `grep-searcher` (the line-oriented reader), `grep-printer` (output), with `ignore` and `globset` supplying the file-set machinery that is often the only part worth borrowing. Those crates are indexed here at the versions ripgrep 15.2.0's `Cargo.lock` actually pins -- `ignore` 0.4.29, not the newer release on crates.io -- because that is the code whose behaviour the probes observed.

ast-grep exposes `ast-grep-core` and `ast-grep-config` for Rust, plus `@ast-grep/napi` for Node and `ast-grep-py` for Python. Their shipped type declarations are indexed alongside the Rust crates.

The usual reason to embed is needing to interleave your own logic with matching. The usual reason not to is that the CLI is a stable process boundary and the library API is not.

## Decision rules

- Borrow `ignore` and `globset` alone if the file-set semantics are what you want. They are the reusable part.
- Shell out to the CLI for a stable boundary; embed when per-match host computation is needed.
- Pin bindings independently of the CLI. They release on their own schedule.
- Read the crate versions from `PROVENANCE.json`; they are lock-resolved, not latest.

## Anti-patterns

- Reimplementing gitignore semantics instead of using `ignore`.
- Assuming the binding version tracks the CLI version.
- Indexing crates.io latest and expecting it to describe the installed binary.

## Checklist

- Is the CLI boundary really insufficient?
- Are the crate versions the lock-resolved ones?
- Is the binding pinned separately?
