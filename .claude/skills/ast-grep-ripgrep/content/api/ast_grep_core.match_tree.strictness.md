# `ast_grep_core::match_tree::strictness`

Crate `ast-grep-core` · 1 public items · structured records in [`model/ast_grep_core.match_tree.strictness.json`](../model/ast_grep_core.match_tree.strictness.json)

## MatchStrictness

`enum` · `ast_grep_core::match_tree::strictness::MatchStrictness`

Also reachable as `ast_grep_core::MatchStrictness`

```rust
enum MatchStrictness
```

**Variants**: `Cst`, `Smart`, `Ast`, `Relaxed`, `Signature`, `Template`

**Implements**: `core::str::traits::FromStr`

**Derives**: Clone

**via `core::str::traits::FromStr`**

```rust
fn from_str(s: &str) -> Result<Self, Self::Err>
```

---
