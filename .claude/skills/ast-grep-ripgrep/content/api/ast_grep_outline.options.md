# `ast_grep_outline::options`

Crate `ast-grep-outline` · 4 public items · structured records in [`model/ast_grep_outline.options.json`](../model/ast_grep_outline.options.json)

## OutlineEntryDetail

`enum` · `ast_grep_outline::options::OutlineEntryDetail`

```rust
enum OutlineEntryDetail
```

**Variants**: `Name`, `Signature`

**Derives**: Clone, Copy, Debug

How much text to compute for each returned entry.

---

## OutlineFlagFilter

`enum` · `ast_grep_outline::options::OutlineFlagFilter`

```rust
enum OutlineFlagFilter
```

**Variants**: `Any`, `Yes`, `No`

**Derives**: Clone, Copy, Debug, Default

Ternary filter for flags derived from literals or runtime predicates.

---

## OutlineExtractorOptions

`struct` · `ast_grep_outline::options::OutlineExtractorOptions`

```rust
struct OutlineExtractorOptions
```

**Fields**: `symbol_types`, `item_regex`, `imports`, `exported`, `detail`, `members`

**Derives**: Clone, Debug, Default

**Methods** (3)

```rust
fn keep_item(&self, item: &OutlineItem<'_>) -> bool
fn keep_member(&self, member: &OutlineMember<'_>) -> bool
fn retain_rule<L>(&self, rule: &SerializableOutlineRule<L>) -> bool
```

Options for compiling and extracting an outline.

---

## OutlineMemberOptions

`struct` · `ast_grep_outline::options::OutlineMemberOptions`

```rust
struct OutlineMemberOptions
```

**Fields**: `public`, `detail`

**Derives**: Clone, Debug, Default

Options that apply to direct item members.

---
