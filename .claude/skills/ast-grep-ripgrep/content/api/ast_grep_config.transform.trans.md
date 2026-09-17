# `ast_grep_config::transform::trans`

Crate `ast-grep-config` · 4 public items · structured records in [`model/ast_grep_config.transform.trans.json`](../model/ast_grep_config.transform.trans.json)

## Trans

`enum` · `ast_grep_config::transform::trans::Trans`

```rust
enum Trans<T>
```

**Variants**: `Substring`, `Replace`, `Convert`, `Rewrite`

Represents a transformation that can be applied to a matched AST node.
Available transformations are `substring`, `replace` and `convert`.

---

## Convert

`struct` · `ast_grep_config::transform::trans::Convert`

```rust
struct Convert<T>
```

**Fields**: `source`, `to_case`, `separated_by`

Converts the source meta variable's text content to a specified case format.

---

## Replace

`struct` · `ast_grep_config::transform::trans::Replace`

```rust
struct Replace<T>
```

**Fields**: `source`, `replace`, `by`

Replaces a substring in the meta variable's text content with another string.

---

## Substring

`struct` · `ast_grep_config::transform::trans::Substring`

```rust
struct Substring<T>
```

**Fields**: `source`, `start_char`, `end_char`

Extracts a substring from the meta variable's text content.

Both `start_char` and `end_char` support negative indexing,
which counts character from the end of an array, moving backwards.

---
