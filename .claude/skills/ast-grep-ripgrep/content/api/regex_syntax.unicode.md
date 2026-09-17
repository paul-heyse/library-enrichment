# `regex_syntax::unicode`

Crate `regex-syntax` · 2 public items · structured records in [`model/regex_syntax.unicode.json`](../model/regex_syntax.unicode.json)

## CaseFoldError

`struct` · `regex_syntax::unicode::CaseFoldError`

Also reachable as `regex_syntax::hir::CaseFoldError`

```rust
struct CaseFoldError
```

**Implements**: `core::error::Error`, `core::fmt::Display`

**Derives**: Debug

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
```

An error that occurs when Unicode-aware simple case folding fails.

This error can occur when the case mapping tables necessary for Unicode
aware case folding are unavailable. This only occurs when the
`unicode-case` feature is disabled. (The feature is enabled by default.)

---

## UnicodeWordError

`struct` · `regex_syntax::unicode::UnicodeWordError`

Also reachable as `regex_syntax::UnicodeWordError`

```rust
struct UnicodeWordError
```

**Implements**: `core::error::Error`, `core::fmt::Display`

**Derives**: Debug

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
```

An error that occurs when the Unicode-aware `\w` class is unavailable.

This error can occur when the data tables necessary for the Unicode aware
Perl character class `\w` are unavailable. This only occurs when the
`unicode-perl` feature is disabled. (The feature is enabled by default.)

---
