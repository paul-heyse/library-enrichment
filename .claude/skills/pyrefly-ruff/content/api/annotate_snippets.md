# `annotate_snippets`

Crate `ruff_annotate_snippets` · 1 public items · structured records in [`model/annotate_snippets.json`](../model/annotate_snippets.json)

## normalize_untrusted_str

`function` · `annotate_snippets::normalize_untrusted_str`

Also reachable as `ruff_annotate_snippets::normalize_untrusted_str`

```rust
fn normalize_untrusted_str(s: &str) -> alloc::string::String
```

Normalize the string to avoid any unicode control characters.

This is important for untrusted input, as it can contain
invalid unicode sequences.

---
