# `ruff_python_trivia::pragmas`

Crate `ruff_python_trivia` · 2 public items · structured records in [`model/ruff_python_trivia.pragmas.json`](../model/ruff_python_trivia.pragmas.json)

## find_trailing_pragma_offset

`function` · `ruff_python_trivia::pragmas::find_trailing_pragma_offset`

Also reachable as `ruff_python_trivia::find_trailing_pragma_offset`

```rust
fn find_trailing_pragma_offset(comment: &str) -> Option<usize>
```

Returns the byte offset within `comment` where a trailing pragma comment starts,
or `None` if no pragma is found.

For a plain pragma like `# noqa: F401`, returns `Some(0)`.
For a nested pragma like `# some text # noqa: F401`, returns the offset of the
trailing `#` that begins the pragma (i.e., the start of `# noqa: F401`).

```
assert_eq!(ruff_python_trivia::find_trailing_pragma_offset("# noqa: F401"), Some(0));
assert_eq!(ruff_python_trivia::find_trailing_pragma_offset("# type: ignore"), Some(0));
assert_eq!(ruff_python_trivia::find_trailing_pragma_offset("# some comment # noqa: F401"), Some(15));
assert_eq!(ruff_python_trivia::find_trailing_pragma_offset("## noqa: F401"), Some(1));
assert_eq!(ruff_python_trivia::find_trailing_pragma_offset("# just a comment"), None);
```

---

## is_pragma_comment

`function` · `ruff_python_trivia::pragmas::is_pragma_comment`

Also reachable as `ruff_python_trivia::is_pragma_comment`

```rust
fn is_pragma_comment(comment: &str) -> bool
```

Returns `true` if a comment appears to be a pragma comment.

```
assert!(ruff_python_trivia::is_pragma_comment("# type: ignore"));
assert!(ruff_python_trivia::is_pragma_comment("# noqa: F401"));
assert!(ruff_python_trivia::is_pragma_comment("# noqa"));
assert!(ruff_python_trivia::is_pragma_comment("# NoQA"));
assert!(ruff_python_trivia::is_pragma_comment("# nosec"));
assert!(ruff_python_trivia::is_pragma_comment("# nosec B602, B607"));
assert!(ruff_python_trivia::is_pragma_comment("# isort: off"));
assert!(ruff_python_trivia::is_pragma_comment("# isort: skip"));
assert!(ruff_python_trivia::is_pragma_comment("# pyrefly: ignore[missing-attribute]"));
```

---
