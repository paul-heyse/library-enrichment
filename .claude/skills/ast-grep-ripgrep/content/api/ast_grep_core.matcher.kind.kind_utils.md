# `ast_grep_core::matcher::kind::kind_utils`

Crate `ast-grep-core` · 2 public items · structured records in [`model/ast_grep_core.matcher.kind.kind_utils.json`](../model/ast_grep_core.matcher.kind.kind_utils.json)

## are_kinds_matching

`function` · `ast_grep_core::matcher::kind::kind_utils::are_kinds_matching`

Also reachable as `ast_grep_core::matcher::kind_utils::are_kinds_matching`

```rust
fn are_kinds_matching(goal: u16, candidate: u16) -> bool
```

---

## is_error_kind

`function` · `ast_grep_core::matcher::kind::kind_utils::is_error_kind`

Also reachable as `ast_grep_core::matcher::kind_utils::is_error_kind`

```rust
fn is_error_kind(kind: u16) -> bool
```

Whether the kind will match parsing error occurred in the source code.
for example, we can use `kind: ERROR` in YAML to find invalid syntax in source.
the name `is_error` implies the matcher itself is error.
But here the matcher itself is valid and it is what it matches is error.

---
