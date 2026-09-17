# `ast_grep_config::rule::range`

Crate `ast-grep-config` · 4 public items · structured records in [`model/ast_grep_config.rule.range.json`](../model/ast_grep_config.rule.range.json)

## RangeMatcherError

`enum` · `ast_grep_config::rule::range::RangeMatcherError`

```rust
enum RangeMatcherError
```

**Variants**: `InvalidRange`

Errors that can occur when creating or using a RangeMatcher

---

## RangeMatcher

`struct` · `ast_grep_config::rule::range::RangeMatcher`

```rust
struct RangeMatcher
```

---

## SerializablePosition

`struct` · `ast_grep_config::rule::range::SerializablePosition`

```rust
struct SerializablePosition
```

**Fields**: `line`, `column`

Represents a zero-based character-wise position in a document

---

## SerializableRange

`struct` · `ast_grep_config::rule::range::SerializableRange`

```rust
struct SerializableRange
```

**Fields**: `start`, `end`

Represents a position in source code using 0-based line and column numbers

---
