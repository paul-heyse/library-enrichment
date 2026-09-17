# `ast_grep_config::rule::nth_child`

Crate `ast-grep-config` · 4 public items · structured records in [`model/ast_grep_config.rule.nth_child.json`](../model/ast_grep_config.rule.nth_child.json)

## NthChildError

`enum` · `ast_grep_config::rule::nth_child::NthChildError`

```rust
enum NthChildError
```

**Variants**: `IllegalCharacter`, `InvalidSyntax`, `InvalidRule`

---

## NthChildSimple

`enum` · `ast_grep_config::rule::nth_child::NthChildSimple`

```rust
enum NthChildSimple
```

**Variants**: `Numeric`, `Functional`

A string or number describing the indices of matching nodes in a list of siblings.

---

## SerializableNthChild

`enum` · `ast_grep_config::rule::nth_child::SerializableNthChild`

```rust
enum SerializableNthChild
```

**Variants**: `Simple`, `Complex`

`nthChild` accepts either a number, a string or an object.

---

## NthChild

`struct` · `ast_grep_config::rule::nth_child::NthChild`

```rust
struct NthChild
```

---
